use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

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
    Encode(crate::ErrorResponse),
    #[error("Error decoding tokens headers: {0}")]
    DecodeHeader(crate::ErrorResponse),
    #[error("Error decoding tokens: {0}")]
    Decode(crate::ErrorResponse),
    #[error("No tokens string available")]
    MissingTokenString,
    #[error("Error using refresh tokens: {0}")]
    RefreshToken(crate::ErrorResponse),
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
