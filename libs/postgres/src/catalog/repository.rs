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

#[derive(Clone, Copy, Debug)]
pub struct CatalogScope {
    pub organization_id: u64,
    pub user_id: u64,
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
    ) -> Result<compallm_query::Connection<ProviderTemplate>, crate::query::QueryCompileError> {
        let context = DbContext::current()
            .map_err(|_| DbErr::Custom("catalog context unavailable".into()))?;
        let db = PostgresConnection::from_context(&context)
            .map_err(|_| DbErr::Custom("catalog connection unavailable".into()))?;
        let terms =
            compallm_core::provider::template::CatalogSearch::try_from(search.unwrap_or_default())
                .map_err(|_| DbErr::Custom("invalid catalog search".into()))?
                .terms;
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
                let model_term = Condition::any()
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
                    );
                model_match = model_match.add(if terms.len() == 1 {
                    model_term
                } else {
                    model_term.add(provider_term)
                });
            }
            let ready = Readiness::find()
                .select_only()
                .expr(
                    Expr::col((Alias::new("provider_catalog_readiness"), R::Readiness))
                        .eq(crate::CatalogReadinessSchema::Ready),
                )
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
                .limit(1)
                .into_query();
            // Keep indexed name predicates outside CASE; verify readiness only for matching identities.
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
                .filter(visibility(scope_owner))
                .filter(model_match.clone())
                .filter(Expr::Case(Box::new(
                    Expr::case(
                        model_match,
                        Expr::case(M::UserId.is_not_null(), true).finally(Expr::from(ready)),
                    )
                    .finally(false),
                )))
                .into_query();
            select = select.filter(
                Condition::any()
                    .add(provider_match)
                    .add(Expr::exists(models)),
            );
        }
        let scope = format!(
            "{scope}:search-v2:{}",
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
        scope: Option<CatalogScope>,
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
            .filter(visibility(scope))
            .filter(
                Condition::any()
                    .add(M::UserId.is_not_null())
                    .add(M::Id.in_subquery(eligible)),
            )
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
    pub async fn model(
        &self,
        provider: &str,
        id: &str,
        scope: Option<CatalogScope>,
    ) -> Result<Option<ModelTemplate>, DbErr> {
        let context = DbContext::current()
            .map_err(|_| DbErr::Custom("catalog context unavailable".into()))?;
        let db = PostgresConnection::from_context(&context)
            .map_err(|_| DbErr::Custom("catalog connection unavailable".into()))?;
        Models::find()
            .filter(M::ProviderTemplateId.eq(provider))
            .filter(M::Id.eq(id))
            .filter(visibility(scope))
            .filter(
                Condition::any().add(M::UserId.is_not_null()).add(
                    M::Id.in_subquery(
                        Readiness::find()
                            .select_only()
                            .column(R::Id)
                            .filter(R::ProviderTemplateId.eq(provider))
                            .filter(R::Readiness.eq(crate::CatalogReadinessSchema::Ready))
                            .into_query(),
                    ),
                ),
            )
            .one(db)
            .await?
            .map(TryInto::try_into)
            .transpose()
    }
}

