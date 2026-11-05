//! Small, protocol-independent execution: typed resources, processors, stages, and plugins.
#![forbid(unsafe_code)]
pub mod app;
pub mod execution;
pub mod plugin;
pub mod processor;
pub mod resources;
pub mod stage;

pub use app::App;
pub use execution::Execution;
pub use plugin::Plugin;

pub mod prelude {
    pub use crate::{
        App, Execution, Plugin,
        processor::{IntoProcessor, Processor},
        resources::{ContextWriter, Local, Res, ResMut, Resources, Shared},
        stage::{IntoProcessors, Stage, StageLabel},
    };
}
