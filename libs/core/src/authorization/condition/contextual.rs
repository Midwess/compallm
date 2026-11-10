use super::Resource;
use crate::{authorization::{MemberPermissionError, policy::AttributeName}, organization::{Group, OrganizationMember, Role}, model::Model, provider::ProviderModel};

impl TryFrom<(&OrganizationMember, &Role)> for Resource {
    type Error = MemberPermissionError;
    fn try_from((member, role): (&OrganizationMember, &Role)) -> Result<Self, Self::Error> {
        Self::try_from(member)?.with_context(AttributeName::Role, Self::try_from(role)?)
    }
}

impl TryFrom<(&Group, &Role)> for Resource {
    type Error = MemberPermissionError;
    fn try_from((group, role): (&Group, &Role)) -> Result<Self, Self::Error> {
        Self::try_from(group)?.with_context(AttributeName::Role, Self::try_from(role)?)
    }
}

impl TryFrom<(&Group, &OrganizationMember)> for Resource {
    type Error = MemberPermissionError;
    fn try_from((group, member): (&Group, &OrganizationMember)) -> Result<Self, Self::Error> {
        Self::try_from(group)?.with_context(AttributeName::Member, Self::try_from(member)?)
    }
}

impl TryFrom<(&Model, &ProviderModel)> for Resource {
    type Error = MemberPermissionError;
    fn try_from((model, provider_model): (&Model, &ProviderModel)) -> Result<Self, Self::Error> {
        Self::try_from(model)?.with_context(AttributeName::ProviderModel, Self::try_from(provider_model)?)
    }
}
