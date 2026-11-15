use compallm_core::{
    authentication::{
        AccessToken, AuthError, EmailOtpAccessToken,
        token::{InvitationToken, TokenValidity},
    },
    organization::invitation::Invitation,
};
use std::time::Duration;

#[test]
fn invitation_claims_require_normalized_email_exact_lifetime_and_distinct_authority() {
    let mut claims = InvitationToken {
        invitation_id: 1,
        organization_id: 2,
        email: "member@example.com".into(),
        validity: TokenValidity::new(Duration::from_secs(InvitationToken::TTL_SECONDS)).unwrap(),
    };
    let now = claims.validity.issued_at;
    assert!(claims.validate(now).is_ok());
    assert!(claims.validate(claims.validity.expires_at).is_err());
    assert!(
        compallm_core::authentication::UserAccessToken::try_from(AccessToken::Invitation(
            claims.clone()
        ))
        .is_err()
    );
    assert!(EmailOtpAccessToken::try_from(AccessToken::Invitation(claims.clone())).is_err());
    claims.email = "Member@example.com".into();
    assert!(claims.validate(now).is_err());
    claims.email = "member@example.com".into();
    claims.validity.expires_at -= 1;
    assert!(claims.validate(now).is_err());
}

#[test]
fn invitation_record_binds_recipient_scope_expiry_and_pending_lifecycle() {
    let claims = InvitationToken {
        invitation_id: 1,
        organization_id: 2,
        email: "member@example.com".into(),
        validity: TokenValidity::new(Duration::from_secs(InvitationToken::TTL_SECONDS)).unwrap(),
    };
    let now = claims.validity.issued_at;
    let row = Invitation {
        id: 1,
        organization_id: 2,
        email: claims.email.clone(),
        inviter_id: 3,
        status: "pending".into(),
        expires_at: claims.validity.expires_at,
        accepted_at: None,
        accepted_by: None,
        revoked_at: None,
    };
    assert!(row.validate(&claims, now).is_ok());
    let mut changed = row.clone();
    changed.email = "other@example.com".into();
    assert!(matches!(
        changed.validate(&claims, now),
        Err(AuthError::InvalidInvitation)
    ));
    let mut changed = row.clone();
    changed.organization_id = 4;
    assert!(changed.validate(&claims, now).is_err());
    for state in ["accepted", "revoked"] {
        let mut changed = row.clone();
        changed.status = state.into();
        assert!(changed.validate(&claims, now).is_err());
    }
}
