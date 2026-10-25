use std::time::Duration;

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use hero::token::jwt::create_jwt_token;

use compallm_core::auth::{
    AuthError, TokenCodec,
    token::{
        AccessToken, EmailOtpAccessToken, EncryptedAccessToken, EncryptedToken, RefreshToken,
        UserAccessToken,
    },
};

const SECRET: &str = "test-signing-secret-at-least-32-bytes";

fn service() -> TokenCodec {
    TokenCodec::new(SECRET.into(), "midwess.com").unwrap()
}

#[test]
fn email_jwt_exposes_ciphertext_not_email_or_code() {
    let challenge = service().issue_email_otp(" James@MIDWESS.COM ").unwrap();
    let payload = challenge.access_token.split('.').nth(1).unwrap();
    let decoded = URL_SAFE_NO_PAD.decode(payload).unwrap();
    let json: serde_json::Value = serde_json::from_slice(&decoded).unwrap();
    assert!(
        json["data"]["encrypted"]
            .as_str()
            .unwrap()
            .starts_with("v1.")
    );
    assert!(json["data"].get("user").is_none());
    assert!(
        !String::from_utf8(decoded)
            .unwrap()
            .contains("james@midwess.com")
    );
    let claims = service()
        .verify_email_otp(&challenge.access_token, &challenge.code)
        .unwrap();
    assert_eq!(claims.email, "james@midwess.com");
    assert_eq!(claims.code, challenge.code);
    assert_eq!(claims.expires_at - claims.issued_at, 600);
}

#[test]
fn otp_comparison_and_expiration_are_strict() {
    let claims = EmailOtpAccessToken::new("james@midwess.com".into(), 1000).unwrap();
    assert!(claims.verify_code(&claims.code, 1599).is_ok());
    assert!(matches!(
        claims.verify_code(&claims.code, 1600),
        Err(AuthError::ExpiredToken)
    ));
    assert!(matches!(
        claims.verify_code("wrong", 1000),
        Err(AuthError::IncorrectCode)
    ));
    let wrong = if claims.code == "000000" {
        "000001"
    } else {
        "000000"
    };
    assert!(matches!(
        claims.verify_code(wrong, 1000),
        Err(AuthError::IncorrectCode)
    ));
}

#[test]
fn old_otp_pairs_remain_valid_after_another_is_issued() {
    let auth = service();
    let old = auth.issue_email_otp("james@midwess.com").unwrap();
    let new = auth.issue_email_otp("james@midwess.com").unwrap();
    assert_ne!(old.access_token, new.access_token);
    assert!(auth.verify_email_otp(&old.access_token, &old.code).is_ok());
    assert!(auth.verify_email_otp(&new.access_token, &new.code).is_ok());
    assert!(auth.verify_email_otp(&old.access_token, &old.code).is_ok());
}

