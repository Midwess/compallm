pub mod greeting;

use loco_rs::app::AppContext;
use utoipa::openapi::OpenApi;
use utoipa_axum::router::OpenApiRouter;

#[derive(utoipa::OpenApi)]
struct ApiDoc;

pub fn api_router() -> OpenApiRouter<AppContext> {
    OpenApiRouter::with_openapi(<ApiDoc as utoipa::OpenApi>::openapi()).merge(greeting::router())
}

pub fn openapi() -> OpenApi {
    api_router().into_openapi()
}
