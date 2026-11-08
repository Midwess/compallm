use compallm_core::{
    compression::{DCompression, DictionaryType},
    persistent::DbContext,
};
use elsa::sync::FrozenMap;
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder};
use tokio::sync::Mutex;

use crate::{
    ZstdDictionarySchema, ZstdDictionaryTypeSchema, postgres_connection::PostgresConnection,
};

type Dictionaries = <ZstdDictionarySchema as ModelTrait>::Entity;
type Column = <Dictionaries as EntityTrait>::Column;

impl From<DictionaryType> for ZstdDictionaryTypeSchema {
    fn from(kind: DictionaryType) -> Self {
        match kind {
            DictionaryType::AccessToken => Self::AccessToken,
        }
    }
}

/// Immutable version cache. References remain valid until this repository is dropped.
#[derive(Default)]
pub struct DictionaryRepository {
    cache: FrozenMap<(DictionaryType, u32), Box<DCompression>>,
    loading: Mutex<()>,
    committed_db: Option<DbContext>,
}

impl DictionaryRepository {
    /// Read only from this committed connection, even when the caller has a transaction.
    pub fn new(committed_db: DbContext) -> anyhow::Result<Self> {
        anyhow::ensure!(
            !committed_db.postgres().is_transaction(),
            "dictionary connection must not be a transaction"
        );
        Ok(Self {
            committed_db: Some(committed_db),
            ..Self::default()
        })
    }

    /// Fetch the current highest committed version and reuse its cached dictionary.
    pub async fn latest(&self, kind: DictionaryType) -> anyhow::Result<Option<&DCompression>> {
        let context = match &self.committed_db {
            Some(db) => db.clone(),
            None => DbContext::current()?,
        };
        anyhow::ensure!(
            !context.postgres().is_transaction(),
            "dictionary reads require a committed connection scope"
        );
        let db = PostgresConnection::from_context(&context)?;
        let row = Dictionaries::find()
            .filter(Column::Type.eq(ZstdDictionaryTypeSchema::from(kind)))
            .order_by_desc(Column::Version)
            .one(db)
            .await?;
        match row {
            Some(row) => self.get(kind, u32::try_from(row.version)?).await,
            None => Ok(None),
        }
    }

    /// Load an exact version's ordered prefix once; missing versions are not cached.
    /// The cache cannot contain uncommitted transaction state.
    pub async fn get(
        &self,
        kind: DictionaryType,
        version: u32,
    ) -> anyhow::Result<Option<&DCompression>> {
        let version_db = i32::try_from(version)?;
        let context = match &self.committed_db {
            Some(db) => db.clone(),
            None => DbContext::current()?,
        };
        anyhow::ensure!(
            !context.postgres().is_transaction(),
            "dictionary reads require a committed connection scope"
        );
        let key = (kind, version);
        if let Some(dictionary) = self.cache.get(&key) {
            return Ok(Some(dictionary));
        }
        let _guard = self.loading.lock().await;
        if let Some(dictionary) = self.cache.get(&key) {
            return Ok(Some(dictionary));
        }
        let db = PostgresConnection::from_context(&context)?;
        let rows = Dictionaries::find()
            .filter(Column::Type.eq(ZstdDictionaryTypeSchema::from(kind)))
            .filter(Column::Version.lte(version_db))
            .order_by_asc(Column::Version)
            .all(db)
            .await?;
        if rows.last().is_none_or(|row| row.version != version_db) {
            return Ok(None);
        }
        let content: String = rows.into_iter().map(|row| row.content).collect();
        Ok(Some(self.cache.insert(
            key,
            Box::new(DCompression::new(kind, version, &content)?),
        )))
    }
}

#[async_trait::async_trait]
impl compallm_core::persistent::dictionary::DictionaryRepository for DictionaryRepository {
    async fn latest(&self, kind: DictionaryType) -> anyhow::Result<Option<&DCompression>> {
        DictionaryRepository::latest(self, kind).await
    }

    async fn get(
        &self,
        kind: DictionaryType,
        version: u32,
    ) -> anyhow::Result<Option<&DCompression>> {
        DictionaryRepository::get(self, kind, version).await
    }
}
