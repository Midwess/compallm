#[derive(
    juniper::GraphQLInputObject, validify::Validify, compallm_validation_derive::ExportValidation,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GroupCreateInput {
    #[modify(trim)]
    #[validate(length(
        min = 1,
        max = 128,
        message = "Group name must contain 1 to 128 characters."
    ))]
    pub name: String,
    #[validate(length(max = 128, message = "Description must be 128 characters or fewer."))]
    pub description: String,
    #[validate(length(max = 32, message = "Choose no more than 32 roles."))]
    pub role_ids: Vec<String>,
}

#[derive(
    juniper::GraphQLInputObject, validify::Validify, compallm_validation_derive::ExportValidation,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GroupUpdateInput {
    #[modify(trim)]
    #[validate(
        length(min = 1, message = "Name is required."),
        length(max = 128, message = "Name must be 128 characters or fewer.")
    )]
    pub name: Option<String>,
    #[validate(length(max = 128, message = "Description must be 128 characters or fewer."))]
    pub description: Option<String>,
    #[validate(length(max = 100, message = "Add no more than 100 members at a time."))]
    pub add_member_ids: Option<Vec<String>>,
    #[validate(length(max = 100, message = "Remove no more than 100 members at a time."))]
    pub remove_member_ids: Option<Vec<String>>,
    #[validate(length(max = 32, message = "Add no more than 32 roles at a time."))]
    pub add_role_ids: Option<Vec<String>>,
    #[validate(length(max = 32, message = "Remove no more than 32 roles at a time."))]
    pub remove_role_ids: Option<Vec<String>>,
}
