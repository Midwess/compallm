use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DarkPalette {
    BasicDark,
    BlueberryDark,
    MarshmallowDark,
}
