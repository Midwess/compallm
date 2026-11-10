//! Authentication metadata and management have a separate protected resource.

use std::sync::LazyLock;

use cedar_policy::SchemaFragment;

use crate::authorization::{
    SystemPolicy,
    policy::{EntityType, Policy},
};

/// Schema definition for a user's private authentication settings, not a stored key.
///
/// Cedar UserAuthentication instances reference their owning User. The principal
/// remains User; this definition never changes authentication or loads credentials.
pub struct UserAuth;

impl SystemPolicy for UserAuth {
    /// Declares private authentication access separately from public profiles.
    ///
    /// - `user.authentication.read`: review one's own authentication-method/status
    ///   metadata, such as registered-method summaries. Never return passwords,
    ///   OTP seeds/codes, recovery codes, private keys, or bearer/refresh tokens.
    /// - `user.authentication.update`: change one's own registered authentication
    ///   methods through the verified registration/removal/recovery workflow.
    ///   This is distinct from editing a display name, role assignment, or reading
    ///   metadata; ownership, reauthentication, and recovery safeguards still apply.
    ///
    /// Both target UserAuthentication, whose `user` reference must match the actor.
    /// Public profile visibility and knowledge of a user's ID grant neither action.
    /// These are account capabilities, not organization-administrator shortcuts;
    /// baseline-path isolation and mutation verification belong to runtime services.
    fn policy() -> SchemaFragment {
        static DEFINITION: LazyLock<SchemaFragment> =
            LazyLock::new(|| Policy::new(EntityType::UserAuthentication).into());
        DEFINITION.clone()
    }
}
