use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::fmt::{Display, Formatter};

#[derive(thiserror::Error, Debug)]
pub enum TokenError {
    #[error("Unknown error handling JWT: {0}")]
    Unknown(#[from] jsonwebtoken::errors::Error),
    #[error("Error decoding base64 value: {0}")]
    Base64Decode(#[from] base64::DecodeError),
    #[error("Error reading UTF-8 value: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("Couldn't read cookie expiration time: {0:?}")]
    ReadingExpiration(#[from] cookie::time::error::ComponentRange),
    #[error("Error signing tokens: {0}")]
    Encode(TokenErrorResponse),
    #[error("Error decoding tokens headers: {0}")]
    DecodeHeader(TokenErrorResponse),
    #[error("Error decoding tokens: {0}")]
    Decode(TokenErrorResponse),
    #[error("No tokens string available")]
    MissingTokenString,
    #[error("Error using refresh tokens: {0}")]
    RefreshToken(TokenErrorResponse),
}

impl IntoResponse for TokenError {
    fn into_response(self) -> Response {
        match self {
            TokenError::ReadingExpiration(e) => {
                tracing::error!("Error reading expiration time: {:?}", e);
                (
                    StatusCode::UNAUTHORIZED,
                    "Error creating expiration time for cookie",
                )
                    .into_response()
            }
            TokenError::Base64Decode(e) => {
                tracing::error!("Error decoding base64 value: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error decoding base64 value",
                )
                    .into_response()
            }
            TokenError::Utf8(e) => {
                tracing::error!("Error reading UTF-8 value: {:?}", e);

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error reading UTF-8 value",
                )
                    .into_response()
            }
            TokenError::Unknown(e) => {
                tracing::error!("Unknown error handling JWT: {:?}", e);
                (StatusCode::UNAUTHORIZED, "Unknown error handling JWT").into_response()
            }
            TokenError::Encode(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
            TokenError::DecodeHeader(e) => {
                (StatusCode::UNAUTHORIZED, e.to_string()).into_response()
            }
            TokenError::Decode(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
            TokenError::MissingTokenString => (
                StatusCode::UNAUTHORIZED,
                String::from("Missing tokens string"),
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
