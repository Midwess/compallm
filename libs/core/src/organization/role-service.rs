use super::{OrganizationMember, Role};
use crate::{
    authorization::all_policies,
    persistent::{
        DbContext,
        role::{RoleRepository, RoleRepositoryError},
    },
};
use cedar_policy::{
    Authorizer, Context, Decision, Entities, Entity, EntityUid, PolicySet, PrincipalConstraint,
    Request, ResourceConstraint, ValidationMode, Validator,
};
use std::collections::BTreeSet;

#[derive(Debug, thiserror::Error)]
pub enum RoleError {
    #[error("role operation is forbidden")]
    Forbidden,
    #[error("role does not exist")]
    NotFound,
    #[error("invalid role definition or policy source")]
    Invalid,
    #[error(transparent)]
    Persistence(#[from] RoleRepositoryError),
}

/// Signed access credentials are verified and current membership reloaded for each call.
/// Current scoped membership is loaded for each operation; supplied policy lists are
/// not trusted actor context. HTTP publication/transport remains separate.
pub struct RoleService<'a> {
    db: &'a DbContext,
    auth: &'a crate::auth::AuthService,
    roles: &'a dyn RoleRepository,
}
impl<'a> RoleService<'a> {
    pub fn new(
        db: &'a DbContext,
        auth: &'a crate::auth::AuthService,
        roles: &'a dyn RoleRepository,
    ) -> Self {
        Self { db, auth, roles }
    }
    async fn member(&self, org: u64, token: &str) -> Result<OrganizationMember, RoleError> {
        let member = self
            .auth
            .current_membership(token)
            .await
            .map_err(|_| RoleError::Forbidden)?;
        if member.organization_id != org {
            return Err(RoleError::Forbidden);
        }
        Ok(member)
    }
    fn authorize(
        member: &OrganizationMember,
        action: &str,
        resource: Entity,
    ) -> Result<(), RoleError> {
        let data = Entities::try_from(member).map_err(|_| RoleError::Forbidden)?;
        let uid = resource.uid();
        let data = if data.get(&uid).is_some() {
            data
        } else {
            data.add_entities([resource], Some(all_policies()))
                .map_err(|_| RoleError::Forbidden)?
        };
        let mut policies = PolicySet::new();
        let mut seen = BTreeSet::new();
        for role in member.effective_roles() {
            if !seen.insert((role.organization_id, role.id)) {
                continue;
            }
            let set = PolicySet::try_from(role).map_err(|_| RoleError::Forbidden)?;
            if !Validator::new(all_policies().clone())
                .validate(&set, ValidationMode::Strict)
                .validation_passed()
            {
                return Err(RoleError::Forbidden);
            }
            for policy in set.policies() {
                policies
                    .add(policy.clone())
                    .map_err(|_| RoleError::Forbidden)?;
            }
        }
        let req = Request::new(
            format!("Compallm::User::\"{}\"", member.user_id)
                .parse()
                .map_err(|_| RoleError::Forbidden)?,
            format!("Compallm::Action::\"{action}\"")
                .parse()
                .map_err(|_| RoleError::Forbidden)?,
            uid,
            Context::empty(),
            Some(all_policies()),
        )
        .map_err(|_| RoleError::Forbidden)?;
        let response = Authorizer::new().is_authorized(&req, &policies, &data);
        if response.decision() != Decision::Allow
            || response.diagnostics().errors().next().is_some()
        {
            return Err(RoleError::Forbidden);
        }
        Ok(())
    }
    fn organization(org: u64) -> Result<Entity, RoleError> {
        let uid: EntityUid = format!("Compallm::Organization::\"{org}\"")
            .parse()
            .map_err(|_| RoleError::Invalid)?;
        Ok(Entity::new_no_attrs(uid, Default::default()))
    }
    fn can_publish(member: &OrganizationMember) -> bool {
        member.is_owner_or_admin()
    }
    fn ceiling(member: &OrganizationMember, role: &Role) -> Result<(), RoleError> {
        if member.effective_roles().any(|r| r.id == Role::OWNER_ID) {
            return Ok(());
        }
        let admin = Role::defaults(role.organization_id).remove(1);
        let allowed = PolicySet::try_from(&admin).map_err(|_| RoleError::Invalid)?;
        let mut actions = std::collections::BTreeSet::new();
        for p in allowed.policies() {
            match p.action_constraint() {
                cedar_policy::ActionConstraint::Eq(uid) => {
                    actions.insert(uid);
                }
                cedar_policy::ActionConstraint::In(ids) => {
                    actions.extend(ids);
                }
                _ => {}
            }
        }
        for p in PolicySet::try_from(role)
            .map_err(|_| RoleError::Invalid)?
            .policies()
        {
            let ids = match p.action_constraint() {
                cedar_policy::ActionConstraint::Eq(uid) => vec![uid],
                cedar_policy::ActionConstraint::In(ids) => ids,
                _ => return Err(RoleError::Forbidden),
            };
            if ids.iter().any(|id| !actions.contains(id)) {
                return Err(RoleError::Forbidden);
            }
            // Admin's default delegation excludes Owner. A general policy cannot
            // express a broader delegation ceiling than this initial contract.
            if ids.iter().any(|id| {
                matches!(
                    id.id().as_ref(),
                    "role.assign"
                        | "role.unassign"
                        | "member.role.assign"
                        | "member.role.unassign"
                        | "group.role.assign"
                        | "group.role.unassign"
                        | "group.member.add"
                        | "group.member.remove"
                )
            }) {
                return Err(RoleError::Forbidden);
            }
        }
        Ok(())
    }
    fn source(role: &Role) -> Result<(), RoleError> {
        if role.cedar_source.len() > 64 * 1024 {
            return Err(RoleError::Invalid);
        }
        let policies = PolicySet::try_from(role).map_err(|_| RoleError::Invalid)?;
        if policies.num_of_policies() > 128
            || !Validator::new(all_policies().clone())
                .validate(&policies, ValidationMode::Strict)
                .validation_passed()
        {
            return Err(RoleError::Invalid);
        }
        let role_uid = Entity::try_from(role)
            .map_err(|_| RoleError::Invalid)?
            .uid();
        let org_uid: EntityUid = format!("Compallm::Organization::\"{}\"", role.organization_id)
            .parse()
            .map_err(|_| RoleError::Invalid)?;
        for policy in policies.policies() {
            if !matches!(policy.principal_constraint(),PrincipalConstraint::In(ref uid) if *uid==role_uid)
            {
                return Err(RoleError::Invalid);
            }
            if !matches!(policy.resource_constraint(),ResourceConstraint::In(ref uid) | ResourceConstraint::IsIn(_,ref uid) if *uid==org_uid)
            {
                return Err(RoleError::Invalid);
            }
            // Reject personal/account actions or unbounded action scopes. Membership
            // and Organization hierarchy alone cannot restrict arbitrary action names.
            match policy.action_constraint() {
                cedar_policy::ActionConstraint::Eq(uid) => {
                    if Self::personal(&uid) {
                        return Err(RoleError::Invalid);
                    }
                }
                cedar_policy::ActionConstraint::In(uids) => {
                    if uids.iter().any(Self::personal) {
                        return Err(RoleError::Invalid);
                    }
                }
                _ => return Err(RoleError::Invalid),
            }
        }
        Ok(())
    }
    fn personal(uid: &EntityUid) -> bool {
        let id: &str = uid.id().as_ref();
        id.starts_with("user.") || id.starts_with("session.")
    }
    pub async fn list(&self, org: u64, token: &str) -> Result<Vec<Role>, RoleError> {
        self.db
            .scope(async {
                let member = self.member(org, token).await?;
                Self::authorize(&member, "role.list", Self::organization(org)?)?;
                let roles = self.roles.list(org).await?;
                for role in &roles {
                    Self::authorize(
                        &member,
                        "role.metadata.read",
                        Entity::try_from(role).map_err(|_| RoleError::Invalid)?,
                    )?;
                    Self::authorize(
                        &member,
                        "role.policy.read",
                        Entity::try_from(role).map_err(|_| RoleError::Invalid)?,
                    )?;
                }
                Ok(roles)
            })
            .await
    }
    pub async fn read(&self, org: u64, token: &str, id: u64) -> Result<Role, RoleError> {
        self.db
            .scope(async {
                let member = self.member(org, token).await?;
                let role = self.roles.find(org, id).await?.ok_or(RoleError::NotFound)?;
                if role.organization_id != org {
                    return Err(RoleError::Forbidden);
                }
                for action in ["role.metadata.read", "role.policy.read"] {
                    Self::authorize(
                        &member,
                        action,
                        Entity::try_from(&role).map_err(|_| RoleError::Invalid)?,
                    )?;
                }
                Ok(role)
            })
            .await
    }
    pub async fn create(&self, token: &str, mut role: Role) -> Result<Role, RoleError> {
        self.db
            .scope(async {
                let member = self.member(role.organization_id, token).await?;
                Self::authorize(
                    &member,
                    "role.create",
                    Self::organization(role.organization_id)?,
                )?;
                if !Self::can_publish(&member) || role.id <= Role::MEMBER_ID {
                    return Err(RoleError::Forbidden);
                }
                role.name = role.name.trim().into();
                role.revision = 1;
                Self::source(&role)?;
                Self::ceiling(&member, &role)?;
                Ok(self.roles.create(role).await?)
            })
            .await
    }
    pub async fn update(&self, token: &str, mut role: Role) -> Result<Role, RoleError> {
        self.db
            .scope(async {
                let member = self.member(role.organization_id, token).await?;
                let previous = self
                    .roles
                    .find(role.organization_id, role.id)
                    .await?
                    .ok_or(RoleError::NotFound)?;
                if previous.id <= Role::MEMBER_ID || role.id <= Role::MEMBER_ID {
                    return Err(RoleError::Forbidden);
                }
                role.name = role.name.trim().into();
                Self::source(&role)?;
                let source_changed = role.cedar_source != previous.cedar_source;
                if source_changed {
                    Self::ceiling(&member, &role)?;
                    Self::authorize(
                        &member,
                        "role.policy.update",
                        Entity::try_from(&previous).map_err(|_| RoleError::Invalid)?,
                    )?;
                    if !Self::can_publish(&member) {
                        return Err(RoleError::Forbidden);
                    }
                }
                if role.name != previous.name || role.description != previous.description {
                    Self::authorize(
                        &member,
                        "role.metadata.update",
                        Entity::try_from(&previous).map_err(|_| RoleError::Invalid)?,
                    )?;
                } else if !source_changed {
                    Self::authorize(
                        &member,
                        "role.metadata.read",
                        Entity::try_from(&previous).map_err(|_| RoleError::Invalid)?,
                    )?;
                }
                Ok(self.roles.update(role).await?)
            })
            .await
    }
}
