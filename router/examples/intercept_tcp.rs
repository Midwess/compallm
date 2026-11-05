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
    router.add_processors(HeaderReceived, InspectHeader { runs: 0 });
    // Synthetic instance only: the consumer supplies the real availability policy.
    let provider = Arc::new(ExampleProvider {
        name: "example-provider",
        available: true,
    });
    router.add_processors(
        BodyStarted,
        move |mut writer: ContextWriter| -> std::io::Result<()> {
            if !provider.available {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotConnected,
                    "provider unavailable",
                ));
            }
            writer.insert_resource(SelectedProvider(Arc::clone(&provider)));
            Ok(())
        },
    );
    router.add_processors(BodyStarted, |provider: Res<SelectedProvider>| {
        println!("selected provider={}", provider.0.name);
    });

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

// Router retains this instance, so its state survives between connections.
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
