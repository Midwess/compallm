#[derive(Default, schemars::JsonSchema, juniper::GraphQLInputObject, compallm_query::IntoQuery)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProviderTemplateFilterInput {
    pub id: Option<compallm_query::filter::StringFilter>,
    pub name: Option<compallm_query::filter::StringFilter>,
    pub or: Option<Vec<ProviderTemplateMatchInput>>,
}
#[derive(schemars::JsonSchema, juniper::GraphQLInputObject)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProviderTemplateOrderInput {
    pub name: Option<compallm_query::OrderByDirection>,
    pub id: Option<compallm_query::OrderByDirection>,
}
impl TryFrom<ProviderTemplateOrderInput> for compallm_query::OrderBy {
    type Error = compallm_query::order::OrderByError;
    fn try_from(value: ProviderTemplateOrderInput) -> Result<Self, Self::Error> {
        let mut fields = [("name", value.name), ("id", value.id)]
            .into_iter()
            .filter_map(|(field, direction)| {
                direction.map(|direction| Self {
                    field: field.into(),
                    direction,
                })
            });
        let result = fields.next().ok_or(Self::Error::ExactlyOneField)?;
        if fields.next().is_some() {
            return Err(Self::Error::ExactlyOneField);
        }
        Ok(result)
    }
}

#[derive(Default, schemars::JsonSchema, juniper::GraphQLInputObject, compallm_query::IntoQuery)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ModelTemplateFilterInput {
    pub id: Option<compallm_query::filter::StringFilter>,
    pub name: Option<compallm_query::filter::StringFilter>,
    pub display_name: Option<compallm_query::filter::StringFilter>,
    pub or: Option<Vec<ModelTemplateMatchInput>>,
}

#[derive(schemars::JsonSchema, juniper::GraphQLInputObject, compallm_query::IntoQuery)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProviderTemplateMatchInput {
    pub id: Option<compallm_query::filter::StringFilter>,
    pub name: Option<compallm_query::filter::StringFilter>,
}
#[derive(schemars::JsonSchema, juniper::GraphQLInputObject, compallm_query::IntoQuery)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ModelTemplateMatchInput {
    pub id: Option<compallm_query::filter::StringFilter>,
    pub name: Option<compallm_query::filter::StringFilter>,
    pub display_name: Option<compallm_query::filter::StringFilter>,
}

#[derive(
    Clone,
    serde::Serialize,
    serde::Deserialize,
    juniper::GraphQLInputObject,
    validify::Validify,
    compallm_validation_derive::ExportValidation,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[validate(validate_template_limits)]
