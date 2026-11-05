//! A model-scoped TCP Router with local typed stages.
pub mod connection;
pub mod router;

pub use connection::header;
pub use router::{processor, resources, stage};

pub use connection::Connection;
pub use router::Router;
pub use stage::{BodyStarted, HeaderReceived};

pub mod prelude {
    pub use crate::{
        Router,
        header::RequestHeader,
        processor::Processor,
        resources::{
            ContextWriter, Local, PeerAddress, Res, ResMut, Resources, RouterModel, Shared,
        },
        stage::{BodyStarted, HeaderReceived, IntoProcessors, Stage, StageLabel},
    };
    pub use sec::{App, Execution, Plugin};
}
