use super::values::{AppearanceMode, ColorPalette, DarkPalette, LightPalette, Visual};

pub type ThemePreferences = Visual<AppearanceMode, LightPalette, DarkPalette>;

impl Default for ThemePreferences {
    fn default() -> Self {
        Self {
            mode: AppearanceMode::System,
            palettes: ColorPalette {
                light: LightPalette::BasicWhite,
                dark: DarkPalette::BasicDark,
            },
        }
    }
}
