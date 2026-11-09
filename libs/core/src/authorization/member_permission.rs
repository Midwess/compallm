use crate::auth::UserAccessToken;

use super::PolicySnapshot;

/// A member's effective permission data bound to one organization.
///
/// This value does not authenticate its source or establish that a session is
/// current. Callers must verify the credential and session freshness before
/// relying on its data. It does not itself make an authorization decision.
#[derive(Clone, PartialEq, Eq)]
pub struct MemberPermission {
    user_id: u64,
    organization_id: u64,
    policy_snapshot: PolicySnapshot,
}

impl MemberPermission {
    pub fn user_id(&self) -> u64 {
        self.user_id
    }

    pub fn organization_id(&self) -> u64 {
        self.organization_id
    }

    pub fn policy_snapshot(&self) -> &PolicySnapshot {
        &self.policy_snapshot
    }
}

impl From<UserAccessToken> for MemberPermission {
    fn from(access: UserAccessToken) -> Self {
        Self {
            user_id: access.session.user_id,
            organization_id: access.session.organization_id,
            policy_snapshot: access.policy_snapshot,
        }
    }
}
