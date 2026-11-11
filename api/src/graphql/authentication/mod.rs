//! Authentication inputs and safe session identity projections.

pub(crate) mod input;
pub(crate) mod mutation;
mod r#type;
pub(crate) use r#type::{EmailChallenge, SessionCredentials};
pub(crate) mod query;
mod session;

pub use session::Session;
