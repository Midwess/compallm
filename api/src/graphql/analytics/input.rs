use compallm_validation_derive::ExportValidation;
use juniper::GraphQLInputObject;
use serde::{Deserialize, Serialize};
use validify::Validify;
#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AnalyticUserProfilePutInput {
    #[validate(length(min = 1, max = 64, message = "Enter a key of 1 to 64 characters."))]
    pub key: String,
    #[validate(length(max = 256, message = "Value must be 256 characters or fewer."))]
    pub value: String,
}
impl From<AnalyticUserProfilePutInput> for compallm_core::analytics::UserProfileValue {
    fn from(v: AnalyticUserProfilePutInput) -> Self {
        Self {
            key: v.key,
            value: v.value,
        }
    }
}
