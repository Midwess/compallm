use super::values::Destination;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct NavigationPreferences {
    pub home: Destination,
}

impl Default for NavigationPreferences {
    fn default() -> Self {
        Self {
            home: Destination::Chat,
        }
    }
}

impl validify::Validate for NavigationPreferences {
    fn validate(&self) -> Result<(), validify::ValidationErrors> {
        if matches!(
            self.home,
            Destination::Chat | Destination::Models | Destination::Providers
        ) {
            return Ok(());
        }
        let mut errors = validify::ValidationErrors::new();
        errors.add(validify::ValidationError::new_schema(
            "unsupported_home_destination",
        ));
        Err(errors)
    }
}
