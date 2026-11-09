//! Authentication inputs and safe session identity projections.

mod input;
pub(crate) mod query;
mod session;

pub use input::{CredentialDelivery, EmailCodeInput, EmailInput};
pub use session::Session;
