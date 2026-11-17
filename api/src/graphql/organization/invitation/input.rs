#[derive(
    juniper::GraphQLInputObject, validify::Validify, compallm_validation_derive::ExportValidation,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InvitationCreateInput {
    #[modify(trim, custom(str::make_ascii_lowercase))]
    #[validate(custom(function = compallm_utils::email::validate_email, message = "Enter a valid email address, such as name@example.com."))]
    pub email: String,
}

#[derive(
    juniper::GraphQLInputObject, validify::Validify, compallm_validation_derive::ExportValidation,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InvitationBatchCreateInput {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Invite between 1 and 100 people at a time."
    ))]
    #[validify]
    pub invitations: Vec<InvitationCreateInput>,
}
