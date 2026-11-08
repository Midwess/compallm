#[derive(Debug, thiserror::Error)]
pub enum AuthorizationError {
    #[error("Cedar error {}", 0)]
    CedarScriptFailed(#[from] anyhow::Error),
}
