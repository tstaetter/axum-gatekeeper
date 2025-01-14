use serde::de::DeserializeOwned;
use serde::Serialize;

/// Generic model trait
pub trait GateKeeperModel: Serialize + DeserializeOwned + Send + Sync {
    /// Return the model object's ID
    fn id(&self) -> uuid::Uuid;

    /// Return the model object's personal secret
    fn secret(&self) -> &'static str;
}
