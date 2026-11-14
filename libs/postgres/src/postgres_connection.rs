//! Concrete SeaORM connection or scoped transaction, usable by repository queries.

use sea_orm::{
    ConnectionTrait, DatabaseConnection, DatabaseConnectionType, DatabaseTransaction, DbBackend,
    DbErr, ExecResult, QueryResult, Statement,
};
use std::sync::{Arc, Mutex, OnceLock};

use crate::rls::{Rls, RlsSettings};

mod active_connection;
use active_connection::ActiveConnection;
use compallm_core::persistent::{Connection, ConnectionError, DbContext, PermissionLoader};
use std::any::Any;

pub struct PostgresConnection {
    conn: Arc<DatabaseConnection>,
    transaction: Option<Arc<TransactionState>>,
    rls: Arc<OnceLock<Option<Rls>>>,
}

pub struct TransactionState(Mutex<Option<Arc<DatabaseTransaction>>>);

impl TransactionState {
    pub(crate) fn get(&self) -> Result<Arc<DatabaseTransaction>, ConnectionError> {
        self.0
            .lock()
            .expect("transaction mutex poisoned")
            .as_ref()
            .cloned()
            .ok_or_else(|| {
                ConnectionError::database(anyhow::anyhow!("transaction scope has already ended"))
            })
    }
}

impl PostgresConnection {
    pub fn context(database: DatabaseConnection) -> DbContext {
        DbContext::new(Arc::new(Self::new(Arc::new(database))))
    }

    pub fn new(database: Arc<DatabaseConnection>) -> Self {
        Self {
            conn: database,
            transaction: None,
            rls: Arc::new(OnceLock::new()),
        }
    }

    /// Use only inside the PostgreSQL adapter to access the active executor.
    pub fn from_context(context: &DbContext) -> Result<&Self, ConnectionError> {
        context.postgres().ensure_open()?;
        context
            .postgres()
            .as_any()
            .downcast_ref::<Self>()
            .ok_or_else(|| {
                ConnectionError::database(anyhow::anyhow!("context is not a PostgreSQL connection"))
            })
    }

    fn query_error(error: ConnectionError) -> DbErr {
        match error {
            ConnectionError::Database(source) => DbErr::Custom(format!("{source:#}")),
        }
    }

    pub(crate) fn transaction(&self, transaction: DatabaseTransaction) -> Self {
        Self {
            conn: self.conn.clone(),
            transaction: Some(Arc::new(TransactionState(Mutex::new(Some(Arc::new(
                transaction,
            )))))),
            rls: self.rls.clone(),
        }
    }

    pub fn rls(&self) -> Option<&Rls> {
        self.rls.get().and_then(Option::as_ref)
    }

    async fn settings(&self) -> Result<Option<&RlsSettings>, DbErr> {
        self.ensure_open().map_err(Self::query_error)?;
        match self.rls.get_or_init(|| None) {
            Some(rls) => rls
                .get()
                .await
                .map(Some)
                .map_err(|error| DbErr::Custom(error.to_string())),
            None => Ok(None),
        }
    }

