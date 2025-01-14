mod service;

#[cfg(feature = "authentication")]
mod authentication_token;
mod error;
#[cfg(feature = "authentication")]
mod refresh_token;
#[cfg(feature = "verification")]
mod verification_token;

#[cfg(feature = "authentication")]
pub use authentication_token::AuthenticationToken;
#[cfg(feature = "authentication")]
pub use refresh_token::RefreshToken;
#[cfg(feature = "verification")]
pub use verification_token::VerificationToken;

use chrono::Utc;
pub use error::*;
pub use service::TokenService;

use crate::error::GateKeeperError;
use crate::model::GateKeeperModel;
use crate::GateKeeperResult;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    sub: String,
}

pub trait Token {
    const EXPIRE_SECS_VAR: &'static str = "AUTH_EXPIRE_SECS";

    /// Create new tokens
    fn new(encoded: String, claims: Claims) -> Self
    where
        Self: Sized;

    /// Create tokens for provided user
    fn try_new_for_user(user: &impl GateKeeperModel) -> GateKeeperResult<Self>
    where
        Self: Sized,
    {
        let iat = Utc::now().timestamp() as usize;
        let exp = iat + std::env::var(Self::EXPIRE_SECS_VAR)?.parse::<usize>()?;
        let claims = Claims {
            exp,
            iat,
            sub: user.id().to_string(),
        };
        let encoded = Self::encode(&claims, user.secret().to_string())?;

        Ok(Self::new(encoded, claims))
    }

    /// Encode given claims to JWT tokens
    fn encode(claims: &Claims, secret: String) -> GateKeeperResult<String> {
        let mut header = Header::new(Algorithm::HS512);

        header.kid = Some(claims.sub.clone());

        let key = EncodingKey::from_secret(secret.as_bytes());
        let encoded = jsonwebtoken::encode(&header, claims, &key).map_err(|e| {
            tracing::error!("Couldn't encode token claims: {}", e);

            let response = TokenErrorResponse::build()
                .message("Couldn't encode token claims".to_string())
                .build();

            GateKeeperError::Token(TokenError::Encode(response))
        })?;

        Ok(encoded)
    }

    /// Decode Token from given `encoded` string using `secret`
    fn decode(encoded: String, secret: &String) -> GateKeeperResult<Self>
    where
        Self: Sized,
    {
        let validation = Validation::new(Algorithm::HS512);
        let claims = jsonwebtoken::decode::<Claims>(
            &encoded,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        )
        .map_err(|e| {
            tracing::error!("Couldn't decode token claims: {}", e);

            let response = TokenErrorResponse::build()
                .message("Couldn't decode token claims".to_string())
                .build();

            GateKeeperError::Token(TokenError::Decode(response))
        })?
        .claims;

        Ok(Self::new(encoded, claims))
    }

    /// Test if tokens is expired
    fn is_expired(&self) -> bool {
        self.get_claims().exp < Utc::now().timestamp() as usize
    }

    /// Return the tokens claims
    fn get_claims(&self) -> &Claims;

    /// Return encoded tokens string
    fn get_encoded(&self) -> &String;

    fn get_headers(&self) -> GateKeeperResult<Header> {
        TokenService::get_token_headers_from_encoded(self.get_encoded().to_string())
    }
}
