use crate::schema::OrganizationMemberSchema;
use anyhow::Context;
use compallm_core::{
    organization::{OrganizationMember, Role},
    persistent::organization::OrganizationRepositoryError,
};
pub struct MappedMember(pub OrganizationMember);
impl From<MappedMember> for OrganizationMember {
    fn from(value: MappedMember) -> Self {
        value.0
    }
}
impl TryFrom<(OrganizationMemberSchema, Role)> for MappedMember {
    type Error = OrganizationRepositoryError;
    fn try_from((model, role): (OrganizationMemberSchema, Role)) -> Result<Self, Self::Error> {
        let organization_id = crate::from_bigint(model.organization_id)
            .context("invalid membership organization ID")?;
        let user_id = crate::from_bigint(model.user_id).context("invalid membership user ID")?;
        if role.organization_id != organization_id {
            return Err(OrganizationRepositoryError::Conflict);
        }
        Ok(Self(OrganizationMember {
            organization_id,
            user_id,
            role,
            groups: Vec::new(),
        }))
    }
}
