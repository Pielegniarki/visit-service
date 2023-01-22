use std::sync::Arc;

use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::{header::HeaderValue, request::Parts, HeaderMap, Request, StatusCode},
};

use crate::models::DatabaseId;

use super::AppState;

pub struct ExtractId(pub DatabaseId);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractId
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        if let Some(plg_id) = parts.headers.get("X-PLG-ID") {
            Ok(ExtractId(plg_id.to_str().unwrap().into()))
        } else {
            Err((StatusCode::BAD_REQUEST, "`X-PLG-ID` header is missing"))
        }
    }
}

pub async fn token_map<Body>(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,

    mut request: Request<Body>,
) -> Request<Body> {
    if let Some(token) = headers.get("X-PLG-Token") {
        let client = &state.http_client;
        let token_str = token.to_str().expect("Invalid ASCII in plg-token header");

        if let Ok(Some(db_id)) = client.get_id(token_str).await {
            if let Ok(hv_id) = HeaderValue::try_from(Into::<String>::into(db_id)) {
                request.headers_mut().append("X-PLG-ID", hv_id);
            }
        }
    }

    request
}
