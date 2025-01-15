use serde::de::DeserializeOwned;
use serde::Serialize;

/// Generic model trait
pub trait GateKeeperModel: Serialize + DeserializeOwned + Send + Sync {
    /// Return the model object's ID
    fn id(&self) -> uuid::Uuid;

    /// Return the model's personal secret
    fn secret(&self) -> &'static str;

    /// Return the model's `first_name`
    fn first_name(&self) -> &str;

    /// Return the model's `last_name`
    fn last_name(&self) -> &str;

    /// Return the model's `email`
    fn email(&self) -> &str;

    #[cfg(feature = "verification")]
    /// Get the models verification token
    fn verification_token(&self) -> Option<crate::tokens::VerificationToken>;
}
