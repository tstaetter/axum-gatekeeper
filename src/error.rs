use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub use crate::tokens::TokenError;

#[derive(thiserror::Error, Debug)]
pub enum GateKeeperError {
    #[cfg(feature = "authentication")]
    #[error("Authentication error: {0}")]
    Authentication(#[from] crate::authentication::AuthenticationError),
    #[cfg(feature = "authorization")]
    #[error("Authorization error: {0}")]
    Authorization(#[from] crate::authorization::AuthorizationError),
    #[cfg(feature = "verification")]
    #[error("Verification error: {0}")]
    Verification(#[from] crate::verification::VerificationError),
    #[error("Cannot parse int value: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Error reading env var: {0}")]
    EnvVar(#[from] std::env::VarError),
    #[error("Token handling error: {0}")]
    Token(#[from] TokenError),
}

impl IntoResponse for GateKeeperError {
    fn into_response(self) -> Response {
        match self {
            #[cfg(feature = "authentication")]
            GateKeeperError::Authentication(e) => {
                tracing::error!("{e:?}");
                (StatusCode::UNAUTHORIZED, "Error authenticating user").into_response()
            }
            #[cfg(feature = "authorization")]
            GateKeeperError::Authorization(e) => {
                tracing::error!("{e:?}");
                (StatusCode::FORBIDDEN, "Error authorizing user").into_response()
            }
            #[cfg(feature = "verification")]
            GateKeeperError::Verification(e) => {
                tracing::error!("{e:?}");
                (StatusCode::FORBIDDEN, "Error verifying token").into_response()
            }
            GateKeeperError::ParseInt(e) => {
                tracing::error!("Int parsing error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Error parsing value").into_response()
            }
            GateKeeperError::EnvVar(e) => {
                tracing::error!("Env var error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Error reading env var").into_response()
            }
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
    use crate::tokens::TokenError;
    use crate::ErrorResponse;
    use http_body_util::BodyExt;

    #[tokio::test]
    async fn test_token_error() -> anyhow::Result<()> {
        let error = ErrorResponse::build()
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
