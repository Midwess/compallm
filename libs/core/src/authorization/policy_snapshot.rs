use std::collections::{BTreeMap, BTreeSet};

use cedar_policy::PolicySet;
use serde::{Deserialize, Serialize};

use super::policy_source::compact_policy_source;
use crate::organization::OrganizationMember;

/// Effective Cedar authority for one session's organization membership.
/// Identity and the authorization version belong to Session, not this payload.
/// Construction does not authenticate membership or authorize policy publication.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicySnapshot {
    /// Merged Cedar source for distinct direct and inherited role records.
    #[serde(rename = "c")]
    pub cedar_source: String,
    /// Direct assignments only; inherited assignments remain in `group_roles`.
    #[serde(rename = "r")]
    pub role_ids: Vec<u64>,
    /// Group ID to its directly assigned roles. Empty groups remain represented.
    #[serde(rename = "g")]
    pub group_roles: BTreeMap<u64, Vec<u64>>,
}

impl PolicySnapshot {
    /// Pack effective role sources and relationships from an already validated membership.
    pub fn new(member: &OrganizationMember) -> Self {
        let sources = member
            .effective_roles()
            .map(|role| (role.id, role.cedar_source.as_str()))
            .collect::<BTreeSet<_>>();
        let cedar_source = sources
            .into_iter()
            .map(|(_, source)| compact_policy_source(source))
            .collect::<Vec<_>>()
            .join("\n");
        let role_ids = member
            .roles
            .iter()
            .map(|role| role.id)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let mut groups = BTreeMap::<u64, BTreeSet<u64>>::new();
        for group in &member.groups {
            groups
                .entry(group.id)
                .or_default()
                .extend(group.roles.iter().map(|role| role.id));
        }
        let group_roles = groups
            .into_iter()
            .map(|(id, roles)| (id, roles.into_iter().collect()))
            .collect();
        Self {
            cedar_source,
            role_ids,
            group_roles,
        }
    }
}

impl TryFrom<&PolicySnapshot> for PolicySet {
    type Error = anyhow::Error;

    /// Parse the merged source using native Cedar forbid-overrides semantics.
    /// The caller must bind this payload to its verified Session and relationships.
    fn try_from(snapshot: &PolicySnapshot) -> Result<Self, Self::Error> {
        Ok(snapshot.cedar_source.parse()?)
    }
}

impl From<&OrganizationMember> for PolicySnapshot {
    fn from(member: &OrganizationMember) -> Self {
        Self::new(member)
    }
}
