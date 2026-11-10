use compallm_core::persistent::user::UserRepositoryError;

#[test]
fn transaction_errors_share_the_persistence_error_boundary() {
    let error = compallm_core::authentication::AuthError::from(UserRepositoryError::database(
        sea_orm::DbErr::Custom("transaction failed".into()),
    ));
    assert!(matches!(
        error,
        compallm_core::authentication::AuthError::Persistence(UserRepositoryError::Database(_))
    ));
}
