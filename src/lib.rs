pub mod authentication;
pub mod error;

pub type GateKeeperResult<T> = Result<T, error::GateKeeperError>;

#[cfg(test)]
mod tests {}
