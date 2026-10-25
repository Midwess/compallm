//! Scoped database context. Operations run on the caller's task and may borrow
//! local data; they do not need `move`, `Send`, or a `'static` lifetime.

use sea_orm::{DatabaseConnection, TransactionTrait};
use std::{future::Future, sync::Arc};

use super::{PersistenceError, postgres_connection::PostgresConnection};

tokio::task_local! {
    static CURRENT: DbContext;
}

/// Shared context. Add other concrete database handles here when needed.
#[derive(Clone)]
pub struct DbContext {
    postgres: Arc<PostgresConnection>,
}

impl DbContext {
    /// Application dependency wiring supplies the already configured database.
    pub fn new(postgres: Arc<DatabaseConnection>) -> Self {
        Self {
            postgres: Arc::new(PostgresConnection::Database(postgres)),
        }
    }

    pub fn current() -> Result<Self, PersistenceError> {
        let context = CURRENT
            .try_with(Clone::clone)
            .map_err(|_| PersistenceError::MissingConnection)?;
        context.postgres.ensure_open()?;
        Ok(context)
    }

    pub fn postgres(&self) -> &PostgresConnection {
        &self.postgres
    }

    pub fn same_context(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.postgres, &other.postgres)
    }

    /// Enter an application context without replacing an active transaction.
    pub async fn scope<F: Future>(&self, operation: F) -> F::Output {
        let context = CURRENT
            .try_with(Clone::clone)
            .unwrap_or_else(|_| self.clone());
        CURRENT.scope(context, operation).await
    }

    /// Share one transaction across repositories; nested calls use savepoints.
    /// Commit on success and rollback on error. Dropping a cancelled future
    /// releases the transaction so SeaORM can roll it back.
    pub async fn transaction<F, T, E>(&self, operation: F) -> Result<T, E>
    where
        F: Future<Output = Result<T, E>>,
        E: From<PersistenceError> + std::fmt::Display,
    {
        let source = match Self::current() {
            Ok(context) => context,
            Err(PersistenceError::MissingConnection) => self.clone(),
            Err(error) => return Err(error.into()),
        };
        let transaction = match source.postgres() {
            PostgresConnection::Database(db) => db.begin().await,
            PostgresConnection::Transaction(state) => state.get()?.begin().await,
        }
        .map_err(PersistenceError::database)?;
        let context = Self {
            postgres: Arc::new(PostgresConnection::transaction(transaction)),
        };
        let guard = TransactionScope(context.clone());
        let result = CURRENT.scope(context.clone(), operation).await;
        let transaction = context.postgres().take_transaction()?;
        let result = match result {
            Ok(value) => {
                transaction
                    .commit()
                    .await
                    .map_err(PersistenceError::database)?;
                Ok(value)
            }
            Err(error) => match transaction.rollback().await {
                Ok(()) => Err(error),
                Err(rollback) => Err(PersistenceError::general(
                    format!("operation failed ({error}); rollback also failed"),
                    rollback.into(),
                )
                .into()),
            },
        };
        drop(guard);
        result
    }
}

struct TransactionScope(DbContext);

impl Drop for TransactionScope {
    fn drop(&mut self) {
        self.0.postgres().cancel();
    }
}
