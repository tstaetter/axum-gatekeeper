#[cfg(feature = "authentication")]
pub mod authentication;
#[cfg(feature = "authorization")]
pub mod authorization;
pub mod error;
mod model;
pub mod tokens;
#[cfg(feature = "verification")]
pub mod verification;

pub type GateKeeperResult<T> = Result<T, error::GateKeeperError>;

#[cfg(test)]
mod tests {}
