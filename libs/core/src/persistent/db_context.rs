//! Scoped database context. Operations run on the caller's task and may borrow
//! local data; they do not need `move`, `Send`, or a `'static` lifetime.

use anyhow::Context;
use async_once_cell::Lazy;
use std::{
    error::Error,
    future::Future,
    pin::Pin,
    sync::{
        Arc, OnceLock,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
};

use super::{Connection, ConnectionError};
use crate::authorization::MemberPermission;

static DEFAULT_CONNECTION: OnceLock<Arc<dyn Connection>> = OnceLock::new();

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
    member_permission: Arc<OnceLock<LazyPermission>>,
    request_started: Arc<AtomicBool>,
}

type PermissionLoadError = Arc<dyn Error + Send + Sync>;
type PermissionResult = Result<Arc<MemberPermission>, PermissionLoadError>;
type PermissionFuture = Pin<Box<dyn Future<Output = PermissionResult> + Send>>;
type LazyPermission = Lazy<PermissionResult, PermissionFuture>;

#[derive(Debug, thiserror::Error)]
pub enum PermissionError {
    #[error("member authority is not available")]
    Missing,
    #[error("member authority could not be loaded")]
    Loading(#[source] PermissionLoadError),
    #[error(transparent)]
    Context(#[from] ConnectionError),
}

#[derive(Default)]
struct TransactionState {
    closed: AtomicBool,
    rollback_only: AtomicBool,
    participants: AtomicUsize,
}

impl DbContext {
    /// Initialize the connection used by default contexts once at startup.
    pub fn initialize_default(postgres: Arc<dyn Connection>) -> Result<(), ConnectionError> {
        postgres.ensure_open()?;
        if postgres.is_transaction() {
            return Err(ConnectionError::database(anyhow::anyhow!(
                "default connection cannot be a transaction"
            )));
        }
        DEFAULT_CONNECTION.set(postgres).map_err(|_| {
            ConnectionError::database(anyhow::anyhow!("default connection is already initialized"))
        })
    }

    /// Application dependency wiring supplies the already configured database.
    pub fn new(postgres: Arc<dyn Connection>) -> Self {
        let transaction = postgres
            .is_transaction()
            .then(|| Arc::new(TransactionState::default()));
        Self {
            postgres,
            transaction,
            member_permission: Arc::new(OnceLock::new()),
            request_started: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Register a loader for the request's member authority once.
    pub fn restrict<F, E>(&self, loader: F) -> Result<Self, ConnectionError>
    where
        F: Future<Output = Result<Arc<MemberPermission>, E>> + Send + 'static,
        E: Error + Send + Sync + 'static,
    {
        self.postgres.ensure_open()?;
        self.ensure_active()?;
        let loader: PermissionFuture = Box::pin(async move {
            loader
                .await
                .map_err(|error| Arc::new(error) as PermissionLoadError)
        });
        self.member_permission.set(Lazy::new(loader)).map_err(|_| {
            ConnectionError::database(anyhow::anyhow!("member authority is already set"))
        })?;
        Ok(self.clone())
    }

    /// Load the member authority once and share the result within this request.
    pub async fn permission(&self) -> Result<&MemberPermission, PermissionError> {
        self.postgres.ensure_open()?;
        self.ensure_active()?;
        let loader = self
            .member_permission
            .get()
            .ok_or(PermissionError::Missing)?;
        let result = loader.get_unpin().await;
        self.postgres.ensure_open()?;
        self.ensure_active()?;
        result
            .as_deref()
            .map_err(|error| PermissionError::Loading(error.clone()))
    }

    /// Enter a new request scope without inheriting another task-local context.
    pub async fn request_scope<F: Future>(
        self,
        operation: F,
    ) -> Result<F::Output, ConnectionError> {
        if CURRENT.try_with(|_| ()).is_ok() {
            return Err(ConnectionError::database(anyhow::anyhow!(
                "a request scope is already active"
            )));
        }
        self.postgres.ensure_open()?;
        self.ensure_active()?;
        if self.postgres.is_transaction() {
            return Err(ConnectionError::database(anyhow::anyhow!(
                "a request context cannot start from a transaction"
            )));
        }
        if self.request_started.swap(true, Ordering::AcqRel) {
            return Err(ConnectionError::database(anyhow::anyhow!(
                "the request context has already been used"
            )));
        }
        Ok(CURRENT.scope(self, operation).await)
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
            member_permission: source.member_permission.clone(),
            request_started: source.request_started.clone(),
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

impl Default for DbContext {
    fn default() -> Self {
        let postgres = DEFAULT_CONNECTION
            .get()
            .expect("default database connection must be initialized");
        Self::new(postgres.clone())
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
