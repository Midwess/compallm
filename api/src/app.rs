use async_trait::async_trait;
use axum::Router as AxumRouter;
#[cfg(feature = "frontend-dev")]
use axum_frontend::{DevConfig, shutdown_requested};
use compallm_postgres::{Migrator, init_id_generator};
use loco_rs::boot::ServeParams;
use loco_rs::{
    Result,
    app::{AppContext, Hooks, Initializer},
    bgworker::Queue,
    boot::{BootResult, StartMode, create_app},
    config::Config,
    controller::AppRoutes,
    environment::Environment,
    task::Tasks,
};
use std::path::Path;

use crate::di::Di;

#[cfg(feature = "frontend-dev")]
const PORTAL_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../portal");

pub struct App;

impl App {
    pub(crate) fn with_api_fallback(router: AxumRouter) -> AxumRouter {
        let not_found = || async {
            (
                axum::http::StatusCode::NOT_FOUND,
                axum::Json(crate::error::ErrorResponse {
                    message: "API endpoint not found".into(),
                }),
            )
        };
        router
            .route("/api", axum::routing::any(not_found))
            .route("/api/{*unmatched}", axum::routing::any(not_found))
    }

    pub(crate) async fn tcp_listener(port: i32) -> Result<tokio::net::TcpListener> {
        let start = u16::try_from(port)
            .ok()
            .filter(|port| (1..=50100).contains(port))
            .ok_or_else(|| {
                loco_rs::Error::Message("API starting port must be between 1 and 50100".into())
            })?;
        let connection = hero::tcp::listener::find_tcp_listener(Some(start))
            .await
            .map_err(|error| loco_rs::Error::Message(error.to_string()))?;
        let address = connection.listener.local_addr()?;
        tracing::info!(requested_port=start,port=address.port(),binding=%address.ip(),
            url=%format!("http://127.0.0.1:{}",address.port()),"API server listening");
        Ok(connection.listener)
    }
}

#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(
        mode: StartMode,
        environment: &Environment,
        config: Config,
    ) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment, config).await
    }

    async fn load_config(environment: &Environment) -> Result<Config> {
        environment.load_from_folder(Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../config")))
    }

    async fn serve(app: AxumRouter, ctx: &AppContext, serve_params: &ServeParams) -> Result<()> {
        use std::net::SocketAddr;

        let listener = Self::tcp_listener(serve_params.port).await?;
        #[cfg(feature = "frontend-dev")]
        let dev_server = {
            crate::generate_contract()
                .map_err(|error| loco_rs::Error::Message(error.to_string()))?;
            let vite = crate::node_binary(Path::new(PORTAL_ROOT), "vite")
                .to_string_lossy()
                .into_owned();
            DevConfig::vite()
                .command([vite])
                .cwd(PORTAL_ROOT)
                .port(41868)
                .spawn()
                .await?
        };
        #[cfg(feature = "frontend-dev")]
        let app = app.merge(dev_server.router(["/"]));

        #[cfg(feature = "frontend-dev")]
        {
            tokio::select! {
                result = axum::serve(
                    listener,
                    app.into_make_service_with_connect_info::<SocketAddr>(),
                ) => result?,
                () = shutdown_requested() => {}
            }

            drop(dev_server);
            Self::on_shutdown(ctx).await;
        }

        #[cfg(not(feature = "frontend-dev"))]
        {
            let shutdown_context = ctx.clone();
            axum::serve(
                listener,
                app.into_make_service_with_connect_info::<SocketAddr>(),
            )
            .with_graceful_shutdown(async move {
                loco_rs::boot::shutdown_signal().await;
                tracing::info!("shutting down...");
                Self::on_shutdown(&shutdown_context).await;
            })
            .await?;
        }
        Ok(())
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![])
    }

    async fn after_context(ctx: AppContext) -> Result<AppContext> {
        init_id_generator(Self::app_name())
            .map_err(|error| loco_rs::Error::Message(error.to_string()))?;
        Di::init(&ctx).await?;
        if let Some(worker) = Di::instance().mailing_worker() {
            use loco_rs::bgworker::BackgroundWorker;
            tokio::spawn(async move {
                if worker.perform(()).await.is_err() {
                    tracing::error!("mailing worker stopped unexpectedly");
                }
            });
        }
        Ok(ctx)
    }

    async fn on_shutdown(_ctx: &AppContext) {
        if let Some(service) = Di::instance().mailing_service() {
            service.shutdown().await;
        }
    }

    async fn before_routes(_ctx: &AppContext) -> Result<AxumRouter<AppContext>> {
        let mut router = AxumRouter::new();
        Di::instance().inject(&mut router);
        Ok(router)
    }

    async fn after_routes(router: AxumRouter, _ctx: &AppContext) -> Result<AxumRouter> {
        let router = Self::with_api_fallback(router);
        #[cfg(not(feature = "frontend-dev"))]
        {
            use axum_frontend::static_router;

            return Ok(router.merge(static_router(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../portal/dist"
            ))));
        }

        #[cfg(feature = "frontend-dev")]
        Ok(router)
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes()
    }

    async fn connect_workers(_ctx: &AppContext, _queue: &Queue) -> Result<()> {
        Ok(())
    }

    fn register_tasks(_tasks: &mut Tasks) {}

    async fn truncate(_ctx: &AppContext) -> Result<()> {
        Ok(())
    }

    async fn seed(_ctx: &AppContext, _base: &Path) -> Result<()> {
        Ok(())
    }
}
