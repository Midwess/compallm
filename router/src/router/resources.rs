use compallm_core::model::Model;
pub use sec::resources::*;
use std::{net::SocketAddr, sync::Arc};

pub struct RouterModel(pub Arc<Model>);
pub struct PeerAddress(pub SocketAddr);
