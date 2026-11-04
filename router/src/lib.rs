//! A model-scoped TCP Router with local typed stages.
pub mod connection;
pub mod header;
pub mod processor;
pub mod resources;
pub mod router;
pub mod stage;

pub use connection::Connection;
pub use router::Router;
pub use stage::{BodyStarted, HeaderReceived};

pub mod prelude {
    pub use crate::{
        Router,
        header::RequestHeader,
        resources::{ContextWriter, Local, PeerAddress, Res, ResMut, Resources, RouterModel},
        stage::{BodyStarted, HeaderReceived, IntoProcessors, Stage, StageLabel},
    };
}
