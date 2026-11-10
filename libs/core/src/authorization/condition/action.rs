use cedar_policy::{Context, EntityId, EntityUid};

use super::CheckPermission;
use crate::authorization::{
    MemberPermission, MemberPermissionError,
    policy::{Action, EntityType},
};

impl CheckPermission for Action {
    fn check_permission(&self, member: &MemberPermission) -> Result<(), MemberPermissionError> {
        if self.resource() != EntityType::Organization || self.context().is_some() {
            return Err(MemberPermissionError::Evaluation);
        }
        let organization = EntityUid::from_type_name_and_id(
            EntityType::Organization.into(),
            EntityId::new(member.organization_id().to_string()),
        );
        member.evaluate(*self, organization, Context::empty(), member.entities())
    }
}
