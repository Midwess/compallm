use crate::{
    header,
    resources::{PeerAddress, Resources, RouterModel},
    schedule::{BodyStarted, HeaderReceived, Schedules},
};
use compallm_core::model::Model;
use std::{io, net::SocketAddr, sync::Arc};
use tokio::{io::BufReader, net::TcpStream};

/// One stream and its independently initialized schedules.
pub struct Connection {
    pub model: Arc<Model>,
    pub peer_addr: SocketAddr,
    pub(crate) stream: TcpStream,
    pub(crate) schedules: Schedules,
}
impl Connection {
    /// Read one header, execute HeaderReceived, then enter BodyStarted. Later stages are future work.
    pub async fn run(mut self) -> io::Result<()> {
        let mut resources = Resources::default();
        resources.insert(RouterModel(self.model));
        resources.insert(PeerAddress(self.peer_addr));
        let mut reader = BufReader::with_capacity(4096, self.stream);
        let header = header::read_header(&mut reader).await?;
        resources.insert(header);
        self.schedules.run(HeaderReceived, &mut resources)?;
        self.schedules.run(BodyStarted, &mut resources)
    }
}
