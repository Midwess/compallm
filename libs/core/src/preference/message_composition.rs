use super::values::KeyCombination;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MessageCompositionPreferences {
    pub convert_emoticons: bool,
    pub send_shortcut: KeyCombination,
}

impl Default for MessageCompositionPreferences {
    fn default() -> Self {
        Self {
            convert_emoticons: true,
            send_shortcut: KeyCombination::ModifierEnter,
        }
    }
}
