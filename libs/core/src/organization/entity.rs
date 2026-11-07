use std::{
    collections::{BTreeMap, HashMap, HashSet},
    sync::LazyLock,
};

use cedar_policy::{Entities, Entity, EntityUid, RestrictedExpression, SchemaFragment};

use super::Role;

use crate::authorization::SystemPolicy;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OrganizationStatus {
    Active,
    Disabled,
}

#[derive(Clone, Debug)]
pub struct Organization {
    pub id: u64,
    pub name: String,
    pub status: OrganizationStatus,
}

impl Organization {
    pub const MAX_NAME_LENGTH: usize = 63;

    pub fn normalize_name(name: &str) -> Result<String, validify::ValidationErrors> {
        use validify::Validify;
        let mut name = OrganizationName { value: name.into() };
        name.validify()?;
        Ok(name.value)
    }

    fn validate_name_characters(value: &str) -> Result<(), validify::ValidationError> {
        if compallm_utils::text::has_special_characters(value, "-") {
            Err(validify::ValidationError::new_field("organization_name"))
        } else {
            Ok(())
        }
    }
}

#[derive(Clone, Debug)]
pub struct OrganizationMember {
    pub organization_id: u64,
    pub user_id: u64,
    pub roles: Vec<super::Role>,
    pub groups: Vec<super::Group>,
}

#[derive(validify::Validify)]
struct OrganizationName {
    #[modify(trim)]
    #[validate(length(min = 1, max = 63), custom(function = Organization::validate_name_characters))]
    value: String,
}

impl SystemPolicy for Organization {
    /// Declares private organization settings and destructive administration.
    ///
    /// - `organization.settings.read`: inspect non-public organization configuration.
    ///   Public profile/name visibility does not grant access to these settings.
    /// - `organization.settings.update`: change permitted organization configuration;
    ///   public profile reading and settings reading grant no mutation authority.
    /// - `organization.delete`: delete the entire organization through its protected
    ///   ownership, confirmation, retention, and concurrent last-owner safeguards.
    ///
    /// Organization profiles are readable by everyone without RBAC; no profile-read
    /// action exists. Public projections omit settings and membership administration.
    /// Protected targets remain in the verified active organization; an ID or a
    /// role held elsewhere is insufficient. Schema declarations do not grant access.
    fn policy() -> SchemaFragment {
        static DEFINITION: LazyLock<SchemaFragment> = LazyLock::new(|| {
            let (fragment, warnings) = SchemaFragment::from_cedarschema_str(
                r#"
namespace Compallm {
    entity Organization;
    action "organization.settings.read" appliesTo { principal: User, resource: Organization };
    action "organization.settings.update" appliesTo { principal: User, resource: Organization };
    action "organization.delete" appliesTo { principal: User, resource: Organization };
}
"#,
            )
            .expect("Organization SystemPolicy must be a valid static Cedar schema fragment");
            assert!(
                warnings.count() == 0,
                "Organization SystemPolicy must not contain schema warnings"
            );
            fragment
        });
        DEFINITION.clone()
    }
}

