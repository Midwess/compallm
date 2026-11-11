pub mod user;

use async_trait::async_trait;
use modql::filter::ListOptions;
use validify::Validify;

#[async_trait]
pub trait Queryable<Query>: Send + Sync
where
    Query: Validify + Send + 'static,
{
    type Entity: Send;
    type Error: Send;

    async fn list(
        &self,
        query: Query,
        options: ListOptions,
    ) -> Result<Vec<Self::Entity>, Self::Error>;

    async fn one(
        &self,
        query: Query,
        options: ListOptions,
    ) -> Result<Option<Self::Entity>, Self::Error> {
        let rows = self.list(query, options.with_limit(1)).await?;
        Ok(rows.into_iter().next())
    }
}
