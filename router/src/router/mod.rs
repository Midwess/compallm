pub mod processor;
pub mod resources;
pub mod stage;

use crate::{
    connection::Connection,
    stage::{IntoProcessors, Stage, StageDefinitions, StageLabel},
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
    stages: Arc<StageDefinitions>,
}

impl Router {
    pub fn new(model: Model) -> Self {
        Self {
            model: Arc::new(model),
            connections: JoinSet::new(),
            stages: Arc::new(StageDefinitions::default()),
        }
    }

    /// Register prototypes instantiated independently for every new connection.
    pub fn add_processors<M, C>(&mut self, label: impl StageLabel, processors: C)
    where
        C: IntoProcessors<M> + Clone + Send + Sync + 'static,
    {
        Arc::make_mut(&mut self.stages).add(label, processors);
    }

    /// Build connection-local processors, including non-Clone structs, through a factory.
    pub fn configure_stage(
        &mut self,
        label: impl StageLabel,
        configure: impl Fn(&mut Stage) + Send + Sync + 'static,
    ) {
        Arc::make_mut(&mut self.stages).configure(label, configure);
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
            stages: Arc::clone(&self.stages),
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
