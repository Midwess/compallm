//! A model-scoped TCP Router with local typed schedules.
pub mod connection;
pub mod header;
pub mod resources;
pub mod router;
pub mod schedule;
pub mod system;

pub use connection::Connection;
pub use router::Router;
pub use schedule::HeaderReceived;

pub mod prelude {
    pub use crate::{
        Router,
        header::RequestHeader,
        resources::{Local, PeerAddress, Res, ResMut, Resources, RouterModel},
        schedule::{HeaderReceived, IntoSystems, Schedule, ScheduleLabel},
    };
}
