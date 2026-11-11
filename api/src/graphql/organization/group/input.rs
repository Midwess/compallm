#[derive(juniper::GraphQLInputObject, validify::Validify, compallm_validation_derive::ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GroupCreateInput {
    #[modify(trim)]
    #[validate(length(min = 1, max = 128))]
    pub name: String,
    pub description: String,
    pub role_ids: Vec<String>,
}

