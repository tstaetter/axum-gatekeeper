use crate::model::GateKeeperModel;
use crate::tokens::{Claims, Token, TokenService};
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

    pub async fn try_from_base64(hash: &str, db: &AppDatabase) -> GateKeeperResult<Self> {
        let encoded = String::from_utf8(general_purpose::URL_SAFE.decode(hash)?)?;
        let user = TokenService::try_get_user_for_encoded(encoded.clone(), db).await?;

        Self::decode(encoded, &user.secret().to_string())
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
