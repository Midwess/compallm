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

impl ProviderRepository {
    pub async fn find_by_id(&self, id: u64) -> Result<Option<Provider>, ProviderRepositoryError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let Some(row) = Providers::find_by_id(Self::database_id(id, "provider ID")?)
            .one(db)
            .await
            .map_err(Self::write_error)?
        else {
            return Ok(None);
        };
        let mut value = Provider::try_from(row)?;
        type ModelEntity = <crate::ProviderModelSchema as ModelTrait>::Entity;
        type ModelColumn = <ModelEntity as EntityTrait>::Column;
        value.models = ModelEntity::find()
            .filter(ModelColumn::ProviderId.eq(id as i64))
            .order_by_asc(ModelColumn::Id)
            .all(db)
            .await
            .map_err(Self::write_error)?
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;
        Ok(Some(value))
    }
}

impl ProviderRepository {
    pub async fn list(
        &self,
        pagination: compallm_query::Pagination,
        scope: &str,
    ) -> Result<compallm_query::Connection<Provider>, crate::query::QueryCompileError> {
        use crate::query::IntoSeaQuery;
        let context = DbContext::current()
            .map_err(|_| DbErr::Custom("provider context unavailable".into()))?;
        let db = PostgresConnection::from_context(&context)
            .map_err(|_| DbErr::Custom("provider connection unavailable".into()))?;
        let result = Box::new(compallm_query::Query::default())
            .plan_page(
                Providers::find(),
                pagination,
                vec![compallm_query::OrderBy {
                    field: "id".into(),
                    direction: compallm_query::OrderByDirection::AscNullsLast,
                }],
                100,
                scope,
            )?
            .fetch(db)
            .await?;
        let mut edges: Vec<compallm_query::Edge<Provider>> = result
            .edges
            .into_iter()
            .map(|edge| {
                Ok(compallm_query::Edge {
                    cursor: edge.cursor,
                    node: Provider::try_from(edge.node)
                        .map_err(|_| DbErr::Custom("invalid stored provider".into()))?,
                })
            })
            .collect::<Result<_, DbErr>>()?;
        if !edges.is_empty() {
            type Models = <crate::ProviderModelSchema as ModelTrait>::Entity;
            type MC = <Models as EntityTrait>::Column;
            let ids = edges.iter().map(|v| v.node.id as i64).collect::<Vec<_>>();
            let rows = Models::find()
                .filter(MC::ProviderId.is_in(ids))
                .order_by_asc(MC::Id)
                .all(db)
                .await?;
            let mut grouped =
                std::collections::HashMap::<u64, Vec<compallm_core::provider::ProviderModel>>::new(
                );
            for row in rows {
                let model = compallm_core::provider::ProviderModel::try_from(row)
                    .map_err(|_| DbErr::Custom("invalid configured model".into()))?;
                grouped.entry(model.provider_id).or_default().push(model);
            }
            for edge in &mut edges {
                edge.node.models = grouped.remove(&edge.node.id).unwrap_or_default();
            }
        }
        Ok(compallm_query::Connection {
            edges,
            page_info: result.page_info,
        })
    }
}
