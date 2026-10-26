use anyhow::Context;
use compallm_core::{
    persistent::{
        DbContext,
        provider::{ProviderRepository as CoreProviderRepository, ProviderRepositoryError},
    },
    provider::{ApiProtocol, Provider, ProviderApiKey},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DbErr, EntityTrait, ModelTrait, QueryFilter,
    QueryOrder, QuerySelect, RuntimeErr, prelude::ChronoUtc,
};

use crate::{
    postgres_connection::PostgresConnection,
    schema::{ApiProtocolSchema, ProviderSchema},
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

    async fn insert(
        &self,
        provider: Provider,
        api_key: ProviderApiKey,
    ) -> Result<Provider, ProviderRepositoryError> {
        let id = Self::database_id(provider.id, "provider ID")?;
        let organization_id = Self::database_id(provider.organization_id, "organization ID")?;
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;

        let model = ProviderActiveModel {
            id: Set(id),
            organization_id: Set(organization_id),
            name: Set(provider.name),
            url: Set(provider.url),
            api_key: Set(api_key.into_storage_value()),
            api_protocol: Set(Self::api_protocol(provider.api_protocol)),
            ..Default::default()
        };
        Provider::try_from(model.insert(db).await.map_err(Self::write_error)?)
    }

    async fn update(
        &self,
        provider: Provider,
        api_key: Option<ProviderApiKey>,
    ) -> Result<Provider, ProviderRepositoryError> {
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
        model.url = Set(provider.url);
        if let Some(api_key) = api_key {
            model.api_key = Set(api_key.into_storage_value());
        }
        model.api_protocol = Set(Self::api_protocol(provider.api_protocol));
        model.updated_at = Set(ChronoUtc::now().fixed_offset());
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
