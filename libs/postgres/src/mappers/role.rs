use crate::{OrganizationGroupSchema, OrganizationRoleSchema, from_bigint};
use anyhow::Context;
use compallm_core::organization::{Group, Role};
use sea_orm::{EntityTrait, ModelTrait, Set};

#[derive(sea_orm::FromQueryResult)]
pub(crate) struct GroupRoleRow {
    pub group_id: i64,
    #[sea_orm(nested)]
    pub role: OrganizationRoleSchema,
}

impl TryFrom<Role>
    for <<OrganizationRoleSchema as ModelTrait>::Entity as EntityTrait>::ActiveModel
{
    type Error = anyhow::Error;

    fn try_from(role: Role) -> Result<Self, Self::Error> {
        Ok(Self {
            organization_id: Set(crate::to_bigint(role.organization_id)?),
            id: Set(crate::to_bigint(role.id)?),
            name: Set(role.name),
            description: Set(role.description),
            cedar_source: Set(role.cedar_source),
            org_owner: Set(role.org_owner),
        })
    }
}

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
