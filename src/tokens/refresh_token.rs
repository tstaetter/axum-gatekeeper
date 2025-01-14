use crate::tokens::{Claims, Token};
use crate::GateKeeperResult;
use cookie::time::OffsetDateTime;
use cookie::Cookie;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct RefreshToken {
    encoded: String,
    claims: Claims,
}

impl RefreshToken {
    pub fn try_as_cookie(&self) -> GateKeeperResult<Cookie> {
        Ok(Cookie::build(("refresh_token", &self.encoded))
            .path("/")
            .expires(OffsetDateTime::from_unix_timestamp(self.claims.exp as i64)?)
            .secure(true)
            .http_only(true)
            .same_site(cookie::SameSite::None)
            .build())
    }
}

impl Token for RefreshToken {
    const EXPIRE_SECS_VAR: &'static str = "REFRESH_EXPIRE_SECS";

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::{Claims, Token};
    use jsonwebtoken::{Algorithm, EncodingKey, Header};

    #[test]
    fn test_encode() -> anyhow::Result<()> {
        let uuid = uuid::Uuid::new_v4();
        let claims = Claims {
            exp: 1,
            iat: 1,
            sub: uuid.to_string(),
        };
        let secret = "test";
        let encoded = RefreshToken::encode(&claims, secret.to_string())?;
        let raw_encoded = move || {
            let mut header = Header::new(Algorithm::HS512);
            let claims = Claims {
                exp: 1,
                iat: 1,
                sub: uuid.to_string(),
            };

            header.kid = Some(claims.sub.clone());

            let key = EncodingKey::from_secret(secret.as_bytes());

            jsonwebtoken::encode(&header, &claims, &key).unwrap_or_default()
        };

        assert_eq!(encoded, raw_encoded());

        Ok(())
    }

    #[test]
    fn test_token_decode() -> anyhow::Result<()> {
        let uuid = uuid::Uuid::new_v4();
        let now = chrono::Utc::now().timestamp() as usize;
        let claims = Claims {
            exp: now + 1000,
            iat: now,
            sub: uuid.to_string(),
        };
        let secret = "test";
        let encoded = RefreshToken::encode(&claims, secret.to_string())?;
        let decoded = RefreshToken::decode(encoded.clone(), &secret.to_string())?;

        assert_eq!(encoded, decoded.encoded);

        Ok(())
    }

    #[test]
    fn test_invalid_token_decode() -> anyhow::Result<()> {
        let uuid = uuid::Uuid::new_v4();
        let claims = Claims {
            exp: 1,
            iat: 1,
            sub: uuid.to_string(),
        };
        let secret = "test";
        let encoded = RefreshToken::encode(&claims, secret.to_string())?;
        let decoded = RefreshToken::decode(encoded.clone(), &secret.to_string());

        // Expired, so is_err must be true
        assert!(decoded.is_err());

        Ok(())
    }

    #[test]
    fn test_is_expired() -> anyhow::Result<()> {
        let uuid = uuid::Uuid::new_v4();
        let claims = Claims {
            exp: 1,
            iat: 1,
            sub: uuid.to_string(),
        };
        let secret = "test";
        let encoded = RefreshToken::encode(&claims, secret.to_string())?;
        let token = RefreshToken::new(encoded, claims);

        assert!(token.is_expired());

        Ok(())
    }

    #[tokio::test]
    async fn test_get_headers() -> anyhow::Result<()> {
        let uuid = uuid::Uuid::new_v4();
        let claims = Claims {
            exp: 1,
            iat: 1,
            sub: uuid.to_string(),
        };
        let secret = "test";
        let encoded = RefreshToken::encode(&claims, secret.to_string())?;
        let token = RefreshToken::new(encoded, claims);
        let headers = token.get_headers()?;

        assert_eq!(headers.kid.unwrap(), uuid.to_string());

        Ok(())
    }

    #[test]
    fn test_try_as_cookie() -> anyhow::Result<()> {
        let uuid = uuid::Uuid::new_v4();
        let claims = Claims {
            exp: 1,
            iat: 1,
            sub: uuid.to_string(),
        };
        let secret = "test";
        let encoded = RefreshToken::encode(&claims, secret.to_string())?;
        let token = RefreshToken::new(encoded.clone(), claims);

        assert_eq!(token.try_as_cookie()?.value(), encoded);

        Ok(())
    }
}