pub struct ModelTemplateLimitsInput {
    #[validate(custom(function=editable_tokens,message="Enter between 100 and 10,000,000 tokens."))]
    pub context: Option<super::super::scalar::ContextTokens>,
    #[validate(custom(function=editable_tokens,message="Enter between 100 and 10,000,000 tokens."))]
    pub input: Option<super::super::scalar::ContextTokens>,
    #[validate(custom(function=editable_tokens,message="Enter between 100 and 10,000,000 tokens."))]
    pub output: Option<super::super::scalar::ContextTokens>,
}
fn validate_template_limits(
    value: &ModelTemplateLimitsInput,
) -> Result<(), validify::ValidationErrors> {
    let mut errors = validify::ValidationErrors::new();
    if let Some(context) = value.context.as_ref().map(|v| v.get()) {
        for (field, cap, message) in [
            (
                "input",
                value.input.as_ref().map(|v| v.get()),
                "Input limit cannot exceed the context window.",
            ),
            (
                "output",
                value.output.as_ref().map(|v| v.get()),
                "Output limit cannot exceed the context window.",
            ),
        ] {
            if cap.is_some_and(|cap| cap > context) {
                let mut error = validify::ValidationError::new_field("context_limit")
                    .with_message(message.into());
                error.set_field(field);
                error.append_location(field);
                errors.add(error);
            }
        }
    }
    if errors.errors().is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
fn validate_template_limits_contract() -> serde_json::Value {
    serde_json::json!({"x-compallm-object-rule":"templateLimits","x-compallm-messages":{"input":"Input limit cannot exceed the context window.","output":"Output limit cannot exceed the context window."}})
}

#[derive(
    serde::Serialize,
    serde::Deserialize,
    juniper::GraphQLInputObject,
    validify::Validify,
    compallm_validation_derive::ExportValidation,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[validate(validate_template_price)]
pub struct ModelTemplatePriceInput {
    pub enabled: bool,
    #[validate(custom(function=super::super::input::model_price, message="Enter a price between $0 and $10,000 per 1M tokens."))]
    pub value: Option<String>,
}

#[derive(
    serde::Serialize,
    serde::Deserialize,
    juniper::GraphQLInputObject,
    validify::Validify,
    compallm_validation_derive::ExportValidation,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ModelTemplateCustomizeInput {
    #[modify(trim)]
    #[validate(length(min=1,max=512,message="Model identifier must contain 1 to 512 characters."),custom(function=compallm_utils::input::upstream_model_id,message="Use the exact upstream model identifier without spaces or control characters."))]
    pub name: Option<String>,
    pub protocol: Option<super::super::input::TextModelProtocol>,
    #[validate(custom(function=compallm_utils::input::endpoint_path,message="Enter a valid endpoint path beginning with /."),length(max=40,message="Path must be 40 characters or fewer."))]
    pub path: Option<String>,
    #[validify]
    pub limits: Option<ModelTemplateLimitsInput>,
    #[validify]
    pub input_price: Option<ModelTemplatePriceInput>,
    #[validify]
    pub output_price: Option<ModelTemplatePriceInput>,
    #[validify]
    pub cached_price: Option<ModelTemplatePriceInput>,
    #[validate(custom(function=template_capabilities,message="Text models require Text; forced tool choice also requires tool calling."))]
    pub capabilities: Option<Vec<super::super::input::ProviderModelCapability>>,
    #[validify]
    pub effort_mapping: Option<super::super::input::ProviderEffortMapping>,
    #[validify]
    #[validate(custom(function=super::super::input::header_entries,message="Header names must be unique, ignoring case."))]
    pub headers: Option<Vec<super::super::input::ProviderHeader>>,
    #[validify]
    #[validate(custom(function=super::super::input::body_entries,message="Body field names must be unique."))]
    pub body_defaults: Option<Vec<super::super::input::ProviderBodyDefault>>,
}

fn editable_tokens(
    value: &super::super::scalar::ContextTokens,
) -> Result<(), validify::ValidationError> {
    if (100..=10_000_000).contains(&value.get()) {
        Ok(())
    } else {
        Err(validify::ValidationError::new_field("editable_tokens"))
    }
}
fn editable_tokens_contract() -> serde_json::Value {
    serde_json::json!({"minimum":100,"maximum":10000000})
}

fn validate_template_price(
    value: &ModelTemplatePriceInput,
) -> Result<(), validify::ValidationErrors> {
    if value.enabled && value.value.is_none() {
        let mut errors = validify::ValidationErrors::new();
        let mut error = validify::ValidationError::new_field_named("value", "required_price")
            .with_message("Enter a custom price.".into());
        error.append_location("value");
        errors.add(error);
        Err(errors)
    } else {
        Ok(())
    }
}
fn validate_template_price_contract() -> serde_json::Value {
    serde_json::json!({"x-compallm-object-rule":"templatePrice","x-compallm-messages":{"x-compallm-object-rule":"Enter a custom price."}})
}

#[derive(
    Clone,
    serde::Serialize,
    serde::Deserialize,
    juniper::GraphQLInputObject,
    validify::Validify,
    compallm_validation_derive::ExportValidation,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[validate(validate_effective_limits)]
pub struct ModelTemplateEffectiveLimitsInput {
    pub context: Option<super::super::scalar::ContextTokens>,
    pub input: Option<super::super::scalar::ContextTokens>,
    pub output: Option<super::super::scalar::ContextTokens>,
}
fn validate_effective_limits(
    v: &ModelTemplateEffectiveLimitsInput,
) -> Result<(), validify::ValidationErrors> {
    validate_template_limits(&ModelTemplateLimitsInput {
        context: v.context.clone(),
        input: v.input.clone(),
        output: v.output.clone(),
    })
}
fn validate_effective_limits_contract() -> serde_json::Value {
    validate_template_limits_contract()
}

fn template_capabilities(
    values: &[super::super::input::ProviderModelCapability],
) -> Result<(), validify::ValidationError> {
    use super::super::input::ProviderModelCapability as C;
    if values.contains(&C::Text)
        && (!values.contains(&C::ForcedToolChoice) || values.contains(&C::ToolCall))
    {
        Ok(())
    } else {
        Err(validify::ValidationError::new_field("model_capabilities"))
    }
}
fn template_capabilities_contract() -> serde_json::Value {
    serde_json::json!({"x-compallm-array-rule":"modelCapabilities"})
}

#[derive(
    serde::Serialize,
    serde::Deserialize,
    juniper::GraphQLInputObject,
    validify::Validify,
    compallm_validation_derive::ExportValidation,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[validate(validate_template_create)]
pub struct ModelTemplateCreateInput {
    #[validify]
    pub configuration: ModelTemplateCustomizeInput,
}
fn validate_template_create(
    value: &ModelTemplateCreateInput,
) -> Result<(), validify::ValidationErrors> {
    let v = &value.configuration;
    let mut errors = validify::ValidationErrors::new();
    for (field, valid, message) in [
        (
            "name",
            v.name.is_some(),
            "Enter the upstream model identifier.",
        ),
        ("protocol", v.protocol.is_some(), "Choose a text operation."),
        ("path", v.path.is_some(), "Enter an endpoint path."),
        (
            "inputPrice",
            v.input_price
                .as_ref()
                .is_some_and(|v| v.enabled && v.value.is_some()),
            "Set an input price for the new model.",
        ),
        (
            "outputPrice",
            v.output_price
                .as_ref()
                .is_some_and(|v| v.enabled && v.value.is_some()),
            "Set an output price for the new model.",
        ),
    ] {
        if !valid {
            let mut error =
                validify::ValidationError::new_field_named(field, "required_model_configuration")
                    .with_message(message.into());
            error.append_location("configuration");
            error.append_location(field);
            errors.add(error);
        }
    }
    if errors.errors().is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
fn validate_template_create_contract() -> serde_json::Value {
    serde_json::json!({"x-compallm-object-rule":"templateCreate","x-compallm-messages":{"name":"Enter the upstream model identifier.","protocol":"Choose a text operation.","path":"Enter an endpoint path.","inputPrice":"Set an input price for the new model.","outputPrice":"Set an output price for the new model."}})
}
