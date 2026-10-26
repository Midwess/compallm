use anyhow::Context;
use compallm_core::{
    organization::{Organization, OrganizationStatus},
    persistent::organization::OrganizationRepositoryError,
};

use crate::schema::{OrganizationSchema, OrganizationStatusSchema};

impl TryFrom<OrganizationSchema> for Organization {
    type Error = OrganizationRepositoryError;

    fn try_from(model: OrganizationSchema) -> Result<Self, Self::Error> {
        let id = crate::from_bigint(model.id).context("invalid organization ID")?;
        let created_by =
            crate::from_bigint(model.created_by).context("invalid organization creator ID")?;
        Ok(Self {
            id,
            name: model.name,
            created_by,
            status: match model.status {
                OrganizationStatusSchema::Active => OrganizationStatus::Active,
                OrganizationStatusSchema::Disabled => OrganizationStatus::Disabled,
            },
        })
    }
}
