use compallm_core::persistent::user::UserRepositoryError;
use std::{error::Error, io};

#[test]
fn database_error_preserves_context_and_source() {
    let source = anyhow::Error::new(io::Error::other("underlying failure"));
    let error = UserRepositoryError::database(source.context("could not load user"));
    assert_eq!(error.to_string(), "database operation failed");
    let UserRepositoryError::Database(source) = error else {
        panic!("expected database error")
    };
    assert_eq!(source.to_string(), "could not load user");
    assert!(source.downcast_ref::<io::Error>().is_some());
}

#[test]
fn anyhow_errors_convert_to_database_errors() {
    let error: UserRepositoryError = anyhow::anyhow!("repository failure").into();
    assert!(matches!(error, UserRepositoryError::Database(_)));
    assert_eq!(error.source().unwrap().to_string(), "repository failure");
}

#[test]
fn transaction_errors_share_the_persistence_error_boundary() {
    let error = compallm_core::auth::AuthError::from(UserRepositoryError::database(
        sea_orm::DbErr::Custom("transaction failed".into()),
    ));
    assert!(matches!(
        error,
        compallm_core::auth::AuthError::Persistence(UserRepositoryError::Database(_))
    ));
}
