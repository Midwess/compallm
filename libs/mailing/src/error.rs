#[derive(Debug, thiserror::Error)]
pub enum MailingError {
    #[error("invalid SMTP configuration: {0}")]
    Configuration(&'static str),
}
