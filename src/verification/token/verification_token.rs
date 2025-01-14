use crate::error::TokenError;
use crate::tokens::{Claims, Token};
use crate::GateKeeperResult;
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct VerificationToken {
    encoded: String,
    claims: Claims,
}

impl VerificationToken {
    /// Try generating a base64 hash of this tokens
    pub fn try_as_base64(&self) -> GateKeeperResult<String> {
        Ok(general_purpose::URL_SAFE.encode(self.encoded.as_bytes()))
    }

    pub async fn try_from_base64(hash: &str, secret: &str) -> GateKeeperResult<Self> {
        let encoded = String::from_utf8(
            general_purpose::URL_SAFE
                .decode(hash)
                .map_err(TokenError::Base64Decode)?,
        )
        .map_err(TokenError::Utf8)?;

        Self::decode(encoded, secret)
    }
}

impl Token for VerificationToken {
    const EXPIRE_SECS_VAR: &'static str = "VERIFICATION_EXPIRE_SECS";

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
mod tests {}
