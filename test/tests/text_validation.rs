use compallm_core::user::User;

#[tokio::test]
async fn user_preparation_normalizes_email_and_derives_username_with_empty_full_name() {
    compallm_postgres::init_id_generator("user-validation-tests").unwrap();
    let user = User::from_email("  James.Dang+dev@MIDWESS.COM  ")
        .await
        .unwrap();
    assert_eq!(user.email, "james.dang+dev@midwess.com");
    assert_eq!(user.username, "james.dang+dev");
    assert!(user.display_name.is_empty());
}
