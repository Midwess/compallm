use compallm_core::authentication::Session;

/// A previously stored record with stable identity; not a new-session constructor.
pub fn stored_session(
    user_id: u64,
    id: u64,
    organization_id: u64,
    name: impl Into<String>,
) -> Session {
    Session {
        id,
        user_id,
        organization_id,
        organization_name: name.into(),
        authorization_version: 1,
        revoked_at: None,
    }
}
