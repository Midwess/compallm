#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("email delivery is not configured")]
    Unavailable,
    #[error("email delivery failed")]
    Delivery(#[from] anyhow::Error),
}
