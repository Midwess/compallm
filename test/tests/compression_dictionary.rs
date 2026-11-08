use compallm_core::compression::{DCompression, DictionaryType};
use compallm_core::persistent::DbContext;
use compallm_postgres::{
    postgres_connection::PostgresConnection, repository::dictionary::DictionaryRepository,
};
use sea_orm::{DbBackend, MockDatabase, Value};
use std::{collections::BTreeMap, sync::Arc};

fn sample_rows(version: i32) -> Vec<BTreeMap<String, Value>> {
    (0..100)
        .map(|index| BTreeMap::from([
            ("type".to_owned(), Value::from("access_token")),
            ("version".to_owned(), Value::from(version)),
            ("sample_index".to_owned(), Value::from(index)),
            ("content".to_owned(), Value::from(format!("{{\"u\":{{\"s\":{{\"i\":{},\"u\":{},\"o\":{version}}},\"p\":{{\"c\":\"permit(principal, action == Compallm::Action::model.list, resource);\",\"r\":[{}]}}}}}}", index * 17 + 1, index * 31 + 2, index))),
        ]))
        .collect()
}

#[tokio::test]
async fn dictionary_roundtrip_preserves_cedar_and_rejects_different_versions() -> anyhow::Result<()>
{
    let original = DCompression::new(
        DictionaryType::AccessToken,
        1,
        "Compallm::Action::provider.read",
    )?;
    let extended = DCompression::new(
        DictionaryType::AccessToken,
        2,
        "Compallm::Action::provider.read Compallm::Role::",
    )?;
    let cedar =
        "permit(principal, action == Compallm::Action::provider.read, resource); // tiếng Việt";
    let encoded = original.compress(cedar).await?;
    assert_eq!(original.decompress(&encoded).await?, cedar);
    assert!(extended.decompress(&encoded).await.is_err());
    assert_eq!(
        extended
            .decompress(&extended.compress(cedar).await?)
            .await?,
        cedar
    );
    Ok(())
}

#[tokio::test]
async fn repository_loads_ordered_version_once_and_preserves_old_dictionary() -> anyhow::Result<()>
{
    let first = sample_rows(1);
    let mut second = first.clone();
    second.extend(sample_rows(2));
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([
                second.clone(),
                first,
                second,
            ])
            .into_connection(),
    );
    let context = DbContext::new(Arc::new(PostgresConnection::new(db.clone())));
    let repository = DictionaryRepository::default();
    context
        .scope(async {
            let (a, b) = tokio::join!(
                repository.get(DictionaryType::AccessToken, 2),
                repository.get(DictionaryType::AccessToken, 2)
            );
            let a = a?.unwrap();
            let b = b?.unwrap();
            assert!(std::ptr::eq(a, b));
            let value = "Compallm::Action::provider.read";
            assert_eq!(a.decompress(&a.compress(value).await?).await?, value);
            let old = repository
                .get(DictionaryType::AccessToken, 1)
                .await?
                .unwrap();
            assert!(old.decompress(&a.compress(value).await?).await.is_err());
            assert!(
                repository
                    .get(DictionaryType::AccessToken, 3)
                    .await?
                    .is_none()
            );
            assert!(std::ptr::eq(
                a,
                repository
                    .get(DictionaryType::AccessToken, 2)
                    .await?
                    .unwrap()
            ));
            Ok::<_, anyhow::Error>(())
        })
        .await?;
    drop(context);
    let log = Arc::try_unwrap(db).unwrap().into_transaction_log();
    assert_eq!(
        log.iter()
            .map(|transaction| transaction.statements().len())
            .sum::<usize>(),
        3
    );
    Ok(())
}

#[tokio::test]
async fn repository_rejects_transactional_reads_without_poisoning_the_cache() -> anyhow::Result<()>
{
    let rows = sample_rows(1);
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([rows])
            .into_connection(),
    );
    let context = DbContext::new(Arc::new(PostgresConnection::new(db)));
    let repository = DictionaryRepository::default();
    let attempted = context
        .transaction(async {
            repository.get(DictionaryType::AccessToken, 1).await?;
            Ok::<_, anyhow::Error>(())
        })
        .await;
    assert!(attempted.is_err());
    let dictionary = context
        .scope(repository.get(DictionaryType::AccessToken, 1))
        .await?
        .unwrap();
    let source = "permit(principal, action, resource);";
    assert_eq!(
        dictionary
            .decompress(&dictionary.compress(source).await?)
            .await?,
        source
    );
    Ok(())
}

