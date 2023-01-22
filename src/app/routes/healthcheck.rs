use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};

use crate::app::AppState;

pub async fn http() -> impl IntoResponse {
    "Server OK"
}

pub async fn db(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let colls = state.db.collections();
    let doctors = colls.visit();

    doctors.name().to_owned()
}
