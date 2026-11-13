use super::values::{CodeTheme, Font, FontFamily, PixelFontSize};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CodeAppearancePreferences {
    pub theme: CodeTheme,
    pub font: Font<FontFamily, PixelFontSize>,
}

impl Default for CodeAppearancePreferences {
    fn default() -> Self {
        Self {
            theme: CodeTheme::FollowInterface,
            font: Font {
                family: FontFamily::Default,
                size: PixelFontSize::Px12,
            },
        }
    }
}
