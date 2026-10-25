use compallm_core::persistent::user::UserRepositoryError;
use compallm_core::persistent::{ConnectionError, DbContext};
use compallm_postgres::postgres_connection::PostgresConnection;
use sea_orm::{ConnectionTrait, DbBackend, MockDatabase, Statement, Transaction};
use std::sync::Arc;

#[tokio::test]
async fn nested_transaction_reuses_outer_context_without_savepoints() {
    let db = Arc::new(MockDatabase::new(DbBackend::Postgres).into_connection());
    let connection = DbContext::new(Arc::new(PostgresConnection::new(db.clone())));
    connection
        .transaction(async {
            let outer = DbContext::current()?;
            connection
                .transaction(async {
                    assert!(DbContext::current()?.same_context(&outer));
                    Ok::<_, ConnectionError>(())
                })
                .await?;
            assert!(DbContext::current()?.same_context(&outer));
            Ok::<_, ConnectionError>(())
        })
        .await
        .unwrap();
    drop(connection);
    assert_eq!(
        Arc::try_unwrap(db).unwrap().into_transaction_log(),
        [Transaction::many([
            Statement::from_string(DbBackend::Postgres, "BEGIN"),
            Statement::from_string(DbBackend::Postgres, "COMMIT"),
        ])]
    );
}

#[tokio::test]
async fn caught_nested_error_still_rolls_back_the_outer_transaction() {
    let db = Arc::new(MockDatabase::new(DbBackend::Postgres).into_connection());
    let connection = DbContext::new(Arc::new(PostgresConnection::new(db.clone())));
    let result = connection
        .transaction(async {
            let nested = connection
                .transaction(async {
                    Err::<(), _>(ConnectionError::database(anyhow::anyhow!(
                        "nested operation failed"
                    )))
                })
                .await;
            assert!(nested.is_err());
            // Swallowing an inner error must never commit partial work.
            Ok::<_, ConnectionError>(())
        })
        .await;
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("database operation failed")
    );
    drop(connection);
    assert_eq!(
        Arc::try_unwrap(db).unwrap().into_transaction_log(),
        [Transaction::many([
            Statement::from_string(DbBackend::Postgres, "BEGIN"),
            Statement::from_string(DbBackend::Postgres, "ROLLBACK"),
        ])]
    );
}

#[tokio::test]
async fn cancelled_nested_operation_prevents_outer_commit() {
    let db = Arc::new(MockDatabase::new(DbBackend::Postgres).into_connection());
    let connection = DbContext::new(Arc::new(PostgresConnection::new(db.clone())));
    let result = connection
        .transaction(async {
            {
                let nested =
                    connection.transaction(std::future::pending::<Result<(), ConnectionError>>());
                tokio::pin!(nested);
                tokio::select! { biased;
                    _ = &mut nested => panic!("nested future must remain pending"),
                    _ = std::future::ready(()) => {}
                }
            }
            Ok::<_, ConnectionError>(())
        })
        .await;
    assert!(result.is_err());
    drop(connection);
    assert_eq!(
        Arc::try_unwrap(db).unwrap().into_transaction_log(),
        [Transaction::many([
            Statement::from_string(DbBackend::Postgres, "BEGIN"),
            Statement::from_string(DbBackend::Postgres, "ROLLBACK"),
        ])]
    );
}

#[tokio::test]
async fn unfinished_nested_future_cannot_escape_a_successful_commit() {
    let db = Arc::new(MockDatabase::new(DbBackend::Postgres).into_connection());
    let connection = DbContext::new(Arc::new(PostgresConnection::new(db.clone())));
    let mut unfinished = None;
    let result = connection
        .transaction(async {
            let mut nested = Box::pin(
                connection.transaction(std::future::pending::<Result<(), ConnectionError>>()),
            );
            tokio::select! { biased;
                _ = &mut nested => panic!("nested future must remain pending"),
                _ = std::future::ready(()) => {}
            }
            unfinished = Some(nested);
            Ok::<_, ConnectionError>(())
        })
        .await;
    assert!(result.is_err());
    drop(unfinished);
    drop(connection);
    assert_eq!(
        Arc::try_unwrap(db).unwrap().into_transaction_log(),
        [Transaction::many([
            Statement::from_string(DbBackend::Postgres, "BEGIN"),
            Statement::from_string(DbBackend::Postgres, "ROLLBACK"),
        ])]
    );
}

#[tokio::test]
async fn cancellation_rolls_back_and_closes_escaped_context() {
    let db = Arc::new(MockDatabase::new(DbBackend::Postgres).into_connection());
    let connection = DbContext::new(Arc::new(PostgresConnection::new(db.clone())));
    let mut escaped = None;
    {
        let transaction = connection.transaction(async {
            escaped = Some(DbContext::current()?);
            std::future::pending::<Result<(), ConnectionError>>().await
        });
        tokio::pin!(transaction);
        tokio::select! { biased;
            _ = &mut transaction => panic!("expected pending"),
            _ = std::future::ready(()) => {}
        }
    }
    assert!(PostgresConnection::from_context(&escaped.unwrap()).is_err());
    drop(connection);
    assert_eq!(
        Arc::try_unwrap(db).unwrap().into_transaction_log(),
        [Transaction::many([
            Statement::from_string(DbBackend::Postgres, "BEGIN"),
            Statement::from_string(DbBackend::Postgres, "ROLLBACK"),
        ])]
    );
}

#[tokio::test]
async fn repositories_execute_queries_on_the_same_transaction() {
    let db = Arc::new(
        MockDatabase::new(DbBackend::Postgres)
            .append_query_results([Vec::<sea_orm::MockRow>::new(), Vec::new()])
            .into_connection(),
    );
    let connection = DbContext::new(Arc::new(PostgresConnection::new(db.clone())));
    connection
        .transaction(async {
            PostgresConnection::from_context(&DbContext::current()?)?
                .query_all_raw(Statement::from_string(DbBackend::Postgres, "SELECT 1"))
                .await
                .map_err(ConnectionError::database)?;
            PostgresConnection::from_context(&DbContext::current()?)?
                .query_all_raw(Statement::from_string(DbBackend::Postgres, "SELECT 2"))
                .await
                .map_err(ConnectionError::database)?;
            Err::<(), _>(UserRepositoryError::EmailAlreadyExists)
        })
        .await
        .unwrap_err();
    drop(connection);
    assert_eq!(
        Arc::try_unwrap(db).unwrap().into_transaction_log(),
        [Transaction::many([
            Statement::from_string(DbBackend::Postgres, "BEGIN"),
            Statement::from_string(DbBackend::Postgres, "SELECT 1"),
            Statement::from_string(DbBackend::Postgres, "SELECT 2"),
            Statement::from_string(DbBackend::Postgres, "ROLLBACK"),
        ])]
    );
}
