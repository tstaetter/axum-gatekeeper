use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(thiserror::Error, Debug, Deserialize)]
pub enum TokenError {
    #[error("Error signing token: {0}")]
    Encoding(TokenErrorResponse),
    #[error("Error decoding token headers: {0}")]
    DecodeHeader(TokenErrorResponse),
    #[error("Error decoding token: {0}")]
    Decode(TokenErrorResponse),
    #[error("No token string available")]
    MissingTokenString,
    #[error("Error using refresh token: {0}")]
    RefreshToken(TokenErrorResponse),
}

impl IntoResponse for TokenError {
    fn into_response(self) -> Response {
        match self {
            TokenError::Encoding(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
            TokenError::DecodeHeader(e) => {
                (StatusCode::UNAUTHORIZED, e.to_string()).into_response()
            }
            TokenError::Decode(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
            TokenError::MissingTokenString => (
                StatusCode::UNAUTHORIZED,
                String::from("Missing token string"),
            )
                .into_response(),
            TokenError::RefreshToken(e) => {
                (StatusCode::UNAUTHORIZED, e.to_string()).into_response()
            }
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct TokenErrorResponse {
    status_code: u16,
    message: String,
}

impl TokenErrorResponse {
    /// Create a builder for TokenErrorResponse
    pub fn build() -> TokenErrorResponseBuilder {
        TokenErrorResponseBuilder::default()
    }
}

impl Display for TokenErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::json!({
            "status_code": self.status_code,
            "message": self.message,
        });

        write!(f, "{json}")
    }
}

#[derive(Debug, Default)]
pub struct TokenErrorResponseBuilder {
    status_code: Option<u16>,
    message: Option<String>,
}

impl TokenErrorResponseBuilder {
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
    pub fn build(self) -> TokenErrorResponse {
        TokenErrorResponse {
            status_code: self
                .status_code
                .unwrap_or(StatusCode::UNAUTHORIZED.as_u16()),
            message: self.message.unwrap_or(String::from("Unauthorized")),
        }
    }
}

impl Default for TokenErrorResponse {
    fn default() -> Self {
        Self {
            status_code: StatusCode::UNAUTHORIZED.as_u16(),
            message: String::from("Unauthorized"),
        }
    }
}
