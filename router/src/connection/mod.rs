pub mod header;

use crate::{
    resources::{PeerAddress, Resources, RouterModel},
    stage::{BodyStarted, HeaderReceived, StageDefinitions, Stages},
};
use compallm_core::model::Model;
use std::{io, net::SocketAddr, sync::Arc};
use tokio::{io::BufReader, net::TcpStream};

/// One stream with shared definitions and independently initialized processor state.
pub struct Connection {
    pub model: Arc<Model>,
    pub peer_addr: SocketAddr,
    pub(crate) stream: TcpStream,
    pub(crate) stages: Arc<StageDefinitions>,
}
impl Connection {
    /// Read one header, execute HeaderReceived, then enter BodyStarted. Later stages are future work.
    pub async fn run(self) -> io::Result<()> {
        let mut stages = Stages::new(self.stages);
        let mut resources = Resources::default();
        resources.insert(RouterModel(self.model));
        resources.insert(PeerAddress(self.peer_addr));
        let mut reader = BufReader::with_capacity(4096, self.stream);
        let header = header::read_header(&mut reader).await?;
        resources.insert(header);
        stages.run(HeaderReceived, &mut resources)?;
        stages.run(BodyStarted, &mut resources)
    }
}
