use super::ColorPalette;
use serde::{Deserialize, Serialize};

/// A visual mode and its context-specific palette choices.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Visual<Mode, Light, Dark> {
    pub mode: Mode,
    pub palettes: ColorPalette<Light, Dark>,
}
