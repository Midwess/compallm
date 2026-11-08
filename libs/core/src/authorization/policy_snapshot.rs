use std::collections::{BTreeMap, BTreeSet};

use cedar_policy::{Entities, Policy, PolicySet, ValidationMode, Validator};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use super::all_policies;
use crate::organization::OrganizationMember;

/// Effective Cedar authority for one session's organization membership.
/// Identity and the authorization version belong to Session, not this payload.
/// Construction does not authenticate membership or authorize policy publication.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicySnapshot {
    /// Merged Cedar source, deduplicated across direct and inherited roles.
    pub cedar_source: String,
    /// Direct assignments only; inherited assignments remain in `group_roles`.
    pub role_ids: Vec<u64>,
    /// Group ID to its directly assigned roles. Empty groups remain represented.
    pub group_roles: BTreeMap<u64, Vec<u64>>,
}

impl PolicySnapshot {
    pub const MAX_ROLES: usize = 256;
    pub const MAX_GROUPS: usize = 256;
    pub const MAX_ASSIGNMENTS: usize = 1024;
    pub const MAX_SOURCE_BYTES: usize = 256 * 1024;
    pub const MAX_STATEMENTS: usize = 1024;
    pub const MAX_JSON_BYTES: usize = 16 * 1024;

    /// Validate membership relationships and merge effective Cedar sources.
    /// Independent roles use Rayon's shared pool; serialized output stays deterministic.
    pub fn new(member: &OrganizationMember) -> anyhow::Result<Self> {
        anyhow::ensure!(
            member.groups.len() <= Self::MAX_GROUPS,
            "snapshot group limit exceeded"
        );
        let assignments = member
            .roles
            .len()
            .saturating_add(member.groups.len())
            .saturating_add(
                member
                    .groups
                    .iter()
                    .map(|group| group.roles.len())
                    .sum::<usize>(),
            );
        anyhow::ensure!(
            assignments <= Self::MAX_ASSIGNMENTS,
            "snapshot assignment limit exceeded"
        );
        let roles = member
            .effective_roles()
            .map(|role| (role.id, role))
            .collect::<BTreeMap<_, _>>();
        anyhow::ensure!(
            roles.len() <= Self::MAX_ROLES,
            "snapshot role limit exceeded"
        );
        let source_bytes = roles
            .values()
            .try_fold(0usize, |total, role| {
                total.checked_add(role.cedar_source.len())
            })
            .ok_or_else(|| anyhow::anyhow!("snapshot source limit exceeded"))?;
        anyhow::ensure!(
            source_bytes <= Self::MAX_SOURCE_BYTES,
            "snapshot source limit exceeded"
        );

        // Reuse entity construction's organization and conflicting-record guards.
        Entities::try_from(member)?;
        let statements = roles
            .par_iter()
            .map(|(_, role)| -> anyhow::Result<_> {
                let policies = PolicySet::try_from(*role)?;
                let validation = Validator::new(all_policies().clone())
                    .validate(&policies, ValidationMode::Strict);
                anyhow::ensure!(
                    validation.validation_passed(),
                    "role {} fails Cedar schema validation: {:?}",
                    role.id,
                    validation.validation_errors().collect::<Vec<_>>()
                );
                let statements = policies
                    .policies()
                    .map(|policy| {
                        let structure = policy.to_json()?;
                        let rendered = Policy::from_json(None, structure.clone())?.to_string();
                        Ok((serde_json::to_string(&structure)?, rendered))
                    })
                    .collect::<anyhow::Result<BTreeMap<_, _>>>()?;
                Ok(statements)
            })
            .collect::<anyhow::Result<Vec<_>>>()?;
        let statements = statements.into_iter().flatten().collect::<BTreeMap<_, _>>();
        anyhow::ensure!(
            statements.len() <= Self::MAX_STATEMENTS,
            "snapshot statement limit exceeded"
        );

        let cedar_source = statements.into_values().collect::<Vec<_>>().join("\n");
        let role_ids = member
            .roles
            .iter()
            .map(|role| role.id)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let group_roles = member
            .groups
            .iter()
            .map(|group| {
                (
                    group.id,
                    group
                        .roles
                        .iter()
                        .map(|role| role.id)
                        .collect::<BTreeSet<_>>()
                        .into_iter()
                        .collect(),
                )
            })
            .collect();
        let snapshot = Self {
            cedar_source,
            role_ids,
            group_roles,
        };
        anyhow::ensure!(
            serde_json::to_vec(&snapshot)?.len() <= Self::MAX_JSON_BYTES,
            "snapshot JSON limit exceeded"
        );

        // Validate the complete script as the same policy set consumers will load.
        PolicySet::try_from(&snapshot)?;
        Ok(snapshot)
    }
}

impl TryFrom<&PolicySnapshot> for PolicySet {
    type Error = anyhow::Error;

    /// Parse the merged source using native Cedar forbid-overrides semantics.
    /// The caller must bind this payload to its verified Session and relationships.
    fn try_from(snapshot: &PolicySnapshot) -> Result<Self, Self::Error> {
        let result: PolicySet = snapshot.cedar_source.parse()?;
        anyhow::ensure!(
            result.templates().next().is_none(),
            "unlinked snapshot templates are unsupported"
        );
        let validation =
            Validator::new(all_policies().clone()).validate(&result, ValidationMode::Strict);
        anyhow::ensure!(
            validation.validation_passed(),
            "snapshot fails Cedar schema validation"
        );
        Ok(result)
    }
}

impl TryFrom<&OrganizationMember> for PolicySnapshot {
    type Error = anyhow::Error;

    fn try_from(member: &OrganizationMember) -> Result<Self, Self::Error> {
        Self::new(member)
    }
}
