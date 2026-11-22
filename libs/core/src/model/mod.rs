//! Organization-owned logical models.

pub mod entity;
pub mod template;

pub use entity::Model;
pub use template::ModelTemplate;

#[path = "template-customization.rs"]
mod template_customization;
pub use template_customization::TemplateCustomization;
pub(crate) fn invalid_template(message: &'static str) -> validify::ValidationErrors {
    let mut errors = validify::ValidationErrors::new();
    errors.add(
        validify::ValidationError::new_schema("model_configuration").with_message(message.into()),
    );
    errors
}

pub(crate) fn invalid_template_field(
    field: &'static str,
    message: &'static str,
) -> validify::ValidationErrors {
    let mut errors = validify::ValidationErrors::new();
    let mut error = validify::ValidationError::new_field_named(field, "model_configuration")
        .with_message(message.into());
    error.append_location("limits");
    error.append_location(field);
    errors.add(error);
    errors
}
