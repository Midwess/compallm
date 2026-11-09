//! Application-owned Cedar vocabulary. Schema definitions do not grant access.

pub mod error;
mod member_permission;
pub mod policy_snapshot;
pub mod rbac;
pub use member_permission::MemberPermission;
pub use policy_snapshot::PolicySnapshot;

use std::sync::LazyLock;

use cedar_policy::{Schema, SchemaFragment};

use crate::{
    auth::{Session, UserAuth},
    model::Model,
    organization::{Group, Organization, OrganizationMember, Role},
    provider::{Provider, ProviderModel},
    user::User,
};

/// A resource entity's static Cedar type and action declarations.
///
/// Despite the name, this returns a schema fragment, not permit/forbid policies.
/// Implementations cache their application-owned definitions and panic on invalid
/// static declarations. Untrusted role source must use fallible parsing instead.
pub trait SystemPolicy {
    /// Describe every declared action's use case, target, visibility boundary,
    /// and required additional checks on the implementing method. Schema validity
    /// does not establish ownership, grant authority, or safe response projection.
    fn policy() -> SchemaFragment;
}

/// Combine all SystemPolicy definitions into one cached Cedar schema.
///
/// Returns type/action declarations, not permit/forbid rules. Initialize before
/// accepting policy writes.
///
/// Duplicate definitions and unresolved references are programmer errors. This
/// function performs no authorization, database access, or token processing.
pub fn all_policies() -> &'static Schema {
    static SCHEMA: LazyLock<Schema> = LazyLock::new(|| {
        Schema::from_schema_fragments([
            User::policy(),
            Role::policy(),
            Group::policy(),
            UserAuth::policy(),
            Organization::policy(),
            OrganizationMember::policy(),
            Provider::policy(),
            ProviderModel::policy(),
            Model::policy(),
            Session::policy(),
        ])
        .expect("Core SystemPolicy fragments must form a complete, valid Cedar schema")
    });
    &SCHEMA
}
