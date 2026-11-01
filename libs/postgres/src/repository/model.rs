use anyhow::Context;
use compallm_core::{
    model::Model,
    persistent::{
        DbContext,
        model::{ModelRepository as CoreModelRepository, ModelRepositoryError},
    },
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DbErr, EntityTrait, ModelTrait, QueryFilter,
    QueryOrder, QuerySelect, RuntimeErr, prelude::ChronoUtc,
};

use crate::{
    mappers::model::price_to_database, postgres_connection::PostgresConnection, schema::ModelSchema,
};

type Models = <ModelSchema as ModelTrait>::Entity;
type ModelColumn = <Models as EntityTrait>::Column;
type ModelActiveModel = <Models as EntityTrait>::ActiveModel;

#[derive(Default)]
pub struct ModelRepository;

impl ModelRepository {
    fn database_id(id: u64, field: &str) -> Result<i64, ModelRepositoryError> {
        if id == 0 {
            return Err(ModelRepositoryError::database(anyhow::anyhow!(
                "{field} must be positive"
            )));
        }
        crate::to_bigint(id)
            .with_context(|| format!("{field} exceeds PostgreSQL BIGINT"))
            .map_err(Into::into)
    }

    fn write_error(error: DbErr) -> ModelRepositoryError {
        if let DbErr::Exec(RuntimeErr::SqlxError(driver))
        | DbErr::Query(RuntimeErr::SqlxError(driver)) = &error
            && let sea_orm::sqlx::Error::Database(database) = driver.as_ref()
            && database.is_unique_violation()
            && database.constraint() == Some("models_organization_id_name_unique")
        {
            return ModelRepositoryError::NameAlreadyExists;
        }
        ModelRepositoryError::database(error)
    }
}

#[async_trait::async_trait]
impl CoreModelRepository for ModelRepository {
    async fn find_all(
        &self,
        organization_id: u64,
        limit: u64,
    ) -> Result<Vec<Model>, ModelRepositoryError> {
        let organization_id = Self::database_id(organization_id, "organization ID")?;
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;

        Models::find()
            .filter(ModelColumn::OrganizationId.eq(organization_id))
            .order_by_asc(ModelColumn::Id)
            .limit(limit)
            .all(db)
            .await
            .map_err(ModelRepositoryError::database)?
            .into_iter()
            .map(Model::try_from)
            .collect()
    }

    async fn insert(&self, model: Model) -> Result<Model, ModelRepositoryError> {
        let id = Self::database_id(model.id, "model ID")?;
        let organization_id = Self::database_id(model.organization_id, "organization ID")?;
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;

        let active_model = ModelActiveModel {
            id: Set(id),
            organization_id: Set(organization_id),
            name: Set(model.name),
            input_price_per_million_tokens: Set(price_to_database(
                model.input_price_per_million_tokens,
            )?),
            output_price_per_million_tokens: Set(price_to_database(
                model.output_price_per_million_tokens,
            )?),
            cached_price_per_million_tokens: Set(price_to_database(
                model.cached_price_per_million_tokens,
            )?),
            settings: Set(model.settings),
            ..Default::default()
        };
        Model::try_from(active_model.insert(db).await.map_err(Self::write_error)?)
    }

    async fn update(&self, model: Model) -> Result<Model, ModelRepositoryError> {
        let organization_id = Self::database_id(model.organization_id, "organization ID")?;
        let model_id = Self::database_id(model.id, "model ID")?;
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;

        let stored = Models::find_by_id(model_id)
            .filter(ModelColumn::OrganizationId.eq(organization_id))
            .one(db)
            .await
            .map_err(ModelRepositoryError::database)?
            .ok_or(ModelRepositoryError::NotFound)?;
        let mut active_model: ModelActiveModel = stored.into();
        active_model.name = Set(model.name);
        active_model.input_price_per_million_tokens =
            Set(price_to_database(model.input_price_per_million_tokens)?);
        active_model.output_price_per_million_tokens =
            Set(price_to_database(model.output_price_per_million_tokens)?);
        active_model.cached_price_per_million_tokens =
            Set(price_to_database(model.cached_price_per_million_tokens)?);
        active_model.settings = Set(model.settings);
        active_model.updated_at = Set(ChronoUtc::now().fixed_offset());
        Model::try_from(active_model.update(db).await.map_err(Self::write_error)?)
    }

    async fn delete(
        &self,
        organization_id: u64,
        model_id: u64,
    ) -> Result<Model, ModelRepositoryError> {
        let organization_id = Self::database_id(organization_id, "organization ID")?;
        let model_id = Self::database_id(model_id, "model ID")?;
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;

        let mut models = Models::delete_many()
            .filter(ModelColumn::Id.eq(model_id))
            .filter(ModelColumn::OrganizationId.eq(organization_id))
            .exec_with_returning(db)
            .await
            .map_err(ModelRepositoryError::database)?;
        Model::try_from(models.pop().ok_or(ModelRepositoryError::NotFound)?)
    }
}
