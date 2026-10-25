//! Concrete SeaORM connection or scoped transaction, usable by repository queries.

use sea_orm::{
    ConnectionTrait, DatabaseConnection, DatabaseTransaction, DbBackend, DbErr, ExecResult,
    QueryResult, Statement,
};
use std::sync::{Arc, Mutex};

use super::PersistenceError;

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
            .ok_or(PersistenceError::ClosedConnection)
    }
}

impl PostgresConnection {
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
            unreachable!("only transaction scopes can finish a transaction")
        };
        let transaction = state
            .0
            .lock()
            .expect("transaction mutex poisoned")
            .take()
            .ok_or(PersistenceError::ClosedConnection)?;
        Arc::try_unwrap(transaction).map_err(|_| PersistenceError::BusyTransaction)
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
                    .map_err(DbErr::from)?
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
                    .map_err(DbErr::from)?
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
                    .map_err(DbErr::from)?
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
                    .map_err(DbErr::from)?
                    .query_all_raw(statement)
                    .await
            }
        }
    }
}

impl From<PersistenceError> for DbErr {
    fn from(error: PersistenceError) -> Self {
        Self::Custom(error.to_string())
    }
}
