use compallm_core::persistent::{ConnectionError, DbContext};
use modql::filter::ListOptions;
use sea_orm::{EntityTrait, Iterable, PrimaryKeyToColumn, QueryOrder, QueryTrait, Select};

use validify::{ValidationErrors, Validify};

use crate::postgres_connection::PostgresConnection;

pub trait IntoSelect {
    type Table: EntityTrait;

    fn into_select(self) -> Select<Self::Table>;
}

pub trait QueryDefinition<Query>: Send + Sync
where
    Query: IntoSelect + Validify + Send + 'static,
{
    type Entity: TryFrom<<Query::Table as EntityTrait>::Model, Error = Self::Error> + Send;
    type Error: From<anyhow::Error> + From<ConnectionError> + From<ValidationErrors> + Send;
    type Options: From<ListOptions> + Into<ListOptions> + Validify;
}

pub async fn list<Definition, Query>(
    mut query: Query,
    options: ListOptions,
) -> Result<Vec<Definition::Entity>, Definition::Error>
where
    Definition: QueryDefinition<Query>,
    Query: IntoSelect + Validify + Send + 'static,
{
    query.validify()?;
    let mut options = Definition::Options::from(options);
    options.validify()?;
    let options: ListOptions = options.into();
    let mut select = query.into_select();
    options.apply_to_sea_query(QueryTrait::query(&mut select));
    for key in <Query::Table as EntityTrait>::PrimaryKey::iter() {
        select = select.order_by_asc(key.into_column());
    }
    let context = DbContext::current()?;
    let db = PostgresConnection::from_context(&context)?;
    select
        .all(db)
        .await
        .map_err(anyhow::Error::from)?
        .into_iter()
        .map(Definition::Entity::try_from)
        .collect()
}

#[macro_export]
macro_rules! impl_queryable {
    ($repository:ty, $query:ty) => {
        #[async_trait::async_trait]
        impl compallm_core::repository::Queryable<$query> for $repository {
            type Entity = <Self as $crate::repository::queryable::QueryDefinition<$query>>::Entity;
            type Error = <Self as $crate::repository::queryable::QueryDefinition<$query>>::Error;

            async fn list(
                &self,
                query: $query,
                options: modql::filter::ListOptions,
            ) -> Result<Vec<Self::Entity>, Self::Error> {
                $crate::repository::queryable::list::<Self, $query>(query, options).await
            }
        }
    };
}
