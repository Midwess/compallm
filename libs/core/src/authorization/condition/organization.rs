use std::collections::{HashMap, HashSet};
use cedar_policy::Entities;
use super::Resource;
use crate::{authorization::{MemberPermissionError, policy::EntityType}, organization::{Organization, OrganizationMember}};

impl TryFrom<&Organization> for Resource {
    type Error = MemberPermissionError;
    fn try_from(organization: &Organization) -> Result<Self, Self::Error> {
        Self::new(EntityType::Organization, organization.id, organization.id, HashMap::new(), HashSet::new())
    }
}

impl TryFrom<&OrganizationMember> for Resource {
    type Error = MemberPermissionError;
    fn try_from(member: &OrganizationMember) -> Result<Self, Self::Error> {
        let mut resource = Self::new(EntityType::Member, member.organization_id, member.user_id, HashMap::new(), HashSet::new())?;
        for role in member.effective_roles() { Self::try_from(role)?; }
        for group in &member.groups { Self::try_from(group)?; }
        resource.entities = Entities::try_from(member)
            .map_err(|_| MemberPermissionError::Evaluation)?.iter().cloned().collect();
        Ok(resource)
    }
}
