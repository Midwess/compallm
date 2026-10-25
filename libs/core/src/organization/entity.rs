#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OrganizationStatus {
    Active,
    Disabled,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
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

impl Organization {
    pub const MAX_NAME_LENGTH: usize = 63;

    pub fn is_valid_name(name: &str) -> bool {
        !name.is_empty()
            && name.len() <= Self::MAX_NAME_LENGTH
            && !compallm_utils::text::has_special_characters(name, "-")
    }
}

#[derive(Clone, Debug)]
pub struct OrganizationMember {
    pub organization_id: u64,
    pub user_id: u64,
    pub role: OrganizationRole,
}
