use std::{net::SocketAddr, sync::Arc};

use axum::{
    routing::{get, put},
    Router, middleware::map_request_with_state,
};
use tower::ServiceBuilder;
use tower_http::{trace::TraceLayer, cors::CorsLayer};
use token_extractor::token_map;

use crate::db::DB;
use crate::http_client::HttpClient;

mod routes;
mod token_extractor;

pub struct AppState {
    db: DB,
    http_client: HttpClient
}

pub struct App;

impl App {
    pub async fn serve(db: DB, http_client: HttpClient) -> anyhow::Result<()> {
        let state = Arc::new(AppState { db, http_client });

        let services = ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(CorsLayer::permissive());

        let app = Router::new()
            .route("/", get(routes::index))
            .nest("/healthcheck", api::healthcheck())
            .nest("/visits", api::visits())
            .layer(map_request_with_state(state.clone(), token_map))
            .with_state(state)
            .layer(services);


        let addr = SocketAddr::from(([0, 0, 0, 0], 4002));

        tracing::debug!("listening on {}", addr);

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}

mod api {
    use axum::routing::post;

    use super::*;

    pub fn healthcheck() -> Router<Arc<AppState>> {
        Router::new()
            .route("/http", get(routes::healthcheck::http))
            .route("/db", get(routes::healthcheck::db))
    }

    pub fn visits() -> Router<Arc<AppState>> {
        Router::new()
            .route("/scheduleVisit", post(routes::visits::schedule_visit))
            .route("/getAllOfPatient", get(routes::visits::list_all_of_patient))
            .route("/getAllOfDoctor", get(routes::visits::list_all_of_doctor))
            .route("/closeVisit", post(routes::visits::close_visit))
    }

}
