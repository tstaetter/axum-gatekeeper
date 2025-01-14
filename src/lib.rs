use axum::http::StatusCode;
use std::fmt::{Display, Formatter};

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

#[derive(serde::Deserialize, Debug)]
pub struct ErrorResponse {
    status_code: u16,
    message: String,
}

impl ErrorResponse {
    /// Create a builder for ErrorResponse
    pub fn build() -> ErrorResponseBuilder {
        ErrorResponseBuilder::default()
    }
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::json!({
            "status_code": self.status_code,
            "message": self.message,
        });

        write!(f, "{json}")
    }
}

#[derive(Debug, Default)]
pub struct ErrorResponseBuilder {
    status_code: Option<u16>,
    message: Option<String>,
}

impl ErrorResponseBuilder {
    /// Set field `status_code`
    pub fn status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = Some(status_code.as_u16());
        self
    }

    /// Set field `message`
    pub fn message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    /// Actually create the response object
    pub fn build(self) -> ErrorResponse {
        ErrorResponse {
            status_code: self
                .status_code
                .unwrap_or(StatusCode::UNAUTHORIZED.as_u16()),
            message: self.message.unwrap_or(String::from("Unauthorized")),
        }
    }
}

impl Default for ErrorResponse {
    fn default() -> Self {
        Self {
            status_code: StatusCode::UNAUTHORIZED.as_u16(),
            message: String::from("Unauthorized"),
        }
    }
}

#[cfg(test)]
mod tests {}
