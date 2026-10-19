use std::error::Error;

#[cfg(not(feature = "frontend-dev"))]
use axum_frontend::static_router;
#[cfg(feature = "frontend-dev")]
use axum_frontend::{DevConfig, shutdown_requested};
use cosin_server::api;

#[cfg(feature = "frontend-dev")]
const PROJECT_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "cosin=info,axum_frontend=info".into()),
        )
        .init();

    let api = api::router();

    #[cfg(feature = "frontend-dev")]
    let (app, dev_server) = {
        let dev_server = DevConfig::vite()
            .command(["npm", "run", "dev:vite", "--"])
            .cwd(PROJECT_ROOT)
            .port(5173)
            .spawn()
            .await?;

        let app = api.merge(dev_server.router(["/"]));
        (app, dev_server)
    };

    #[cfg(not(feature = "frontend-dev"))]
    let app = api.merge(static_router(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../dist"
    )));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    tracing::info!("application available at http://127.0.0.1:3000");

    #[cfg(feature = "frontend-dev")]
    {
        tokio::select! {
            result = axum::serve(listener, app) => result?,
            () = shutdown_requested() => {}
        }

        // Dropping the guard stops Vite and its HMR WebSocket cleanly.
        drop(dev_server);
    }

    #[cfg(not(feature = "frontend-dev"))]
    axum::serve(listener, app).await?;

    Ok(())
}
