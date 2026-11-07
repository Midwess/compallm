//! Application-owned Cedar vocabulary. Schema definitions do not grant access.

use std::sync::LazyLock;

use cedar_policy::{Schema, SchemaFragment};

use crate::{
    auth::{Session, UserAuth},
    model::Model,
    organization::{Organization, OrganizationMember},
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

/// Assemble the complete schema once. Initialize before accepting policy writes.
///
/// Duplicate definitions and unresolved references are programmer errors. This
/// function performs no authorization, database access, or token processing.
pub fn schema() -> &'static Schema {
    static SCHEMA: LazyLock<Schema> = LazyLock::new(|| {
        Schema::from_schema_fragments([
            User::policy(),
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
