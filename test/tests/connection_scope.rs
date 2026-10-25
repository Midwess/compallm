use compallm_core::persistent::{ConnectionError, DbContext};
use compallm_postgres::postgres_connection::PostgresConnection;
use sea_orm::{DbBackend, MockDatabase};
use std::sync::Arc;

struct Fixture;

impl Fixture {
    fn connection() -> DbContext {
        DbContext::new(Arc::new(PostgresConnection::new(Arc::new(
            MockDatabase::new(DbBackend::Postgres).into_connection(),
        ))))
    }
}

#[tokio::test]
async fn scope_shares_context_and_restores_outer_after_transaction() {
    let connection = Fixture::connection();
    assert!(matches!(
        DbContext::current(),
        Err(ConnectionError::Database(_))
    ));
    connection
        .scope(async {
            assert!(DbContext::current().unwrap().same_context(&connection));
            connection
                .transaction(async {
                    let transaction = DbContext::current()?;
                    assert!(!transaction.same_context(&connection));
                    connection
                        .scope(async {
                            assert!(DbContext::current().unwrap().same_context(&transaction));
                        })
                        .await;
                    Ok::<_, ConnectionError>(())
                })
                .await
                .unwrap();
            assert!(DbContext::current().unwrap().same_context(&connection));
        })
        .await;
    assert!(matches!(
        DbContext::current(),
        Err(ConnectionError::Database(_))
    ));
}

#[tokio::test]
async fn concurrent_scopes_are_isolated_and_spawn_does_not_inherit() {
    let one = Fixture::connection();
    let two = Fixture::connection();
    tokio::join!(
        one.scope(async {
            tokio::task::yield_now().await;
            assert!(DbContext::current().unwrap().same_context(&one));
            assert!(
                tokio::spawn(async {
                    matches!(DbContext::current(), Err(ConnectionError::Database(_)))
                })
                .await
                .unwrap()
            );
        }),
        two.scope(async {
            tokio::task::yield_now().await;
            assert!(DbContext::current().unwrap().same_context(&two));
        })
    );
}

#[tokio::test(flavor = "current_thread")]
async fn concurrent_transactions_stay_isolated_on_the_same_thread() {
    let one = Fixture::connection();
    let two = Fixture::connection();
    let (first, second) = tokio::join!(
        one.transaction(async {
            let transaction = DbContext::current()?;
            tokio::task::yield_now().await;
            assert!(DbContext::current()?.same_context(&transaction));
            one.transaction(async {
                assert!(DbContext::current()?.same_context(&transaction));
                Ok::<_, ConnectionError>(())
            })
            .await
        }),
        two.transaction(async {
            let transaction = DbContext::current()?;
            tokio::task::yield_now().await;
            assert!(DbContext::current()?.same_context(&transaction));
            two.transaction(async {
                Err::<(), _>(ConnectionError::database(anyhow::anyhow!(
                    "independent transaction failed"
                )))
            })
            .await
        }),
    );
    assert!(first.is_ok());
    assert!(second.is_err());
    assert!(DbContext::current().is_err());
}

#[tokio::test]
async fn transaction_can_borrow_and_return_local_data_without_move() {
    let connection = Fixture::connection();
    let email = String::from("james@midwess.com");
    let borrowed = compallm_core::transaction!(connection, {
        tokio::task::yield_now().await;
        assert!(!DbContext::current()?.same_context(&connection));
        Ok::<_, ConnectionError>(email.as_str())
    })
    .unwrap();
    assert_eq!(borrowed, email.as_str());
}

#[tokio::test]
async fn completed_transaction_context_cannot_query_or_start_another_transaction() {
    let connection = Fixture::connection();
    let escaped = connection
        .transaction(async { DbContext::current() })
        .await
        .unwrap();
    assert!(PostgresConnection::from_context(&escaped).is_err());
    escaped
        .scope(async {
            assert!(matches!(
                DbContext::current(),
                Err(ConnectionError::Database(_))
            ));
            assert!(matches!(
                connection
                    .transaction(async { Ok::<_, ConnectionError>(()) })
                    .await,
                Err(ConnectionError::Database(_))
            ));
        })
        .await;
}
