pub mod processor;
pub mod resources;
pub mod stage;

use crate::{
    connection::Connection,
    stage::{IntoProcessors, Stage, StageLabel},
};
use compallm_core::model::Model;
use std::{io, sync::Arc};
use tokio::{
    net::TcpStream,
    task::{JoinError, JoinSet},
};

/// A caller-selected Core model and the handles of its connection tasks.
/// Dropping Router aborts tracked tasks. Consumers own admission limits.
pub struct Router {
    model: Arc<Model>,
    connections: JoinSet<io::Result<()>>,
    app: sec::App,
}

impl Router {
    pub fn new(model: Model) -> Self {
        Self {
            model: Arc::new(model),
            connections: JoinSet::new(),
            app: sec::App::new(),
        }
    }

    /// Register prototypes instantiated independently for every new connection.
    pub fn add_processors<M, C>(&mut self, label: impl StageLabel, processors: C)
    where
        C: IntoProcessors<M> + Clone + Send + Sync + 'static,
    {
        self.app.add_processors(label, processors);
    }

    /// Build connection-local processors, including non-Clone structs, through a factory.
    pub fn configure_stage(
        &mut self,
        label: impl StageLabel,
        configure: impl Fn(&mut Stage) + Send + Sync + 'static,
    ) {
        self.app.configure_stage(label, configure);
    }

    pub fn add_plugins<M>(&mut self, plugins: impl sec::plugin::Plugins<M>) {
        self.app.add_plugins(plugins);
    }

    pub fn insert_shared_resource<T: Send + Sync + 'static>(&mut self, resource: Arc<T>) {
        self.app.insert_shared_resource(resource);
    }

    pub fn set_max_parallelism(&mut self, limit: std::num::NonZeroUsize) {
        self.app.set_max_parallelism(limit);
    }

    pub fn model(&self) -> &Model {
        &self.model
    }

    /// Admit a supplied stream and spawn its run future on the active Tokio runtime.
    pub fn on_connection(&mut self, stream: TcpStream) -> io::Result<()> {
        let connection = Connection {
            model: Arc::clone(&self.model),
            peer_addr: stream.peer_addr()?,
            stream,
            execution: self.app.execution(),
        };
        self.connections.spawn(connection.run());
        Ok(())
    }

    /// Includes completed connections whose task results have not been reaped.
    pub fn pending_connections(&self) -> usize {
        self.connections.len()
    }

    /// Reap one task, preserving both task and connection I/O failures.
    pub async fn join_next(&mut self) -> Option<Result<io::Result<()>, JoinError>> {
        self.connections.join_next().await
    }
}
