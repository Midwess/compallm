use compallm_core::{
    authentication::{Session, UserAccessToken},
    authorization::{MemberPermission, PolicySnapshot},
    organization::OrganizationMember,
    persistent::{ConnectionError, DbContext, PermissionError},
};
use compallm_postgres::postgres_connection::PostgresConnection;
use sea_orm::{DbBackend, MockDatabase};
use std::{
    convert::Infallible,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

struct Fixture;

impl Fixture {
    fn connection() -> DbContext {
        DbContext::new(Arc::new(PostgresConnection::new(Arc::new(
            MockDatabase::new(DbBackend::Postgres).into_connection(),
        ))))
    }

    fn permission(user_id: u64, organization_id: u64) -> MemberPermission {
        let membership = OrganizationMember {
            user_id,
            organization_id,
            roles: Vec::new(),
            groups: Vec::new(),
        };
        let access = UserAccessToken::new(
            Session {
                id: 1,
                user_id,
                organization_id,
                organization_name: String::new(),
                authorization_version: 1,
                revoked_at: None,
            },
            PolicySnapshot::new(&membership).unwrap(),
            Duration::from_secs(60),
        )
        .unwrap();
        MemberPermission::try_from(access).unwrap()
    }
}

#[tokio::test]
async fn default_contexts_share_connection_without_sharing_member_authority() {
    let connection: Arc<dyn compallm_core::persistent::Connection> =
        Arc::new(PostgresConnection::new(Arc::new(
            MockDatabase::new(DbBackend::Postgres).into_connection(),
        )));
    DbContext::initialize_default(connection).unwrap();
    let first = DbContext::default();
    let second = DbContext::default();
    first
        .restrict(async { Ok::<_, Infallible>(Arc::new(Fixture::permission(1, 7))) })
        .unwrap();
    assert_eq!(first.permission().await.unwrap().user_id(), 1);
    assert!(matches!(
        second.permission().await,
        Err(PermissionError::Missing)
    ));
    assert!(!std::ptr::eq(first.postgres(), second.postgres()));
    let reused = first.clone();
    first
        .request_scope(async {
            assert_eq!(
                DbContext::current()
                    .unwrap()
                    .permission()
                    .await
                    .unwrap()
                    .user_id(),
                1
            );
        })
        .await
        .unwrap();
    assert!(reused.request_scope(async {}).await.is_err());
}

#[tokio::test(flavor = "current_thread")]
async fn request_authority_is_isolated_and_nested_work_retains_it() {
    let connection: Arc<dyn compallm_core::persistent::Connection> =
        Arc::new(PostgresConnection::new(Arc::new(
            MockDatabase::new(DbBackend::Postgres).into_connection(),
        )));
    let first_context = DbContext::new(connection.clone());
    let second_context = DbContext::new(connection);
    let loads = Arc::new(AtomicUsize::new(0));
    let first_loads = loads.clone();
    first_context
        .restrict(async move {
            first_loads.fetch_add(1, Ordering::SeqCst);
            tokio::task::yield_now().await;
            Ok::<_, Infallible>(Arc::new(Fixture::permission(1, 7)))
        })
        .unwrap();
    let (first, second) = tokio::join!(
        first_context.request_scope(async {
            let context = DbContext::current().unwrap();
            assert_eq!(loads.load(Ordering::SeqCst), 0);
            let (left, right) = tokio::join!(context.permission(), context.permission());
            assert!(std::ptr::eq(left.unwrap(), right.unwrap()));
            assert_eq!(loads.load(Ordering::SeqCst), 1);
            tokio::task::yield_now().await;
            assert_eq!(context.permission().await.unwrap().organization_id(), 7);
            assert!(
                context
                    .restrict(async { Ok::<_, Infallible>(Arc::new(Fixture::permission(2, 8))) })
                    .is_err()
            );
            context
                .transaction(async {
                    let nested = DbContext::current()?;
                    assert_eq!(nested.permission().await.unwrap().user_id(), 1);
                    assert_eq!(nested.permission().await.unwrap().organization_id(), 7);
                    Ok::<_, ConnectionError>(())
                })
                .await
                .unwrap();
        }),
        second_context.request_scope(async {
            let context = DbContext::current().unwrap();
            assert!(matches!(
                context.permission().await,
                Err(PermissionError::Missing)
            ));
            context
                .restrict(async { Ok::<_, Infallible>(Arc::new(Fixture::permission(2, 8))) })
                .unwrap();
            tokio::task::yield_now().await;
            assert_eq!(context.permission().await.unwrap().user_id(), 2);
            assert_eq!(context.permission().await.unwrap().organization_id(), 8);
        })
    );
    first.unwrap();
    second.unwrap();
    assert!(DbContext::current().is_err());
}

#[tokio::test]
async fn request_scope_does_not_replace_an_active_context() {
    let root = Fixture::connection();
    root.clone()
        .request_scope(async {
            let context = DbContext::current().unwrap();
            context
                .restrict(async { Ok::<_, Infallible>(Arc::new(Fixture::permission(1, 7))) })
                .unwrap();
            assert!(root.clone().request_scope(async {}).await.is_err());
            assert_eq!(
                DbContext::current()
                    .unwrap()
                    .permission()
                    .await
                    .unwrap()
                    .user_id(),
                1
            );
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn permission_loader_survives_waiter_cancellation_and_caches_failure() {
    let context = Fixture::connection();
    let (started_tx, started_rx) = tokio::sync::oneshot::channel();
    let proceed = Arc::new(tokio::sync::Notify::new());
    let loader_proceed = proceed.clone();
    let loads = Arc::new(AtomicUsize::new(0));
    let loader_count = loads.clone();
    context
        .restrict(async move {
            loader_count.fetch_add(1, Ordering::SeqCst);
            let _ = started_tx.send(());
            loader_proceed.notified().await;
            Err::<Arc<MemberPermission>, _>(std::io::Error::other("unavailable"))
        })
        .unwrap();

    let waiting_context = context.clone();
    let waiter = tokio::spawn(async move {
        let _ = waiting_context.permission().await;
    });
    started_rx.await.unwrap();
    waiter.abort();
    let _ = waiter.await;
    proceed.notify_one();
    let Err(PermissionError::Loading(source)) = context.permission().await else {
        panic!("unexpected permission failure");
    };
    assert!(source.downcast_ref::<std::io::Error>().is_some());
    assert!(matches!(
        context.permission().await,
        Err(PermissionError::Loading(_))
    ));
    assert_eq!(loads.load(Ordering::SeqCst), 1);
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
            assert!(std::ptr::eq(
                DbContext::current().unwrap().postgres(),
                connection.postgres()
            ));
            connection
                .transaction(async {
                    let transaction = DbContext::current()?;
                    assert!(!std::ptr::eq(transaction.postgres(), connection.postgres()));
                    connection
                        .scope(async {
                            assert!(std::ptr::eq(
                                DbContext::current().unwrap().postgres(),
                                transaction.postgres()
                            ));
                        })
                        .await;
                    Ok::<_, ConnectionError>(())
                })
                .await
                .unwrap();
            assert!(std::ptr::eq(
                DbContext::current().unwrap().postgres(),
                connection.postgres()
            ));
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
            assert!(std::ptr::eq(
                DbContext::current().unwrap().postgres(),
                one.postgres()
            ));
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
            assert!(std::ptr::eq(
                DbContext::current().unwrap().postgres(),
                two.postgres()
            ));
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
            assert!(std::ptr::eq(
                DbContext::current()?.postgres(),
                transaction.postgres()
            ));
            one.transaction(async {
                assert!(std::ptr::eq(
                    DbContext::current()?.postgres(),
                    transaction.postgres()
                ));
                Ok::<_, ConnectionError>(())
            })
            .await
        }),
        two.transaction(async {
            let transaction = DbContext::current()?;
            tokio::task::yield_now().await;
            assert!(std::ptr::eq(
                DbContext::current()?.postgres(),
                transaction.postgres()
            ));
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
        assert!(!std::ptr::eq(
            DbContext::current()?.postgres(),
            connection.postgres()
        ));
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

#[tokio::test]
async fn explicit_trusted_scope_restores_and_cannot_remove_request_restrictions() {
    let template = Fixture::connection();
    let request = template.fresh().unwrap();
    let trusted = template.fresh().unwrap();
    request
        .restrict(async { Ok::<_, Infallible>(Arc::new(Fixture::permission(1, 7))) })
        .unwrap();
    assert!(request.fresh().is_err());
    assert!(request.independent_scope(async {}).await.is_err());
    request
        .request_scope(async {
            let restricted = DbContext::current().unwrap();
            let result = trusted
                .independent_scope(async {
                    let current = DbContext::current().unwrap();
                    assert!(
                        PostgresConnection::from_context(&current)
                            .unwrap()
                            .rls()
                            .is_none()
                    );
                    Err::<(), _>(ConnectionError::database(anyhow::anyhow!(
                        "trusted operation failed"
                    )))
                })
                .await
                .unwrap();
            assert!(result.is_err());
            assert!(std::ptr::eq(
                DbContext::current().unwrap().postgres(),
                restricted.postgres()
            ));
            let mut cancelled = Box::pin(trusted.independent_scope(std::future::pending::<()>()));
            tokio::select! {
                biased;
                _ = &mut cancelled => unreachable!(),
                _ = std::future::ready(()) => {}
            }
            drop(cancelled);
            assert!(std::ptr::eq(
                DbContext::current().unwrap().postgres(),
                restricted.postgres()
            ));
            assert_eq!(
                DbContext::current()
                    .unwrap()
                    .permission()
                    .await
                    .unwrap()
                    .organization_id(),
                7
            );
        })
        .await
        .unwrap();
    assert!(DbContext::current().is_err());
}

#[tokio::test]
async fn lazy_rls_shares_preparation_and_retains_failures_after_waiter_cancellation() {
    let db = Fixture::connection();
    db.restrict(async { Ok::<_, Infallible>(Arc::new(Fixture::permission(1, 7))) })
        .unwrap();
    let connection = PostgresConnection::from_context(&db).unwrap();
    let rls = connection.rls().unwrap();
    let (left, right) = tokio::join!(rls.get(), rls.get());
    let (left, right) = (left.unwrap(), right.unwrap());
    assert!(std::ptr::eq(left, right));
    db.transaction(async {
        let current = DbContext::current()?;
        let settings = PostgresConnection::from_context(&current)?
            .rls()
            .unwrap()
            .get()
            .await
            .unwrap();
        assert!(std::ptr::eq(left, settings));
        Ok::<_, ConnectionError>(())
    })
    .await
    .unwrap();

    let failed = Fixture::connection();
    let loads = Arc::new(AtomicUsize::new(0));
    let counter = loads.clone();
    let (started, ready) = tokio::sync::oneshot::channel();
    let proceed = Arc::new(tokio::sync::Notify::new());
    let release = proceed.clone();
    failed
        .restrict(async move {
            counter.fetch_add(1, Ordering::SeqCst);
            let _ = started.send(());
            release.notified().await;
            Err::<Arc<MemberPermission>, _>(std::io::Error::other("membership unavailable"))
        })
        .unwrap();
    let waiting = failed.clone();
    let waiter = tokio::spawn(async move {
        let _ = PostgresConnection::from_context(&waiting)
            .unwrap()
            .rls()
            .unwrap()
            .get()
            .await;
    });
    ready.await.unwrap();
    waiter.abort();
    let _ = waiter.await;
    proceed.notify_one();
    let connection = PostgresConnection::from_context(&failed).unwrap();
    let rls = connection.rls().unwrap();
    let first = rls.get().await.err().unwrap();
    let second = rls.get().await.err().unwrap();
    assert!(Arc::ptr_eq(&first, &second));
    assert!(first.downcast_ref::<std::io::Error>().is_some());
    assert_eq!(loads.load(Ordering::SeqCst), 1);
}
