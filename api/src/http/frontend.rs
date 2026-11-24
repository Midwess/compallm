use axum::{
    Router,
    body::Body,
    extract::State,
    http::{HeaderMap, Request, StatusCode},
    response::Response,
};
use std::{path::Path, process::Stdio};

#[derive(Clone, serde::Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct FrontendSettings {
    pub port: u16,
    pub managed: bool,
    pub public_origin: String,
}

impl Default for FrontendSettings {
    fn default() -> Self {
        Self {
            port: 4868,
            managed: true,
            public_origin: String::new(),
        }
    }
}

#[derive(Clone)]
struct Upstream {
    client: reqwest::Client,
    origin: String,
    public_origin: String,
}

pub struct Frontend {
    upstream: Upstream,
    child: Option<tokio::process::Child>,
}

impl Frontend {
    pub async fn start(
        root: &Path,
        api_port: u16,
        settings: FrontendSettings,
    ) -> anyhow::Result<Self> {
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;
        anyhow::ensure!(
            settings.port > 0 && settings.port != api_port,
            "Frontend port must differ from API port"
        );
        if !settings.public_origin.is_empty() {
            let public = reqwest::Url::parse(&settings.public_origin)?;
            anyhow::ensure!(
                matches!(public.scheme(), "http" | "https")
                    && public.host_str().is_some()
                    && public.path() == "/",
                "Frontend public_origin must be an HTTP origin"
            );
        }
        let origin = format!("http://127.0.0.1:{}", settings.port);
        let child = if settings.managed {
            let probe =
                tokio::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, settings.port))
                    .await?;
            drop(probe);
            anyhow::ensure!(
                root.join("build/index.js").is_file(),
                "Build the Portal first: npm --prefix portal run build"
            );
            let child = tokio::process::Command::new("node")
                .arg("build/index.js")
                .current_dir(root)
                .env("HOST", "127.0.0.1")
                .env("PORT", settings.port.to_string())
                .env("PROTOCOL_HEADER", "x-forwarded-proto")
                .env("HOST_HEADER", "x-forwarded-host")
                .env("RUST_API_ORIGIN", format!("http://127.0.0.1:{api_port}"))
                .stdin(Stdio::null())
                .kill_on_drop(true)
                .spawn()?;
            Some(child)
        } else {
            None
        };
        let mut frontend = Self {
            upstream: Upstream {
                client,
                origin,
                public_origin: settings.public_origin,
            },
            child,
        };
        use hero::cancellation::{CancellationToken, FutureExtension};
        let deadline = CancellationToken::timeout(std::time::Duration::from_secs(15));
        loop {
            if let Some(child) = &mut frontend.child {
                anyhow::ensure!(
                    child.try_wait()?.is_none(),
                    "Portal server exited during startup"
                );
            }
            if tokio::net::TcpStream::connect((std::net::Ipv4Addr::LOCALHOST, settings.port))
                .with_cancel(&deadline)
                .await?
                .is_ok()
            {
                break;
            }
            CancellationToken::timeout(std::time::Duration::from_millis(50))
                .cancelled()
                .with_cancel(&deadline)
                .await?;
        }
        Ok(frontend)
    }

    pub fn router(&self) -> Router {
        Router::new()
            .fallback(proxy)
            .with_state(self.upstream.clone())
    }
}

impl Drop for Frontend {
    fn drop(&mut self) {
        if let Some(child) = &mut self.child {
            let _ = child.start_kill();
        }
    }
}

fn strip_hop_headers(headers: &mut HeaderMap) {
    let named = headers
        .get_all("connection")
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|v| v.split(','))
        .map(|v| v.trim().to_owned())
        .collect::<Vec<_>>();
    for name in named {
        headers.remove(name);
    }
    for name in [
        "connection",
        "keep-alive",
        "proxy-authenticate",
        "proxy-authorization",
        "te",
        "trailer",
        "transfer-encoding",
        "upgrade",
    ] {
        headers.remove(name);
    }
}

async fn proxy(
    State(upstream): State<Upstream>,
    request: Request<Body>,
) -> Result<Response, StatusCode> {
    let (parts, body) = request.into_parts();
    let public_host = parts
        .headers
        .get("host")
        .cloned()
        .ok_or(StatusCode::BAD_REQUEST)?;
    let mut headers = parts.headers;
    strip_hop_headers(&mut headers);
    for name in [
        "forwarded",
        "x-forwarded-for",
        "x-forwarded-host",
        "x-forwarded-proto",
        "x-real-ip",
    ] {
        headers.remove(name);
    }
    let public = if upstream.public_origin.is_empty() {
        None
    } else {
        reqwest::Url::parse(&upstream.public_origin).ok()
    };
    let host = public.as_ref().map(|url| match url.port() {
        Some(port) => format!("{}:{port}", url.host_str().unwrap_or_default()),
        None => url.host_str().unwrap_or_default().to_owned(),
    });
    headers.insert(
        "x-forwarded-host",
        match host {
            Some(host) => host
                .parse()
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
            None => public_host,
        },
    );
    headers.insert(
        "x-forwarded-proto",
        public
            .as_ref()
            .map_or("http", |url| url.scheme())
            .parse()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    );
    let target = format!(
        "{}{}",
        upstream.origin,
        parts.uri.path_and_query().map_or("/", |v| v.as_str())
    );
    use hero::cancellation::{CancellationToken, FutureExtension};
    let deadline = CancellationToken::timeout(std::time::Duration::from_secs(45));
    let response = upstream
        .client
        .request(parts.method, target)
        .headers(headers)
        .body(reqwest::Body::wrap_stream(body.into_data_stream()))
        .send()
        .with_cancel(&deadline)
        .await
        .map_err(|_| StatusCode::GATEWAY_TIMEOUT)?
        .map_err(|error| {
            tracing::warn!(%error, "Portal upstream unavailable");
            StatusCode::BAD_GATEWAY
        })?;
    let status = response.status();
    let mut headers = response.headers().clone();
    strip_hop_headers(&mut headers);
    let mut outgoing = Response::new(Body::from_stream(response.bytes_stream()));
    *outgoing.status_mut() = status;
    *outgoing.headers_mut() = headers;
    Ok(outgoing)
}
