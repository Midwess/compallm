use crate::{
    ProviderCatalogReadinessSchema, ProviderModelTemplateSchema, ProviderTemplateSchema,
    postgres_connection::PostgresConnection, query::IntoSeaQuery,
};
use compallm_core::{model::ModelTemplate, persistent::DbContext, provider::ProviderTemplate};
use compallm_query::{IntoQuery, OrderBy, Pagination};
use sea_orm::{
    ColumnTrait, Condition, DbErr, EntityTrait, ExprTrait, ModelTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait,
    sea_query::{Alias, Expr, Func, LikeExpr},
};

type Providers = <ProviderTemplateSchema as ModelTrait>::Entity;
type Models = <ProviderModelTemplateSchema as ModelTrait>::Entity;
type Readiness = <ProviderCatalogReadinessSchema as ModelTrait>::Entity;
type P = <Providers as EntityTrait>::Column;
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
        search: Option<&str>,
    ) -> Result<compallm_query::Connection<ProviderTemplate>, crate::query::QueryCompileError> {
        let context = DbContext::current()
            .map_err(|_| DbErr::Custom("catalog context unavailable".into()))?;
        let db = PostgresConnection::from_context(&context)
            .map_err(|_| DbErr::Custom("catalog connection unavailable".into()))?;
        let mut terms: Vec<_> = search
            .unwrap_or_default()
            .split_whitespace()
            .map(str::to_lowercase)
            .collect();
        terms.sort_unstable();
        terms.dedup();
        let mut select = Providers::find();
        if !terms.is_empty() {
            let mut provider_match = Condition::all();
            let mut model_match = Condition::all();
            for term in &terms {
                let escaped = term
                    .replace('!', "!!")
                    .replace('%', "!%")
                    .replace('_', "!_");
                let pattern = format!("%{escaped}%");
                let provider_term = Condition::any()
                    .add(
                        Expr::expr(Func::lower(Expr::col((
                            Alias::new("provider_templates"),
                            P::Id,
                        ))))
                        .like(LikeExpr::new(&pattern).escape('!')),
                    )
                    .add(
                        Expr::expr(Func::lower(Expr::col((
                            Alias::new("provider_templates"),
                            P::Name,
                        ))))
                        .like(LikeExpr::new(&pattern).escape('!')),
                    );
                provider_match = provider_match.add(provider_term.clone());
                model_match = model_match.add(
                    Condition::any()
                        .add(provider_term)
                        .add(
                            Expr::expr(Func::lower(Expr::col((
                                Alias::new("provider_model_templates"),
                                M::Id,
                            ))))
                            .like(LikeExpr::new(&pattern).escape('!')),
                        )
                        .add(
                            Expr::expr(Func::lower(Expr::col((
                                Alias::new("provider_model_templates"),
                                M::Name,
                            ))))
                            .like(LikeExpr::new(&pattern).escape('!')),
                        )
                        .add(
                            Expr::expr(Func::lower(Expr::col((
                                Alias::new("provider_model_templates"),
                                M::DisplayName,
                            ))))
                            .like(LikeExpr::new(&pattern).escape('!')),
                        ),
                );
            }
            let ready = Readiness::find()
                .select_only()
                .expr(Expr::val(1))
                .filter(
                    Expr::col((
                        Alias::new("provider_catalog_readiness"),
                        R::ProviderTemplateId,
                    ))
                    .eq(Expr::col((
                        Alias::new("provider_model_templates"),
                        M::ProviderTemplateId,
                    ))),
                )
                .filter(
                    Expr::col((Alias::new("provider_catalog_readiness"), R::Id))
                        .eq(Expr::col((Alias::new("provider_model_templates"), M::Id))),
                )
                .filter(R::Readiness.eq(crate::CatalogReadinessSchema::Ready))
                .into_query();
            // Evaluate readiness only after a name match; the readiness view inspects configuration JSON.
            let models = Models::find()
                .select_only()
                .expr(Expr::val(1))
                .filter(
                    Expr::col((
                        Alias::new("provider_model_templates"),
                        M::ProviderTemplateId,
                    ))
                    .eq(Expr::col((Alias::new("provider_templates"), P::Id))),
                )
                .filter(Expr::Case(Box::new(
                    Expr::case(model_match, Expr::exists(ready)).finally(false),
                )))
                .into_query();
            select = select.filter(
                Condition::any()
                    .add(provider_match)
                    .add(Expr::exists(models)),
            );
        }
        let scope = format!(
            "{scope}:search:{}",
            serde_json::to_string(&terms).expect("search terms serialize")
        );
        let result = query
            .plan_page(select, pagination, ordering, 100, &scope)?
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