#[test]
fn jwt_signature_and_payload_tampering_are_rejected() {
    let auth = service();
    let challenge = auth.issue_email_otp("james@midwess.com").unwrap();
    let other = TokenCodec::new(
        "another-signing-secret-at-least-32-bytes".into(),
        "midwess.com",
    )
    .unwrap();
    assert!(matches!(
        other.parse_access_token(&challenge.access_token).err(),
        Some(AuthError::InvalidToken)
    ));
    let mut parts: Vec<String> = challenge
        .access_token
        .split('.')
        .map(str::to_owned)
        .collect();
    parts[1] = URL_SAFE_NO_PAD.encode(br#"{"data":{"user":{}},"exp":9999999999,"iat":1}"#);
    assert!(matches!(
        auth.parse_access_token(&parts.join(".")).err(),
        Some(AuthError::InvalidToken)
    ));
}

#[test]
fn ciphertext_requires_its_configured_secret_and_integrity_tag() {
    let now = compallm_utils::time::unix_timestamp();
    let claims = EmailOtpAccessToken::new("james@midwess.com".into(), now).unwrap();
    let cipher = EncryptedAccessToken::new(SECRET).unwrap();
    let AccessToken::Encrypted(payload) = cipher.encrypt(AccessToken::EmailOtp(claims)).unwrap()
    else {
        panic!("expected encrypted wrapper")
    };
    assert!(matches!(
        EncryptedAccessToken::new("another-signing-secret-at-least-32-bytes")
            .unwrap()
            .decrypt(payload.clone())
            .err(),
        Some(AuthError::TokenCrypto(_))
    ));
    let mut bytes = URL_SAFE_NO_PAD
        .decode(payload.0.strip_prefix("v1.").unwrap())
        .unwrap();
    *bytes.last_mut().unwrap() ^= 1;
    let altered = EncryptedToken(format!("v1.{}", URL_SAFE_NO_PAD.encode(bytes)));
    assert!(matches!(
        cipher.decrypt(altered).err(),
        Some(AuthError::TokenCrypto(_))
    ));
}

#[test]
fn valid_jwt_cannot_extend_expired_inner_claims() {
    let now = compallm_utils::time::unix_timestamp();
    let claims = UserAccessToken::new(1, 2, now - 61, Duration::from_secs(60)).unwrap();
    let jwt =
        create_jwt_token(AccessToken::User(claims), SECRET, Duration::from_secs(600)).unwrap();
    assert!(matches!(
        service().parse_user(&jwt).err(),
        Some(AuthError::ExpiredToken)
    ));
}

#[test]
fn token_variants_cannot_authorize_each_other() {
    let auth = service();
    let challenge = auth.issue_email_otp("james@midwess.com").unwrap();
    assert!(matches!(
        auth.parse_user(&challenge.access_token).err(),
        Some(AuthError::WrongTokenVariant)
    ));
    assert!(matches!(
        auth.parse_refresh(&challenge.access_token).err(),
        Some(AuthError::WrongTokenVariant)
    ));
    let issued = auth
        .issue_user_tokens(1, 2, Duration::from_secs(60), Duration::from_secs(600))
        .unwrap();
    assert_eq!(auth.parse_user(&issued.access_token).unwrap().user_id, 1);
    assert_eq!(
        auth.parse_refresh(&issued.refresh_token)
            .unwrap()
            .session_id,
        2
    );
    assert!(matches!(
        auth.parse_email_otp(&issued.access_token).err(),
        Some(AuthError::WrongTokenVariant)
    ));
    assert!(matches!(
        auth.parse_user(&issued.refresh_token).err(),
        Some(AuthError::WrongTokenVariant)
    ));
    assert!(matches!(
        auth.parse_refresh(&issued.access_token).err(),
        Some(AuthError::WrongTokenVariant)
    ));
}

#[test]
fn organization_email_domain_must_match_exactly() {
    for email in [
        "a@evilmidwess.com",
        "a@midwess.com.evil",
        "a@sub.midwess.com",
        "@midwess.com",
        "not an email",
    ] {
        assert!(matches!(
            service().issue_email_otp(email).err(),
            Some(AuthError::InvalidEmail)
        ));
    }
    let issued = service().issue_email_otp("james@midwess.com").unwrap();
    let changed_domain = TokenCodec::new(SECRET.into(), "another.com").unwrap();
    assert!(matches!(
        changed_domain
            .verify_email_otp(&issued.access_token, &issued.code)
            .err(),
        Some(AuthError::InvalidEmail)
    ));
}

#[test]
fn ids_round_trip_as_u64_and_refresh_tokens_have_distinct_ids() {
    let auth = service();
    let one = auth
        .issue_user_tokens(
            9_007_199_254_740_993,
            2,
            Duration::from_secs(60),
            Duration::from_secs(600),
        )
        .unwrap();
    let two = auth
        .issue_user_tokens(
            9_007_199_254_740_993,
            2,
            Duration::from_secs(60),
            Duration::from_secs(600),
        )
        .unwrap();
    let payload = URL_SAFE_NO_PAD
        .decode(one.access_token.split('.').nth(1).unwrap())
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&payload).unwrap();
    assert_eq!(
        json["data"]["user"]["user_id"].as_u64(),
        Some(9_007_199_254_740_993)
    );
    assert_eq!(
        auth.parse_user(&one.access_token).unwrap().user_id,
        9_007_199_254_740_993
    );
    assert_ne!(
        auth.parse_refresh(&one.refresh_token).unwrap().token_id,
        auth.parse_refresh(&two.refresh_token).unwrap().token_id
    );
}

