use compallm_core::persistent::{DbContext, PersistenceError};
use sea_orm::{ConnectionTrait, DbBackend, MockDatabase};
use std::sync::Arc;

struct Fixture;

impl Fixture {
    fn connection() -> DbContext {
        DbContext::new(Arc::new(
            MockDatabase::new(DbBackend::Postgres).into_connection(),
        ))
    }
}

#[tokio::test]
async fn scope_shares_context_and_restores_outer_after_transaction() {
    let connection = Fixture::connection();
    assert!(matches!(
        DbContext::current(),
        Err(PersistenceError::MissingConnection)
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
                    Ok::<_, PersistenceError>(())
                })
                .await
                .unwrap();
            assert!(DbContext::current().unwrap().same_context(&connection));
        })
        .await;
    assert!(matches!(
        DbContext::current(),
        Err(PersistenceError::MissingConnection)
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
                    matches!(
                        DbContext::current(),
                        Err(PersistenceError::MissingConnection)
                    )
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

#[tokio::test]
async fn transaction_can_borrow_and_return_local_data_without_move() {
    let connection = Fixture::connection();
    let email = String::from("james@midwess.com");
    let borrowed = connection
        .transaction(async {
            tokio::task::yield_now().await;
            assert!(!DbContext::current()?.same_context(&connection));
            Ok::<_, PersistenceError>(email.as_str())
        })
        .await
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
    assert!(
        escaped
            .postgres()
            .execute_unprepared("SELECT 1")
            .await
            .is_err()
    );
    escaped
        .scope(async {
            assert!(matches!(
                DbContext::current(),
                Err(PersistenceError::ClosedConnection)
            ));
            assert!(matches!(
                connection
                    .transaction(async { Ok::<_, PersistenceError>(()) })
                    .await,
                Err(PersistenceError::ClosedConnection)
            ));
        })
        .await;
}
