use axum::response::{IntoResponse, Response};

#[derive(thiserror::Error, Debug)]
pub enum AuthenticationError {}

impl IntoResponse for AuthenticationError {
    fn into_response(self) -> Response {
        let response = match self {
            _ => "",
        };

        response.into_response()
    }
}
