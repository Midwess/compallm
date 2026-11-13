use serde::{Deserialize, Serialize};

/// Light and dark palettes used by an appearance choice.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ColorPalette<Light, Dark> {
    pub light: Light,
    pub dark: Dark,
}
