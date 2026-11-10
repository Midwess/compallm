use super::CheckPermission;
use crate::authorization::{MemberPermission, MemberPermissionError, policy::Action};

impl CheckPermission for Action {
    fn check_permission(&self, member: &MemberPermission) -> Result<(), MemberPermissionError> {
        member.require(*self)
    }
}
