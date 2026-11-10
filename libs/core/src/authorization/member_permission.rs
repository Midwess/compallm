use std::collections::{BTreeSet, HashMap, HashSet};

use cedar_policy::{
    Authorizer, Context, Decision, Entities, Entity, EntityUid, PolicySet, Request,
    RestrictedExpression, ValidationMode, Validator,
};

use crate::authentication::UserAccessToken;

use super::{PolicySnapshot, all_policies, policy::Action};

/// A safe result for preparing or evaluating a member's permissions.
#[derive(Debug, thiserror::Error)]
pub enum MemberPermissionError {
    #[error("member permission data cannot be prepared")]
    InvalidAuthority,
    #[error("authorization request cannot be evaluated")]
    Evaluation,
    #[error("action is not permitted")]
    Denied,
}

/// Prepared Cedar authority for a single member and organization.
///
/// This value does not authenticate its source or establish that a session is
/// current. A trusted boundary must verify credentials and session freshness
/// before a protected service relies on it.
#[derive(Clone)]
pub struct MemberPermission {
    user_id: u64,
    organization_id: u64,
    principal: EntityUid,
    policies: PolicySet,
    entities: Entities,
}

impl MemberPermission {
    pub fn user_id(&self) -> u64 {
        self.user_id
    }

    pub fn organization_id(&self) -> u64 {
        self.organization_id
    }

    pub fn principal(&self) -> &EntityUid {
        &self.principal
    }

    pub fn policies(&self) -> &PolicySet {
        &self.policies
    }

    pub fn entities(&self) -> &Entities {
        &self.entities
    }

    pub(super) fn evaluate(
        &self,
        action: Action,
        resource: EntityUid,
        context: Context,
        entities: &Entities,
    ) -> Result<(), MemberPermissionError> {
        let request = Request::new(
            self.principal.clone(),
            action.into(),
            resource,
            context,
            Some(all_policies()),
        )
        .map_err(|_| MemberPermissionError::Evaluation)?;
        let response = Authorizer::new().is_authorized(&request, &self.policies, entities);
        // Cedar skips individual policies on evaluation error. A skipped forbid
        // must not allow an unrelated permit to authorize this application.
        if response.diagnostics().errors().next().is_some() {
            return Err(MemberPermissionError::Evaluation);
        }
        match response.decision() {
            Decision::Allow => Ok(()),
            Decision::Deny => Err(MemberPermissionError::Denied),
        }
    }
}

impl TryFrom<UserAccessToken> for MemberPermission {
    type Error = MemberPermissionError;

    fn try_from(access: UserAccessToken) -> Result<Self, Self::Error> {
        let user_id = access.session.user_id;
        let organization_id = access.session.organization_id;
        let snapshot = access.policy_snapshot;
        let valid_id = |id: u64| id > 0 && id <= i64::MAX as u64;
        if !valid_id(user_id)
            || !valid_id(organization_id)
            || snapshot.role_ids.iter().any(|id| !valid_id(*id))
            || snapshot.group_roles.iter().any(|(group, roles)| {
                !valid_id(*group) || roles.iter().any(|role| !valid_id(*role))
            })
        {
            return Err(MemberPermissionError::InvalidAuthority);
        }
        let policies =
            PolicySet::try_from(&snapshot).map_err(|_| MemberPermissionError::InvalidAuthority)?;
        if policies.templates().next().is_some()
            || !Validator::new(all_policies().clone())
                .validate(&policies, ValidationMode::Strict)
                .validation_passed()
        {
            return Err(MemberPermissionError::InvalidAuthority);
        }
        let principal = format!("Compallm::User::\"{user_id}\"")
            .parse()
            .map_err(|_| MemberPermissionError::InvalidAuthority)?;
        let entities = MemberEntities::try_from((user_id, organization_id, &snapshot))?.0;
        Ok(Self {
            user_id,
            organization_id,
            principal,
            policies,
            entities,
        })
    }
}

/// Local conversion wrapper: the source snapshot is a compact projection rather
/// than a complete OrganizationMember with full Role and Group records.
struct MemberEntities(Entities);

impl TryFrom<(u64, u64, &PolicySnapshot)> for MemberEntities {
    type Error = MemberPermissionError;

    fn try_from(
        (user_id, organization_id, snapshot): (u64, u64, &PolicySnapshot),
    ) -> Result<Self, Self::Error> {
        let org: EntityUid = format!("Compallm::Organization::\"{organization_id}\"")
            .parse()
            .map_err(|_| MemberPermissionError::InvalidAuthority)?;
        let user: EntityUid = format!("Compallm::User::\"{user_id}\"")
            .parse()
            .map_err(|_| MemberPermissionError::InvalidAuthority)?;
        let mut entities = vec![Entity::new_no_attrs(org.clone(), HashSet::new())];
        let roles: BTreeSet<_> = snapshot
            .role_ids
            .iter()
            .chain(snapshot.group_roles.values().flatten())
            .copied()
            .collect();
        for role in roles {
            let uid: EntityUid = format!("Compallm::Role::\"{organization_id}:{role}\"")
                .parse()
                .map_err(|_| MemberPermissionError::InvalidAuthority)?;
            entities.push(
                Entity::new(
                    uid,
                    HashMap::from([(
                        "organization".into(),
                        RestrictedExpression::new_entity_uid(org.clone()),
                    )]),
                    HashSet::from([org.clone()]),
                )
                .map_err(|_| MemberPermissionError::InvalidAuthority)?,
            );
        }
        let mut parents = HashSet::from([org.clone()]);
        for role in &snapshot.role_ids {
            parents.insert(
                format!("Compallm::Role::\"{organization_id}:{role}\"")
                    .parse()
                    .map_err(|_| MemberPermissionError::InvalidAuthority)?,
            );
        }
        for (group, roles) in &snapshot.group_roles {
            let uid: EntityUid = format!("Compallm::Group::\"{organization_id}:{group}\"")
                .parse()
                .map_err(|_| MemberPermissionError::InvalidAuthority)?;
            let mut group_parents = HashSet::from([org.clone()]);
            for role in roles {
                group_parents.insert(
                    format!("Compallm::Role::\"{organization_id}:{role}\"")
                        .parse()
                        .map_err(|_| MemberPermissionError::InvalidAuthority)?,
                );
            }
            entities.push(
                Entity::new(
                    uid.clone(),
                    HashMap::from([(
                        "organization".into(),
                        RestrictedExpression::new_entity_uid(org.clone()),
                    )]),
                    group_parents,
                )
                .map_err(|_| MemberPermissionError::InvalidAuthority)?,
            );
            parents.insert(uid);
        }
        entities.push(Entity::new_no_attrs(user, parents));
        Ok(Self(
            Entities::from_entities(entities, Some(all_policies()))
                .map_err(|_| MemberPermissionError::InvalidAuthority)?,
        ))
    }
}
