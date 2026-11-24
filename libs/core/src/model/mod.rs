//! Organization-owned logical models.

pub mod catalog;
pub mod entity;

pub use catalog::ModelCatalog;
pub use entity::Model;

#[path = "catalog-customization.rs"]
mod catalog_customization;
pub use catalog_customization::CatalogCustomization;
pub(crate) fn invalid_catalog(message: &'static str) -> validify::ValidationErrors {
    let mut errors = validify::ValidationErrors::new();
    errors.add(
        validify::ValidationError::new_schema("model_configuration").with_message(message.into()),
    );
    errors
}

pub(crate) fn invalid_catalog_field(
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
