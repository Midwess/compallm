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
    /// Validate membership relationships and merge effective Cedar sources.
    /// Independent roles use Rayon's shared pool; serialized output stays deterministic.
    pub fn new(member: &OrganizationMember) -> anyhow::Result<Self> {
        let roles = member
            .effective_roles()
            .map(|role| (role.id, role))
            .collect::<BTreeMap<_, _>>();

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
