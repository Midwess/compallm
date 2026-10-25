//! Scoped database context. Operations run on the caller's task and may borrow
//! local data; they do not need `move`, `Send`, or a `'static` lifetime.

use anyhow::Context;
use std::{
    future::Future,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
};

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
    transaction: Option<Arc<TransactionState>>,
}

#[derive(Default)]
struct TransactionState {
    closed: AtomicBool,
    rollback_only: AtomicBool,
    participants: AtomicUsize,
}

impl DbContext {
    /// Application dependency wiring supplies the already configured database.
    pub fn new(postgres: Arc<dyn Connection>) -> Self {
        let transaction = postgres
            .is_transaction()
            .then(|| Arc::new(TransactionState::default()));
        Self {
            postgres,
            transaction,
        }
    }

    pub fn current() -> Result<Self, ConnectionError> {
        let context = CURRENT
            .try_with(Clone::clone)
            .context("no database connection scope is active")?;
        context.postgres.ensure_open()?;
        context.ensure_active()?;
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

    /// Nested calls join the current transaction without another BEGIN or savepoint.
    /// Only the outermost call commits or rolls back. A failed, panicked, or
    /// cancelled nested operation marks the shared transaction for rollback.
    pub async fn transaction<F, T, E>(&self, operation: F) -> Result<T, E>
    where
        F: Future<Output = Result<T, E>>,
        E: From<ConnectionError> + std::fmt::Display,
    {
        let source = CURRENT
            .try_with(Clone::clone)
            .unwrap_or_else(|_| self.clone());
        source.postgres.ensure_open()?;
        source.ensure_active()?;
        if let Some(state) = &source.transaction {
            state.participants.fetch_add(1, Ordering::AcqRel);
            let mut participant = TransactionParticipant {
                state: state.clone(),
                succeeded: false,
            };
            // Register before rechecking closure, so commit cannot race with
            // a participant entering through an explicitly propagated scope.
            source.ensure_active()?;
            let result = CURRENT.scope(source.clone(), operation).await;
            participant.succeeded = result.is_ok();
            return result;
        }
        let transaction = source.postgres().begin().await?;
        let context = Self {
            postgres: transaction,
            transaction: Some(Arc::new(TransactionState::default())),
        };
        let guard = TransactionScope(context.clone());
        let result = CURRENT.scope(context.clone(), operation).await;
        context
            .transaction
            .as_ref()
            .expect("transaction state exists")
            .closed
            .store(true, Ordering::Release);
        // Do not commit partial work when an inner error was caught or an
        // unfinished participating future escaped the outer operation.
        let result = result.and_then(|value| {
            context.ensure_committable()?;
            if context
                .transaction
                .as_ref()
                .expect("transaction state exists")
                .participants
                .load(Ordering::Acquire)
                != 0
            {
                return Err(ConnectionError::database(anyhow::anyhow!(
                    "transaction still has unfinished nested operations"
                ))
                .into());
            }
            Ok(value)
        });
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

    fn ensure_committable(&self) -> Result<(), ConnectionError> {
        if self
            .transaction
            .as_ref()
            .is_some_and(|state| state.rollback_only.load(Ordering::Acquire))
        {
            return Err(ConnectionError::database(anyhow::anyhow!(
                "transaction is marked for rollback after a nested failure"
            )));
        }
        Ok(())
    }

    fn ensure_active(&self) -> Result<(), ConnectionError> {
        if self
            .transaction
            .as_ref()
            .is_some_and(|state| state.closed.load(Ordering::Acquire))
        {
            return Err(ConnectionError::database(anyhow::anyhow!(
                "transaction scope has already ended"
            )));
        }
        self.ensure_committable()
    }
}

struct TransactionParticipant {
    state: Arc<TransactionState>,
    succeeded: bool,
}

impl Drop for TransactionParticipant {
    fn drop(&mut self) {
        if !self.succeeded {
            self.state.rollback_only.store(true, Ordering::Release);
        }
        self.state.participants.fetch_sub(1, Ordering::AcqRel);
    }
}

struct TransactionScope(DbContext);

impl Drop for TransactionScope {
    fn drop(&mut self) {
        if let Some(state) = &self.0.transaction {
            state.closed.store(true, Ordering::Release);
        }
        self.0.postgres().cancel();
    }
}
