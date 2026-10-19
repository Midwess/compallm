use std::sync::{
    Arc,
    atomic::{AtomicU32, Ordering},
};

use axum::{Json, Router, extract::State};
use utoipa::openapi::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::api_types::{CreateGreeting, Greeting};

#[derive(utoipa::OpenApi)]
struct ApiDoc;

#[derive(Clone, Default)]
struct AppState {
    request_count: Arc<AtomicU32>,
}

pub fn router() -> Router {
    let (router, _) = documented_routes()
        .with_state(AppState::default())
        .split_for_parts();
    router
}

pub fn openapi() -> OpenApi {
    documented_routes().into_openapi()
}

fn documented_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::with_openapi(<ApiDoc as utoipa::OpenApi>::openapi())
        .routes(routes!(get_greeting, create_greeting))
}

#[utoipa::path(
    get,
    path = "/api/greeting",
    operation_id = "getGreeting",
    responses(
        (status = OK, description = "Return the default greeting", body = Greeting)
    )
)]
async fn get_greeting(State(state): State<AppState>) -> Json<Greeting> {
    Json(greeting(&state, "Vite"))
}

#[utoipa::path(
    post,
    path = "/api/greeting",
    operation_id = "createGreeting",
    request_body = CreateGreeting,
    responses(
        (status = OK, description = "Create a personalized greeting", body = Greeting)
    )
)]
async fn create_greeting(
    State(state): State<AppState>,
    Json(input): Json<CreateGreeting>,
) -> Json<Greeting> {
    let name = input.name.trim();
    Json(greeting(
        &state,
        if name.is_empty() { "Vite" } else { name },
    ))
}

fn greeting(state: &AppState, name: &str) -> Greeting {
    let request_count = state.request_count.fetch_add(1, Ordering::Relaxed) + 1;

    Greeting {
        message: format!("Hello, {name}!"),
        served_by: "Axum".to_owned(),
        request_count,
    }
}
