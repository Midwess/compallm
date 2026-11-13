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
        layout: vec![],
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
        preferences.layout[0].items,
        vec![
            Destination::Plugins,
            Destination::Providers,
            Destination::Models,
            Destination::Notebooks
        ]
    );
    assert_eq!(preferences.layout.len(), 4);
    let stored = serde_json::json!({"badgeStyle":"count","entries":[{"item":"plugins","visibility":"hidden"}]});
    let mut legacy: SidebarPreferences = serde_json::from_value(stored).unwrap();
    legacy.append_missing_defaults();
    assert_eq!(legacy.layout[0].items[0], Destination::Plugins);
    assert_eq!(
        legacy.entries.len(),
        SidebarPreferences::default().entries.len()
    );
    assert_eq!(
        preferences.entries[..4].to_vec(),
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

#[test]
fn sidebar_layout_moves_items_across_categories_and_preserves_visibility() {
    let mut preferences = UserPreferences::default();
    preferences.sidebar.entries[3].visibility = SidebarVisibility::Hidden;
    let mut layout = preferences.sidebar.layout.clone();
    let plugins = layout[0].items.pop().unwrap();
    layout[2].items.insert(0, plugins);
    layout.swap(0, 2);
    UserPreferencesPatch {
        sidebar: Some(SidebarPatch {
            layout: Some(layout.clone()),
            ..Default::default()
        }),
        ..Default::default()
    }
    .apply(&mut preferences)
    .unwrap();
    assert_eq!(preferences.sidebar.layout, layout);
    assert_eq!(preferences.sidebar.entries[0].item, Destination::Plugins);
    assert_eq!(
        preferences.sidebar.entries[0].visibility,
        SidebarVisibility::Hidden
    );
    let encoded = serde_json::to_value(&preferences.sidebar).unwrap();
    let decoded: SidebarPreferences = serde_json::from_value(encoded).unwrap();
    assert_eq!(decoded, preferences.sidebar);
}

#[test]
fn sidebar_layout_rejects_duplicate_items_and_incomplete_layouts() {
    let mut sidebar = SidebarPreferences::default();
    let original = sidebar.clone();
    let mut duplicate = sidebar.layout.clone();
    duplicate[1].items.push(Destination::Plugins);
    assert!(sidebar.replace_layout(duplicate).is_err());
    assert_eq!(sidebar, original);
    let mut incomplete = sidebar.layout.clone();
    incomplete[0].items.pop();
    assert!(sidebar.replace_layout(incomplete).is_err());
    assert_eq!(sidebar, original);
    let mut categories = sidebar.layout.clone();
    categories[1].category = categories[0].category;
    assert!(sidebar.replace_layout(categories).is_err());
}
