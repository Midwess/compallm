use std::collections::{BTreeMap, BTreeSet};

use cedar_policy::PolicySet;
use cedar_policy_formatter::lexer::get_token_stream;
use serde::{Deserialize, Serialize};

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
            .map(|(_, source)| Self::compact(source))
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

    /// Remove unnecessary whitespace without altering comments or Cedar tokens.
    fn compact(source: &str) -> String {
        let Some((tokens, _)) = get_token_stream(source) else { return source.to_owned(); };
        let Some(first) = tokens.first() else { return source.to_owned(); };
        let mut result = String::with_capacity(source.len());
        result.push_str(&source[..first.span.start]);
        for (index, token) in tokens.iter().enumerate() {
            if index > 0 {
                let previous = &tokens[index - 1];
                let gap = &source[previous.span.end..token.span.start];
                if gap.chars().any(|character| !character.is_whitespace()) {
                    result.push_str(gap);
                } else {
                    let left = &source[previous.span.clone()];
                    let right = &source[token.span.clone()];
                    let pair = format!("{left}{right}");
                    let unchanged = get_token_stream(&pair).is_some_and(|(pair_tokens, _)| {
                        pair_tokens.len() == 2 && pair_tokens[0].token == previous.token
                            && pair_tokens[1].token == token.token && pair_tokens[0].span.end == left.len()
                    });
                    if !unchanged { result.push(' '); }
                }
            }
            result.push_str(&source[token.span.clone()]);
        }
        result.push_str(&source[tokens.last().expect("nonempty tokens").span.end..]);
        result
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
