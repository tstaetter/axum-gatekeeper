mod token;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub use token::TokenError;

#[derive(thiserror::Error, Debug)]
pub enum GateKeeperError {
    #[error("Token handling token: {0}")]
    Token(#[from] TokenError),
}

impl IntoResponse for GateKeeperError {
    fn into_response(self) -> Response {
        match self {
            GateKeeperError::Token(e) => {
                tracing::error!("Token error: {:?}", e);
                (StatusCode::UNAUTHORIZED, e).into_response()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::token::TokenErrorResponse;
    use http_body_util::BodyExt;

    #[tokio::test]
    async fn test_token_error() -> anyhow::Result<()> {
        let error = TokenErrorResponse::build()
            .status_code(StatusCode::IM_A_TEAPOT)
            .message("foo => bar".to_string())
            .build();
        let e = GateKeeperError::Token(TokenError::DecodeHeader(error));
        let response = e.into_response();
        let status = response.status();
        let body = response.into_body().collect().await?.to_bytes();
        let body = serde_json::from_slice::<serde_json::Value>(&body)?;
        let expected = serde_json::json!({
            "status_code": 418,
            "message": "foo => bar"
        });

        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(body, expected);

        Ok(())
    }
}
