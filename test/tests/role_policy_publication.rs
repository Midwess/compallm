#[test]
fn policy_publication_rejects_expressions_row_security_cannot_translate() {
    let supported = r#"permit(principal, action == Compallm::Action::"model.use", resource is Compallm::Model) when { resource.enabled };"#;
    let unsupported = r#"permit(principal, action == Compallm::Action::"model.use", resource is Compallm::Model) when { resource.enabled || principal in [Compallm::User::"1"] };"#;
    assert!(compallm_postgres::rls::validate_policy_source(supported).is_ok());
    assert!(compallm_postgres::rls::validate_policy_source(unsupported).is_err());
}
