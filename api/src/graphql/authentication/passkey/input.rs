use super::super::input::{
    AuthenticationPurpose, CredentialDelivery, cookie_delivery, login_purpose,
};
use compallm_validation_derive::ExportValidation;
use juniper::GraphQLInputObject;
use validify::Validify;

#[derive(GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PasskeyRegistrationInput {
    #[modify(trim)]
    #[validate(length(
        min = 1,
        max = 64,
        message = "Enter a passkey name of 1–64 characters."
    ))]
    pub name: String,
}
#[derive(GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[validate(Self::validate_delivery)]
pub struct PasskeyFinishInput {
    #[graphql(default = login_purpose())]
    #[serde(default = "login_purpose")]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub purpose: Option<AuthenticationPurpose>,
    #[validate(length(min = 1, max = 2048))]
    pub token: String,
    #[validate(length(min = 1, max = 2048))]
    pub id: String,
    #[validate(length(min = 1, max = 2048))]
    pub raw_id: String,
    #[validate(length(min = 1, max = 16384))]
    pub client_data_json: String,
    #[validate(length(max = 49152))]
    pub attestation_object: Option<String>,
    #[validate(length(max = 16384))]
    pub authenticator_data: Option<String>,
    #[validate(length(max = 2048))]
    pub signature: Option<String>,
    #[validate(length(max = 256))]
    pub user_handle: Option<String>,
    pub transports: Option<Vec<String>>,
    #[graphql(default = cookie_delivery())]
    #[serde(default = "cookie_delivery")]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub delivery: Option<CredentialDelivery>,
}
impl PasskeyFinishInput {
    fn validate_delivery(value: &Self) -> Result<(), validify::ValidationErrors> {
        super::super::input::require_delivery(value.delivery)?;
        super::super::input::require_purpose(value.purpose)
    }
    fn validate_delivery_contract() -> serde_json::Value {
        serde_json::json!({})
    }
}
impl From<&PasskeyFinishInput> for serde_json::Value {
    fn from(input: &PasskeyFinishInput) -> Self {
        let response = if let Some(attestation) = &input.attestation_object {
            serde_json::json!({"clientDataJSON":input.client_data_json,"attestationObject":attestation,"transports":input.transports.clone().unwrap_or_default()})
        } else {
            serde_json::json!({"clientDataJSON":input.client_data_json,"authenticatorData":input.authenticator_data,"signature":input.signature,"userHandle":input.user_handle})
        };
        serde_json::json!({"id":input.id,"rawId":input.raw_id,"type":"public-key","response":response,"extensions":{}})
    }
}
