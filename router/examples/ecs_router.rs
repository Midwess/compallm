//! Run with `cargo run -p router --example ecs_router`.
//!
//! Three loopback streams demonstrate the existing HeaderReceived / BodyStarted
//! lifecycle. Auth and providers are synthetic; the Router stops before reading
//! the body. Streaming, persistence, and replay in ECS-Router.drawio are future work.

use compallm_core::{
    model::Model,
    provider::{ModelLimits, Pricing, RequestPricing, ServiceKind},
};
use router::prelude::*;
use std::{io, num::NonZeroUsize, sync::Arc};
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

// Ordinary structs become typed Resources without a derive macro.
struct Principal {
    user: &'static str,
    organization_id: u64,
}

struct SelectedProvider(Arc<DemoProvider>);

// Shared services belong to App; connection-local Resources hold their results.
struct DemoAuth;

impl DemoAuth {
    async fn authenticate(&self, header: &RequestHeader) -> io::Result<Principal> {
        // Stand in for an asynchronous credential lookup without an external service.
        tokio::task::yield_now().await;
        let credential = header
            .fields
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("authorization"))
            .map(|(_, value)| value.as_slice());
        let user = match credential {
            Some(b"Bearer demo-alice") => "alice",
            Some(b"Bearer demo-bob") => "bob",
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "demo credential rejected",
                ));
            }
        };
        Ok(Principal {
            user,
            organization_id: 10,
        })
    }
}

struct DemoProvider {
    name: &'static str,
    model_id: u64,
    available: bool,
}

struct ProviderPool {
    instances: Vec<Arc<DemoProvider>>,
}

struct AuthorizationPlugin;

impl Plugin for AuthorizationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_shared_resource(Arc::new(DemoAuth));
        app.add_processors(
            HeaderReceived,
            (
                async_processor(authorize),
                check_model_access.after(authorize),
                inspect_header.after(authorize),
            ),
        );
    }
}

struct RoutingPlugin {
    providers: Arc<ProviderPool>,
}

impl Plugin for RoutingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_shared_resource(Arc::clone(&self.providers));
        app.add_processors(BodyStarted, (select_provider, inspect_provider).chain());
    }
}

async fn authorize(
    header: Owned<RequestHeader>,
    auth: Shared<DemoAuth>,
    mut writer: ContextWriter,
) -> io::Result<()> {
    // Owned clones the header, so no hecs borrow is held across this await.
    let principal = auth.authenticate(&header).await?;
    println!(
        "authorize: user={} target={}",
        principal.user, header.target
    );
    writer.insert_resource(principal);
    Ok(())
}

fn check_model_access(principal: Res<Principal>, model: Res<RouterModel>) -> io::Result<()> {
    if !model.0.enabled || principal.organization_id != model.0.organization_id {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "model is unavailable to this principal",
        ));
    }
    println!("check_model_access: user={} allowed", principal.user);
    Ok(())
}

fn inspect_header(header: Res<RequestHeader>, principal: Res<Principal>, mut runs: Local<usize>) {
    // This processor has its own counter in every connection's Execution.
    *runs += 1;
    println!(
        "inspect_header: user={} method={} target={} local_runs={}",
        principal.user, header.method, header.target, *runs,
    );
}

fn select_provider(
    principal: Res<Principal>,
    model: Res<RouterModel>,
    providers: Shared<ProviderPool>,
    mut writer: ContextWriter,
) -> io::Result<()> {
    // Read the same availability snapshot in every connection, without probing.
    let provider = providers
        .instances
        .iter()
        .find(|provider| provider.available && provider.model_id == model.0.id)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotConnected, "no demo provider available"))?;
    println!(
        "select_provider: user={} provider={}",
        principal.user, provider.name,
    );
    writer.insert_resource(SelectedProvider(Arc::clone(provider)));
    Ok(())
}

fn inspect_provider(
    provider: Res<SelectedProvider>,
    principal: Res<Principal>,
    model: Res<RouterModel>,
) {
    // chain() makes SEC apply SelectedProvider before this processor runs.
    println!(
        "inspect_provider: user={} model={} provider={}",
        principal.user, model.0.name, provider.0.name,
    );
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let model = Model {
        id: 1,
        organization_id: 10,
        name: "general-chat".into(),
        service_kind: ServiceKind::Llm,
        enabled: true,
        capabilities: vec![],
        limits: ModelLimits::default(),
        thinking: None,
        pricing: Pricing::Request(RequestPricing {
            usd_per_request: 0.into(),
        }),
    };
    let providers = Arc::new(ProviderPool {
        instances: vec![
            Arc::new(DemoProvider {
                name: "unavailable-demo",
                model_id: model.id,
                available: false,
            }),
            Arc::new(DemoProvider {
                name: "available-demo",
                model_id: model.id,
                available: true,
            }),
        ],
    });

    let mut router = Router::new(model);
    router.set_max_parallelism(NonZeroUsize::new(2).unwrap());
    router.add_plugins((AuthorizationPlugin, RoutingPlugin { providers }));

    // Socket acquisition belongs to the caller. All three connections are
    // admitted before their headers arrive; Router owns no listener.
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let mut clients = Vec::new();
    for _ in 0..3 {
        let client = TcpStream::connect(listener.local_addr()?).await?;
        let (stream, _) = listener.accept().await?;
        router.on_connection(stream)?;
        clients.push(client);
    }
    for (client, (target, credential)) in clients.iter_mut().zip([
        ("/alice", "demo-alice"),
        ("/bob", "demo-bob"),
        ("/rejected", "invalid"),
    ]) {
        let request = format!(
            "POST {target} HTTP/1.1\r\nHost: localhost\r\nAuthorization: Bearer {credential}\r\nContent-Length: 0\r\n\r\n",
        );
        client.write_all(request.as_bytes()).await?;
    }

    let (mut accepted, mut rejected) = (0, 0);
    while let Some(result) = router.join_next().await {
        match result.map_err(io::Error::other)? {
            Ok(()) => accepted += 1,
            Err(error) if error.kind() == io::ErrorKind::PermissionDenied => {
                rejected += 1;
                println!("connection rejected: {error}");
            }
            Err(error) => return Err(error),
        }
    }
    println!(
        "Finished: {accepted} accepted, {rejected} rejected. No body or upstream request was sent."
    );
    Ok(())
}
