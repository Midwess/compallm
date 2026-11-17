use crate::{postgres_connection::PostgresConnection, schema::AnalyticUsersProfileSchema};
use compallm_core::{
    analytics::UserProfileValue,
    persistent::{
        DbContext,
        analytics::{AnalyticsRepository as Repository, AnalyticsRepositoryError as Error},
    },
};
use sea_orm::{EntityTrait, ModelTrait, Set, sea_query::OnConflict};
type Entity = <AnalyticUsersProfileSchema as ModelTrait>::Entity;
type Column = <Entity as EntityTrait>::Column;
type ActiveModel = <Entity as EntityTrait>::ActiveModel;
#[derive(Default)]
pub struct AnalyticsRepository;
impl From<AnalyticUsersProfileSchema> for UserProfileValue {
    fn from(m: AnalyticUsersProfileSchema) -> Self {
        Self {
            key: m.key,
            value: m.value,
        }
    }
}
#[async_trait::async_trait]
impl Repository for AnalyticsRepository {
    async fn find(&self, user_id: u64, key: &str) -> Result<Option<UserProfileValue>, Error> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        Ok(Entity::find_by_id((
            crate::to_bigint(user_id).map_err(anyhow::Error::from)?,
            key.to_owned(),
        ))
        .one(db)
        .await
        .map_err(anyhow::Error::from)?
        .map(Into::into))
    }
    async fn put(&self, user_id: u64, value: UserProfileValue) -> Result<UserProfileValue, Error> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        Entity::insert(ActiveModel {
            user_id: Set(crate::to_bigint(user_id).map_err(anyhow::Error::from)?),
            key: Set(value.key.clone()),
            value: Set(value.value.clone()),
        })
        .on_conflict(
            OnConflict::columns([Column::UserId, Column::Key])
                .update_column(Column::Value)
                .to_owned(),
        )
        .exec(db)
        .await
        .map_err(anyhow::Error::from)?;
        Ok(value)
    }
}
