use compallm_core::preference::{
    UserPreferences,
    patch::{SidebarPatch, UserPreferencesPatch},
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
fn partial_sidebar_reorder_preserves_other_items_visibility() {
    let mut preferences = UserPreferences::default();
    preferences.sidebar.entries[0].visibility = SidebarVisibility::Hidden;
    preferences.sidebar.entries[2].visibility = SidebarVisibility::WhenBadged;
    UserPreferencesPatch {
        sidebar: Some(SidebarPatch {
            order: Some(vec![Destination::Plugins]),
            ..Default::default()
        }),
        ..Default::default()
    }
    .apply(&mut preferences)
    .unwrap();
    assert_eq!(preferences.sidebar.entries[0].item, Destination::Plugins);
    assert_eq!(
        preferences.sidebar.entries[1].visibility,
        SidebarVisibility::Hidden
    );
    assert_eq!(
        preferences.sidebar.entries[3].visibility,
        SidebarVisibility::WhenBadged
    );
}

#[test]
fn default_code_appearance_follows_interface() {
    assert_eq!(
        UserPreferences::default().code_appearance.theme,
        CodeTheme::FollowInterface
    );
}
