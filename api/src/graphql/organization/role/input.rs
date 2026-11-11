#[derive(
    juniper::GraphQLInputObject, validify::Validify, compallm_validation_derive::ExportValidation,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RoleCreateInput {
    #[modify(trim)]
    #[validate(length(min = 1, max = 64))]
    pub name: String,
    pub description: String,
    pub cedar_source: String,
}
