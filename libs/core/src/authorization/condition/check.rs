use super::{And, Resource, ResourceCondition};
use crate::authorization::{MemberPermission, MemberPermissionError, policy::Action};

pub trait CheckPermission {
    fn check_permission(&self, member: &MemberPermission) -> Result<(), MemberPermissionError>;

    fn and<C: CheckPermission>(self, other: C) -> And<Self, C>
    where
        Self: Sized,
    {
        And::new(self, other)
    }
}

pub trait ForAction: Sized {
    fn for_action(self, action: Action) -> ResourceCondition<Self>;
}

impl<T: Copy> ForAction for T
where
    Resource: TryFrom<T, Error = MemberPermissionError>,
{
    fn for_action(self, action: Action) -> ResourceCondition<Self> {
        ResourceCondition::new(self, action)
    }
}

impl<C: CheckPermission + ?Sized> CheckPermission for &C {
    fn check_permission(&self, member: &MemberPermission) -> Result<(), MemberPermissionError> {
        C::check_permission(*self, member)
    }
}
