pub mod email;
pub mod error;
pub mod template;

pub use email::{EmailRequest, EmailService};
pub use error::EmailError;
pub use template::{EmailTemplate, OtpCode};
