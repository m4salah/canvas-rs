use axum::{http::StatusCode, response::IntoResponse, routing::get};

pub fn router() -> axum::Router {
    axum::Router::new().route("/", get(health))
}

async fn health() -> impl IntoResponse {
    StatusCode::OK
}
