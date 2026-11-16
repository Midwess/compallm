pub mod input;
pub mod me;
pub mod mutation;
pub mod query;
mod r#type;
pub use r#type::{AvatarUpload, User};

pub(crate) fn avatar_url(
    path: &str,
    storage: &Option<std::sync::Arc<compallm_storage::S3Storage>>,
) -> Option<String> {
    let mut url = storage.as_ref()?.public_url(path).ok()?;
    // A new resolution refetches replacements at the fixed object key.
    let version = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?
        .as_nanos();
    url.push_str(&format!("?v={version}"));
    Some(url)
}
