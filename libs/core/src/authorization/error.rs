#[derive(Debug, thiserror::Error)]
pub enum AuthorizationError {
    #[error(transparent)]
    Permission(#[from] super::MemberPermissionError),
    #[error("Cedar error {}", 0)]
    CedarScriptFailed(#[from] anyhow::Error),
    #[error(transparent)]
    Membership(#[from] crate::organization::member::MembershipAssignmentError),
    #[error(transparent)]
    Persistence(#[from] crate::persistent::organization::OrganizationRepositoryError),
    #[error(transparent)]
    Connection(#[from] crate::persistent::ConnectionError),
    #[error(transparent)]
    Auth(#[from] crate::authentication::AuthError),
    #[error(transparent)]
    Group(#[from] crate::organization::group::GroupCreationError),
    #[error(transparent)]
    Role(#[from] crate::organization::role::RoleCreationError),
    #[error(transparent)]
    RoleDeletion(#[from] crate::organization::role::RoleDeletionError),
}
