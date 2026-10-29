use compallm_core::{
    auth::{AccessToken, AuthError, RefreshToken, Session, UserAccessToken, UserTokens},
    organization::OrganizationRole,
};
use std::time::Duration;
const LIFETIME: Duration = Duration::from_secs(30 * 24 * 60 * 60);
const SECRET: &str = "test-signing-secret-at-least-32-bytes";

#[test]
fn switching_changes_scope_and_identifier_and_both_tokens_reference_the_new_session() {
    let initial = RefreshToken::from_session(&Session::new(1, 7, 84), LIFETIME).unwrap();
    let mut session = initial.session.clone();
    let created = session.created_at;
    session.switch_organization(&initial, 85).unwrap();
    assert_eq!(session.id, 7);
    assert_eq!(session.organization_id, 85);
    assert_eq!(session.created_at, created);
    assert_ne!(session.refresh_token_id, initial.session.refresh_token_id);
    let tokens = UserTokens::new(
        &session,
        OrganizationRole::Member,
        SECRET,
        Duration::from_secs(60),
        LIFETIME,
    )
    .unwrap();
    let access =
        UserAccessToken::try_from(AccessToken::decode(&tokens.access_token, SECRET).unwrap())
            .unwrap();
    let refresh =
        RefreshToken::try_from(AccessToken::decode(&tokens.refresh_token, SECRET).unwrap())
            .unwrap();
    assert_eq!(access.session, session);
    assert_eq!(refresh.session, session);
    assert!(matches!(
        session.refresh(&initial, LIFETIME),
        Err(AuthError::InvalidSession)
    ));
    assert!(session.revoked_at.is_none());
}

#[test]
fn renewal_preserves_the_identifier_and_can_continue_beyond_the_original_login_month() {
    let mut token = RefreshToken::from_session(&Session::new(1, 7, 84), LIFETIME).unwrap();
    let now = token.validity.issued_at;
    token.validity.issued_at -= 29 * 24 * 60 * 60;
    token.validity.expires_at -= 29 * 24 * 60 * 60;
    let mut session = token.session.clone();
    session.created_at = now - 60 * 24 * 60 * 60;
    let renewed = session.refresh(&token, LIFETIME).unwrap();
    assert_eq!(
        renewed.session.refresh_token_id,
        token.session.refresh_token_id
    );
    assert_eq!(renewed.session, session);
    assert!(renewed.validity.expires_at > token.validity.expires_at);
    assert!(session.refresh(&token, LIFETIME).is_ok());
    assert!(session.refresh(&renewed, LIFETIME).is_ok());
    assert_eq!(session.created_at, now - 60 * 24 * 60 * 60);
    assert!(session.revoked_at.is_none());
}

#[test]
fn expired_credentials_are_rejected_without_revoking_the_session() {
    let mut token = RefreshToken::from_session(&Session::new(1, 7, 84), LIFETIME).unwrap();
    let session = token.session.clone();
    token.validity.issued_at -= LIFETIME.as_secs() + 1;
    token.validity.expires_at -= LIFETIME.as_secs() + 1;
    assert!(matches!(
        session.refresh(&token, LIFETIME),
        Err(AuthError::ExpiredToken)
    ));
    assert!(session.revoked_at.is_none());
    assert_eq!(session.refresh_token_id, token.session.refresh_token_id);
}

#[test]
fn foreign_or_replaced_credentials_do_not_revoke_a_session() {
    let token = RefreshToken::from_session(&Session::new(1, 7, 84), LIFETIME).unwrap();
    let mut session = token.session.clone();
    for foreign_session in [
        Session {
            user_id: 2,
            ..token.session.clone()
        },
        Session {
            organization_id: 85,
            ..token.session.clone()
        },
        Session {
            id: 8,
            ..token.session.clone()
        },
    ] {
        let foreign = RefreshToken::from_session(&foreign_session, LIFETIME).unwrap();
        assert!(matches!(
            session.refresh(&foreign, LIFETIME),
            Err(AuthError::InvalidSession)
        ));
        assert!(session.revoked_at.is_none());
    }
    session.switch_organization(&token, 84).unwrap();
    assert!(matches!(
        session.refresh(&token, LIFETIME),
        Err(AuthError::InvalidSession)
    ));
    assert!(session.revoked_at.is_none());
}

#[test]
fn revocation_is_idempotent_and_blocks_renewal_switching_and_encoding() {
    let token = RefreshToken::from_session(&Session::new(1, 7, 84), LIFETIME).unwrap();
    let mut session = token.session.clone();
    session.revoke();
    let at = session.revoked_at;
    session.revoke();
    assert_eq!(session.revoked_at, at);
    assert!(matches!(
        session.refresh(&token, LIFETIME),
        Err(AuthError::InvalidSession)
    ));
    assert!(matches!(
        session.switch_organization(&token, 85),
        Err(AuthError::InvalidSession)
    ));
    assert!(matches!(
        RefreshToken::from_session(&session, LIFETIME),
        Err(AuthError::InvalidSession)
    ));
}
