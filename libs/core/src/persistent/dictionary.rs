use crate::compression::{DCompression, DictionaryType};

/// Committed, versioned compression dictionaries.
#[async_trait::async_trait]
pub trait DictionaryRepository: Send + Sync {
    async fn latest(&self, kind: DictionaryType) -> anyhow::Result<Option<&DCompression>>;
    async fn get(
        &self,
        kind: DictionaryType,
        version: u32,
    ) -> anyhow::Result<Option<&DCompression>>;
}
