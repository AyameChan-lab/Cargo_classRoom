use axum::response::{IntoResponse, Response};
use axum::{Router, extract::Path, http::StatusCode, routing::get};

pub fn routes() -> Router {
    Router::new().route("/{code}", get(return_error))
}

async fn return_error(Path(code): Path<u16>) -> Response {
    match StatusCode::from_u16(code) {
        Ok(status) => (status, format!("Simulated Error: {}", status)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Invalid Status Code").into_response(),
    }
}
