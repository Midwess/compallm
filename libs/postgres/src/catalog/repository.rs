use crate::{
    CatalogModelRecordSchema, CatalogProviderBookSchema, postgres_connection::PostgresConnection,
    query::IntoSeaQuery,
};
use compallm_core::{model::ModelCatalog, persistent::DbContext, provider::ProviderCatalog};
use compallm_query::{IntoQuery, OrderBy, Pagination};
use sea_orm::{
    ColumnTrait, Condition, DbErr, EntityTrait, ExprTrait, ModelTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait,
    sea_query::{Alias, Expr, Func, LikeExpr},
};

type Providers = <CatalogProviderBookSchema as ModelTrait>::Entity;
type Models = <CatalogModelRecordSchema as ModelTrait>::Entity;
type P = <Providers as EntityTrait>::Column;
type M = <Models as EntityTrait>::Column;

#[derive(Default)]
pub struct CatalogRepository;

#[derive(Clone, Copy, Debug)]
pub struct CatalogScope {
    pub organization_id: u64,
    pub user_id: u64,
}

fn eligible_models() -> Condition {
    let providers = Providers::find()
        .select_only()
        .column(P::Id)
        .filter(P::ApiKeySupported.eq(true))
        .into_query();
    Condition::all()
        .add(M::ServiceKind.eq("llm"))
        .add(M::ProviderCatalogId.in_subquery(providers))
        .add(M::Availability.is_not_in(["deprecated", "unsupported"]))
}

fn visibility(scope: Option<CatalogScope>) -> Condition {
    let global = Condition::all().add(M::UserId.is_null());
    let Some(scope) = scope else {
        return global;
    };
    let own = Condition::all()
        .add(M::OrganizationId.eq(scope.organization_id as i64))
        .add(M::UserId.eq(scope.user_id as i64));
    let bases = Models::find()
        .select_only()
        .column(M::BaseRowId)
        .filter(own.clone())
        .filter(M::BaseRowId.is_not_null())
        .into_query();
    Condition::any()
        .add(own)
        .add(global.add(M::RowId.not_in_subquery(bases)))
}

impl CatalogRepository {
    pub async fn list(
        &self,
        query: Box<dyn IntoQuery>,
        pagination: Pagination,
        ordering: Vec<OrderBy>,
        scope: &str,
        search: Option<&str>,
        scope_owner: Option<CatalogScope>,
    ) -> Result<compallm_query::Connection<ProviderCatalog>, crate::query::QueryCompileError> {
        let context = DbContext::current()
            .map_err(|_| DbErr::Custom("catalog context unavailable".into()))?;
        let db = PostgresConnection::from_context(&context)
            .map_err(|_| DbErr::Custom("catalog connection unavailable".into()))?;
        let terms =
            compallm_core::provider::catalog::CatalogSearch::try_from(search.unwrap_or_default())
                .map_err(|_| DbErr::Custom("invalid catalog search".into()))?
                .terms;
        let mut select = Providers::find();
        {
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
                            Alias::new("catalog_provider_book"),
                            P::Id,
                        ))))
                        .like(LikeExpr::new(&pattern).escape('!')),
                    )
                    .add(
                        Expr::expr(Func::lower(Expr::col((
                            Alias::new("catalog_provider_book"),
                            P::Name,
                        ))))
                        .like(LikeExpr::new(&pattern).escape('!')),
                    );
                provider_match = provider_match.add(provider_term.clone());
                let model_term = Condition::any()
                    .add(
                        Expr::expr(Func::lower(Expr::col((
                            Alias::new("catalog_model_records"),
                            M::Id,
                        ))))
                        .like(LikeExpr::new(&pattern).escape('!')),
                    )
                    .add(
                        Expr::expr(Func::lower(Expr::col((
                            Alias::new("catalog_model_records"),
                            M::Name,
                        ))))
                        .like(LikeExpr::new(&pattern).escape('!')),
                    )
                    .add(
                        Expr::expr(Func::lower(Expr::col((
                            Alias::new("catalog_model_records"),
                            M::DisplayName,
                        ))))
                        .like(LikeExpr::new(&pattern).escape('!')),
                    );
                model_match = model_match.add(if terms.len() == 1 {
                    model_term
                } else {
                    model_term.add(provider_term)
                });
            }
            let matches = Condition::any().add(provider_match).add(model_match);
            let models = Models::find()
                .select_only()
                .expr(Expr::val(1))
                .filter(
                    Expr::col((Alias::new("catalog_model_records"), M::ProviderCatalogId))
                        .eq(Expr::col((Alias::new("catalog_provider_book"), P::Id))),
                )
                .filter(visibility(scope_owner))
                .filter(matches.clone())
                .filter(eligible_models())
                .into_query();
            select = select.filter(Expr::exists(models));
        }
        select = select.filter(P::ApiKeySupported.eq(true));
        let scope = format!(
            "{scope}:catalog-v1:{}",
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
                node: ProviderCatalog::try_from(edge.node)?,
            });
        }
        Ok(compallm_query::Connection {
            edges,
            page_info: result.page_info,
        })
    }
    pub async fn provider(&self, id: &str) -> Result<Option<ProviderCatalog>, DbErr> {
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
        scope: Option<CatalogScope>,
    ) -> Result<Vec<ModelCatalog>, crate::query::QueryCompileError> {
        let context = DbContext::current()
            .map_err(|_| DbErr::Custom("catalog context unavailable".into()))?;
        let db = PostgresConnection::from_context(&context)
            .map_err(|_| DbErr::Custom("catalog connection unavailable".into()))?;
        let select = Models::find()
            .filter(M::ProviderCatalogId.eq(provider))
            .filter(visibility(scope))
            .filter(eligible_models())
            .order_by_asc(M::Name)
            .order_by_asc(M::Id);
        let rows = query
            .apply_to_sea_query(
                select,
                Pagination::default(),
                vec![],
                Pagination::DEFAULT_MAX_ROWS,
            )?
            .limit(None)
            .all(db)
            .await?;
        super::assembly::resolve(db, rows)
            .await
            .map(|rows| rows.into_iter().map(|row| row.catalog).collect())
            .map_err(Into::into)
    }
    pub async fn model(
        &self,
        provider: &str,
        id: &str,
        scope: Option<CatalogScope>,
    ) -> Result<Option<ModelCatalog>, DbErr> {
        let context = DbContext::current()
            .map_err(|_| DbErr::Custom("catalog context unavailable".into()))?;
        let db = PostgresConnection::from_context(&context)
            .map_err(|_| DbErr::Custom("catalog connection unavailable".into()))?;
        let rows = Models::find()
            .filter(M::ProviderCatalogId.eq(provider))
            .filter(M::Id.eq(id))
            .filter(visibility(scope))
            .filter(eligible_models())
            .all(db)
            .await?;
        Ok(super::assembly::resolve(db, rows)
            .await?
            .into_iter()
            .next()
            .map(|row| row.catalog))
    }
}

