//! Bounded, process-local SMTP delivery implementing Core's email capability.
pub mod config;
pub mod error;
mod recipient;
mod service;

pub use config::{SmtpSettings, SmtpTls};
pub use error::MailingError;
pub use service::{MailingService, MailingStatus};
