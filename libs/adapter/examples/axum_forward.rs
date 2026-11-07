//! Run with ANTHROPIC_API_KEY and ANTHROPIC_MODEL set:
//! cargo run -p adapter --example axum_forward
//!
//! POST http://127.0.0.1:3000/translate with:
//! {"model":"client-model","messages":[{"role":"user","content":"Hello"}]}
//!
//! Returns raw Anthropic responses, NOT OpenAI responses. No fallback or auth.
//! Example codecs reject unsupported fields and messages over 64 KiB.

#[path = "axum_forward_support/codecs.rs"]
mod codecs;
#[path = "axum_forward_support/store.rs"]
mod store;

use std::{num::NonZeroU32, sync::Arc, time::Duration};

use adapter::t2t::{Text2TextRequestTranslator, request::RequestTranslatorError};
use axum::{
    Router,
    body::Body,
    extract::{Request, State},
    http::{StatusCode, header},
    response::Response,
    routing::post,
};
use codecs::{AnthropicTextEncoder, OpenAiTextDecoder, TargetProperties};
use futures_util::StreamExt;
use store::DiskReplayStore;
use tokio::sync::Semaphore;

#[derive(Clone)]
struct AppState {
    client: reqwest::Client,
    url: reqwest::Url,
    properties: TargetProperties,
    admission: Arc<Semaphore>,
}

type HttpError = (StatusCode, &'static str);

async fn forward(State(state): State<AppState>, request: Request) -> Result<Response, HttpError> {
    let _permit = state.admission.try_acquire().map_err(|_| {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            "example concurrency limit reached",
        )
    })?;
    if request.headers().get(header::CONTENT_ENCODING).is_some() {
        return Err((
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "compressed requests are unsupported",
        ));
    }
    let is_json = request
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| {
            value
                .split(';')
                .next()
                .is_some_and(|mime| mime.trim().eq_ignore_ascii_case("application/json"))
        });
    if !is_json {
        return Err((StatusCode::UNSUPPORTED_MEDIA_TYPE, "use application/json"));
    }
    let replay = DiskReplayStore::new().await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "cannot create replay storage",
        )
    })?;
    // Axum supplies chunks directly; replay storage caps the total at 8 MiB.
    let input = request.into_body().into_data_stream();
    let mut translator = Text2TextRequestTranslator::new(input, OpenAiTextDecoder::default, replay);

    // Validate through the same bounded codecs before sending any provider bytes.
    // Only this example performs a preflight pass; no output is collected.
    let validation = async {
        let body = translator
            .forward(AnthropicTextEncoder::new(&state.properties))
            .await
            .map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    "request upload or retention failed (limit 8 MiB)",
                )
            })?;
        let mut body = std::pin::pin!(body);
        while let Some(chunk) = body.next().await {
            chunk.map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    "unsupported or invalid text-only OpenAI request (message limit 64 KiB)",
                )
            })?;
        }
        Ok::<_, HttpError>(())
    };
    let validation = tokio::time::timeout(Duration::from_secs(30), validation).await;
    match validation {
        Ok(Ok(())) => {}
        result => {
            let _ = translator.close().await;
            return Err(match result {
                Ok(Err(error)) => error,
                _ => (StatusCode::REQUEST_TIMEOUT, "upload/validation timeout"),
            });
        }
    }

    let body = match translator
        .forward(AnthropicTextEncoder::new(&state.properties))
        .await
    {
        Ok(body) => body,
        Err(_) => {
            let _ = translator.close().await;
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "cannot open request replay",
            ));
        }
    };
    let upstream = state
        .client
        .post(state.url)
        .header(header::CONTENT_TYPE, "application/json")
        .body(reqwest::Body::wrap_stream(body))
        .send()
        .await;

    // An early upstream response can leave the request body in flight. Its reader
    // owns a temp-file guard, so dropping this store cannot invalidate that reader.
    match translator.close().await {
        Ok(()) | Err(RequestTranslatorError::AttemptActive) => {}
        Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "replay cleanup failed")),
    }
    let upstream = upstream.map_err(|_| (StatusCode::BAD_GATEWAY, "provider request failed"))?;
    let mut response = Response::builder().status(upstream.status());
    if let Some(content_type) = upstream.headers().get(header::CONTENT_TYPE) {
        response = response.header(header::CONTENT_TYPE, content_type);
    }
    response
        .body(Body::from_stream(upstream.bytes_stream()))
        .map_err(|_| (StatusCode::BAD_GATEWAY, "invalid provider response"))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = std::env::var("ANTHROPIC_API_KEY")?;
    if key.is_empty() {
        return Err("ANTHROPIC_API_KEY must not be empty".into());
    }
    let model = std::env::var("ANTHROPIC_MODEL")?;
    if model.is_empty() || model.len() > 256 {
        return Err("ANTHROPIC_MODEL must have 1–256 bytes".into());
    }
    let url = reqwest::Url::parse(
        &std::env::var("ANTHROPIC_URL")
            .unwrap_or_else(|_| "https://api.anthropic.com/v1/messages".into()),
    )?;
    let local = matches!(url.host_str(), Some("127.0.0.1" | "[::1]" | "localhost"));
    if (url.scheme() != "https" && !(url.scheme() == "http" && local))
        || !url.username().is_empty()
        || url.password().is_some()
        || url.fragment().is_some()
    {
        return Err(
            "ANTHROPIC_URL must be HTTPS (or loopback HTTP), without credentials or fragment"
                .into(),
        );
    }
    let mut headers = reqwest::header::HeaderMap::new();
    let mut key = reqwest::header::HeaderValue::from_str(&key)?;
    key.set_sensitive(true);
    headers.insert("x-api-key", key);
    headers.insert(
        "anthropic-version",
        reqwest::header::HeaderValue::from_static("2023-06-01"),
    );
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .redirect(reqwest::redirect::Policy::none())
        .retry(reqwest::retry::never())
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(120))
        .build()?;
    let state = AppState {
        client,
        url,
        properties: TargetProperties {
            model,
            max_tokens: NonZeroU32::new(256).unwrap(),
        },
        admission: Arc::new(Semaphore::new(8)),
    };
    let app = Router::new()
        .route("/translate", post(forward))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("POST http://127.0.0.1:3000/translate (raw Anthropic response)");
    axum::serve(listener, app).await?;
    Ok(())
}
