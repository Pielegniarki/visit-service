use std::{net::SocketAddr, sync::Arc};

use axum::{
    routing::{get, put},
    Router,
};

use crate::db::DB;

mod routes;

pub struct AppState {
    db: DB
}

pub struct App;

impl App {
    pub async fn serve(db: DB) -> anyhow::Result<()> {
        let state = Arc::new(AppState { db });

        let app = Router::new()
            .route("/", get(routes::index))
            .nest("/healthcheck", api::healthcheck())
            .nest("/visits", api::visits())
            .with_state(state);

        #[cfg(debug_assertions)]
        let app = app.layer(tower_http::cors::CorsLayer::permissive());

        let addr = SocketAddr::from(([0, 0, 0, 0], 4002));

        tracing::debug!("listening on {}", addr);

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}

mod api {
    use super::*;

    pub fn healthcheck() -> Router<Arc<AppState>> {
        Router::new()
            .route("/http", get(routes::healthcheck::http))
            .route("/db", get(routes::healthcheck::db))
    }

    pub fn visits() -> Router<Arc<AppState>> {
        Router::new()
            .route("/scheduleVisit", put(routes::visits::schedule_visit))
            .route("/getAllOfPatient", get(routes::visits::list_all_of_patient))
            .route("/getAllOfDoctor", get(routes::visits::list_all_of_doctor))
    }

}
