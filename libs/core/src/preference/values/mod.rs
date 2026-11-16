//! Values composed by preference settings.

pub mod appearance_mode;
pub mod badge_style;
pub mod code_theme;
pub mod color_palette;
pub mod dark_palette;
pub mod destination;
pub mod font;
pub mod font_family;
pub mod key_combination;
pub mod light_palette;
pub mod naming_format;
pub mod pixel_font_size;
pub mod relative_font_size;
pub mod sidebar_visibility;
pub mod visual;
pub mod weekday;

pub use appearance_mode::AppearanceMode;
pub use badge_style::BadgeStyle;
pub use code_theme::CodeTheme;
pub use color_palette::ColorPalette;
pub use dark_palette::DarkPalette;
pub use destination::Destination;
pub use font::Font;
pub use font_family::FontFamily;
pub use key_combination::KeyCombination;
pub use light_palette::LightPalette;
pub use naming_format::NamingFormat;
pub use pixel_font_size::PixelFontSize;
pub use relative_font_size::RelativeFontSize;
pub use sidebar_visibility::SidebarVisibility;
pub use visual::Visual;
pub use weekday::Weekday;

pub mod sidebar_category;
pub use sidebar_category::SidebarCategory;
