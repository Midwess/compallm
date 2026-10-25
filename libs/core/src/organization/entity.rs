#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OrganizationStatus {
    Active,
    Disabled,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OrganizationRole {
    Owner,
    Admin,
    Member,
}

#[derive(Clone, Debug)]
pub struct Organization {
    pub id: u64,
    pub name: String,
    pub status: OrganizationStatus,
    pub created_by: u64,
}

#[derive(Clone, Debug)]
pub struct OrganizationMember {
    pub organization_id: u64,
    pub user_id: u64,
    pub role: OrganizationRole,
}
