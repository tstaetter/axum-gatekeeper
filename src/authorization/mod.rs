//! Module containing middleware and traits used for authorizing a user
//!
//! Only available on feature `authorization`
mod error;
mod middleware;

pub use error::*;
pub use middleware::*;

/// Return the owner's ID so we can check for ownership against some e.g. User model
pub trait AuthorizeOwner: Sized {
    fn owner_id(&self) -> uuid::Uuid;
}
