use anyhow::Context;
use compallm_core::{
    organization::{OrganizationMember, OrganizationRole},
    persistent::organization::OrganizationRepositoryError,
};

use crate::schema::{OrganizationMemberSchema, OrganizationRoleSchema};

impl TryFrom<OrganizationMemberSchema> for OrganizationMember {
    type Error = OrganizationRepositoryError;

    fn try_from(model: OrganizationMemberSchema) -> Result<Self, Self::Error> {
        let organization_id = crate::from_bigint(model.organization_id)
            .context("invalid membership organization ID")?;
        let user_id = crate::from_bigint(model.user_id).context("invalid membership user ID")?;
        if organization_id == 0 || user_id == 0 {
            return Err(OrganizationRepositoryError::database(anyhow::anyhow!(
                "membership IDs must be positive"
            )));
        }
        Ok(Self {
            organization_id,
            user_id,
            role: match model.role {
                OrganizationRoleSchema::Owner => OrganizationRole::Owner,
                OrganizationRoleSchema::Admin => OrganizationRole::Admin,
                OrganizationRoleSchema::Member => OrganizationRole::Member,
            },
        })
    }
}
