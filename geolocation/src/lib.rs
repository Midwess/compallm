mod database;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::{HeaderValue, header::CACHE_CONTROL},
};
pub use database::CityDatabase;
use serde::Serialize;
use std::{net::IpAddr, sync::Arc};

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
        .route("/v1/locations/{ip}", axum::routing::get(locate))
        .with_state(database)
}

async fn locate(
    State(database): State<Arc<CityDatabase>>,
    Path(ip): Path<IpAddr>,
) -> ([(axum::http::HeaderName, HeaderValue); 1], Json<Location>) {
    (
        [(CACHE_CONTROL, HeaderValue::from_static("no-store"))],
        Json(Location {
            location: database.location(ip),
            attribution: "https://db-ip.com",
        }),
    )
}
