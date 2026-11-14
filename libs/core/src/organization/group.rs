use super::Role;
use crate::authorization::{
    SystemPolicy,
    policy::{EntityType, Policy},
};
use cedar_policy::{Entity, EntityUid, RestrictedExpression, SchemaFragment};
use hero::distributed_id::{generator::DistributedIdError, try_gen_id};
use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GroupInfo {
    pub id: u64,
    pub name: String,
    pub description: String,
}

/// Group-assigned roles retain their provenance; groups do not contain groups.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Group {
    pub id: u64,
    pub organization_id: u64,
    pub name: String,
    pub description: String,
    pub roles: Vec<Role>,
}

#[derive(Debug, thiserror::Error)]
pub enum GroupCreationError {
    #[error("group name must contain 1 to 128 characters")]
    InvalidName(#[source] validify::ValidationErrors),
    #[error("group description must contain at most 128 characters")]
    InvalidDescription(#[source] validify::ValidationErrors),
    #[error("group relationship changes must contain distinct non-overlapping identities")]
    InvalidChanges,
    #[error("a group can have at most 32 roles")]
    TooManyRoles,
    #[error("group roles must belong to the same organization")]
    ForeignRole,
    #[error("group roles must be distinct")]
    DuplicateRole,
    #[error("ID generation failed")]
    IdGeneration(#[from] DistributedIdError),
}

#[derive(Clone, Debug, Default)]
pub struct GroupChanges {
    pub name: Option<String>,
    pub description: Option<String>,
    pub add_member_ids: Vec<u64>,
    pub remove_member_ids: Vec<u64>,
    pub add_role_ids: Vec<u64>,
    pub remove_role_ids: Vec<u64>,
}

#[derive(Clone, Debug)]
pub struct MemberInfo {
    pub id: u64,
    pub display_name: String,
    pub email: String,
    pub available: bool,
}

#[derive(Clone, Debug)]
pub struct MemberOption {
    pub member: MemberInfo,
    pub included: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum GroupDeletionError {
    #[error("Remove all members from this group before deleting it")]
    Assigned,
}

impl Group {
    /// Maximum number of roles assigned to a group.
    pub const MAX_ROLES: usize = 32;
    /// Maximum group name length, in Unicode characters, after trimming.
    pub const MAX_NAME_LENGTH: usize = 128;

    /// Normalize and validate a group name before a management write.
    pub fn normalize_name(name: &str) -> Result<String, validify::ValidationErrors> {
        use validify::Validify;
        let mut name = GroupName { value: name.into() };
        name.validify()?;
        Ok(name.value)
    }

    pub fn validate_description(value: &str) -> Result<(), validify::ValidationErrors> {
        use validify::Validify;
        GroupDescription {
            value: value.into(),
        }
        .validify()
    }

    pub async fn new(
        organization_id: u64,
        name: impl Into<String>,
        description: impl Into<String>,
        roles: Vec<Role>,
    ) -> Result<Self, GroupCreationError> {
        if roles.len() > Self::MAX_ROLES {
            return Err(GroupCreationError::TooManyRoles);
        }
        let description = description.into();
        Self::validate_description(&description).map_err(GroupCreationError::InvalidDescription)?;
        let name = Self::normalize_name(&name.into()).map_err(GroupCreationError::InvalidName)?;
        let mut role_ids = HashSet::new();
        for role in &roles {
            if role.organization_id != organization_id {
                return Err(GroupCreationError::ForeignRole);
            }
            if !role_ids.insert(role.id) {
                return Err(GroupCreationError::DuplicateRole);
            }
        }
        Ok(Self {
            id: try_gen_id().await?,
            organization_id,
            name,
            description,
            roles,
        })
    }
}

#[derive(validify::Validify)]
struct GroupName {
    #[modify(trim)]
    #[validate(length(min = 1, max = 128))]
    value: String,
}

#[derive(validify::Validify)]
struct GroupDescription {
    #[validate(length(max = 128))]
    value: String,
}

impl TryFrom<&Group> for Entity {
    type Error = anyhow::Error;
    fn try_from(group: &Group) -> Result<Self, Self::Error> {
        let uid: EntityUid = format!(
            "Compallm::Group::\"{}:{}\"",
            group.organization_id, group.id
        )
        .parse()?;
        let org: EntityUid =
            format!("Compallm::Organization::\"{}\"", group.organization_id).parse()?;
        let mut parents = HashSet::from([org.clone()]);
        for role in &group.roles {
            anyhow::ensure!(
                role.organization_id == group.organization_id,
                "foreign group role"
            );
            parents.insert(Entity::try_from(role)?.uid());
        }
        Ok(Entity::new(
            uid,
            HashMap::from([(
                "organization".into(),
                RestrictedExpression::new_entity_uid(org),
            )]),
            parents,
        )?)
    }
}

impl SystemPolicy for Group {
    /// `group.read/create` target the Organization. `group.metadata.update`
    /// changes group configuration; `group.delete` removes that group's assignments.
    /// `group.member.add/remove` identify the Member in context.member, without
    /// deleting a user; `group.role.assign/unassign` identify context.role and also
    /// require authority to delegate that Role. Both ends must share the actual
    /// organization; name edits cannot grant roles. Runtime writes invalidate sessions.
    fn policy() -> SchemaFragment {
        static DEFINITION: LazyLock<SchemaFragment> =
            LazyLock::new(|| Policy::new(EntityType::Group).into());
        DEFINITION.clone()
    }
}
