use compallm_validation_derive::ExportValidation;
use juniper::GraphQLInputObject;
use serde::{Deserialize, Serialize};
use validify::Validify;

#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChatCreateInput {
    #[validate(length(min = 1), custom(function = compallm_utils::input::nonblank))]
    pub message: String,
    #[validate(length(min = 1))]
    pub model_id: String,
}
