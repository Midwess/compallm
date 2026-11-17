use compallm_geolocation::{CityDatabase, router};
use std::{net::SocketAddr, path::Path, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bind: SocketAddr = std::env::var("GEOLOCATION_BIND")
        .unwrap_or_else(|_| "127.0.0.1:41869".into())
        .parse()?;
    let path = std::env::var("DBIP_DATABASE")
        .unwrap_or_else(|_| "artifacts/geoip/dbip-city-lite.mmdb".into());
    let database = Arc::new(CityDatabase::open(Path::new(&path))?);
    let listener = tokio::net::TcpListener::bind(bind).await?;
    println!("Geolocation listening on {}", listener.local_addr()?);
    axum::serve(listener, router(database))
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await?;
    Ok(())
}
