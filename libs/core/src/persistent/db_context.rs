//! Scoped database context. Operations run on the caller's task and may borrow
//! local data; they do not need `move`, `Send`, or a `'static` lifetime.

use anyhow::Context;
use std::{future::Future, sync::Arc};

use super::{Connection, ConnectionError};

tokio::task_local! {
    static CURRENT: DbContext;
}

/// Await a scoped transaction without an explicit async block or `.await`.
/// The body may borrow local data and use `?`; it must return a `Result`.
/// `return` inside the body exits the transaction body, not the enclosing function.
///
/// ```
/// # use compallm_core::{transaction, persistent::{DbContext, ConnectionError}};
/// # async fn example(db: &DbContext) -> Result<(), ConnectionError> {
/// let email = String::from("james@midwess.com");
/// let borrowed = transaction!(db, {
///     let _context = DbContext::current()?;
///     Ok::<_, ConnectionError>(email.as_str())
/// })?;
/// assert_eq!(borrowed, email.as_str());
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! transaction {
    ($context:expr, $body:block $(,)?) => {
        ($context).transaction(async { $body }).await
    };
}

/// Shared context. Add other concrete database handles here when needed.
#[derive(Clone)]
pub struct DbContext {
    postgres: Arc<dyn Connection>,
}

impl DbContext {
    /// Application dependency wiring supplies the already configured database.
    pub fn new(postgres: Arc<dyn Connection>) -> Self {
        Self { postgres }
    }

    pub fn current() -> Result<Self, ConnectionError> {
        let context = CURRENT
            .try_with(Clone::clone)
            .context("no database connection scope is active")?;
        context.postgres.ensure_open()?;
        Ok(context)
    }

    pub fn postgres(&self) -> &dyn Connection {
        self.postgres.as_ref()
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
    /// releases the transaction so its adapter can roll it back.
    pub async fn transaction<F, T, E>(&self, operation: F) -> Result<T, E>
    where
        F: Future<Output = Result<T, E>>,
        E: From<ConnectionError> + std::fmt::Display,
    {
        let source = CURRENT
            .try_with(Clone::clone)
            .unwrap_or_else(|_| self.clone());
        source.postgres.ensure_open()?;
        let transaction = source.postgres().begin().await?;
        let context = Self {
            postgres: transaction,
        };
        let guard = TransactionScope(context.clone());
        let result = CURRENT.scope(context.clone(), operation).await;
        let result = match result {
            Ok(value) => {
                context.postgres().commit().await?;
                Ok(value)
            }
            Err(error) => match context.postgres().rollback().await {
                Ok(()) => Err(error),
                Err(rollback) => Err(ConnectionError::database(
                    anyhow::Error::new(rollback)
                        .context(format!("operation failed ({error}); rollback also failed")),
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
