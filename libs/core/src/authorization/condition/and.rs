use super::CheckPermission;
use crate::authorization::{MemberPermission, MemberPermissionError};

pub struct And<L, R> {
    left: L,
    right: R,
}

impl<L, R> And<L, R> {
    pub(super) fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

impl<L: CheckPermission, R: CheckPermission> CheckPermission for And<L, R> {
    fn check_permission(&self, member: &MemberPermission) -> Result<(), MemberPermissionError> {
        self.left.check_permission(member)?;
        self.right.check_permission(member)
    }
}
