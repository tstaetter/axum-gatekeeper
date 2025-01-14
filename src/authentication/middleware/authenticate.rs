use crate::GateKeeperResult;
use axum::{body::Body, extract::Request, middleware::Next, response::Response};

pub async fn authenticate_user(req: Request, next: Next) -> GateKeeperResult<Response<Body>> {
    tracing::debug!("Using middleware::authenticate_user");

    Ok(next.run(req).await)
}
