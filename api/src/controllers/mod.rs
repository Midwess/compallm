pub mod auth;

use crate::di::ServiceState;
use utoipa::openapi::OpenApi;
use utoipa_axum::router::OpenApiRouter;

#[derive(utoipa::OpenApi)]
struct ApiDoc;

pub fn api_router() -> OpenApiRouter<ServiceState> {
    OpenApiRouter::with_openapi(<ApiDoc as utoipa::OpenApi>::openapi()).merge(auth::router())
}

pub fn openapi() -> OpenApi {
    api_router().into_openapi()
}
