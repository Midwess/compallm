use compallm_core::user::User;

#[tokio::test]
async fn user_preparation_normalizes_email_and_derives_display_name() {
    compallm_postgres::init_id_generator("user-validation-tests").unwrap();
    let user = User::from_email("  James.Dang+dev@MIDWESS.COM  ")
        .await
        .unwrap();
    assert_eq!(user.email, "james.dang+dev@midwess.com");
    assert_eq!(user.display_name, "james.dang+dev");
}
