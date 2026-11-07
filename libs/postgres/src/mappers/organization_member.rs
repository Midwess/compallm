use anyhow::Context;
use compallm_core::{
    organization::OrganizationMember, persistent::organization::OrganizationRepositoryError,
};

use crate::schema::OrganizationMemberSchema;

impl TryFrom<OrganizationMemberSchema> for OrganizationMember {
    type Error = OrganizationRepositoryError;

    fn try_from(model: OrganizationMemberSchema) -> Result<Self, Self::Error> {
        let organization_id = crate::from_bigint(model.organization_id)
            .context("invalid membership organization ID")?;
        let user_id = crate::from_bigint(model.user_id).context("invalid membership user ID")?;
        Ok(Self {
            organization_id,
            user_id,
            roles: Vec::new(),
            groups: Vec::new(),
        })
    }
}
