use anyhow::Context;
use compallm_core::{
    persistent::{
        DbContext,
        provider::{
            MAX_PROVIDER_FIND_LIMIT, MAX_PROVIDERS_PER_ORGANIZATION,
            ProviderRepository as CoreProviderRepository, ProviderRepositoryError,
        },
    },
    provider::{ApiProtocol, NewProvider, Provider, ProviderUpdate},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DbErr, EntityTrait, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, RuntimeErr, prelude::ChronoUtc,
};

use crate::{
    postgres_connection::PostgresConnection,
    schema::{ApiProtocolSchema, OrganizationSchema, ProviderSchema},
};

type Providers = <ProviderSchema as ModelTrait>::Entity;
type ProviderColumn = <Providers as EntityTrait>::Column;
type ProviderActiveModel = <Providers as EntityTrait>::ActiveModel;
type Organizations = <OrganizationSchema as ModelTrait>::Entity;

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

    fn api_protocol(protocol: ApiProtocol) -> ApiProtocolSchema {
        match protocol {
            ApiProtocol::Anthropic => ApiProtocolSchema::Anthropic,
            ApiProtocol::Openai => ApiProtocolSchema::Openai,
            ApiProtocol::OpenaiCompatible => ApiProtocolSchema::OpenaiCompatible,
        }
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
    async fn find_all(
        &self,
        organization_id: u64,
        limit: u64,
    ) -> Result<Vec<Provider>, ProviderRepositoryError> {
        if !(1..=MAX_PROVIDER_FIND_LIMIT).contains(&limit) {
            return Err(ProviderRepositoryError::InvalidLimit);
        }
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

    async fn insert(&self, provider: NewProvider) -> Result<Provider, ProviderRepositoryError> {
        let id = Self::database_id(provider.id, "provider ID")?;
        let organization_id = Self::database_id(provider.organization_id, "organization ID")?;
        let context = DbContext::current()?;
        if !context.postgres().is_transaction() {
            return Err(ProviderRepositoryError::database(anyhow::anyhow!(
                "provider insertion requires an active transaction"
            )));
        }
        let db = PostgresConnection::from_context(&context)?;

        // Serialize inserts for one organization so count + insert cannot race.
        let organization = Organizations::find_by_id(organization_id)
            .lock_exclusive()
            .one(db)
            .await
            .map_err(ProviderRepositoryError::database)?;
        if organization.is_none() {
            return Err(ProviderRepositoryError::database(anyhow::anyhow!(
                "provider organization does not exist"
            )));
        }

        let count = Providers::find()
            .filter(ProviderColumn::OrganizationId.eq(organization_id))
            .count(db)
            .await
            .map_err(ProviderRepositoryError::database)?;
        if count >= MAX_PROVIDERS_PER_ORGANIZATION {
            return Err(ProviderRepositoryError::MaximumProvidersReached);
        }

        let model = ProviderActiveModel {
            id: Set(id),
            organization_id: Set(organization_id),
            name: Set(provider.name),
            url: Set(provider.url),
            api_key: Set(provider.api_key.into_storage_value()),
            api_protocol: Set(Self::api_protocol(provider.api_protocol)),
            ..Default::default()
        };
        Provider::try_from(model.insert(db).await.map_err(Self::write_error)?)
    }

    async fn update(
        &self,
        organization_id: u64,
        provider_id: u64,
        update: ProviderUpdate,
    ) -> Result<Provider, ProviderRepositoryError> {
        let organization_id = Self::database_id(organization_id, "organization ID")?;
        let provider_id = Self::database_id(provider_id, "provider ID")?;
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;

        let model = Providers::find_by_id(provider_id)
            .filter(ProviderColumn::OrganizationId.eq(organization_id))
            .one(db)
            .await
            .map_err(ProviderRepositoryError::database)?
            .ok_or(ProviderRepositoryError::NotFound)?;
        let mut model: ProviderActiveModel = model.into();
        model.name = Set(update.name);
        model.url = Set(update.url);
        if let Some(api_key) = update.api_key {
            model.api_key = Set(api_key.into_storage_value());
        }
        model.api_protocol = Set(Self::api_protocol(update.api_protocol));
        model.updated_at = Set(ChronoUtc::now().fixed_offset());
        Provider::try_from(model.update(db).await.map_err(Self::write_error)?)
    }

    async fn delete(
        &self,
        organization_id: u64,
        provider_id: u64,
    ) -> Result<(), ProviderRepositoryError> {
        let organization_id = Self::database_id(organization_id, "organization ID")?;
        let provider_id = Self::database_id(provider_id, "provider ID")?;
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;

        let result = Providers::delete_many()
            .filter(ProviderColumn::Id.eq(provider_id))
            .filter(ProviderColumn::OrganizationId.eq(organization_id))
            .exec(db)
            .await
            .map_err(ProviderRepositoryError::database)?;
        if result.rows_affected != 1 {
            return Err(ProviderRepositoryError::NotFound);
        }
        Ok(())
    }
}