    async fn active_connection(&self) -> Result<Option<ActiveConnection<'_>>, DbErr> {
        let settings = self.settings().await?;
        if matches!(
            &self.conn.inner,
            DatabaseConnectionType::SqlxPostgresPoolConnection(_)
        ) {
            ActiveConnection::acquire(self.conn.get_postgres_connection_pool(), settings)
                .await
                .map(Some)
        } else if settings.is_some() {
            Err(DbErr::Custom(
                "restricted queries require a PostgreSQL pool".into(),
            ))
        } else {
            Ok(None)
        }
    }

    async fn ensure_transaction_access(&self) -> Result<(), DbErr> {
        if self.settings().await?.is_some()
            && !matches!(
                &self.conn.inner,
                DatabaseConnectionType::SqlxPostgresPoolConnection(_)
            )
        {
            return Err(DbErr::Custom(
                "restricted queries require a PostgreSQL pool".into(),
            ));
        }
        Ok(())
    }

    pub(crate) fn ensure_open(&self) -> Result<(), ConnectionError> {
        if let Some(state) = &self.transaction {
            state.get()?;
        }
        Ok(())
    }

    pub(crate) fn take_transaction(&self) -> Result<DatabaseTransaction, ConnectionError> {
        let Some(state) = &self.transaction else {
            return Err(ConnectionError::database(anyhow::anyhow!(
                "only transaction scopes can finish a transaction"
            )));
        };
        let transaction = state
            .0
            .lock()
            .expect("transaction mutex poisoned")
            .take()
            .ok_or_else(|| {
                ConnectionError::database(anyhow::anyhow!("transaction scope has already ended"))
            })?;
        Arc::try_unwrap(transaction).map_err(|_| {
            ConnectionError::database(anyhow::anyhow!(
                "transaction still has running database operations"
            ))
        })
    }

    pub(crate) fn cancel(&self) {
        if let Some(state) = &self.transaction {
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
        if let Some(state) = &self.transaction {
            self.ensure_transaction_access().await?;
            state
                .get()
                .map_err(Self::query_error)?
                .execute_raw(statement)
                .await
        } else if let Some(mut active) = self.active_connection().await? {
            let result = active.execute(&statement).await?;
            active.release().await?;
            Ok(result)
        } else {
            self.conn.execute_raw(statement).await
        }
    }

    async fn execute_unprepared(&self, sql: &str) -> Result<ExecResult, DbErr> {
        if let Some(state) = &self.transaction {
            self.ensure_transaction_access().await?;
            state
                .get()
                .map_err(Self::query_error)?
                .execute_unprepared(sql)
                .await
        } else if let Some(mut active) = self.active_connection().await? {
            let result = active.execute_unprepared(sql).await?;
            active.release().await?;
            Ok(result)
        } else {
            self.conn.execute_unprepared(sql).await
        }
    }

    async fn query_one_raw(&self, statement: Statement) -> Result<Option<QueryResult>, DbErr> {
        if let Some(state) = &self.transaction {
            self.ensure_transaction_access().await?;
            state
                .get()
                .map_err(Self::query_error)?
                .query_one_raw(statement)
                .await
        } else if let Some(mut active) = self.active_connection().await? {
            let result = active.query_one(&statement).await?;
            active.release().await?;
            Ok(result)
        } else {
            self.conn.query_one_raw(statement).await
        }
    }

    async fn query_all_raw(&self, statement: Statement) -> Result<Vec<QueryResult>, DbErr> {
        if let Some(state) = &self.transaction {
            self.ensure_transaction_access().await?;
            state
                .get()
                .map_err(Self::query_error)?
                .query_all_raw(statement)
                .await
        } else if let Some(mut active) = self.active_connection().await? {
            let result = active.query_all(&statement).await?;
            active.release().await?;
            Ok(result)
        } else {
            self.conn.query_all_raw(statement).await
        }
    }
}

#[async_trait::async_trait]
impl Connection for PostgresConnection {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_transaction(&self) -> bool {
        self.transaction.is_some()
    }

    fn ensure_open(&self) -> Result<(), ConnectionError> {
        PostgresConnection::ensure_open(self)
    }

    fn scoped(&self) -> Arc<dyn Connection> {
        let rls = if self.transaction.is_some() || self.rls().is_some() {
            self.rls.clone()
        } else {
            Arc::new(OnceLock::new())
        };
        Arc::new(Self {
            conn: self.conn.clone(),
            transaction: self.transaction.clone(),
            rls,
        })
    }

    fn restrict(&self, permission: PermissionLoader) -> Result<(), ConnectionError> {
        self.ensure_open()?;
        if self.transaction.is_some() {
            return Err(ConnectionError::database(anyhow::anyhow!(
                "transaction cannot acquire a new restriction"
            )));
        }
        self.rls.set(Some(Rls::new(permission))).map_err(|_| {
            ConnectionError::database(anyhow::anyhow!(
                "connection is already restricted or a transaction has started"
            ))
        })
    }

    async fn begin(&self) -> Result<Arc<dyn Connection>, ConnectionError> {
        use sea_orm::TransactionTrait;
        self.ensure_open()?;
        let settings = self.settings().await.map_err(ConnectionError::database)?;
        let transaction = if let Some(state) = &self.transaction {
            state.get()?.begin().await
        } else {
            self.conn.begin().await
        }
        .map_err(ConnectionError::database)?;
        if let Some(settings) = settings
            && matches!(
                &self.conn.inner,
                DatabaseConnectionType::SqlxPostgresPoolConnection(_)
            )
        {
            transaction
                .execute_unprepared("SET LOCAL ROLE compallm_restricted")
                .await
                .map_err(ConnectionError::database)?;
            let (keys, values): (Vec<String>, Vec<String>) =
                settings.settings().iter().cloned().unzip();
            transaction.execute_raw(Statement::from_sql_and_values(
                DbBackend::Postgres,
                "SELECT set_config(key, value, true) FROM unnest($1::text[], $2::text[]) AS settings(key, value)",
                [keys.into(), values.into()],
            )).await.map_err(ConnectionError::database)?;
        }
        Ok(Arc::new(self.transaction(transaction)))
    }

    async fn commit(&self) -> Result<(), ConnectionError> {
        self.take_transaction()?
            .commit()
            .await
            .map_err(ConnectionError::database)
    }

    async fn rollback(&self) -> Result<(), ConnectionError> {
        self.take_transaction()?
            .rollback()
            .await
            .map_err(ConnectionError::database)
    }

    fn cancel(&self) {
        PostgresConnection::cancel(self);
    }
}
