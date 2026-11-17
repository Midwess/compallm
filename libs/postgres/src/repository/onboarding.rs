use crate::{
    postgres_connection::PostgresConnection,
    schema::{OrganizationMemberOnboardingSchema, OrganizationOnboardingSettingSchema},
};
use compallm_core::{
    onboarding::{Context, Progress, Settings},
    persistent::{
        DbContext,
        onboarding::{OnboardingRepository as Repository, OnboardingRepositoryError as Error},
    },
};
use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait, QuerySelect, Set, sea_query::OnConflict};
type SettingsEntity = <OrganizationOnboardingSettingSchema as ModelTrait>::Entity;
type SettingsColumn = <SettingsEntity as EntityTrait>::Column;
type SettingsModel = <SettingsEntity as EntityTrait>::ActiveModel;
type ProgressEntity = <OrganizationMemberOnboardingSchema as ModelTrait>::Entity;
type ProgressColumn = <ProgressEntity as EntityTrait>::Column;
type ProgressModel = <ProgressEntity as EntityTrait>::ActiveModel;
#[derive(Default)]
pub struct OnboardingRepository;
fn id(value: u64) -> Result<i64, Error> {
    crate::to_bigint(value).map_err(|e| Error(e.into()))
}
#[async_trait::async_trait]
impl Repository for OnboardingRepository {
    async fn settings(&self, organization_id: u64) -> Result<Option<Settings>, Error> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        SettingsEntity::find_by_id(id(organization_id)?)
            .one(db)
            .await
            .map_err(anyhow::Error::from)?
            .map(|m| serde_json::from_value(m.settings).map_err(|e| Error(e.into())))
            .transpose()
    }
    async fn save_settings(
        &self,
        organization_id: u64,
        value: Settings,
    ) -> Result<Settings, Error> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        SettingsEntity::insert(SettingsModel {
            organization_id: Set(id(organization_id)?),
            settings: Set(serde_json::to_value(&value).map_err(anyhow::Error::from)?),
        })
        .on_conflict(
            OnConflict::column(SettingsColumn::OrganizationId)
                .update_column(SettingsColumn::Settings)
                .to_owned(),
        )
        .exec(db)
        .await
        .map_err(anyhow::Error::from)?;
        Ok(value)
    }
    async fn initialize(
        &self,
        organization_id: u64,
        user_id: u64,
        value: Progress,
    ) -> Result<(), Error> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        ProgressEntity::insert(ProgressModel {
            organization_id: Set(id(organization_id)?),
            user_id: Set(id(user_id)?),
            progress: Set(serde_json::to_value(value).map_err(anyhow::Error::from)?),
            completed: Set(false),
        })
        .on_conflict(
            OnConflict::columns([ProgressColumn::OrganizationId, ProgressColumn::UserId])
                .do_nothing()
                .to_owned(),
        )
        .try_insert()
        .exec(db)
        .await
        .map_err(anyhow::Error::from)?;
        Ok(())
    }
    async fn find(
        &self,
        organization_id: u64,
        user_id: u64,
        lock: bool,
    ) -> Result<Option<Context>, Error> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let key = (id(organization_id)?, id(user_id)?);
        let query = ProgressEntity::find_by_id(key);
        let row = if lock {
            query.lock_exclusive().one(db).await
        } else {
            query.one(db).await
        }
        .map_err(anyhow::Error::from)?;
        let Some(row) = row else {
            return Ok(None);
        };
        Ok(Some(Context {
            required: !row.completed,
            progress: serde_json::from_value(row.progress).map_err(anyhow::Error::from)?,
        }))
    }
    async fn save_progress(
        &self,
        organization_id: u64,
        user_id: u64,
        value: Progress,
    ) -> Result<(), Error> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        ProgressModel {
            organization_id: Set(id(organization_id)?),
            user_id: Set(id(user_id)?),
            progress: Set(serde_json::to_value(value).map_err(anyhow::Error::from)?),
            ..Default::default()
        }
        .update(db)
        .await
        .map_err(anyhow::Error::from)?;
        Ok(())
    }
    async fn complete(&self, organization_id: u64, user_id: u64) -> Result<(), Error> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        ProgressModel {
            organization_id: Set(id(organization_id)?),
            user_id: Set(id(user_id)?),
            completed: Set(true),
            ..Default::default()
        }
        .update(db)
        .await
        .map_err(anyhow::Error::from)?;
        Ok(())
    }
}