impl CatalogRepository {
    pub async fn personal_row(
        &self,
        provider: &str,
        id: &str,
        scope: CatalogScope,
    ) -> Result<Option<super::assembly::ModelRecord>, DbErr> {
        let context = DbContext::current()
            .map_err(|_| DbErr::Custom("catalog context unavailable".into()))?;
        let db = PostgresConnection::from_context(&context)
            .map_err(|_| DbErr::Custom("catalog connection unavailable".into()))?;
        let rows = Models::find()
            .filter(M::ProviderCatalogId.eq(provider))
            .filter(M::Id.eq(id))
            .filter(M::OrganizationId.eq(scope.organization_id as i64))
            .filter(M::UserId.eq(scope.user_id as i64))
            .all(db)
            .await?;
        Ok(super::assembly::resolve(db, rows).await?.into_iter().next())
    }
    pub async fn global_row(
        &self,
        provider: &str,
        id: &str,
    ) -> Result<Option<super::assembly::ModelRecord>, DbErr> {
        let context = DbContext::current()
            .map_err(|_| DbErr::Custom("catalog context unavailable".into()))?;
        let db = PostgresConnection::from_context(&context)
            .map_err(|_| DbErr::Custom("catalog connection unavailable".into()))?;
        let rows = Models::find()
            .filter(M::ProviderCatalogId.eq(provider))
            .filter(M::Id.eq(id))
            .filter(M::UserId.is_null())
            .all(db)
            .await?;
        Ok(super::assembly::resolve(db, rows).await?.into_iter().next())
    }
    pub async fn save_personal(
        &self,
        value: ModelCatalog,
        scope: CatalogScope,
        base: Option<uuid::Uuid>,
        changed: &[String],
    ) -> Result<ModelCatalog, DbErr> {
        super::write::save(value, scope, base, changed)
            .await
            .map_err(|e| DbErr::Custom(e.to_string()))
    }
    pub async fn delete_personal(
        &self,
        provider: &str,
        id: &str,
        scope: CatalogScope,
        custom: bool,
    ) -> Result<bool, DbErr> {
        super::write::delete(provider, id, scope, custom)
            .await
            .map_err(|e| DbErr::Custom(e.to_string()))
    }
}