#[test]
fn malformed_tokens_and_invalid_lifetimes_are_rejected() {
    let auth = service();
    for input in ["", "not.jwt", "a.b.c"] {
        assert!(matches!(
            auth.parse_access_token(input).err(),
            Some(AuthError::InvalidToken)
        ));
    }
    assert!(matches!(
        auth.issue_user_tokens(1, 2, Duration::ZERO, Duration::from_secs(60))
            .err(),
        Some(AuthError::InvalidConfiguration)
    ));
    assert!(matches!(
        UserAccessToken::new(
            0,
            2,
            compallm_utils::time::unix_timestamp(),
            Duration::from_secs(60)
        )
        .err(),
        Some(AuthError::InvalidToken)
    ));
}

#[test]
fn encryption_supports_typed_conversion_and_fallback_for_multiple_variants() {
    let cipher = EncryptedAccessToken::new(SECRET).unwrap();
    let user = UserAccessToken::new(
        1,
        2,
        compallm_utils::time::unix_timestamp(),
        Duration::from_secs(60),
    )
    .unwrap();
    let encrypted = cipher.encrypt(AccessToken::User(user)).unwrap();
    let AccessToken::Encrypted(encrypted) = encrypted else {
        panic!("expected encrypted wrapper")
    };
    let decrypted = cipher.decrypt(encrypted).unwrap();
    assert!(matches!(decrypted, AccessToken::User(_)));
    let user = UserAccessToken::try_from(decrypted).unwrap();
    assert_eq!(user.user_id, 1);
    let refresh = RefreshToken::new(
        1,
        2,
        compallm_utils::time::unix_timestamp(),
        Duration::from_secs(600),
    )
    .unwrap();
    let encrypted = cipher.encrypt(AccessToken::Refresh(refresh)).unwrap();
    let AccessToken::Encrypted(encrypted) = encrypted else {
        panic!("expected encrypted wrapper")
    };
    let refresh = RefreshToken::try_from(cipher.decrypt(encrypted).unwrap()).unwrap();
    assert_eq!(refresh.session_id, 2);
}

#[test]
fn plaintext_otp_and_nested_encryption_are_rejected() {
    let claims = EmailOtpAccessToken::new(
        "james@midwess.com".into(),
        compallm_utils::time::unix_timestamp(),
    )
    .unwrap();
    let jwt = create_jwt_token(
        AccessToken::EmailOtp(claims.clone()),
        SECRET,
        Duration::from_secs(600),
    )
    .unwrap();
    assert!(matches!(
        service().parse_access_token(&jwt).err(),
        Some(AuthError::WrongTokenVariant)
    ));
    assert!(matches!(
        service().parse_email_otp(&jwt).err(),
        Some(AuthError::WrongTokenVariant)
    ));
    let cipher = EncryptedAccessToken::new(SECRET).unwrap();
    let encrypted = cipher.encrypt(AccessToken::EmailOtp(claims)).unwrap();
    assert!(matches!(
        cipher.encrypt(encrypted).err(),
        Some(AuthError::InvalidToken)
    ));
}

#[test]
fn injected_secret_controls_both_jwt_and_encryption() {
    assert!(matches!(
        EncryptedAccessToken::new("short").err(),
        Some(AuthError::InvalidConfiguration)
    ));
    let cipher = EncryptedAccessToken::new(SECRET).unwrap();
    let email = EmailOtpAccessToken::new(
        "james@midwess.com".into(),
        compallm_utils::time::unix_timestamp(),
    )
    .unwrap();
    let encrypted = cipher.encrypt(AccessToken::EmailOtp(email)).unwrap();
    let jwt = create_jwt_token(encrypted, SECRET, Duration::from_secs(600)).unwrap();
    let parsed = service().parse_email_otp(&jwt).unwrap();
    assert_eq!(parsed.email, "james@midwess.com");
}

#[test]
fn encrypted_token_errors_preserve_their_source() {
    use std::error::Error;

    let cipher = EncryptedAccessToken::new(SECRET).unwrap();
    let error = cipher
        .decrypt(EncryptedToken("v1.!!!!".into()))
        .err()
        .unwrap();
    assert!(matches!(&error, AuthError::TokenEncoding(_)));
    assert!(error.source().unwrap().is::<base64::DecodeError>());
}
