//! Module containing middleware and traits used for authenticating a user
//!
//! Only available on feature `authentication`
mod error;
mod middleware;

pub use error::*;
pub use middleware::*;