impl SystemPolicy for OrganizationMember {
    /// Declares membership enumeration, metadata, and administration use cases.
    ///
    /// - `member.list`: enumerate permitted membership summaries in an Organization;
    ///   this requires a membership-listing grant, not merely knowledge of its ID.
    /// - `member.metadata.read`: inspect protected membership state and role assignments, not a
    ///   public user profile. It exposes no authentication or private account data;
    ///   the membership response contract must define its exact projection.
    /// - `member.invite`: invite someone into the destination Organization; invitation
    ///   verification and any initial role assignment require their own checks.
    /// - `member.remove`: remove the targeted membership, never the global user.
    ///   Self-leave and last-owner safeguards belong to the membership workflow.
    /// - `member.role.assign` / `member.role.unassign`: change a specific Member/Role
    ///   relationship using `context.role`. Both must belong to the active organization.
    ///   Delegation ceilings, built-in-role protection, and concurrent removal guards
    ///   are additional checks; permission to edit a member is not permission to escalate.
    ///
    /// Membership administration is separate from unrestricted public profile reading. Role
    /// context is a typed reference, not evidence that the role exists or is assignable.
    fn policy() -> SchemaFragment {
        static DEFINITION: LazyLock<SchemaFragment> = LazyLock::new(|| {
            let (fragment, warnings) = SchemaFragment::from_cedarschema_str(r#"
namespace Compallm {
    entity Member in [Organization] { organization: Organization, user: User };
    action "member.list" appliesTo { principal: User, resource: Organization };
    action "member.invite" appliesTo { principal: User, resource: Organization };
    action "member.metadata.read" appliesTo { principal: User, resource: Member };
    action "member.remove" appliesTo { principal: User, resource: Member };
    action "member.role.assign" appliesTo { principal: User, resource: Member, context: { role: Role } };
    action "member.role.unassign" appliesTo { principal: User, resource: Member, context: { role: Role } };
}
"#).expect("OrganizationMember SystemPolicy must be a valid static Cedar schema fragment");
            assert!(
                warnings.count() == 0,
                "OrganizationMember SystemPolicy must not contain schema warnings"
            );
            fragment
        });
        DEFINITION.clone()
    }
}

impl OrganizationMember {
    pub fn effective_roles(&self) -> impl Iterator<Item = &super::Role> {
        self.roles
            .iter()
            .chain(self.groups.iter().flat_map(|g| g.roles.iter()))
    }
    /// Compatibility label only; never infer privilege from a role display name.
    pub fn compatibility_role(&self) -> String {
        for (id, label) in [
            (super::Role::OWNER_ID, super::Role::OWNER),
            (super::Role::ADMIN_ID, super::Role::ADMIN),
            (super::Role::MEMBER_ID, super::Role::MEMBER),
        ] {
            if self
                .effective_roles()
                .any(|r| r.organization_id == self.organization_id && r.id == id)
            {
                return label.into();
            }
        }
        super::Role::CUSTOM.into()
    }
    pub fn is_owner_or_admin(&self) -> bool {
        self.effective_roles().any(|r| {
            r.organization_id == self.organization_id
                && matches!(r.id, super::Role::OWNER_ID | super::Role::ADMIN_ID)
        })
    }
}

impl TryFrom<&OrganizationMember> for Entities {
    type Error = anyhow::Error;
    fn try_from(member: &OrganizationMember) -> Result<Self, Self::Error> {
        anyhow::ensure!(
            member.organization_id > 0
                && member.organization_id <= i64::MAX as u64
                && member.user_id > 0
                && member.user_id <= i64::MAX as u64,
            "invalid membership identity"
        );
        let org: EntityUid =
            format!("Compallm::Organization::\"{}\"", member.organization_id).parse()?;
        let user: EntityUid = format!("Compallm::User::\"{}\"", member.user_id).parse()?;
        let membership: EntityUid = format!(
            "Compallm::Member::\"{}:{}\"",
            member.organization_id, member.user_id
        )
        .parse()?;
        let mut entities = BTreeMap::new();
        let mut insert = |entity: Entity| -> anyhow::Result<()> {
            if let Some(previous) = entities.insert(entity.uid(), entity.clone()) {
                anyhow::ensure!(previous == entity, "conflicting authorization entity");
            }
            Ok(())
        };
        insert(Entity::new_no_attrs(org.clone(), HashSet::new()))?;
        insert(Entity::new(
            membership,
            HashMap::from([
                (
                    "organization".into(),
                    RestrictedExpression::new_entity_uid(org.clone()),
                ),
                (
                    "user".into(),
                    RestrictedExpression::new_entity_uid(user.clone()),
                ),
            ]),
            HashSet::from([org.clone()]),
        )?)?;
        let mut role_records: BTreeMap<(u64, u64), &Role> = BTreeMap::new();
        for role in member.effective_roles() {
            anyhow::ensure!(
                role.organization_id == member.organization_id,
                "foreign membership role"
            );
            if let Some(previous) = role_records.insert((role.organization_id, role.id), role) {
                anyhow::ensure!(previous == role, "conflicting role source/revision");
            }
            insert(Entity::try_from(role)?)?;
        }
        let mut parents = HashSet::from([org]);
        for role in &member.roles {
            parents.insert(Entity::try_from(role)?.uid());
        }
        for group in &member.groups {
            anyhow::ensure!(
                group.organization_id == member.organization_id,
                "foreign membership group"
            );
            let entity = Entity::try_from(group)?;
            parents.insert(entity.uid());
            insert(entity)?;
        }
        insert(Entity::new_no_attrs(user, parents))?;
        Ok(Entities::from_entities(
            entities.into_values(),
            Some(crate::authorization::all_policies()),
        )?)
    }
}
