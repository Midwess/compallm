pub mod header;

use crate::{
    resources::{PeerAddress, RouterModel},
    stage::{BodyStarted, HeaderReceived},
};
use compallm_core::model::Model;
use std::{io, net::SocketAddr, sync::Arc};
use tokio::{io::BufReader, net::TcpStream};

/// One stream with shared definitions and independently initialized processor state.
pub struct Connection {
    pub model: Arc<Model>,
    pub peer_addr: SocketAddr,
    pub(crate) stream: TcpStream,
    pub(crate) execution: sec::Execution,
}
impl Connection {
    /// Read one header, execute HeaderReceived, then enter BodyStarted. Later stages are future work.
    pub async fn run(mut self) -> io::Result<()> {
        self.execution
            .resources_mut()
            .insert(RouterModel(self.model));
        self.execution
            .resources_mut()
            .insert(PeerAddress(self.peer_addr));
        let mut reader = BufReader::with_capacity(4096, self.stream);
        let header = header::read_header(&mut reader).await?;
        self.execution.resources_mut().insert(header);
        self.execution.run_stage(HeaderReceived)?;
        self.execution.run_stage(BodyStarted)
    }
}
