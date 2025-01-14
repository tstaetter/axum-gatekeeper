use crate::error::{GateKeeperError, TokenError};
use crate::ErrorResponse;
use crate::GateKeeperResult;

pub struct TokenService;

impl TokenService {
    pub fn get_token_headers_from_encoded(
        encoded: String,
    ) -> GateKeeperResult<jsonwebtoken::Header> {
        jsonwebtoken::decode_header(&encoded).map_err(|e| {
            tracing::error!("Error decoding token header: {}", e);

            let response = ErrorResponse::build()
                .message("Error decoding token headers".to_string())
                .build();

            GateKeeperError::Token(TokenError::DecodeHeader(response))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::TokenService;
    use crate::tokens::{Claims, Token};

    struct TestToken {
        encoded: String,
        claims: Claims,
    }

    impl Token for TestToken {
        fn new(encoded: String, claims: Claims) -> Self
        where
            Self: Sized,
        {
            Self { encoded, claims }
        }

        fn get_claims(&self) -> &Claims {
            &self.claims
        }

        fn get_encoded(&self) -> &String {
            &self.encoded
        }
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
        let encoded = TestToken::encode(&claims, secret.to_string())?;
        let headers = TokenService::get_token_headers_from_encoded(encoded)?;

        assert_eq!(uuid.to_string(), headers.kid.unwrap());

        Ok(())
    }
}
