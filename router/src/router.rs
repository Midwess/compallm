use crate::{
    connection::Connection,
    schedule::{IntoSystems, Schedule, ScheduleDefinitions, ScheduleLabel},
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
    schedules: ScheduleDefinitions,
}

impl Router {
    pub fn new(model: Model) -> Self {
        Self {
            model: Arc::new(model),
            connections: JoinSet::new(),
            schedules: ScheduleDefinitions::default(),
        }
    }

    /// Register typed systems for future connections, executed in registration order.
    pub fn add_systems<M, C>(&mut self, label: impl ScheduleLabel, systems: C)
    where
        C: IntoSystems<M> + Clone + Send + Sync + 'static,
    {
        self.schedules.add(label, systems);
    }

    /// Configure an independently initialized schedule for each connection.
    pub fn configure_schedule(
        &mut self,
        label: impl ScheduleLabel,
        configure: impl Fn(&mut Schedule) + Send + Sync + 'static,
    ) {
        self.schedules.configure(label, configure);
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
            schedules: self.schedules.build(),
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
