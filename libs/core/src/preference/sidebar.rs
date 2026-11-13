use super::values::{BadgeStyle, Destination, SidebarVisibility};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SidebarEntry {
    pub item: Destination,
    pub visibility: SidebarVisibility,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SidebarPreferences {
    pub badge_style: BadgeStyle,
    /// Entries are in display order.
    pub entries: Vec<SidebarEntry>,
}

impl validify::Validate for SidebarPreferences {
    fn validate(&self) -> Result<(), validify::ValidationErrors> {
        if self
            .entries
            .iter()
            .any(|entry| entry.item == Destination::Chat)
        {
            let mut errors = validify::ValidationErrors::new();
            errors.add(validify::ValidationError::new_schema(
                "unsupported_sidebar_destination",
            ));
            return Err(errors);
        }
        let mut seen = std::collections::HashSet::new();
        if self.entries.iter().any(|entry| !seen.insert(entry.item)) {
            let mut errors = validify::ValidationErrors::new();
            errors.add(validify::ValidationError::new_schema(
                "duplicate_sidebar_item",
            ));
            return Err(errors);
        }
        Ok(())
    }
}

impl SidebarPreferences {
    /// Append newly introduced items in their default order without disturbing saved choices.
    pub fn append_missing_defaults(&mut self) {
        for entry in Self::default().entries {
            if !self
                .entries
                .iter()
                .any(|existing| existing.item == entry.item)
            {
                self.entries.push(entry);
            }
        }
    }
}

impl Default for SidebarPreferences {
    fn default() -> Self {
        let items = [
            Destination::Providers,
            Destination::Models,
            Destination::Notebooks,
            Destination::Plugins,
        ];
        Self {
            badge_style: BadgeStyle::Dot,
            entries: items
                .into_iter()
                .map(|item| SidebarEntry {
                    item,
                    visibility: SidebarVisibility::Always,
                })
                .collect(),
        }
    }
}
