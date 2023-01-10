use axum::response::IntoResponse;

pub mod visits;
pub mod healthcheck;

pub async fn index() -> impl IntoResponse {
    "Hello, World!"
}