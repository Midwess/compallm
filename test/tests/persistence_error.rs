use compallm_core::persistent::PersistenceError;
use std::{error::Error, io};

#[test]
fn general_error_preserves_message_and_source() {
    let source = anyhow::Error::new(io::Error::other("underlying failure"));
    let error = PersistenceError::general("could not load user", source);
    assert_eq!(error.to_string(), "could not load user");
    assert!(error.source().unwrap().is::<io::Error>());
}

#[test]
fn anyhow_errors_convert_to_general_persistence_errors() {
    let error: PersistenceError = anyhow::anyhow!("repository failure").into();
    assert!(matches!(error, PersistenceError::General { .. }));
    assert_eq!(error.source().unwrap().to_string(), "repository failure");
}

#[test]
fn transaction_errors_share_the_persistence_error_boundary() {
    let error = compallm_core::auth::AuthError::from(PersistenceError::database(
        sea_orm::DbErr::Custom("transaction failed".into()),
    ));
    assert!(matches!(
        error,
        compallm_core::auth::AuthError::Persistence(PersistenceError::Database(_))
    ));
}
