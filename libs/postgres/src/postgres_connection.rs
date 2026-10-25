//! Concrete SeaORM connection or scoped transaction, usable by repository queries.

use sea_orm::{
    ConnectionTrait, DatabaseConnection, DatabaseTransaction, DbBackend, DbErr, ExecResult,
    QueryResult, Statement,
};
use std::sync::{Arc, Mutex};

use compallm_core::persistent::{Connection, DbContext, PersistenceError};
use std::any::Any;

/// SeaORM's `ConnectionTrait` has generic methods and is not object safe.
/// This enum lets repositories use the same query API in either context.
pub enum PostgresConnection {
    Database(Arc<DatabaseConnection>),
    Transaction(TransactionState),
}

pub struct TransactionState(Mutex<Option<Arc<DatabaseTransaction>>>);

impl TransactionState {
    pub(crate) fn get(&self) -> Result<Arc<DatabaseTransaction>, PersistenceError> {
        self.0
            .lock()
            .expect("transaction mutex poisoned")
            .as_ref()
            .cloned()
            .ok_or_else(|| {
                PersistenceError::database(anyhow::anyhow!("transaction scope has already ended"))
            })
    }
}

impl PostgresConnection {
    pub fn new(database: Arc<DatabaseConnection>) -> Self {
        Self::Database(database)
    }

    /// Use only inside the PostgreSQL adapter to access the active executor.
    pub fn from_context(context: &DbContext) -> Result<&Self, PersistenceError> {
        context.postgres().ensure_open()?;
        context
            .postgres()
            .as_any()
            .downcast_ref::<Self>()
            .ok_or_else(|| {
                PersistenceError::database(anyhow::anyhow!(
                    "context is not a PostgreSQL connection"
                ))
            })
    }

    fn query_error(error: PersistenceError) -> DbErr {
        match error {
            PersistenceError::Database(source) => DbErr::Custom(format!("{source:#}")),
            error => DbErr::Custom(error.to_string()),
        }
    }

    pub(crate) fn transaction(transaction: DatabaseTransaction) -> Self {
        Self::Transaction(TransactionState(Mutex::new(Some(Arc::new(transaction)))))
    }

    pub(crate) fn ensure_open(&self) -> Result<(), PersistenceError> {
        if let Self::Transaction(state) = self {
            state.get()?;
        }
        Ok(())
    }

    pub(crate) fn take_transaction(&self) -> Result<DatabaseTransaction, PersistenceError> {
        let Self::Transaction(state) = self else {
            return Err(PersistenceError::database(anyhow::anyhow!(
                "only transaction scopes can finish a transaction"
            )));
        };
        let transaction = state
            .0
            .lock()
            .expect("transaction mutex poisoned")
            .take()
            .ok_or_else(|| {
                PersistenceError::database(anyhow::anyhow!("transaction scope has already ended"))
            })?;
        Arc::try_unwrap(transaction).map_err(|_| {
            PersistenceError::database(anyhow::anyhow!(
                "transaction still has running database operations"
            ))
        })
    }

    pub(crate) fn cancel(&self) {
        if let Self::Transaction(state) = self {
            // Close escaped contexts too; SeaORM rolls back on transaction drop.
            state.0.lock().expect("transaction mutex poisoned").take();
        }
    }
}

#[async_trait::async_trait]
impl ConnectionTrait for PostgresConnection {
    fn get_database_backend(&self) -> DbBackend {
        DbBackend::Postgres
    }

    async fn execute_raw(&self, statement: Statement) -> Result<ExecResult, DbErr> {
        match self {
            Self::Database(db) => db.execute_raw(statement).await,
            Self::Transaction(state) => {
                state
                    .get()
                    .map_err(Self::query_error)?
                    .execute_raw(statement)
                    .await
            }
        }
    }
    async fn execute_unprepared(&self, sql: &str) -> Result<ExecResult, DbErr> {
        match self {
            Self::Database(db) => db.execute_unprepared(sql).await,
            Self::Transaction(state) => {
                state
                    .get()
                    .map_err(Self::query_error)?
                    .execute_unprepared(sql)
                    .await
            }
        }
    }
    async fn query_one_raw(&self, statement: Statement) -> Result<Option<QueryResult>, DbErr> {
        match self {
            Self::Database(db) => db.query_one_raw(statement).await,
            Self::Transaction(state) => {
                state
                    .get()
                    .map_err(Self::query_error)?
                    .query_one_raw(statement)
                    .await
            }
        }
    }
    async fn query_all_raw(&self, statement: Statement) -> Result<Vec<QueryResult>, DbErr> {
        match self {
            Self::Database(db) => db.query_all_raw(statement).await,
            Self::Transaction(state) => {
                state
                    .get()
                    .map_err(Self::query_error)?
                    .query_all_raw(statement)
                    .await
            }
        }
    }
}

#[async_trait::async_trait]
impl Connection for PostgresConnection {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn ensure_open(&self) -> Result<(), PersistenceError> {
        PostgresConnection::ensure_open(self)
    }

    async fn begin(&self) -> Result<Arc<dyn Connection>, PersistenceError> {
        use sea_orm::TransactionTrait;
        let transaction = match self {
            Self::Database(db) => db.begin().await,
            Self::Transaction(state) => state.get()?.begin().await,
        }
        .map_err(PersistenceError::database)?;
        Ok(Arc::new(Self::transaction(transaction)))
    }

    async fn commit(&self) -> Result<(), PersistenceError> {
        self.take_transaction()?
            .commit()
            .await
            .map_err(PersistenceError::database)
    }

    async fn rollback(&self) -> Result<(), PersistenceError> {
        self.take_transaction()?
            .rollback()
            .await
            .map_err(PersistenceError::database)
    }

    fn cancel(&self) {
        PostgresConnection::cancel(self);
    }
}
