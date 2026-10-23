use async_trait::async_trait;
use axum::Router as AxumRouter;
#[cfg(feature = "frontend-dev")]
use axum_frontend::{DevConfig, shutdown_requested};
use compallm_postgres::{Migrator, init_id_generator};
#[cfg(feature = "frontend-dev")]
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

use crate::{controllers, controllers::greeting::RequestCounter};

#[cfg(feature = "frontend-dev")]
const PROJECT_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");

pub struct App;

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

    #[cfg(feature = "frontend-dev")]
    async fn serve(app: AxumRouter, ctx: &AppContext, serve_params: &ServeParams) -> Result<()> {
        use std::net::SocketAddr;

        crate::generate_contract().map_err(|error| loco_rs::Error::Message(error.to_string()))?;
        let vite = crate::node_binary(Path::new(PROJECT_ROOT), "vite")
            .to_string_lossy()
            .into_owned();
        let dev_server = DevConfig::vite()
            .command([vite])
            .cwd(PROJECT_ROOT)
            .port(5173)
            .spawn()
            .await?;
        let app = app.merge(dev_server.router(["/"]));
        let listener = tokio::net::TcpListener::bind(format!(
            "{}:{}",
            serve_params.binding, serve_params.port
        ))
        .await?;

        tokio::select! {
            result = axum::serve(
                listener,
                app.into_make_service_with_connect_info::<SocketAddr>(),
            ) => result?,
            () = shutdown_requested() => {}
        }

        drop(dev_server);
        Self::on_shutdown(ctx).await;
        Ok(())
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![])
    }

    async fn after_context(ctx: AppContext) -> Result<AppContext> {
        init_id_generator(Self::app_name())
            .map_err(|error| loco_rs::Error::Message(error.to_string()))?;
        ctx.shared_store.insert(RequestCounter::default());
        Ok(ctx)
    }

    async fn before_routes(_ctx: &AppContext) -> Result<AxumRouter<AppContext>> {
        let (router, _) = controllers::api_router().split_for_parts();
        Ok(router)
    }

    async fn after_routes(router: AxumRouter, _ctx: &AppContext) -> Result<AxumRouter> {
        #[cfg(not(feature = "frontend-dev"))]
        {
            use axum_frontend::static_router;

            return Ok(router.merge(static_router(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../dist"
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
