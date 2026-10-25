use async_trait::async_trait;

use super::{EmailError, EmailTemplate};

/// Email delivery request. Templates contain secrets; do not log their output.
/// Transports render `text.to_string()` as HTML and `text.plain_text()` as text.
pub struct EmailRequest {
    pub to: String,
    pub subject: String,
    pub text: EmailTemplate,
}

#[async_trait]
pub trait EmailService: Send + Sync {
    async fn send(&self, request: EmailRequest) -> Result<(), EmailError>;
}
