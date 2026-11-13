use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyCombination {
    Enter,
    /// Command on macOS and Control on other platforms.
    ModifierEnter,
}
