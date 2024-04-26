use crate::templates;
use axum::{response::IntoResponse, routing::get};

pub fn router() -> axum::Router {
    axum::Router::new().route("/", get(home))
}

async fn home() -> impl IntoResponse {
    templates::Index
}
