use crate::{OrganizationGroupSchema, OrganizationRoleSchema, from_bigint};
use anyhow::Context;
use compallm_core::organization::{Group, Role};

impl TryFrom<OrganizationRoleSchema> for Role {
    type Error = anyhow::Error;
    fn try_from(row: OrganizationRoleSchema) -> Result<Self, Self::Error> {
        let role = Self {
            id: from_bigint(row.id).context("invalid role ID")?,
            organization_id: from_bigint(row.organization_id)
                .context("invalid role organization")?,
            name: row.name,
            description: row.description,
            cedar_source: row.cedar_source,
            org_owner: row.org_owner,
        };
        Ok(role)
    }
}

pub struct MappedGroup(Group);

impl From<MappedGroup> for Group {
    fn from(mapped: MappedGroup) -> Self {
        mapped.0
    }
}

impl TryFrom<(OrganizationGroupSchema, Vec<Role>)> for MappedGroup {
    type Error = anyhow::Error;
    fn try_from((row, roles): (OrganizationGroupSchema, Vec<Role>)) -> Result<Self, Self::Error> {
        let group = Group {
            id: from_bigint(row.id)?,
            organization_id: from_bigint(row.organization_id)?,
            name: row.name,
            description: row.description,
            roles,
        };
        cedar_policy::Entity::try_from(&group)?;
        Ok(Self(group))
    }
}
