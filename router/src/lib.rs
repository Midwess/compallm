//! A model-scoped TCP Router with local typed schedules.
pub mod connection;
pub mod header;
pub mod resources;
pub mod router;
pub mod schedule;
pub mod system;

pub use connection::Connection;
pub use router::Router;
pub use schedule::{BodyStarted, HeaderReceived};

pub mod prelude {
    pub use crate::{
        Router,
        header::RequestHeader,
        resources::{Commands, Local, PeerAddress, Res, ResMut, Resources, RouterModel},
        schedule::{BodyStarted, HeaderReceived, IntoSystems, Schedule, ScheduleLabel},
    };
}
