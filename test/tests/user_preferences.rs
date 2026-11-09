use compallm_core::user::preferences::{
    BadgeStyle, CodeTheme, SidebarEntry, SidebarItem, SidebarPreferences, SidebarVisibility,
    UserPreferences,
};

#[test]
fn missing_sidebar_items_follow_saved_order_and_keep_choices() {
    let mut preferences = SidebarPreferences {
        badge_style: BadgeStyle::Count,
        entries: vec![
            SidebarEntry {
                item: SidebarItem::Plugins,
                visibility: SidebarVisibility::Hidden,
            },
            SidebarEntry {
                item: SidebarItem::Providers,
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
                item: SidebarItem::Plugins,
                visibility: SidebarVisibility::Hidden
            },
            SidebarEntry {
                item: SidebarItem::Providers,
                visibility: SidebarVisibility::WhenBadged
            },
            SidebarEntry {
                item: SidebarItem::Models,
                visibility: SidebarVisibility::Always
            },
            SidebarEntry {
                item: SidebarItem::Notebooks,
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
