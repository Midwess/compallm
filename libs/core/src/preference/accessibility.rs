use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AccessibilityPreferences {
    pub underline_links: bool,
    pub reduce_motion: bool,
    pub pointer_cursors: bool,
}

impl Default for AccessibilityPreferences {
    fn default() -> Self {
        Self {
            underline_links: false,
            reduce_motion: true,
            pointer_cursors: false,
        }
    }
}
