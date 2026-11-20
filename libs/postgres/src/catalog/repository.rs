use crate::{
    ProviderCatalogAuthReadinessSchema, ProviderModelTemplateSchema, ProviderTemplateSchema,
    postgres_connection::PostgresConnection, query::IntoSeaQuery,
};
use compallm_core::{
    persistent::DbContext,
    provider_template::{ModelTemplate, ProviderTemplate},
};
use compallm_query::{IntoQuery, OrderBy, Pagination};
use sea_orm::{
    ColumnTrait, DbErr, EntityTrait, ModelTrait, QueryFilter, QueryOrder, QuerySelect, QueryTrait,
};

type Summaries = super::summary::Entity;
type Providers = <ProviderTemplateSchema as ModelTrait>::Entity;
type Models = <ProviderModelTemplateSchema as ModelTrait>::Entity;
type Readiness = <ProviderCatalogAuthReadinessSchema as ModelTrait>::Entity;
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
    ) -> Result<
        compallm_query::Connection<compallm_core::provider_template::ProviderTemplateSummary>,
        crate::query::QueryCompileError,
    > {
        let context = DbContext::current()
            .map_err(|_| DbErr::Custom("catalog context unavailable".into()))?;
        let db = PostgresConnection::from_context(&context)
            .map_err(|_| DbErr::Custom("catalog connection unavailable".into()))?;
        let result = query
            .plan_page(Summaries::find(), pagination, ordering, 100, scope)?
            .fetch(db)
            .await?;
        let mut edges = Vec::with_capacity(result.edges.len());
        for edge in result.edges {
            edges.push(compallm_query::Edge {
                cursor: edge.cursor,
                node: compallm_core::provider_template::ProviderTemplateSummary::try_from(
                    edge.node,
                )?,
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
    ) -> Result<Vec<compallm_core::provider_template::ModelTemplateSummary>, DbErr> {
        let context = DbContext::current()
            .map_err(|_| DbErr::Custom("catalog context unavailable".into()))?;
        let db = PostgresConnection::from_context(&context)
            .map_err(|_| DbErr::Custom("catalog connection unavailable".into()))?;
        let eligible = Readiness::find()
            .select_only()
            .column(R::Id)
            .filter(R::ProviderTemplateId.eq(provider))
            .filter(R::AuthMethod.eq("api_key"))
            .filter(R::Readiness.eq(crate::CatalogReadinessSchema::Ready))
            .into_query();
        let rows = Models::find()
            .select_only()
            .columns([
                M::ProviderTemplateId,
                M::Id,
                M::UpstreamModelId,
                M::DisplayName,
                M::Description,
                M::Operations,
                M::Capabilities,
                M::MaxContextTokens,
                M::MaxInputTokens,
                M::MaxOutputTokens,
                M::EffortMapping,
                M::InputPrice,
                M::OutputPrice,
                M::CachedPrice,
            ])
            .filter(M::ProviderTemplateId.eq(provider))
            .filter(M::Id.in_subquery(eligible))
            .order_by_asc(M::UpstreamModelId)
            .order_by_asc(M::Id)
            .into_model::<ModelSummaryRow>()
            .all(db)
            .await?;
        rows.into_iter().map(TryInto::try_into).collect()
    }
    pub async fn model_details(
        &self,
        provider: &str,
        ids: Vec<String>,
    ) -> Result<Vec<ModelTemplate>, DbErr> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let context = DbContext::current()
            .map_err(|_| DbErr::Custom("catalog context unavailable".into()))?;
        let db = PostgresConnection::from_context(&context)
            .map_err(|_| DbErr::Custom("catalog connection unavailable".into()))?;
        let eligible = Readiness::find()
            .select_only()
            .column(R::Id)
            .filter(R::ProviderTemplateId.eq(provider))
            .filter(R::AuthMethod.eq("api_key"))
            .filter(R::Readiness.eq(crate::CatalogReadinessSchema::Ready))
            .into_query();
        Models::find()
            .filter(M::ProviderTemplateId.eq(provider))
            .filter(M::Id.is_in(ids))
            .filter(M::Id.in_subquery(eligible))
            .all(db)
            .await?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
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
                        .filter(R::AuthMethod.eq("api_key"))
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

#[derive(sea_orm::FromQueryResult)]
struct ModelSummaryRow {
    provider_template_id: String,
    id: String,
    upstream_model_id: String,
    display_name: Option<String>,
    description: Option<String>,
    operations: serde_json::Value,
    capabilities: serde_json::Value,
    max_context_tokens: Option<i64>,
    max_input_tokens: Option<i64>,
    max_output_tokens: Option<i64>,
    effort_mapping: serde_json::Value,
    input_price: Option<sea_orm::prelude::Decimal>,
    output_price: Option<sea_orm::prelude::Decimal>,
    cached_price: Option<sea_orm::prelude::Decimal>,
}
impl TryFrom<ModelSummaryRow> for compallm_core::provider_template::ModelTemplateSummary {
    type Error = DbErr;
    fn try_from(v: ModelSummaryRow) -> Result<Self, Self::Error> {
        Ok(Self {
            provider_template_id: v.provider_template_id,
            id: v.id,
            upstream_model_id: v.upstream_model_id,
            display_name: v.display_name,
            description: v.description,
            max_context_tokens: v
                .max_context_tokens
                .map(u32::try_from)
                .transpose()
                .map_err(|_| DbErr::Custom("invalid context limit".into()))?,
            max_input_tokens: v
                .max_input_tokens
                .map(u32::try_from)
                .transpose()
                .map_err(|_| DbErr::Custom("invalid input limit".into()))?,
            max_output_tokens: v
                .max_output_tokens
                .map(u32::try_from)
                .transpose()
                .map_err(|_| DbErr::Custom("invalid output limit".into()))?,
            input_price: v.input_price,
            output_price: v.output_price,
            cached_price: v.cached_price,
            effort_mapping: serde_json::from_value(v.effort_mapping)
                .map_err(|_| DbErr::Custom("invalid effort mapping".into()))?,
            operations: serde_json::from_value(v.operations)
                .map_err(|_| DbErr::Custom("invalid catalog operations".into()))?,
            capabilities: serde_json::from_value(v.capabilities)
                .map_err(|_| DbErr::Custom("invalid catalog capabilities".into()))?,
        })
    }
}
