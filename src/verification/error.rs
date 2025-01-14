use axum::response::{IntoResponse, Response};

#[derive(thiserror::Error, Debug)]
pub enum VerificationError {}

impl IntoResponse for VerificationError {
    fn into_response(self) -> Response {
        let response = match self {
            _ => "",
        };

        response.into_response()
    }
}
