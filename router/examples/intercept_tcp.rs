use compallm_core::{
    model::Model,
    provider::{ModelLimits, Pricing, RequestPricing, ServiceKind},
};
use router::prelude::*;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:7000".into());
    let listener = TcpListener::bind(address).await?;
    println!("general-chat listening on {}", listener.local_addr()?);
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
            usd_per_request: "0".parse().unwrap(),
        }),
    };
    let mut router = Router::new(model);
    router.add_plugins((
        HeaderPlugin,
        ProviderSelectionPlugin {
            provider: Arc::new(ExampleProvider {
                name: "example-provider",
                available: true,
            }),
        },
    ));

    // Socket acquisition stays outside Router.
    let (stream, _) = listener.accept().await?;
    drop(listener);
    router.on_connection(stream)?;
    tokio::select! {
        result = router.join_next() => {
            if let Some(result) = result { result??; }
        }
        result = tokio::signal::ctrl_c() => { result?; }
    }
    Ok(())
}

// Router clones this prototype; each connection owns its own counter.
#[derive(Clone)]
struct InspectHeader {
    runs: usize,
}

impl Processor for InspectHeader {
    fn run(&mut self, resources: &Resources) -> std::io::Result<()> {
        let header = resources.get::<RequestHeader>()?;
        let model = resources.get::<RouterModel>()?;
        let peer = resources.get::<PeerAddress>()?;
        self.runs += 1;
        println!(
            "run={} model={} peer={} method={} target={}",
            self.runs, model.0.name, peer.0, header.method, header.target
        );
        Ok(())
    }
}

struct ExampleProvider {
    name: &'static str,
    available: bool,
}
struct SelectedProvider(Arc<ExampleProvider>);

struct HeaderPlugin;
impl Plugin for HeaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_processors(HeaderReceived, InspectHeader { runs: 0 });
    }
}

struct ProviderSelectionPlugin {
    provider: Arc<ExampleProvider>,
}
impl Plugin for ProviderSelectionPlugin {
    fn build(&self, app: &mut App) {
        // A shared instance; readiness policy is still supplied by the consumer.
        app.insert_shared_resource(Arc::clone(&self.provider));
        app.add_processors(
            BodyStarted,
            (select_provider, |provider: Res<SelectedProvider>| {
                println!("selected provider={}", provider.0.name);
            })
                .chain(),
        );
    }
}

fn select_provider(
    provider: Shared<ExampleProvider>,
    mut writer: ContextWriter,
) -> std::io::Result<()> {
    if !provider.available {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotConnected,
            "provider unavailable",
        ));
    }
    // Clone the underlying instance handle; SEC owns only the shared-resource view.
    writer.insert_resource(SelectedProvider(provider.into_arc()));
    Ok(())
}