impl CatalogRepository {
    pub async fn personal_row(
        &self,
        provider: &str,
        id: &str,
        scope: CatalogScope,
    ) -> Result<Option<crate::ProviderModelTemplateSchema>, DbErr> {
        let context = DbContext::current()
            .map_err(|_| DbErr::Custom("catalog context unavailable".into()))?;
        let db = PostgresConnection::from_context(&context)
            .map_err(|_| DbErr::Custom("catalog connection unavailable".into()))?;
        Models::find()
            .filter(M::ProviderTemplateId.eq(provider))
            .filter(M::Id.eq(id))
            .filter(M::OrganizationId.eq(scope.organization_id as i64))
            .filter(M::UserId.eq(scope.user_id as i64))
            .one(db)
            .await
    }
    pub async fn global_row(
        &self,
        provider: &str,
        id: &str,
    ) -> Result<Option<crate::ProviderModelTemplateSchema>, DbErr> {
        let context = DbContext::current()
            .map_err(|_| DbErr::Custom("catalog context unavailable".into()))?;
        let db = PostgresConnection::from_context(&context)
            .map_err(|_| DbErr::Custom("catalog connection unavailable".into()))?;
        Models::find()
            .filter(M::ProviderTemplateId.eq(provider))
            .filter(M::Id.eq(id))
            .filter(M::UserId.is_null())
            .one(db)
            .await
    }
    pub async fn save_personal(
        &self,
        value: ModelTemplate,
        scope: CatalogScope,
        base: Option<uuid::Uuid>,
        changed: &[String],
    ) -> Result<ModelTemplate, DbErr> {
        use sea_orm::{ActiveValue::Set, sea_query::OnConflict};
        let context = DbContext::current()
            .map_err(|_| DbErr::Custom("catalog context unavailable".into()))?;
        let db = PostgresConnection::from_context(&context)
            .map_err(|_| DbErr::Custom("catalog connection unavailable".into()))?;
        let json = |v| {
            serde_json::to_value(v)
                .map_err(|_| DbErr::Custom("invalid template configuration".into()))
        };
        let encoded = serde_json::to_value(&value)
            .map_err(|_| DbErr::Custom("invalid template configuration".into()))?;
        let get = |name: &str| {
            encoded
                .get(name)
                .cloned()
                .unwrap_or(serde_json::Value::Null)
        };
        type ModelWrite = <Models as EntityTrait>::ActiveModel;
        let row = ModelWrite {
            row_id: Set(uuid::Uuid::new_v4()),
            custom_fields: Set(get("customFields")),
            organization_id: Set(Some(scope.organization_id as i64)),
            user_id: Set(Some(scope.user_id as i64)),
            base_row_id: Set(base),
            research_id: Set(None),
            provider_template_id: Set(value.provider_template_id.clone()),
            id: Set(value.id.clone()),
            name: Set(value.name),
            display_name: Set(value.display_name),
            description: Set(value.description),
            service_kind: Set(value.service_kind.map(Into::into)),
            connections: Set(get("connections")),
            headers: Set(get("headers")),
            body_defaults: Set(get("bodyDefaults")),
            text: Set(get("text")),
            vision: Set(get("vision")),
            pdf: Set(get("pdf")),
            audio: Set(get("audio")),
            video: Set(get("video")),
            image_generation: Set(get("imageGeneration")),
            tools: Set(get("tools")),
            structured_output: Set(get("structuredOutput")),
            search: Set(get("search")),
            limits: Set(get("limits")),
            billing: Set(value.billing.map(json).transpose()?),
            route_selection: Set(value
                .route_selection
                .map(|v| {
                    serde_json::to_value(v)
                        .map_err(|_| DbErr::Custom("invalid route configuration".into()))
                })
                .transpose()?),
        };
        let mut conflict =
            OnConflict::columns([M::OrganizationId, M::UserId, M::ProviderTemplateId, M::Id]);
        conflict.target_and_where(M::UserId.is_not_null());
        let has = |name: &str| changed.iter().any(|v| v == name);
        let mut columns = Vec::new();
        for (name, column) in [
            ("name", M::Name),
            ("path", M::Connections),
            ("headers", M::Headers),
            ("body_defaults", M::BodyDefaults),
            ("limits", M::Limits),
        ] {
            if has(name) {
                columns.push(column);
            }
        }
        if has("capabilities") {
            columns.extend([
                M::Vision,
                M::Pdf,
                M::Audio,
                M::Video,
                M::ImageGeneration,
                M::Tools,
                M::StructuredOutput,
                M::Search,
            ]);
        }
        if columns.is_empty() {
            conflict.value(
                M::Name,
                Expr::col((Alias::new("provider_model_templates"), M::Name)),
            );
        } else {
            conflict.update_columns(columns);
        }
        let mut text: sea_orm::sea_query::Expr =
            Expr::col((Alias::new("provider_model_templates"), M::Text));
        let mut text_changed = false;
        for (enabled, path) in [
            (has("limits"), vec!["input", "configuration", "maxTokens"]),
            (has("limits"), vec!["output", "configuration", "maxTokens"]),
            (has("input_price"), vec!["input", "pricing"]),
            (has("output_price"), vec!["output", "pricing"]),
            (
                has("cached_price"),
                vec![
                    "input",
                    "pricing",
                    "configuration",
                    "fixed",
                    "token",
                    "cache",
                ],
            ),
            (
                has("capabilities") || has("effort_mapping"),
                vec!["output", "configuration", "thinking"],
            ),
        ] {
            if enabled {
                let pointer = format!("/{}", path.join("/"));
                let value = get("text")
                    .pointer(&pointer)
                    .cloned()
                    .unwrap_or(serde_json::Value::Null);
                text = Func::cust(Alias::new("jsonb_set"))
                    .args([
                        text,
                        Expr::val(path.into_iter().map(String::from).collect::<Vec<_>>())
                            .cast_as(Alias::new("text[]")),
                        Expr::val(value),
                        Expr::val(true),
                    ])
                    .into();
                text_changed = true;
            }
        }
        if text_changed {
            conflict.value(M::Text, text);
        }
        let mut flags: sea_orm::sea_query::Expr =
            Expr::col((Alias::new("provider_model_templates"), M::CustomFields));
        for name in [
            "contextLimitEnabled",
            "inputLimitEnabled",
            "outputLimitEnabled",
            "inputPriceEnabled",
            "outputPriceEnabled",
            "cachedPriceEnabled",
        ] {
            let relevant = if name.ends_with("LimitEnabled") {
                has("limits")
            } else {
                has(match name {
                    "inputPriceEnabled" => "input_price",
                    "outputPriceEnabled" => "output_price",
                    _ => "cached_price",
                })
            };
            if relevant {
                flags = flags.binary(sea_orm::sea_query::BinOper::Sub, Expr::val(name));
                if value.custom_fields.contains(name) {
                    flags = flags.binary(
                        sea_orm::sea_query::BinOper::Custom("||"),
                        Expr::val(serde_json::json!([name])),
                    );
                }
            }
        }
        conflict.value(M::CustomFields, flags);
        Models::insert(row)
            .on_conflict(conflict.to_owned())
            .exec(db)
            .await?;
        self.personal_row(&value.provider_template_id, &value.id, scope)
            .await?
            .ok_or_else(|| DbErr::Custom("template save unavailable".into()))?
            .try_into()
    }
    pub async fn delete_personal(
        &self,
        provider: &str,
        id: &str,
        scope: CatalogScope,
        custom: bool,
    ) -> Result<bool, DbErr> {
        let context = DbContext::current()
            .map_err(|_| DbErr::Custom("catalog context unavailable".into()))?;
        let db = PostgresConnection::from_context(&context)
            .map_err(|_| DbErr::Custom("catalog connection unavailable".into()))?;
        let mut query = Models::delete_many()
            .filter(M::ProviderTemplateId.eq(provider))
            .filter(M::Id.eq(id))
            .filter(M::OrganizationId.eq(scope.organization_id as i64))
            .filter(M::UserId.eq(scope.user_id as i64));
        query = if custom {
            query.filter(M::BaseRowId.is_null())
        } else {
            query.filter(M::BaseRowId.is_not_null())
        };
        Ok(query.exec(db).await?.rows_affected > 0)
    }
}
