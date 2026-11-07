use compallm_utils::email::validate_email;
use serde_json::Value;

#[test]
fn shared_auth_email_acceptance_preserves_the_rust_policy() {
    let cases: Vec<Value> = serde_json::from_str(include_str!("auth-input-fixtures.json")).unwrap();
    for (index, case) in cases.iter().enumerate() {
        let value = case["value"].as_str().unwrap().trim().to_ascii_lowercase();
        assert_eq!(
            validate_email(&value).is_ok(),
            case["valid"].as_bool().unwrap(),
            "email fixture {index}"
        );
    }
}
