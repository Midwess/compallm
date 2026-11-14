use anyhow::Context;
use compallm_core::{
    persistent::{
        DbContext,
        provider::{ProviderRepository as CoreProviderRepository, ProviderRepositoryError},
    },
    provider::Provider,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DbErr, EntityTrait, ModelTrait, QueryFilter,
    QueryOrder, QuerySelect, RuntimeErr,
};

use crate::{
    mappers::provider::ProviderInsert, postgres_connection::PostgresConnection,
    schema::ProviderSchema,
};

type Providers = <ProviderSchema as ModelTrait>::Entity;
type ProviderColumn = <Providers as EntityTrait>::Column;
type ProviderActiveModel = <Providers as EntityTrait>::ActiveModel;

#[derive(Default)]
pub struct ProviderRepository;

impl ProviderRepository {
    fn database_id(id: u64, field: &str) -> Result<i64, ProviderRepositoryError> {
        if id == 0 {
            return Err(ProviderRepositoryError::database(anyhow::anyhow!(
                "{field} must be positive"
            )));
        }
        crate::to_bigint(id)
            .with_context(|| format!("{field} exceeds PostgreSQL BIGINT"))
            .map_err(Into::into)
    }

    fn write_error(error: DbErr) -> ProviderRepositoryError {
        if let DbErr::Exec(RuntimeErr::SqlxError(driver))
        | DbErr::Query(RuntimeErr::SqlxError(driver)) = &error
            && let sea_orm::sqlx::Error::Database(database) = driver.as_ref()
            && database.is_unique_violation()
            && database.constraint() == Some("providers_organization_id_name_unique")
        {
            return ProviderRepositoryError::NameAlreadyExists;
        }
        ProviderRepositoryError::database(error)
    }
}

#[async_trait::async_trait]
impl CoreProviderRepository for ProviderRepository {
    async fn find_by_id(
        &self,
        organization_id: u64,
        id: u64,
    ) -> Result<Option<Provider>, ProviderRepositoryError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        Providers::find_by_id(Self::database_id(id, "resource ID")?)
            .filter(
                ProviderColumn::OrganizationId
                    .eq(Self::database_id(organization_id, "organization ID")?),
            )
            .one(db)
            .await
            .map_err(ProviderRepositoryError::database)?
            .map(Provider::try_from)
            .transpose()
    }
    async fn find_model_by_id(
        &self,
        organization_id: u64,
        id: u64,
    ) -> Result<Option<compallm_core::provider::ProviderModel>, ProviderRepositoryError> {
        use crate::ProviderModelSchema;
        type ProviderModels = <ProviderModelSchema as ModelTrait>::Entity;
        type Column = <ProviderModels as EntityTrait>::Column;
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        ProviderModels::find_by_id(Self::database_id(id, "provider model ID")?)
            .filter(
                Column::OrganizationId.eq(Self::database_id(organization_id, "organization ID")?),
            )
            .one(db)
            .await
            .map_err(ProviderRepositoryError::database)?
            .map(compallm_core::provider::ProviderModel::try_from)
            .transpose()
    }

    async fn find_all(
        &self,
        organization_id: u64,
        limit: u64,
    ) -> Result<Vec<Provider>, ProviderRepositoryError> {
        let organization_id = Self::database_id(organization_id, "organization ID")?;
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;

        Providers::find()
            .filter(ProviderColumn::OrganizationId.eq(organization_id))
            .order_by_asc(ProviderColumn::Id)
            .limit(limit)
            .all(db)
            .await
            .map_err(ProviderRepositoryError::database)?
            .into_iter()
            .map(Provider::try_from)
            .collect()
    }

    async fn insert(&self, provider: Provider) -> Result<Provider, ProviderRepositoryError> {
        let input = ProviderInsert::try_from(provider)?;
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;

        let stored = if input.model_writes.is_empty() {
            input.provider.insert(db).await.map_err(Self::write_error)?
        } else {
            compallm_core::transaction!(context, {
                let current = DbContext::current()?;
                let db = PostgresConnection::from_context(&current)?;
                let stored = input.provider.insert(db).await.map_err(Self::write_error)?;
                <crate::ProviderModelSchema as ModelTrait>::Entity::insert_many(input.model_writes)
                    .exec_without_returning(db)
                    .await
                    .map_err(Self::write_error)?;
                Ok::<_, ProviderRepositoryError>(stored)
            })?
        };
        let mut provider = Provider::try_from(stored)?;
        provider.models = input.models;
        Ok(provider)
    }

    async fn update(&self, provider: Provider) -> Result<Provider, ProviderRepositoryError> {
        let organization_id = Self::database_id(provider.organization_id, "organization ID")?;
        let provider_id = Self::database_id(provider.id, "provider ID")?;
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;

        let model = Providers::find_by_id(provider_id)
            .filter(ProviderColumn::OrganizationId.eq(organization_id))
            .one(db)
            .await
            .map_err(ProviderRepositoryError::database)?
            .ok_or(ProviderRepositoryError::NotFound)?;
        let mut model: ProviderActiveModel = model.into();
        model.name = Set(provider.name);
        model.connection =
            Set(serde_json::to_value(provider.connection).context("invalid provider connection")?);
        model.headers =
            Set(serde_json::to_value(provider.headers).context("invalid provider headers")?);
        model.credentials = Set(provider.api_key.into());
        Provider::try_from(model.update(db).await.map_err(Self::write_error)?)
    }

    async fn delete(
        &self,
        organization_id: u64,
        provider_id: u64,
    ) -> Result<Provider, ProviderRepositoryError> {
        let organization_id = Self::database_id(organization_id, "organization ID")?;
        let provider_id = Self::database_id(provider_id, "provider ID")?;
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;

        let mut providers = Providers::delete_many()
            .filter(ProviderColumn::Id.eq(provider_id))
            .filter(ProviderColumn::OrganizationId.eq(organization_id))
            .exec_with_returning(db)
            .await
            .map_err(ProviderRepositoryError::database)?;
        Provider::try_from(providers.pop().ok_or(ProviderRepositoryError::NotFound)?)
    }
}
