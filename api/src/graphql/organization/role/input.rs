#[derive(
    juniper::GraphQLInputObject, validify::Validify, compallm_validation_derive::ExportValidation,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RoleCreateInput {
    #[modify(trim)]
    #[validate(length(min = 1, max = 64))]
    pub name: String,
    #[validate(length(max = 128))]
    pub description: String,
    /// Comments and unnecessary whitespace are removed before the length limit is checked.
    #[modify(custom(compallm_core::authorization::policy_source::compact_policy_input))]
    #[validate(length(max = 10192))]
    pub cedar_source: String,
}

#[derive(
    juniper::GraphQLInputObject, validify::Validify, compallm_validation_derive::ExportValidation,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RoleUpdateInput {
    #[modify(trim)]
    #[validate(length(min = 1, max = 64))]
    pub name: Option<String>,
    #[validate(length(max = 128))]
    pub description: Option<String>,
    /// Comments and unnecessary whitespace are removed before the length limit is checked.
    #[modify(custom(compallm_core::authorization::policy_source::compact_policy_input))]
    #[validate(length(max = 10192))]
    pub cedar_source: Option<String>,
}
