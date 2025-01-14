use axum::response::{IntoResponse, Response};

#[derive(thiserror::Error, Debug)]
pub enum AuthorizationError {}

impl IntoResponse for AuthorizationError {
    fn into_response(self) -> Response {
        let response = match self {
            _ => "",
        };

        response.into_response()
    }
}
