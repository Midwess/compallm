//! Authentication inputs and safe session identity projections.

mod input;
mod query;
mod session;

pub use input::{CredentialDelivery, EmailCodeInput, EmailInput};
pub(crate) use query::AuthQuery;
pub use session::Session;
