use std::sync::{
    Arc,
    atomic::{AtomicU32, Ordering},
};

use loco_rs::prelude::*;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::dtos::greeting::{CreateGreeting, Greeting};

#[derive(Clone, Default)]
pub struct RequestCounter(Arc<AtomicU32>);

impl RequestCounter {
    fn next(&self) -> u32 {
        self.0.fetch_add(1, Ordering::Relaxed) + 1
    }
}

pub fn router() -> OpenApiRouter<AppContext> {
    OpenApiRouter::new().routes(routes!(get_greeting, create_greeting))
}

#[utoipa::path(
    get,
    path = "/api/greeting",
    operation_id = "getGreeting",
    responses(
        (status = OK, description = "Return the default greeting", body = Greeting)
    )
)]
async fn get_greeting(SharedStore(counter): SharedStore<RequestCounter>) -> Result<Response> {
    format::json(greeting(&counter, "Vite"))
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
    SharedStore(counter): SharedStore<RequestCounter>,
    Json(input): Json<CreateGreeting>,
) -> Result<Response> {
    let name = input.name.trim();
    format::json(greeting(
        &counter,
        if name.is_empty() { "Vite" } else { name },
    ))
}

fn greeting(counter: &RequestCounter, name: &str) -> Greeting {
    Greeting {
        message: format!("Hello, {name}!"),
        served_by: "Loco + Axum".to_owned(),
        request_count: counter.next(),
    }
}
