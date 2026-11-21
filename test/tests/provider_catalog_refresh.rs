use compallm_postgres::Migrator;
#[path = "support/catalog_legacy_provider_model_templates.rs"]
mod legacy_models;
#[path = "support/catalog_legacy_provider_templates.rs"]
mod legacy_providers;
use legacy_providers::Model as ProviderTemplateSchema;
use sea_orm::{
    ColumnTrait, Database, EntityTrait, ExprTrait, ModelTrait, QueryFilter, QueryOrder,
    TransactionTrait,
    sea_query::{Alias, Expr, Query},
};
use sea_orm_migration::{MigratorTrait, SchemaManager};

type Providers = <ProviderTemplateSchema as ModelTrait>::Entity;
type Column = <Providers as EntityTrait>::Column;
type Models = <legacy_models::Model as ModelTrait>::Entity;
type ModelColumn = <Models as EntityTrait>::Column;

#[tokio::test]
async fn catalog_refresh_round_trips_and_preserves_concurrent_edits() {
    let Ok(url) = std::env::var("PROVIDER_CATALOG_TEST_URL") else {
        return;
    };
    assert!(
        url.ends_with("/compallm_provider_catalog_test"),
        "use the dedicated catalog test database"
    );
    let db = Database::connect(url).await.unwrap();
    let migrations = Migrator::migrations();
    for suffix in [
        "create_provider_templates",
        "create_provider_model_templates",
        "seed_provider_templates",
    ] {
        let migration = migrations
            .iter()
            .find(|m| m.name().ends_with(suffix))
            .unwrap();
        let transaction = db.begin().await.unwrap();
        migration
            .up(&SchemaManager::new(&transaction))
            .await
            .unwrap();
        transaction.commit().await.unwrap();
    }
    let migration = migrations
        .iter()
        .find(|m| m.name().ends_with("provider_catalog"))
        .unwrap();
    let transaction = db.begin().await.unwrap();
    migration
        .up(&SchemaManager::new(&transaction))
        .await
        .unwrap();
    transaction.commit().await.unwrap();

    let providers = Providers::find()
        .order_by_asc(Column::Id)
        .all(&db)
        .await
        .unwrap();
    assert_eq!(providers.len(), 145);
    assert!(providers.iter().all(|p| {
        p.logo_path.starts_with("/assets/providers/")
            && p.source_refs
                .as_array()
                .is_some_and(|refs| !refs.is_empty())
    }));
    let openai = providers.iter().find(|p| p.id == "openai").unwrap();
    let mut credential_url_source = providers
        .iter()
        .flat_map(|provider| provider.model_info_sources.as_array().unwrap())
        .find(|source| source["kind"] == "api" && source["status"] == "observed")
        .unwrap()
        .clone();
    credential_url_source["url"] =
        serde_json::json!("https://models.example/info?api_key=must-not-store");
    for invalid_source in [
        serde_json::json!([]),
        serde_json::json!([credential_url_source]),
    ] {
        let transaction = db.begin().await.unwrap();
        assert!(
            Providers::update_many()
                .col_expr(Column::ModelInfoSources, Expr::value(invalid_source))
                .filter(Column::Id.eq("openai"))
                .exec(&transaction)
                .await
                .is_err()
        );
        transaction.rollback().await.unwrap();
    }
    let models = <legacy_models::Model as ModelTrait>::Entity::find()
        .all(&db)
        .await
        .unwrap();
    assert!(models.iter().all(|model| {
        compallm_postgres::catalog::model_info::ModelInfoSources::try_from(
            model.model_info_sources.clone(),
        )
        .is_ok()
    }));
    let removal_manifest: serde_json::Value = serde_json::from_str(include_str!(
        "../../libs/postgres/src/migration/m20261117_000047_provider_catalog/removed-unreferenced-incomplete-models.json"
    )).unwrap();
    for row in removal_manifest["removed_rows"].as_array().unwrap() {
        assert!(!models.iter().any(|model| model.provider_template_id
            == row["provider_template_id"].as_str().unwrap()
            && model.id == row["id"].as_str().unwrap()));
    }
    assert_eq!(models.len(), 2348);
    let retained = models
        .iter()
        .find(|m| {
            m.provider_template_id == "nvidia" && m.upstream_model_id == "openai/whisper-large-v3"
        })
        .unwrap();
    assert_eq!(retained.protocols, serde_json::json!([]));
    assert_ne!(
        retained.service_kind,
        compallm_postgres::CatalogServiceKindSchema::Text
    );
    for id in [
        "google/diffusiongemma-26b-a4b-it",
        "meta/llama-3.2-11b-vision-instruct",
        "meta/llama-3.2-90b-vision-instruct",
        "meta/llama-guard-4-12b",
        "minimaxai/minimax-m2.7",
        "minimaxai/minimax-m3",
    ] {
        let model = models
            .iter()
            .find(|m| m.provider_template_id == "nvidia" && m.upstream_model_id == id)
            .unwrap();
        assert_eq!(model.protocols, serde_json::json!([]));
        assert!(model.unresolved_fields.get("operations").is_some());
    }
    let ready = models
        .iter()
        .find(|m| {
            m.readiness == compallm_postgres::CatalogReadinessSchema::Ready
                && m.pricing_mode == compallm_postgres::CatalogPricingModeSchema::Fixed
                && m.cache_billing == compallm_postgres::CatalogCacheBillingSchema::SeparateRate
        })
        .unwrap();
    let fallback = serde_json::json!({
        "currency":"USD", "unit_quantity":1000000,
        "input_price":"0.25", "output_price":"1", "cached_price":"0.25",
        "basis":"same_model_other_provider", "reference_provider":"reference-host",
        "reference_model":ready.upstream_model_id, "source_urls":["https://example.com/pricing"],
        "confidence":"low", "notes":"Estimated fallback from another host; cache cost uses full input rate."
    });
    let transaction = db.begin().await.unwrap();
    let mut cfg = ready.pricing_config.clone();
    cfg["fallback"] = fallback.clone();
    Models::update_many()
        .col_expr(ModelColumn::PricingConfig, Expr::value(cfg))
        .filter(ModelColumn::ProviderTemplateId.eq(ready.provider_template_id.clone()))
        .filter(ModelColumn::Id.eq(ready.id.clone()))
        .exec(&transaction)
        .await
        .unwrap();
    let with_fallback = Models::find_by_id((ready.provider_template_id.clone(), ready.id.clone()))
        .one(&transaction)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(with_fallback.input_price, ready.input_price);
    assert_eq!(with_fallback.readiness, ready.readiness);
    assert_eq!(with_fallback.resolution_plan, ready.resolution_plan);
    assert_eq!(with_fallback.pricing_config["fallback"], fallback);
    transaction.rollback().await.unwrap();
    let incomplete = models
        .iter()
        .find(|m| !m.unresolved_fields.as_object().unwrap().is_empty())
        .unwrap();
    let mut no_zero_evidence = ready.pricing_config.clone();
    no_zero_evidence
        .as_object_mut()
        .unwrap()
        .remove("zero_price_evidence");
    let invalid_updates = [
        (
            ready,
            vec![
                (
                    ModelColumn::InputPrice,
                    Expr::value(sea_orm::prelude::Decimal::ZERO),
                ),
                (ModelColumn::PricingConfig, Expr::value(no_zero_evidence)),
            ],
        ),
        (
            ready,
            vec![(
                ModelColumn::InputPrice,
                Expr::value(Option::<sea_orm::prelude::Decimal>::None),
            )],
        ),
        (
            ready,
            vec![(
                ModelColumn::CacheBilling,
                Expr::val("unsupported").cast_as(Alias::new("catalog_cache_billing")),
            )],
        ),
        (
            ready,
            vec![(
                ModelColumn::PricingMode,
                Expr::val("api").cast_as(Alias::new("catalog_pricing_mode")),
            )],
        ),
        (
            ready,
            vec![(
                ModelColumn::Capabilities,
                Expr::value(serde_json::json!([])),
            )],
        ),
        (
            ready,
            vec![(
                ModelColumn::BillingModel,
                Expr::val("subscription").cast_as(Alias::new("catalog_billing_model")),
            )],
        ),
        (
            ready,
            vec![(
                ModelColumn::BillingModel,
                Expr::val("free").cast_as(Alias::new("catalog_billing_model")),
            )],
        ),
        (
            ready,
            vec![(
                ModelColumn::ResolutionPlan,
                Expr::value(
                    serde_json::json!({"input_price":{"method":"api","source_index":99999}}),
                ),
            )],
        ),
        (
            incomplete,
            vec![(
                ModelColumn::UnresolvedFields,
                Expr::value(serde_json::json!({})),
            )],
        ),
    ];
    for (model, values) in invalid_updates {
        let transaction = db.begin().await.unwrap();
        let mut update = Models::update_many();
        for (column, value) in values {
            update = update.col_expr(column, value);
        }
        assert!(
            update
                .filter(ModelColumn::ProviderTemplateId.eq(model.provider_template_id.clone()))
                .filter(ModelColumn::Id.eq(model.id.clone()))
                .exec(&transaction)
                .await
                .is_err()
        );
        transaction.rollback().await.unwrap();
    }
    let api_model = models
        .iter()
        .find(|m| {
            m.readiness == compallm_postgres::CatalogReadinessSchema::Ready
                && m.provider_template_id == "openrouter"
                && m.pricing_mode == compallm_postgres::CatalogPricingModeSchema::Fixed
                && m.cache_billing == compallm_postgres::CatalogCacheBillingSchema::SeparateRate
                && m.model_info_sources
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|source| {
                        source["kind"] == "api"
                            && source["status"] == "observed"
                            && source["response"]["identity_value"] == m.upstream_model_id
                            && [
                                "max_context_tokens",
                                "input_price",
                                "output_price",
                                "cached_price",
                            ]
                            .iter()
                            .all(|f| {
                                source["response"]["fields"][f]["pointer"].is_string()
                                    && (f.starts_with("max_")
                                        || matches!(
                                            source["response"]["fields"][f]["unit"].as_str(),
                                            Some("usd_per_token" | "usd_per_million_tokens")
                                        ))
                            })
                    })
        })
        .unwrap();
    let source_index = api_model
        .model_info_sources
        .as_array()
        .unwrap()
        .iter()
        .position(|source| {
            source["kind"] == "api"
                && source["status"] == "observed"
                && source["response"]["identity_value"] == api_model.upstream_model_id
                && [
                    "max_context_tokens",
                    "input_price",
                    "output_price",
                    "cached_price",
                ]
                .iter()
                .all(|f| {
                    source["response"]["fields"][f]["pointer"].is_string()
                        && (f.starts_with("max_")
                            || matches!(
                                source["response"]["fields"][f]["unit"].as_str(),
                                Some("usd_per_token" | "usd_per_million_tokens")
                            ))
                })
        })
        .unwrap();
    let transaction = db.begin().await.unwrap();
    let mut plan = serde_json::json!({});
    for f in [
        "max_context_tokens",
        "input_price",
        "output_price",
        "cached_price",
    ] {
        plan[f] = serde_json::json!({"method":"api","source_index":source_index});
    }
    let mut api_pricing = api_model.pricing_config.clone();
    api_pricing
        .as_object_mut()
        .unwrap()
        .remove("zero_price_evidence");
    Models::update_many()
        .col_expr(ModelColumn::PricingConfig, Expr::value(api_pricing))
        .col_expr(
            ModelColumn::PricingMode,
            Expr::val("api").cast_as(Alias::new("catalog_pricing_mode")),
        )
        .col_expr(
            ModelColumn::SpecificationMode,
            Expr::val("api").cast_as(Alias::new("catalog_specification_mode")),
        )
        .col_expr(ModelColumn::ResolutionPlan, Expr::value(plan))
        .col_expr(
            ModelColumn::MaxContextTokens,
            Expr::value(Option::<i64>::None),
        )
        .col_expr(
            ModelColumn::InputPrice,
            Expr::value(Option::<sea_orm::prelude::Decimal>::None),
        )
        .col_expr(
            ModelColumn::OutputPrice,
            Expr::value(Option::<sea_orm::prelude::Decimal>::None),
        )
        .col_expr(
            ModelColumn::CachedPrice,
            Expr::value(Option::<sea_orm::prelude::Decimal>::None),
        )
        .filter(ModelColumn::ProviderTemplateId.eq(api_model.provider_template_id.clone()))
        .filter(ModelColumn::Id.eq(api_model.id.clone()))
        .exec(&transaction)
        .await
        .unwrap();
    let fetched =
        Models::find_by_id((api_model.provider_template_id.clone(), api_model.id.clone()))
            .one(&transaction)
            .await
            .unwrap()
            .unwrap();
    assert_eq!(
        fetched.readiness,
        compallm_postgres::CatalogReadinessSchema::NeedsFetch
    );
    transaction.rollback().await.unwrap();
    let subscription = models
        .iter()
        .find(|m| {
            m.billing_model == compallm_postgres::CatalogBillingModelSchema::Subscription
                && m.unresolved_fields == serde_json::json!({})
        })
        .unwrap();
    assert_eq!(
        subscription.readiness,
        compallm_postgres::CatalogReadinessSchema::NeedsConfiguration
    );
    assert!(
        subscription.input_price.is_none()
            && subscription.output_price.is_none()
            && subscription.cached_price.is_none()
    );
    let transaction = db.begin().await.unwrap();
    assert!(Models::update_many().col_expr(ModelColumn::PricingConfig,Expr::value(serde_json::json!({"subscription":{"product":"plan","plan_mode":"stored","plan":"pro","source_urls":["https://example.com/plan"],"entitlement":{}}}))).filter(ModelColumn::ProviderTemplateId.eq(subscription.provider_template_id.clone())).filter(ModelColumn::Id.eq(subscription.id.clone())).exec(&transaction).await.is_err());
    transaction.rollback().await.unwrap();
    let transaction = db.begin().await.unwrap();
    let mut proxy_config = subscription.pricing_config.clone();
    proxy_config["cost_proxy"] = fallback.clone();
    Models::update_many()
        .col_expr(ModelColumn::PricingConfig, Expr::value(proxy_config))
        .filter(ModelColumn::ProviderTemplateId.eq(subscription.provider_template_id.clone()))
        .filter(ModelColumn::Id.eq(subscription.id.clone()))
        .exec(&transaction)
        .await
        .unwrap();
    let proxy_model = Models::find_by_id((
        subscription.provider_template_id.clone(),
        subscription.id.clone(),
    ))
    .one(&transaction)
    .await
    .unwrap()
    .unwrap();
    assert!(proxy_model.input_price.is_none() && proxy_model.output_price.is_none());
    assert_eq!(proxy_model.billing_model, subscription.billing_model);
    assert_eq!(proxy_model.pricing_config["cost_proxy"], fallback);
    let mut complete_plan = subscription.pricing_config.clone();
    complete_plan["subscription"]["plan_mode"] = serde_json::json!("stored");
    complete_plan["subscription"]
        .as_object_mut()
        .unwrap()
        .remove("required_inputs");
    complete_plan["subscription"]["plan"] = serde_json::json!("documented-plan");
    complete_plan["subscription"]["entitlement"] = serde_json::json!({"unit":"requests","quota":100,"period":"month","overage_policy":"blocked"});
    complete_plan["subscription"]["fee"] =
        serde_json::json!({"amount":"20","currency":"USD","period":"month"});
    Models::update_many()
        .col_expr(ModelColumn::PricingConfig, Expr::value(complete_plan))
        .filter(ModelColumn::ProviderTemplateId.eq(subscription.provider_template_id.clone()))
        .filter(ModelColumn::Id.eq(subscription.id.clone()))
        .exec(&transaction)
        .await
        .unwrap();
    assert_eq!(
        Models::find_by_id((
            subscription.provider_template_id.clone(),
            subscription.id.clone()
        ))
        .one(&transaction)
        .await
        .unwrap()
        .unwrap()
        .readiness,
        compallm_postgres::CatalogReadinessSchema::Ready
    );
    transaction.rollback().await.unwrap();
    let transaction = db.begin().await.unwrap();
    assert!(
        Models::update_many()
            .col_expr(
                ModelColumn::Readiness,
                Expr::val("ready").cast_as(Alias::new("catalog_readiness"))
            )
            .filter(ModelColumn::ProviderTemplateId.eq(incomplete.provider_template_id.clone()))
            .filter(ModelColumn::Id.eq(incomplete.id.clone()))
            .exec(&transaction)
            .await
            .is_err()
    );
    transaction.rollback().await.unwrap();
    let tiered = models
        .iter()
        .find(|m| m.pricing_mode == compallm_postgres::CatalogPricingModeSchema::Tiered)
        .unwrap();
    let mut gap = tiered.pricing_config.clone();
    gap["tiers"][0]["from_tokens"] = serde_json::json!(1);
    let transaction = db.begin().await.unwrap();
    assert!(
        Models::update_many()
            .col_expr(ModelColumn::PricingConfig, Expr::value(gap))
            .filter(ModelColumn::ProviderTemplateId.eq(tiered.provider_template_id.clone()))
            .filter(ModelColumn::Id.eq(tiered.id.clone()))
            .exec(&transaction)
            .await
            .is_err()
    );
    transaction.rollback().await.unwrap();
    let mut overlap = tiered.pricing_config.clone();
    overlap["tiers"][1]["from_tokens"] = serde_json::json!(0);
    let transaction = db.begin().await.unwrap();
    assert!(
        Models::update_many()
            .col_expr(ModelColumn::PricingConfig, Expr::value(overlap))
            .filter(ModelColumn::ProviderTemplateId.eq(tiered.provider_template_id.clone()))
            .filter(ModelColumn::Id.eq(tiered.id.clone()))
            .exec(&transaction)
            .await
            .is_err()
    );
    transaction.rollback().await.unwrap();
    let transaction = db.begin().await.unwrap();
    assert!(
        Providers::update_many()
            .col_expr(
                Column::AuthConfig,
                Expr::value(serde_json::json!({"api_key":{"status":"configured","placements":[]}}))
            )
            .filter(Column::Id.eq("openai"))
            .exec(&transaction)
            .await
            .is_err()
    );
    transaction.rollback().await.unwrap();
    let transaction = db.begin().await.unwrap();
    Providers::update_many().col_expr(Column::AuthConfig,Expr::value(serde_json::json!({"api_key":{"status":"unknown","reason":"Placement not established"}}))).filter(Column::Id.eq("openai")).exec(&transaction).await.unwrap();
    let query = Query::select()
        .column(Alias::new("readiness"))
        .from(Alias::new("provider_catalog_readiness"))
        .and_where(Expr::col(Alias::new("provider_template_id")).eq("openai"))
        .to_owned();
    let rows = transaction.query_all(&query).await.unwrap();
    assert!(!rows.is_empty());
    assert!(rows.iter().all(|r| {
        r.try_get::<compallm_postgres::CatalogReadinessSchema>("", "readiness")
            .unwrap()
            == compallm_postgres::CatalogReadinessSchema::Incomplete
    }));
    transaction.rollback().await.unwrap();
    let transaction = db.begin().await.unwrap();
    let mut methods = openai.auth_methods.clone();
    methods
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!("oauth"));
    let mut auth = openai.auth_config.clone();
    auth["oauth"] = serde_json::json!({"status":"unknown","reason":"Optional flow not documented"});
    Providers::update_many()
        .col_expr(Column::AuthMethods, Expr::value(methods))
        .col_expr(Column::AuthConfig, Expr::value(auth))
        .filter(Column::Id.eq("openai"))
        .exec(&transaction)
        .await
        .unwrap();
    assert_eq!(
        Providers::find_by_id("openai")
            .one(&transaction)
            .await
            .unwrap()
            .unwrap()
            .readiness,
        compallm_postgres::CatalogReadinessSchema::Ready
    );
    let query = Query::select()
        .columns([Alias::new("auth_method"), Alias::new("provider_readiness")])
        .from(Alias::new("provider_catalog_auth_readiness"))
        .and_where(Expr::col(Alias::new("provider_template_id")).eq("openai"))
        .to_owned();
    let auth_rows = transaction.query_all(&query).await.unwrap();
    assert!(
        auth_rows
            .iter()
            .any(|r| r.try_get::<String>("", "auth_method").unwrap() == "api_key")
    );
    for row in auth_rows {
        let expected = if row.try_get::<String>("", "auth_method").unwrap() == "oauth" {
            compallm_postgres::CatalogReadinessSchema::Incomplete
        } else {
            compallm_postgres::CatalogReadinessSchema::Ready
        };
        assert_eq!(
            row.try_get::<compallm_postgres::CatalogReadinessSchema>("", "provider_readiness")
                .unwrap(),
            expected
        );
    }
    transaction.rollback().await.unwrap();
    for (placements, expected) in [
        (
            serde_json::json!([
                {"operations":["openai_chat_completions"],"adapter_requirements":[]}
            ]),
            compallm_postgres::CatalogReadinessSchema::Incomplete,
        ),
        (
            serde_json::json!([
                {"operations":["openai_chat_completions","openai_responses"],"adapter_requirements":["custom_signing"]}
            ]),
            compallm_postgres::CatalogReadinessSchema::Incomplete,
        ),
        (
            serde_json::json!([
                {"operations":["openai_chat_completions","openai_responses"],"adapter_requirements":[]}
            ]),
            compallm_postgres::CatalogReadinessSchema::Ready,
        ),
    ] {
        let provider = serde_json::json!({"endpoint_mode":"fixed","auth_methods":["api_key","oauth"],
            "auth_config":{"api_key":{"status":"configured","placements":placements},
                "oauth":{"status":"unknown","reason":"Optional flow unknown"}},"field_metadata":{}});
        let query = Query::select()
            .expr_as(
                sea_orm::sea_query::Func::cust(Alias::new("catalog_provider_model_state"))
                    .arg(Expr::value(provider))
                    .arg(Expr::value(serde_json::json!([
                        "openai_chat_completions",
                        "openai_responses"
                    ]))),
                Alias::new("state"),
            )
            .to_owned();
        assert_eq!(
            db.query_one(&query)
                .await
                .unwrap()
                .unwrap()
                .try_get::<compallm_postgres::CatalogReadinessSchema>("", "state")
                .unwrap(),
            expected
        );
    }
    let route_model = models
        .iter()
        .find(|m| m.cache_billing == compallm_postgres::CatalogCacheBillingSchema::PerRoute)
        .unwrap();
    assert_eq!(
        route_model.readiness,
        compallm_postgres::CatalogReadinessSchema::NeedsConfiguration
    );
    let mut route_variants = Vec::new();
    let mut cfg = route_model.pricing_config.clone();
    cfg["route_policy"]["allow_fallbacks"] = serde_json::json!(true);
    route_variants.push(cfg);
    let mut cfg = route_model.pricing_config.clone();
    cfg["route_options"][0]["rates"]
        .as_object_mut()
        .unwrap()
        .remove("cached_price");
    route_variants.push(cfg);
    let mut cfg = route_model.pricing_config.clone();
    cfg["required_inputs"][0]["allowed_values"] = serde_json::json!(["another-route"]);
    route_variants.push(cfg);
    let mut cfg = route_model.pricing_config.clone();
    cfg["route_options"][0]["rates"]["cached_price"] = serde_json::json!("999");
    route_variants.push(cfg);
    for cfg in route_variants {
        let transaction = db.begin().await.unwrap();
        assert!(
            Models::update_many()
                .col_expr(ModelColumn::PricingConfig, Expr::value(cfg))
                .filter(
                    ModelColumn::ProviderTemplateId.eq(route_model.provider_template_id.clone())
                )
                .filter(ModelColumn::Id.eq(route_model.id.clone()))
                .exec(&transaction)
                .await
                .is_err()
        );
        transaction.rollback().await.unwrap();
    }
    let free_model = models
        .iter()
        .find(|m| {
            m.billing_model == compallm_postgres::CatalogBillingModelSchema::Free
                && m.readiness == compallm_postgres::CatalogReadinessSchema::Ready
        })
        .unwrap();
    assert!(
        free_model.input_price.is_none()
            && free_model.output_price.is_none()
            && free_model.cached_price.is_none()
    );
    assert_eq!(
        free_model.cache_billing,
        compallm_postgres::CatalogCacheBillingSchema::Included
    );
    assert_eq!(
        free_model.readiness,
        compallm_postgres::CatalogReadinessSchema::Ready
    );
    let silicon_models: Vec<_> = models
        .iter()
        .filter(|m| m.provider_template_id == "siliconflow")
        .collect();
    assert_eq!(silicon_models.len(), 132);
    let no_cache = silicon_models
        .iter()
        .find(|m| m.upstream_model_id == "Qwen/Qwen3.6-27B")
        .unwrap();
    assert_eq!(
        no_cache.cache_billing,
        compallm_postgres::CatalogCacheBillingSchema::Unsupported
    );
    assert!(no_cache.cached_price.is_none());
    let scheduled = silicon_models
        .iter()
        .find(|m| m.pricing_mode == compallm_postgres::CatalogPricingModeSchema::Scheduled)
        .unwrap();
    for incomplete in [false, true] {
        let mut cfg = scheduled.pricing_config.clone();
        if incomplete {
            cfg["schedule"]["slots"].as_array_mut().unwrap().pop();
        } else {
            cfg["schedule"]["slots"][0]["from_hour"] = serde_json::json!(1);
        }
        let transaction = db.begin().await.unwrap();
        assert!(
            Models::update_many()
                .col_expr(ModelColumn::PricingConfig, Expr::value(cfg))
                .filter(ModelColumn::ProviderTemplateId.eq(scheduled.provider_template_id.clone()))
                .filter(ModelColumn::Id.eq(scheduled.id.clone()))
                .exec(&transaction)
                .await
                .is_err()
        );
        transaction.rollback().await.unwrap();
    }
    let removed_pricing: serde_json::Value = serde_json::from_str(include_str!(
        "../../libs/postgres/src/migration/m20261117_000047_provider_catalog/removed-unreferenced-nonscalar-price-models.json"
    )).unwrap();
    for row in removed_pricing["removed_rows"].as_array().unwrap() {
        assert!(!models.iter().any(|model| model.provider_template_id
            == row["provider_template_id"].as_str().unwrap()
            && model.id == row["id"].as_str().unwrap()));
    }
    let precision = models
        .iter()
        .find(|m| m.provider_template_id == "morph" && m.upstream_model_id == "morph-dsv4flash")
        .unwrap();
    assert_eq!(precision.input_price.unwrap().to_string(), "0.141953125");
    Providers::update_many()
        .col_expr(Column::Name, Expr::value("Local catalog edit"))
        .filter(Column::Id.eq("openai"))
        .exec(&db)
        .await
        .unwrap();
    let transaction = db.begin().await.unwrap();
    assert!(
        migration
            .down(&SchemaManager::new(&transaction))
            .await
            .is_err()
    );
    transaction.rollback().await.unwrap();
    assert_eq!(
        Providers::find_by_id("openai")
            .one(&db)
            .await
            .unwrap()
            .unwrap()
            .name,
        "Local catalog edit"
    );
    Providers::update_many()
        .col_expr(Column::Name, Expr::value(openai.name.clone()))
        .filter(Column::Id.eq("openai"))
        .exec(&db)
        .await
        .unwrap();

    let transaction = db.begin().await.unwrap();
    migration
        .down(&SchemaManager::new(&transaction))
        .await
        .unwrap();
    transaction.commit().await.unwrap();
    use sea_orm::ConnectionTrait;
    for (table, count) in [
        ("provider_templates", 143),
        ("provider_model_templates", 1273),
    ] {
        let query = Query::select()
            .column(Alias::new("id"))
            .from(Alias::new(table))
            .to_owned();
        assert_eq!(db.query_all(&query).await.unwrap().len(), count);
    }
    let transaction = db.begin().await.unwrap();
    migration
        .up(&SchemaManager::new(&transaction))
        .await
        .unwrap();
    transaction.commit().await.unwrap();
    assert_eq!(
        Providers::find()
            .order_by_asc(Column::Id)
            .all(&db)
            .await
            .unwrap(),
        providers
    );
    let transaction = db.begin().await.unwrap();
    migration
        .down(&SchemaManager::new(&transaction))
        .await
        .unwrap();
    transaction.commit().await.unwrap();
    let manager = SchemaManager::new(&db);
    for table in ["provider_model_templates", "provider_templates"] {
        manager
            .drop_table(
                sea_orm_migration::prelude::Table::drop()
                    .table(Alias::new(table))
                    .to_owned(),
            )
            .await
            .unwrap();
    }
}

#[test]
fn catalog_sources_accept_plain_reference_names() {
    use compallm_postgres::catalog::model_info::ModelInfoSources;

    for name in ["opensource(l)", "opensource(9)"] {
        let reference = serde_json::json!([{
            "kind": "repository",
            "url": name,
            "access": "public",
            "status": "reference",
            "fields": ["description"],
            "revision": "catalog-revision",
            "notes": "Catalog source reference"
        }]);
        assert!(ModelInfoSources::try_from(reference.clone()).is_ok());
        let mut api = reference;
        api[0]["kind"] = serde_json::json!("api");
        api[0]["request"] = serde_json::json!({"method": "GET", "authentication": "none"});
        assert!(ModelInfoSources::try_from(api).is_err());
    }
}
