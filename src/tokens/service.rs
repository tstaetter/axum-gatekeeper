use crate::db::AppDatabase;
use crate::error::{AppError, GateKeeperError, TokenError};
use crate::model::GateKeeperModel;
use crate::models::User;
use crate::tokens::TokenErrorResponse;
use crate::{AppResult, GateKeeperResult};

pub struct TokenService;

impl TokenService {
    pub fn get_token_headers_from_encoded(
        encoded: String,
    ) -> GateKeeperResult<jsonwebtoken::Header> {
        jsonwebtoken::decode_header(&encoded).map_err(|e| {
            tracing::error!("Error decoding token header: {}", e);

            let response = TokenErrorResponse::build()
                .message("Error decoding token headers".to_string())
                .build();

            GateKeeperError::Token(TokenError::DecodeHeader(response))
        })
    }

    pub async fn try_get_user_for_encoded(
        encoded: String,
        db: &AppDatabase,
    ) -> GateKeeperResult<impl GateKeeperModel> {
        let kid = Self::get_token_headers_from_encoded(encoded)?
            .kid
            .unwrap_or_default();
        let kid = uuid::Uuid::parse_str(kid.as_str())?;
        let user = db.find_one::<User, uuid::Uuid>("ext_id", kid).await?;

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use crate::db::AppDatabase;
    use crate::models::user::tests::valid_test_user;
    use crate::tokens::{AuthenticationToken, Claims, RefreshToken, Token, TokenService};

    #[tokio::test]
    async fn test_try_get_user_for_encoded() -> anyhow::Result<()> {
        dotenv::dotenv().ok();

        let db = AppDatabase::try_new().await?;
        let user = valid_test_user()?;
        let user = db.insert_one(&user).await?;
        let token = AuthenticationToken::try_new_for_user(&user)?;
        let svc_user =
            TokenService::try_get_user_for_encoded(token.get_encoded().to_string(), &db).await?;

        assert_eq!(user.id.unwrap(), svc_user.id.unwrap());

        Ok(())
    }

    #[tokio::test]
    async fn test_get_token_headers_from_encoded() -> anyhow::Result<()> {
        let uuid = uuid::Uuid::new_v4();
        let now = chrono::Utc::now().timestamp() as usize;
        let claims = Claims {
            exp: now + 1000,
            iat: now,
            sub: uuid.to_string(),
        };
        let secret = "test";
        let encoded = RefreshToken::encode(&claims, secret.to_string())?;
        let headers = TokenService::get_token_headers_from_encoded(encoded)?;

        assert_eq!(uuid.to_string(), headers.kid.unwrap());

        Ok(())
    }
}
