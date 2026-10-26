use compallm_core::{
    auth::{AccessToken, AuthError, RefreshToken, Session, UserAccessToken, UserTokens},
    organization::OrganizationRole,
};
use std::time::Duration;
const LIFETIME: Duration = Duration::from_secs(30 * 24 * 60 * 60);
const SECRET: &str = "test-signing-secret-at-least-32-bytes";

#[test]
fn refresh_changes_the_organization_and_both_tokens_reference_the_updated_session() {
    let initial = RefreshToken::new(1, 7, 84, LIFETIME).unwrap();
    let mut session = Session::new(&initial);
    let created = session.created_at;
    session.refresh(&initial, 85, LIFETIME).unwrap();
    assert_eq!(session.organization_id, 85);
    assert_eq!(session.id, 7);
    assert_eq!(session.created_at, created);
    assert_ne!(session.refresh_token_id, initial.token_id);
    let tokens = UserTokens::new(
        &session,
        OrganizationRole::Member,
        SECRET,
        Duration::from_secs(60),
    )
    .unwrap();
    let user =
        UserAccessToken::try_from(AccessToken::decode(&tokens.access_token, SECRET).unwrap())
            .unwrap();
    let refresh =
        RefreshToken::try_from(AccessToken::decode(&tokens.refresh_token, SECRET).unwrap())
            .unwrap();
    assert_eq!(user.session(), session.reference());
    assert_eq!(refresh.session(), session.reference());
    assert_eq!(refresh.token_id, session.refresh_token_id);
    assert!(matches!(
        session.refresh(&initial, 84, LIFETIME),
        Err(AuthError::InvalidSession)
    ));
    assert!(
        session.revoked_at.is_none(),
        "previous organization cannot mutate the moved session"
    );
}

#[test]
fn revoked_or_expired_database_sessions_cannot_be_refreshed_or_encoded() {
    let token = RefreshToken::new(1, 7, 84, LIFETIME).unwrap();
    for revoked in [false, true] {
        let mut session = Session::new(&token);
        if revoked {
            session.revoked_at = Some(compallm_utils::time::unix_timestamp());
        } else {
            session.expires_at = compallm_utils::time::unix_timestamp();
        }
        assert!(matches!(
            session.refresh(&token, 84, LIFETIME),
            Err(AuthError::InvalidSession)
        ));
        assert!(matches!(
            RefreshToken::from_session(&session),
            Err(AuthError::InvalidSession)
        ));
        assert_eq!(session.refresh_token_id, token.token_id);
    }
}

#[test]
fn same_organization_reuse_revokes_the_session_but_foreign_references_do_not() {
    let token = RefreshToken::new(1, 7, 84, LIFETIME).unwrap();
    let mut session = Session::new(&token);
    for foreign in [
        RefreshToken::new(2, 7, 84, LIFETIME).unwrap(),
        RefreshToken::new(1, 7, 85, LIFETIME).unwrap(),
        RefreshToken::new(1, 8, 84, LIFETIME).unwrap(),
    ] {
        assert!(matches!(
            session.refresh(&foreign, 84, LIFETIME),
            Err(AuthError::InvalidSession)
        ));
        assert!(session.revoked_at.is_none());
    }
    session.refresh(&token, 84, LIFETIME).unwrap();
    assert!(matches!(
        session.refresh(&token, 84, LIFETIME),
        Err(AuthError::RefreshTokenReused)
    ));
    assert!(session.revoked_at.is_some());
}