#[tokio::test]
async fn configured_repository_reads_committed_dictionary_during_transaction() -> anyhow::Result<()>
{
    let rows = sample_rows(1);
    let committed = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([vec![rows[0].clone()], rows])
            .into_connection(),
    );
    let writer = Arc::new(MockDatabase::new(DbBackend::Postgres).into_connection());
    let committed_context = DbContext::new(Arc::new(PostgresConnection::new(committed.clone())));
    let writer_context = DbContext::new(Arc::new(PostgresConnection::new(writer.clone())));
    let repository = DictionaryRepository::new(committed_context)?;
    writer_context
        .transaction(async {
            let dictionary = repository
                .latest(DictionaryType::AccessToken)
                .await?
                .unwrap();
            assert_eq!(dictionary.version, 1);
            assert_eq!(
                dictionary
                    .decompress(&dictionary.compress("hello").await?)
                    .await?,
                "hello"
            );
            Ok::<_, anyhow::Error>(())
        })
        .await?;
    drop(repository);
    drop(writer_context);
    let writer_log = Arc::try_unwrap(writer).unwrap().into_transaction_log();
    let committed_log = Arc::try_unwrap(committed).unwrap().into_transaction_log();
    assert!(
        writer_log
            .iter()
            .flat_map(|tx| tx.statements())
            .all(|statement| !statement.sql.contains("zstd_dictionary"))
    );
    assert_eq!(committed_log.len(), 2);
    Ok(())
}

#[tokio::test]
async fn repository_latest_tracks_new_versions_and_reuses_cached_prefixes() -> anyhow::Result<()> {
    let first = sample_rows(1);
    let mut third = first.clone();
    third.extend(sample_rows(3));
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([
                vec![first[0].clone()],
                first.clone(),
                vec![first[0].clone()],
                vec![third.last().unwrap().clone()],
                third,
                first,
                vec![],
            ])
            .into_connection(),
    );
    let context = DbContext::new(Arc::new(PostgresConnection::new(db)));
    let repository = DictionaryRepository::default();
    context
        .scope(async {
            let first = repository
                .latest(DictionaryType::AccessToken)
                .await?
                .unwrap();
            assert_eq!(first.version, 1);
            let again = repository
                .latest(DictionaryType::AccessToken)
                .await?
                .unwrap();
            assert!(std::ptr::eq(first, again));
            let latest = repository
                .latest(DictionaryType::AccessToken)
                .await?
                .unwrap();
            assert_eq!(latest.version, 3);
            assert!(std::ptr::eq(
                first,
                repository
                    .get(DictionaryType::AccessToken, 1)
                    .await?
                    .unwrap()
            ));
            assert!(
                repository
                    .get(DictionaryType::AccessToken, 2)
                    .await?
                    .is_none()
            );
            assert!(
                repository
                    .latest(DictionaryType::AccessToken)
                    .await?
                    .is_none()
            );
            Ok::<_, anyhow::Error>(())
        })
        .await
}

#[tokio::test]
async fn initial_seed_compresses_complete_default_access_claims() -> anyhow::Result<()> {
    use compallm_core::{
        auth::{AccessToken, Session, UserAccessToken},
        authorization::PolicySnapshot,
        organization::{OrganizationMember, Role},
    };
    use sea_orm_migration::{MigratorTrait, SchemaManager};
    let migration = compallm_postgres::Migrator::migrations()
        .into_iter()
        .find(|migration| migration.name().ends_with("seed_access_token_dictionary"))
        .unwrap();
    let db = MockDatabase::new(DbBackend::Postgres)
        .append_query_results([vec![BTreeMap::from([(
            "version".to_owned(),
            Value::from(1_i32),
        )])]])
        .into_connection();
    migration.up(&SchemaManager::new(&db)).await?;
    let log = db.into_transaction_log();
    let statement = &log[0].statements()[0];
    let samples = statement.values.as_ref().unwrap().0.iter().map(|value| match value {
        Value::String(Some(content)) => Ok(content.to_string()),
        _ => Err(anyhow::anyhow!("expected dictionary sample")),
    }).collect::<anyhow::Result<Vec<_>>>()?;
    let dictionary = DCompression::train(DictionaryType::AccessToken, 1, samples).await?;
    compallm_postgres::init_id_generator("dictionary-seed-test")?;
    let roles = Role::defaults(84).await?;
    for role in roles {
        let member = OrganizationMember {
            organization_id: 84,
            user_id: 42,
            roles: vec![role],
            groups: vec![],
        };
        let session = Session::new(42, 84, "Research team").await?;
        let token = AccessToken::User(UserAccessToken::new(
            session,
            PolicySnapshot::new(&member),
            std::time::Duration::from_secs(60),
        )?);
        let source = serde_json::to_string(&token)?;
        let compressed = dictionary.compress(&source).await?;
        assert_eq!(dictionary.decompress(&compressed).await?, source);
        assert!(compressed.len() < source.len());
    }
    Ok(())
}
