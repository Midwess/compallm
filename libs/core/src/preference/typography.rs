use super::values::{Font, FontFamily, RelativeFontSize};

pub type TypographyPreferences = Font<FontFamily, RelativeFontSize>;

impl Default for TypographyPreferences {
    fn default() -> Self {
        Self {
            family: FontFamily::Default,
            size: RelativeFontSize::Default,
        }
    }
}

impl validify::Validate for TypographyPreferences {
    fn validate(&self) -> Result<(), validify::ValidationErrors> {
        if self.family == FontFamily::Default {
            return Ok(());
        }
        let mut errors = validify::ValidationErrors::new();
        errors.add(validify::ValidationError::new_schema(
            "unsupported_interface_font",
        ));
        Err(errors)
    }
}
