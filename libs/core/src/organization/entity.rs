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
}

impl Organization {
    pub const MAX_NAME_LENGTH: usize = 63;

    pub fn normalize_name(name: &str) -> Result<String, validify::ValidationErrors> {
        use validify::Validify;
        let mut name = OrganizationName { value: name.into() };
        name.validify()?;
        Ok(name.value)
    }

    fn validate_name_characters(value: &str) -> Result<(), validify::ValidationError> {
        if compallm_utils::text::has_special_characters(value, "-") {
            Err(validify::ValidationError::new_field("organization_name"))
        } else {
            Ok(())
        }
    }
}

#[derive(Clone, Debug)]
pub struct OrganizationMember {
    pub organization_id: u64,
    pub user_id: u64,
    pub role: OrganizationRole,
}

#[derive(validify::Validify)]
struct OrganizationName {
    #[modify(trim)]
    #[validate(length(min = 1, max = 63), custom(function = Organization::validate_name_characters))]
    value: String,
}
