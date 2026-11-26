mod capture;
pub mod component;
mod container;
mod decoder;
mod encoder;
mod error;
mod grammar;
mod state;
mod token;
mod value;

pub use component::JsonToken;
pub(crate) use component::StringReference;
pub(crate) use component::{RetainedString, StringLayout};
pub use container::JsonContainer;
pub use decoder::JsonDecoder;
pub use encoder::{JsonEncodeError, JsonEncoder};
pub use error::JsonDecodeError;
pub use value::{JsonArgument, JsonValue};

pub const INTERNAL_BUFFER_BYTES: usize = 4 * 1024;
