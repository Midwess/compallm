pub mod header;

use crate::{
    resources::{PeerAddress, Resources, RouterModel},
    stage::{BodyStarted, HeaderReceived, Stages},
};
use compallm_core::model::Model;
use std::{
    io,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::{io::BufReader, net::TcpStream};

/// One stream and a shared handle to the Router's persistent stages.
pub struct Connection {
    pub model: Arc<Model>,
    pub peer_addr: SocketAddr,
    pub(crate) stream: TcpStream,
    pub(crate) stages: Arc<Mutex<Stages>>,
}
impl Connection {
    /// Read one header, execute HeaderReceived, then enter BodyStarted. Later stages are future work.
    pub async fn run(self) -> io::Result<()> {
        let mut resources = Resources::default();
        resources.insert(RouterModel(self.model));
        resources.insert(PeerAddress(self.peer_addr));
        let mut reader = BufReader::with_capacity(4096, self.stream);
        let header = header::read_header(&mut reader).await?;
        resources.insert(header);
        // Synchronous execution only: no mutex guard is held while awaiting I/O.
        let mut stages = self
            .stages
            .lock()
            .map_err(|_| io::Error::other("Router stages poisoned"))?;
        stages.run(HeaderReceived, &mut resources)?;
        stages.run(BodyStarted, &mut resources)
    }
}
