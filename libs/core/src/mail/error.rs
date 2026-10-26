#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("email recipient is throttled")]
    Throttled,
    #[error("email queue remained full for 30 seconds")]
    QueueTimeout,
    #[error("email delivery is not configured")]
    Unavailable,
    #[error("email delivery failed")]
    Delivery(#[from] anyhow::Error),
}
