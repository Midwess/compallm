use compallm_core::{
    authentication::{Session, UserAccessToken},
    authorization::{MemberPermission, PolicySnapshot},
    persistent::{DbContext, organization::OrganizationRepository as _},
    provider::Protocol,
};
use compallm_postgres::{
    catalog::repository::{CatalogRepository, CatalogScope},
    postgres_connection::PostgresConnection,
};
use sea_orm::{Database, EntityTrait, ModelTrait};
use std::{sync::Arc, time::Duration};

#[tokio::test]
#[ignore = "requires PERSONAL_CATALOG_TEST_URL with all migrations applied to an isolated database"]
async fn private_templates_layer_reset_and_preserve_independent_commits() -> anyhow::Result<()> {
    let pool = Arc::new(Database::connect(std::env::var("PERSONAL_CATALOG_TEST_URL")?).await?);
    let root = DbContext::new(Arc::new(PostgresConnection::new(pool.clone())));
    type Members = <compallm_postgres::OrganizationMemberSchema as ModelTrait>::Entity;
    let member = Members::find()
        .one(pool.as_ref())
        .await?
        .ok_or_else(|| anyhow::anyhow!("missing membership"))?;
    let repository = compallm_postgres::repository::organization::OrganizationRepository;
    let (member, _) = root
        .scope(repository.find_membership(member.organization_id as u64, member.user_id as u64))
        .await?
        .ok_or_else(|| anyhow::anyhow!("missing active membership"))?;
    let actor = UserAccessToken::new(
        Session {
            id: 1,
            user_id: member.user_id,
            organization_id: member.organization_id,
            authorization_version: 1,
            expires_at: compallm_utils::time::unix_timestamp() + 3600,
            last_seen_at: compallm_utils::time::unix_timestamp(),
            revoked_at: None,
            metadata: Default::default(),
        },
        PolicySnapshot::new(&member)?,
        Duration::from_secs(3600),
    )?;
    let scope = CatalogScope {
        organization_id: member.organization_id,
        user_id: member.user_id,
    };
    let permission = Arc::new(MemberPermission::try_from(actor)?);
    let personal = root
        .fresh()?
        .restrict(async move { Ok::<_, std::convert::Infallible>(permission) })?;
    let catalog = CatalogRepository;
    let original = root
        .scope(async { Ok::<_, anyhow::Error>(catalog.model("openai", "gpt-4o", None).await?) })
        .await?
        .ok_or_else(|| anyhow::anyhow!("missing ready fixture"))?;
    let base = root
        .scope(async { Ok::<_, anyhow::Error>(catalog.global_row("openai", "gpt-4o").await?) })
        .await?
        .unwrap()
        .row_id;
    let mut named = original.clone();
    named.name = "private-name".into();
    personal
        .scope(async {
            Ok::<_, anyhow::Error>(
                catalog
                    .save_personal(named, scope, Some(base), &["name".into()])
                    .await?,
            )
        })
        .await?;
    assert_eq!(
        personal
            .scope(async {
                Ok::<_, anyhow::Error>(catalog.model("openai", "gpt-4o", Some(scope)).await?)
            })
            .await?
            .unwrap()
            .name,
        "private-name"
    );
    assert_eq!(
        root.scope(async {
            Ok::<_, anyhow::Error>(catalog.model("openai", "gpt-4o", None).await?)
        })
        .await?
        .unwrap()
        .name,
        original.name
    );
    personal
        .scope(async {
            Ok::<_, anyhow::Error>(
                catalog
                    .list(
                        Box::new(compallm_query::Query::default()),
                        compallm_query::Pagination {
                            first: Some(20),
                            ..Default::default()
                        },
                        vec![compallm_query::OrderBy {
                            field: "name".into(),
                            direction: compallm_query::OrderByDirection::AscNullsLast,
                        }],
                        "private-catalog-check",
                        Some("private-name"),
                        Some(scope),
                    )
                    .await?,
            )
        })
        .await?;
    let mut stale = original.clone();
    stale.headers.insert("x-test".into(), "value".into());
    personal
        .scope(async {
            Ok::<_, anyhow::Error>(
                catalog
                    .save_personal(stale, scope, Some(base), &["headers".into()])
                    .await?,
            )
        })
        .await?;
    let value = personal
        .scope(async {
            Ok::<_, anyhow::Error>(catalog.model("openai", "gpt-4o", Some(scope)).await?)
        })
        .await?
        .unwrap();
    assert_eq!(value.name, "private-name");
    assert_eq!(value.headers["x-test"], "value");
    let mut limits = value.clone();
    compallm_core::model::CatalogCustomization {
        limits: Some((Some(256000), Some(200000), Some(64000))),
        ..Default::default()
    }
    .apply(&mut limits, Some(&original))?;
    personal
        .scope(async {
            Ok::<_, anyhow::Error>(
                catalog
                    .save_personal(limits, scope, Some(base), &["limits".into()])
                    .await?,
            )
        })
        .await?;
    let value = personal
        .scope(async {
            Ok::<_, anyhow::Error>(catalog.model("openai", "gpt-4o", Some(scope)).await?)
        })
        .await?
        .unwrap();
    assert_eq!(value.limits.context_tokens, Some(256000));
    assert_eq!(
        value.text.input.configuration().unwrap().max_tokens,
        Some(200000)
    );
    let mut priced = value.clone();
    compallm_core::model::CatalogCustomization {
        input_price: Some(Some("1.25".parse()?)),
        cached_price: Some(Some("0.25".parse()?)),
        ..Default::default()
    }
    .apply(&mut priced, Some(&original))?;
    let priced = personal
        .scope(async {
            Ok::<_, anyhow::Error>(
                catalog
                    .save_personal(
                        priced,
                        scope,
                        Some(base),
                        &["input_price".into(), "cached_price".into()],
                    )
                    .await?,
            )
        })
        .await?;
    let encoded = serde_json::to_value(&priced)?;
    assert_eq!(
        encoded
            .pointer("/text/input/pricing/configuration/fixed/token/rate/amount")
            .unwrap(),
        "1.25"
    );
    assert_eq!(
        encoded
            .pointer(
                "/text/input/pricing/configuration/fixed/token/cache/configuration/rate/amount"
            )
            .unwrap(),
        "0.25"
    );
    let configured = compallm_core::provider::ProviderModel::try_from((&priced, Protocol::Openai))?;
    personal
        .scope(async {
            Ok::<_, anyhow::Error>(
                catalog
                    .delete_personal("openai", "gpt-4o", scope, false)
                    .await?,
            )
        })
        .await?;
    assert_eq!(
        personal
            .scope(async {
                Ok::<_, anyhow::Error>(catalog.model("openai", "gpt-4o", Some(scope)).await?)
            })
            .await?
            .unwrap(),
        original
    );
    assert_eq!(configured.name, "private-name");
    Ok(())
}

