use crate::authorization::{
    SystemPolicy,
    policy::{EntityType, Policy},
};
use cedar_policy::{
    Entity, EntityUid, PolicySet, RestrictedExpression, SchemaFragment, ValidationMode, Validator,
};
use hero::distributed_id::{generator::DistributedIdError, try_gen_id};
use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};

/// One organization-owned role. Display names never establish authority.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Role {
    pub id: u64,
    pub organization_id: u64,
    pub name: String,
    pub description: String,
    pub cedar_source: String,
    pub org_owner: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum RoleCreationError {
    #[error("role name must contain 1 to 64 characters")]
    InvalidName(#[source] validify::ValidationErrors),
    #[error("invalid role description")]
    InvalidDescription(#[source] validify::ValidationErrors),
    #[error("invalid Cedar policy")]
    InvalidPolicy(#[source] anyhow::Error),
    #[error("ID generation failed")]
    IdGeneration(#[from] DistributedIdError),
}

#[derive(Debug, thiserror::Error)]
pub enum RoleDeletionError {
    #[error("the organization owner role cannot be deleted")]
    OwnerRole,
    #[error("Remove all members and groups assigned to this role before deleting it")]
    Assigned,
}

impl Role {
    pub fn color(&self) -> String {
        use sha2::{Digest, Sha256};
        let hash = Sha256::digest(self.name.as_bytes());
        let value = u16::from_be_bytes([hash[1], hash[2]]);
        let hue = if hash[0] < 160 {
            (300 + value % 140) % 360
        } else {
            value % 360
        };
        let saturation = 65 + u16::from(hash[3]) * 20 / 255;
        let brightness = 200 + u16::from(hash[4]) * 45 / 255;
        let chroma = brightness * saturation / 100;
        let remainder = hue % 120;
        let middle = chroma * (60 - remainder.abs_diff(60)) / 60;
        let base = brightness - chroma;
        let (red, green, blue) = match hue / 60 {
            0 => (chroma, middle, 0),
            1 => (middle, chroma, 0),
            2 => (0, chroma, middle),
            3 => (0, middle, chroma),
            4 => (middle, 0, chroma),
            _ => (chroma, 0, middle),
        };
        format!("#{:02x}{:02x}{:02x}", red + base, green + base, blue + base)
    }

    /// Whether this is the organization's designated Owner role.
    pub fn org_owner(&self) -> bool {
        self.org_owner
    }

    /// Check the role invariant before deleting its persisted record.
    pub fn ensure_deletable(&self) -> Result<(), RoleDeletionError> {
        if self.org_owner() {
            return Err(RoleDeletionError::OwnerRole);
        }
        Ok(())
    }

    /// Validate Cedar syntax and the application schema without authorizing publication.
    pub fn validate_policy(source: &str) -> anyhow::Result<()> {
        use validify::Validify;
        let mut input = RolePolicySource {
            value: source.to_owned(),
        };
        input.validify()?;
        Self::validate_policy_schema(&input.value)
    }

    pub fn prepare_policy_source(
        source: &str,
        organization_id: u64,
        role_id: u64,
    ) -> anyhow::Result<String> {
        Self::validate_policy(source)?;
        let policies =
            crate::authorization::policy::role_policy_set(source, organization_id, role_id)?;
        Self::validate_policy_set(&policies)?;
        Ok(crate::authorization::policy::compact_policy_source(source))
    }

    fn validate_policy_schema(source: &str) -> anyhow::Result<()> {
        Self::validate_policy_set(&source.parse()?)
    }

    fn validate_policy_set(policies: &PolicySet) -> anyhow::Result<()> {
        let result = Validator::new(crate::authorization::all_policies().clone())
            .validate(policies, ValidationMode::Strict);
        anyhow::ensure!(
            result.validation_passed(),
            "Cedar schema validation failed: {:?}",
            result.validation_errors().collect::<Vec<_>>()
        );
        Ok(())
    }
    /// Maximum role name length, in Unicode characters, after trimming.
    pub const MAX_NAME_LENGTH: usize = 64;

    /// Normalize and validate a role name before a management write.
    pub fn normalize_name(name: &str) -> Result<String, validify::ValidationErrors> {
        use validify::Validify;
        let mut name = RoleName { value: name.into() };
        name.validify()?;
        Ok(name.value)
    }

    pub fn validate_description(description: &str) -> Result<(), validify::ValidationErrors> {
        use validify::Validify;
        RoleDescription {
            value: description.to_owned(),
        }
        .validify()
    }

    const OWNER_POLICY: &str = r###"// Application-owned initial Owner template.
permit (
    principal in ?principal,
    action in [Compallm::Action::"organization.settings.read", Compallm::Action::"organization.settings.update", Compallm::Action::"organization.delete", Compallm::Action::"member.read", Compallm::Action::"member.invite", Compallm::Action::"member.remove", Compallm::Action::"provider.read", Compallm::Action::"provider.create", Compallm::Action::"provider.configuration.update", Compallm::Action::"provider.delete", Compallm::Action::"provider.credential.replace", Compallm::Action::"provider.model.read", Compallm::Action::"provider.model.create", Compallm::Action::"provider.model.configuration.update", Compallm::Action::"provider.model.pricing.update", Compallm::Action::"provider.model.delete", Compallm::Action::"model.read", Compallm::Action::"model.create", Compallm::Action::"model.configuration.update", Compallm::Action::"model.delete", Compallm::Action::"model.use", Compallm::Action::"model.pricing.update", Compallm::Action::"role.read", Compallm::Action::"role.create", Compallm::Action::"role.metadata.update", Compallm::Action::"role.policy.read", Compallm::Action::"role.policy.update", Compallm::Action::"role.delete", Compallm::Action::"role.assign", Compallm::Action::"role.unassign", Compallm::Action::"group.read", Compallm::Action::"group.create", Compallm::Action::"group.metadata.update", Compallm::Action::"group.delete"],
    resource in ?resource
);

permit (
    principal in ?principal,
    action in [Compallm::Action::"member.role.assign", Compallm::Action::"member.role.unassign", Compallm::Action::"group.role.assign", Compallm::Action::"group.role.unassign"],
    resource in ?resource
) when { context.role.organization == Compallm::Organization::"__ORG__" };

permit (
    principal in ?principal,
    action in [Compallm::Action::"group.member.add", Compallm::Action::"group.member.remove"],
    resource in ?resource
) when { context.member.organization == Compallm::Organization::"__ORG__" };

permit (
    principal in ?principal,
    action in [Compallm::Action::"model.route.add", Compallm::Action::"model.route.update", Compallm::Action::"model.route.remove"],
    resource in ?resource
) when { context.providerModel.organization == Compallm::Organization::"__ORG__" };

forbid(principal in ?principal, action == Compallm::Action::"model.use",
 resource is Compallm::Model in ?resource) unless { resource.enabled };
"###;
    const ADMIN_POLICY: &str = r###"// Application-owned initial Admin template.
permit (
    principal in ?principal,
    action in [Compallm::Action::"organization.settings.read", Compallm::Action::"organization.settings.update", Compallm::Action::"member.read", Compallm::Action::"member.invite", Compallm::Action::"member.remove", Compallm::Action::"provider.read", Compallm::Action::"provider.create", Compallm::Action::"provider.configuration.update", Compallm::Action::"provider.delete", Compallm::Action::"provider.credential.replace", Compallm::Action::"provider.model.read", Compallm::Action::"provider.model.create", Compallm::Action::"provider.model.configuration.update", Compallm::Action::"provider.model.pricing.update", Compallm::Action::"provider.model.delete", Compallm::Action::"model.read", Compallm::Action::"model.create", Compallm::Action::"model.configuration.update", Compallm::Action::"model.delete", Compallm::Action::"model.use", Compallm::Action::"model.pricing.update", Compallm::Action::"role.read", Compallm::Action::"role.create", Compallm::Action::"role.metadata.update", Compallm::Action::"role.policy.read", Compallm::Action::"role.policy.update", Compallm::Action::"role.delete", Compallm::Action::"group.read", Compallm::Action::"group.create", Compallm::Action::"group.metadata.update", Compallm::Action::"group.delete"],
    resource in ?resource
);

permit (
    principal in ?principal,
    action in [Compallm::Action::"member.role.assign", Compallm::Action::"member.role.unassign", Compallm::Action::"group.role.assign", Compallm::Action::"group.role.unassign"],
    resource in ?resource
) when { context.role.organization == Compallm::Organization::"__ORG__" && context.role != Compallm::Role::"__ORG__:__OWNER__" };

permit (
    principal in ?principal,
    action in [Compallm::Action::"group.member.add", Compallm::Action::"group.member.remove"],
    resource in ?resource
) when { context.member.organization == Compallm::Organization::"__ORG__" }
unless { resource in Compallm::Role::"__ORG__:__OWNER__" };

permit (
    principal in ?principal,
    action in [Compallm::Action::"model.route.add", Compallm::Action::"model.route.update", Compallm::Action::"model.route.remove"],
    resource in ?resource
) when { context.providerModel.organization == Compallm::Organization::"__ORG__" };

permit (
 principal in ?principal,
 action in [Compallm::Action::"role.assign", Compallm::Action::"role.unassign"],
 resource is Compallm::Role in ?resource
) unless { resource == Compallm::Role::"__ORG__:__OWNER__" };

forbid(principal in ?principal, action == Compallm::Action::"model.use",
 resource is Compallm::Model in ?resource) unless { resource.enabled };
"###;
    const MEMBER_POLICY: &str = r###"// Default members discover models and invoke enabled logical models only.
permit(principal in ?principal, action == Compallm::Action::"model.read",
 resource == ?resource);
permit(principal in ?principal, action == Compallm::Action::"model.use",
 resource is Compallm::Model in ?resource) when { resource.enabled };
"###;

    /// Initial role records rendered with application-generated IDs, never reserved identities.
    /// Labels do not establish authority; slots bind to generated role and organization IDs.
    pub async fn defaults(organization_id: u64) -> Result<Vec<Self>, RoleCreationError> {
        let mut roles = vec![
            Self::new(organization_id, "Owner", "", "").await?,
            Self::new(organization_id, "Admin", "", "").await?,
            Self::new(organization_id, "Member", "", "").await?,
        ];
        roles[0].org_owner = true;
        let owner_id = roles[0].id;
        for (role, source) in
            roles
                .iter_mut()
                .zip([Self::OWNER_POLICY, Self::ADMIN_POLICY, Self::MEMBER_POLICY])
        {
            let source = source
                .replace("__ORG__", &organization_id.to_string())
                .replace("__OWNER__", &owner_id.to_string());
            role.cedar_source = Self::prepare_policy_source(&source, organization_id, role.id)
                .map_err(RoleCreationError::InvalidPolicy)?;
        }
        Ok(roles)
    }

    pub async fn new(
        organization_id: u64,
        name: impl Into<String>,
        description: impl Into<String>,
        cedar_source: impl Into<String>,
    ) -> Result<Self, RoleCreationError> {
        let name = Self::normalize_name(&name.into()).map_err(RoleCreationError::InvalidName)?;
        let description = description.into();
        Self::validate_description(&description).map_err(RoleCreationError::InvalidDescription)?;
        let cedar_source =
            crate::authorization::policy::compact_policy_source(&cedar_source.into());
        Self::validate_policy(&cedar_source).map_err(RoleCreationError::InvalidPolicy)?;
        let id = try_gen_id().await?;
        let cedar_source = Self::prepare_policy_source(&cedar_source, organization_id, id)
            .map_err(RoleCreationError::InvalidPolicy)?;
        Ok(Self {
            id,
            organization_id,
            name,
            description,
            cedar_source,
            org_owner: false,
        })
    }
}

#[derive(validify::Validify)]
struct RoleName {
    #[modify(trim)]
    #[validate(length(min = 1, max = 64))]
    value: String,
}

#[derive(validify::Validify)]
struct RoleDescription {
    #[validate(length(max = 128))]
    value: String,
}

#[derive(validify::Validify)]
struct RolePolicySource {
    #[modify(custom(crate::authorization::policy::compact_policy_input))]
    #[validate(length(max = 10192))]
    value: String,
}

impl TryFrom<&Role> for Entity {
    type Error = anyhow::Error;
    fn try_from(role: &Role) -> Result<Self, Self::Error> {
        let uid: EntityUid =
            format!("Compallm::Role::\"{}:{}\"", role.organization_id, role.id).parse()?;
        let org: EntityUid =
            format!("Compallm::Organization::\"{}\"", role.organization_id).parse()?;
        Ok(Entity::new(
            uid,
            HashMap::from([(
                "organization".into(),
                RestrictedExpression::new_entity_uid(org.clone()),
            )]),
            HashSet::from([org]),
        )?)
    }
}

impl TryFrom<&Role> for PolicySet {
    type Error = anyhow::Error;
    fn try_from(role: &Role) -> Result<Self, Self::Error> {
        let parsed = crate::authorization::policy::role_policy_set(
            &role.cedar_source,
            role.organization_id,
            role.id,
        )?;
        use sha2::{Digest, Sha256};
        let mut result = PolicySet::new();
        for policy in parsed.policies() {
            let digest = Sha256::digest(serde_json::to_vec(&policy.to_json()?)?);
            let id = format!(
                "role:{}:{}:statement:{digest:x}",
                role.organization_id, role.id
            )
            .parse()
            .expect("generated policy ID");
            if result.policy(&id).is_none() {
                result.add(policy.new_id(id))?;
            }
        }
        Ok(result)
    }
}

impl SystemPolicy for Role {
    /// Declares protected role metadata, policy source, and delegation operations.
    /// `role.read` enumerates roles and `role.create` creates a custom role in an
    /// Organization. `role.metadata.update` changes safe labels;
    /// `role.policy.read/update` inspect/publish canonical Cedar separately.
    /// `role.delete` removes a role, subject to the management workflow safeguards.
    /// `role.assign/unassign` authorize delegating this Role, in addition to checks
    /// on the target membership/group. All changes require organization-scope
    /// guards. These declarations do not implement publication or delegation checks;
    /// role management and its authorization flow remain separate work.
    fn policy() -> SchemaFragment {
        static DEFINITION: LazyLock<SchemaFragment> =
            LazyLock::new(|| Policy::new(EntityType::Role).into());
        DEFINITION.clone()
    }
}
