use compallm_validation_derive::ExportValidation;
use juniper::GraphQLInputObject;
use serde::{Deserialize, Serialize};
use validify::Validify;
#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AnalyticUserProfilePutInput {
    #[validate(length(min = 1, max = 64))]
    pub key: String,
    #[validate(length(max = 256))]
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