#[path = "support/graphql.rs"]
mod graphql_support;

#[tokio::test]
#[ignore = "requires PERSONAL_CATALOG_TEST_URL with all migrations applied to an isolated database"]
async fn provider_graphql_creates_only_selected_models_and_keeps_personal_defaults_private()
-> anyhow::Result<()> {
    use compallm_core::{
        authentication::AccessToken, compression::DictionaryType,
        persistent::session::SessionRepository as _,
    };
    use sea_orm::{ColumnTrait, QueryFilter};
    let pool = Arc::new(Database::connect(std::env::var("PERSONAL_CATALOG_TEST_URL")?).await?);
    let root = DbContext::new(Arc::new(PostgresConnection::new(pool.clone())));
    DbContext::initialize_default(root.postgres().scoped())?;
    type Roles = <compallm_postgres::OrganizationRoleSchema as ModelTrait>::Entity;
    type RC = <Roles as EntityTrait>::Column;
    type Members = <compallm_postgres::OrganizationMemberSchema as ModelTrait>::Entity;
    type MC = <Members as EntityTrait>::Column;
    let role = Roles::find()
        .filter(RC::OrgOwner.eq(true))
        .one(pool.as_ref())
        .await?
        .unwrap();
    let identity = Members::find()
        .filter(MC::OrganizationId.eq(role.organization_id))
        .filter(MC::RoleId.eq(role.id))
        .one(pool.as_ref())
        .await?
        .unwrap();
    let organizations = compallm_postgres::repository::organization::OrganizationRepository;
    let (member, organization) = root
        .scope(
            organizations.find_membership(identity.organization_id as u64, identity.user_id as u64),
        )
        .await?
        .unwrap();
    compallm_postgres::init_id_generator("provider-graphql-integration")?;
    let mut session = Session::new(member.user_id, member.organization_id).await?;
    session.expires_at = compallm_utils::time::unix_timestamp() + 86400;
    root.scope(compallm_postgres::repository::session::SessionRepository.create(session.clone()))
        .await?;
    let dictionary =
        compallm_postgres::repository::dictionary::DictionaryRepository::new(root.clone())?;
    let dictionary = dictionary
        .latest(DictionaryType::AccessToken)
        .await?
        .unwrap();
    let test_name = format!("Provider GraphQL integration {}", session.id);
    let secret = "provider-flow-test-signing-secret-at-least-32-bytes";
    let token = AccessToken::User(UserAccessToken::new(
        session,
        PolicySnapshot::new(&member)?,
        Duration::from_secs(3600),
    )?)
    .compress(dictionary)
    .await?
    .encode(secret, Duration::from_secs(3600))?;
    let di = Box::leak(Box::new(compallm_api::di::Di::new(
        root,
        compallm_api::di::AuthSettings {
            jwt_secret: secret.into(),
            user_token_ttl_seconds: 3600,
            refresh_token_ttl_seconds: 86400,
        },
        compallm_api::di::GraphqlSettings {
            cookie_secure: false,
            public_origin: "http://localhost".into(),
            ..Default::default()
        },
        None,
    )));
    di.initialize_services()?;
    let router = graphql_support::browser_router(di);
    let execute = async |query: &str| graphql_support::execute(&router, &token, query).await;
    let global = execute(
        r#"{modelCatalog(providerCatalogId:"openai",id:"gpt-4o"){node{name origin inputPrice}}}"#,
    )
    .await?;
    assert!(global["errors"].is_null(), "{global}");
    let changed=execute(r#"mutation{modelCatalogCustomize(providerCatalogId:"openai",id:"gpt-4o",input:{name:"private-upstream"}){node{name origin}}}"#).await?;
    assert_eq!(
        changed["data"]["modelCatalogCustomize"]["node"]["origin"], "OVERRIDE",
        "{changed}"
    );
    let public =
        execute(r#"{modelCatalog(providerCatalogId:"openai",id:"gpt-4o"){node{name}}}"#).await?;
    assert_eq!(
        public["data"]["modelCatalog"]["node"]["name"],
        global["data"]["modelCatalog"]["node"]["name"]
    );
    let search=execute(r#"{effectiveProviderCatalogs(search:"private-upstream",first:20){edges{node{id}} pageInfo{hasNextPage}}}"#).await?;
    assert_eq!(
        search["data"]["effectiveProviderCatalogs"]["edges"][0]["node"]["id"], "openai",
        "{search}"
    );
    for term in [
        "openai",
        "OPENAI",
        " openai ",
        "gpt-4o",
        "openai gpt-4o",
        "%",
        "_",
    ] {
        let searched = execute(&format!(
            "{{effectiveProviderCatalogs(search:{},first:100){{edges{{node{{id}}}}}}}}",
            serde_json::to_string(term)?
        ))
        .await?;
        assert!(searched["errors"].is_null(), "{term}: {searched}");
        let ids = searched["data"]["effectiveProviderCatalogs"]["edges"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v["node"]["id"].as_str().unwrap())
            .collect::<Vec<_>>();
        if term.trim().eq_ignore_ascii_case("openai") || term.contains("gpt-4o") {
            assert!(ids.contains(&"openai"), "{term}: {ids:?}");
        }
        if term == "%" {
            assert!(
                ids.is_empty(),
                "literal wildcard unexpectedly matched {ids:?}"
            );
        }
    }
    let added=execute(r#"mutation{modelCatalogCreate(providerCatalogId:"openai",input:{configuration:{name:"custom-upstream-text",protocol:OPENAI_CHAT_COMPLETIONS,path:"/chat/completions",limits:{context:128000},inputPrice:{enabled:true,value:"0.1"},outputPrice:{enabled:true,value:"0.2"},capabilities:[TEXT]}}){node{id origin inputPrice}}}"#).await?;
    assert_eq!(
        added["data"]["modelCatalogCreate"]["node"]["origin"], "CUSTOM",
        "{added}"
    );
    let custom_id = added["data"]["modelCatalogCreate"]["node"]["id"]
        .as_str()
        .unwrap();
    let deleted = execute(&format!(
        "mutation{{modelCatalogDelete(providerCatalogId:\"openai\",id:\"{custom_id}\"){{node}}}}"
    ))
    .await?;
    assert_eq!(
        deleted["data"]["modelCatalogDelete"]["node"], true,
        "{deleted}"
    );
    let create=execute(r#"mutation{providerCreate(input:{name:"Provider GraphQL integration",providerCatalogId:"openai",baseUrl:"https://api.openai.com/v1",apiKey:"test-only-provider-key",headers:[],models:[{id:"gpt-4o",protocol:OPENAI_CHAT_COMPLETIONS}]}){node{id name models{id name enabled}}}}"#.replace("Provider GraphQL integration",&test_name).as_str()).await?;
    assert!(create["errors"].is_null(), "{create}");
    let provider = &create["data"]["providerCreate"]["node"];
    assert_eq!(provider["models"].as_array().unwrap().len(), 1);
    assert_eq!(provider["models"][0]["name"], "private-upstream");
    assert!(
        execute(
            r#"mutation{modelCatalogReset(providerCatalogId:"openai",id:"gpt-4o"){node{origin}}}"#
        )
        .await?["errors"]
            .is_null()
    );
    let detail = execute(&format!(
        "{{provider(id:\"{}\"){{node{{models{{name}}}}}}}}",
        provider["id"].as_str().unwrap()
    ))
    .await?;
    assert_eq!(
        detail["data"]["provider"]["node"]["models"][0]["name"],
        "private-upstream"
    );
    if let Ok(path) = std::env::var("PROVIDER_BROWSER_FIXTURE") {
        std::fs::write(
            path,
            serde_json::to_vec(
                &serde_json::json!({"token":token,"organization":organization.name,"provider":provider["id"],"userId":member.user_id.to_string(),"organizationId":member.organization_id.to_string()}),
            )?,
        )?;
    }
    Ok(())
}

#[tokio::test]
#[ignore = "requires PERSONAL_CATALOG_TEST_URL with all migrations applied to an isolated database"]
async fn relational_catalog_migration_preserves_catalog_and_readiness_through_rollback()
-> anyhow::Result<()> {
    use sea_orm::{QueryOrder, TransactionTrait};
    use sea_orm_migration::{MigratorTrait, SchemaManager};
    let db = Database::connect(std::env::var("PERSONAL_CATALOG_TEST_URL")?).await?;
    let tx = db.begin().await?;
    type Models = <compallm_postgres::CatalogModelRecordSchema as ModelTrait>::Entity;
    type MC = <Models as EntityTrait>::Column;
    let before = Models::find().order_by_asc(MC::RowId).all(&tx).await?;
    let migration = compallm_postgres::Migrator::migrations()
        .into_iter()
        .find(|m| m.name().ends_with("catalog_relational"))
        .unwrap();
    migration.down(&SchemaManager::new(&tx)).await?;
    migration.up(&SchemaManager::new(&tx)).await?;
    assert_eq!(
        Models::find().order_by_asc(MC::RowId).all(&tx).await?,
        before
    );
    tx.rollback().await?;
    Ok(())
}

#[tokio::test]
#[ignore = "requires PERSONAL_CATALOG_TEST_URL with all migrations applied to an isolated database"]
async fn provider_chooser_only_lists_providers_with_configurable_models() -> anyhow::Result<()> {
    let pool = Arc::new(Database::connect(std::env::var("PERSONAL_CATALOG_TEST_URL")?).await?);
    let root = DbContext::new(Arc::new(PostgresConnection::new(pool)));
    root.scope(async {
        let catalog = CatalogRepository;
        for search in [
            None,
            Some("openai"),
            Some("together"),
            Some("opus"),
            Some("alicode"),
        ] {
            let providers = catalog
                .list(
                    Box::new(compallm_query::Query::default()),
                    compallm_query::Pagination {
                        first: Some(100),
                        ..Default::default()
                    },
                    vec![],
                    "chooser-test",
                    search,
                    None,
                )
                .await?;
            if search.is_none() || search == Some("openai") || search == Some("opus") {
                assert!(!providers.edges.is_empty());
            }
            assert!(providers.edges.len() <= 100);
            for edge in providers.edges {
                let models = catalog
                    .models(
                        &edge.node.id,
                        Box::new(compallm_query::Query::default()),
                        None,
                    )
                    .await?;
                assert!(!models.is_empty(), "empty provider {}", edge.node.id);
            }
        }
        let configured = catalog
            .models("alicode", Box::new(compallm_query::Query::default()), None)
            .await?;
        assert!(!configured.is_empty());
        for model in configured {
            assert!(catalog.model("alicode", &model.id, None).await?.is_some());
        }
        assert!(
            catalog
                .models("bedrock", Box::new(compallm_query::Query::default()), None)
                .await?
                .iter()
                .all(|m| m.service_kind == Some(compallm_core::provider::ServiceKind::Llm))
        );
        assert!(
            catalog
                .models(
                    "ollama-local",
                    Box::new(compallm_query::Query::default()),
                    None
                )
                .await?
                .is_empty()
        );
        Ok::<_, anyhow::Error>(())
    })
    .await
}
