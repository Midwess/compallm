use hero::cancellation::{CancellationToken, FutureExtension};
use serde::Deserialize;
use std::{net::IpAddr, time::Duration};

/// Optional, bounded enrichment through the internal location server.
#[derive(Default)]
pub struct LocationClient {
    client: Option<reqwest::Client>,
    endpoint: String,
}

impl LocationClient {
    pub fn new(endpoint: &str) -> Result<Self, reqwest::Error> {
        Ok(Self {
            client: Some(
                reqwest::Client::builder()
                    .connect_timeout(Duration::from_millis(150))
                    .pool_max_idle_per_host(2)
                    .redirect(reqwest::redirect::Policy::none())
                    .build()?,
            ),
            endpoint: endpoint.trim_end_matches('/').to_owned(),
        })
    }

    pub async fn location(&self, ip: IpAddr) -> Option<String> {
        if self.endpoint.is_empty() {
            return None;
        }
        let cancellation = CancellationToken::current_or_new().child_token();
        cancellation.cancel_after(Duration::from_millis(300));
        let mut response = self
            .client
            .as_ref()?
            .get(format!("{}/v1/locations/{ip}", self.endpoint))
            .send()
            .with_cancel(&cancellation)
            .await
            .ok()?
            .ok()?;
        if !response.status().is_success() || response.content_length().is_some_and(|n| n > 4096) {
            return None;
        }
        let mut body = Vec::with_capacity(512);
        while let Some(chunk) = response
            .chunk()
            .with_cancel(&cancellation)
            .await
            .ok()?
            .ok()?
        {
            if body.len() + chunk.len() > 4096 {
                return None;
            }
            body.extend_from_slice(&chunk);
        }
        let reply: Location = serde_json::from_slice(&body).ok()?;
        reply
            .location
            .filter(|value| !value.is_empty() && value.len() <= 256)
    }
}

#[derive(Deserialize)]
struct Location {
    location: Option<String>,
}
