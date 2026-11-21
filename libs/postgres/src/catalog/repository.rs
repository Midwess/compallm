use crate::{
    ProviderCatalogReadinessSchema, ProviderModelTemplateSchema, ProviderTemplateSchema,
    postgres_connection::PostgresConnection, query::IntoSeaQuery,
};
use compallm_core::{model::ModelTemplate, persistent::DbContext, provider::ProviderTemplate};
use compallm_query::{IntoQuery, OrderBy, Pagination};
use sea_orm::{
    ColumnTrait, DbErr, EntityTrait, ModelTrait, QueryFilter, QueryOrder, QuerySelect, QueryTrait,
};

type Providers = <ProviderTemplateSchema as ModelTrait>::Entity;
type Models = <ProviderModelTemplateSchema as ModelTrait>::Entity;
type Readiness = <ProviderCatalogReadinessSchema as ModelTrait>::Entity;
type M = <Models as EntityTrait>::Column;
type R = <Readiness as EntityTrait>::Column;

#[derive(Default)]
pub struct CatalogRepository;

impl CatalogRepository {
    pub async fn list(
        &self,
        query: Box<dyn IntoQuery>,
        pagination: Pagination,
        ordering: Vec<OrderBy>,
        scope: &str,
    ) -> Result<compallm_query::Connection<ProviderTemplate>, crate::query::QueryCompileError> {
        let context = DbContext::current()
            .map_err(|_| DbErr::Custom("catalog context unavailable".into()))?;
        let db = PostgresConnection::from_context(&context)
            .map_err(|_| DbErr::Custom("catalog connection unavailable".into()))?;
        let result = query
            .plan_page(Providers::find(), pagination, ordering, 100, scope)?
            .fetch(db)
            .await?;
        let mut edges = Vec::with_capacity(result.edges.len());
        for edge in result.edges {
            edges.push(compallm_query::Edge {
                cursor: edge.cursor,
                node: ProviderTemplate::try_from(edge.node)?,
            });
        }
        Ok(compallm_query::Connection {
            edges,
            page_info: result.page_info,
        })
    }
    pub async fn provider(&self, id: &str) -> Result<Option<ProviderTemplate>, DbErr> {
        let context = DbContext::current()
            .map_err(|_| DbErr::Custom("catalog context unavailable".into()))?;
        let db = PostgresConnection::from_context(&context)
            .map_err(|_| DbErr::Custom("catalog connection unavailable".into()))?;
        Providers::find_by_id(id)
            .one(db)
            .await?
            .map(TryInto::try_into)
            .transpose()
    }
    pub async fn models(
        &self,
        provider: &str,
        query: Box<dyn IntoQuery>,
    ) -> Result<Vec<ModelTemplate>, crate::query::QueryCompileError> {
        let context = DbContext::current()
            .map_err(|_| DbErr::Custom("catalog context unavailable".into()))?;
        let db = PostgresConnection::from_context(&context)
            .map_err(|_| DbErr::Custom("catalog connection unavailable".into()))?;
        let eligible = Readiness::find()
            .select_only()
            .column(R::Id)
            .filter(R::ProviderTemplateId.eq(provider))
            .filter(R::Readiness.eq(crate::CatalogReadinessSchema::Ready))
            .into_query();
        let select = Models::find()
            .filter(M::ProviderTemplateId.eq(provider))
            .filter(M::Id.in_subquery(eligible))
            .order_by_asc(M::Name)
            .order_by_asc(M::Id);
        query
            .apply_to_sea_query(
                select,
                Pagination::default(),
                vec![],
                Pagination::DEFAULT_MAX_ROWS,
            )?
            .limit(None)
            .all(db)
            .await?
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>, _>>()
            .map_err(Into::into)
    }
    pub async fn model(&self, provider: &str, id: &str) -> Result<Option<ModelTemplate>, DbErr> {
        let context = DbContext::current()
            .map_err(|_| DbErr::Custom("catalog context unavailable".into()))?;
        let db = PostgresConnection::from_context(&context)
            .map_err(|_| DbErr::Custom("catalog connection unavailable".into()))?;
        Models::find_by_id((provider.to_owned(), id.to_owned()))
            .filter(
                M::Id.in_subquery(
                    Readiness::find()
                        .select_only()
                        .column(R::Id)
                        .filter(R::ProviderTemplateId.eq(provider))
                        .filter(R::Readiness.eq(crate::CatalogReadinessSchema::Ready))
                        .into_query(),
                ),
            )
            .one(db)
            .await?
            .map(TryInto::try_into)
            .transpose()
    }
}
