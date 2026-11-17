mod database;

use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, State},
    http::{
        HeaderValue, Method,
        header::{CACHE_CONTROL, CONTENT_TYPE},
    },
};
pub use database::CityDatabase;
use serde::{Deserialize, Serialize};
use std::{net::IpAddr, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocationRequest {
    pub ip: IpAddr,
}

#[derive(Serialize)]
pub struct Location {
    pub location: Option<String>,
    pub attribution: &'static str,
}

pub fn router(database: Arc<CityDatabase>) -> Router {
    Router::new()
        .route(
            "/health",
            axum::routing::get(|| async { axum::http::StatusCode::NO_CONTENT }),
        )
        .route("/v1/locations", axum::routing::post(locate))
        .layer(DefaultBodyLimit::max(1024))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::POST])
                .allow_headers([CONTENT_TYPE]),
        )
        .with_state(database)
}

async fn locate(
    State(database): State<Arc<CityDatabase>>,
    Json(request): Json<LocationRequest>,
) -> ([(axum::http::HeaderName, HeaderValue); 1], Json<Location>) {
    (
        [(CACHE_CONTROL, HeaderValue::from_static("no-store"))],
        Json(Location {
            location: database.location(request.ip),
            attribution: "https://db-ip.com",
        }),
    )
}
