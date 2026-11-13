use compallm_core::preference::{
    UserPreferences,
    sidebar::{SidebarEntry, SidebarPreferences},
    values::{BadgeStyle, CodeTheme, Destination, SidebarVisibility},
};

#[test]
fn missing_sidebar_items_follow_saved_order_and_keep_choices() {
    let mut preferences = SidebarPreferences {
        badge_style: BadgeStyle::Count,
        entries: vec![
            SidebarEntry {
                item: Destination::Plugins,
                visibility: SidebarVisibility::Hidden,
            },
            SidebarEntry {
                item: Destination::Providers,
                visibility: SidebarVisibility::WhenBadged,
            },
        ],
    };
    preferences.append_missing_defaults();
    assert_eq!(preferences.badge_style, BadgeStyle::Count);
    assert_eq!(
        preferences.entries,
        vec![
            SidebarEntry {
                item: Destination::Plugins,
                visibility: SidebarVisibility::Hidden
            },
            SidebarEntry {
                item: Destination::Providers,
                visibility: SidebarVisibility::WhenBadged
            },
            SidebarEntry {
                item: Destination::Models,
                visibility: SidebarVisibility::Always
            },
            SidebarEntry {
                item: Destination::Notebooks,
                visibility: SidebarVisibility::Always
            },
        ]
    );
}

#[test]
fn default_code_appearance_follows_interface() {
    assert_eq!(
        UserPreferences::default().code_appearance.theme,
        CodeTheme::FollowInterface
    );
}
