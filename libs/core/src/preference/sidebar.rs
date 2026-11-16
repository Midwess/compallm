use super::values::{BadgeStyle, Destination, SidebarCategory, SidebarVisibility};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SidebarEntry {
    pub item: Destination,
    pub visibility: SidebarVisibility,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SidebarSection {
    pub category: SidebarCategory,
    pub items: Vec<Destination>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SidebarPreferences {
    pub badge_style: BadgeStyle,
    /// Entries are in display order.
    pub entries: Vec<SidebarEntry>,
    #[serde(default)]
    pub layout: Vec<SidebarSection>,
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
        let mut categories = std::collections::HashSet::new();
        let mut items = std::collections::HashSet::new();
        if self.layout.iter().any(|section| {
            !categories.insert(section.category)
                || section
                    .items
                    .iter()
                    .any(|item| *item == Destination::Chat || !items.insert(*item))
        }) {
            return Err(invalid_layout());
        }
        Ok(())
    }
}

fn invalid_layout() -> validify::ValidationErrors {
    let mut errors = validify::ValidationErrors::new();
    errors.add(validify::ValidationError::new_schema(
        "invalid_sidebar_layout",
    ));
    errors
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
        let defaults = default_layout();
        for section in &defaults {
            if !self
                .layout
                .iter()
                .any(|saved| saved.category == section.category)
            {
                self.layout.push(SidebarSection {
                    category: section.category,
                    items: vec![],
                });
            }
        }
        for entry in &self.entries {
            if self
                .layout
                .iter()
                .any(|section| section.items.contains(&entry.item))
            {
                continue;
            }
            if let Some(section) = defaults
                .iter()
                .find(|section| section.items.contains(&entry.item))
            {
                self.layout
                    .iter_mut()
                    .find(|saved| saved.category == section.category)
                    .expect("default category")
                    .items
                    .push(entry.item);
            }
        }
    }

    pub fn replace_layout(
        &mut self,
        layout: Vec<SidebarSection>,
    ) -> Result<(), validify::ValidationErrors> {
        let mut candidate = self.clone();
        candidate.append_missing_defaults();
        candidate.layout = layout;
        validify::Validate::validate(&candidate)?;
        let defaults = Self::default();
        if candidate.layout.len() != defaults.layout.len()
            || candidate
                .layout
                .iter()
                .map(|section| section.items.len())
                .sum::<usize>()
                != defaults.entries.len()
        {
            return Err(invalid_layout());
        }
        self.layout = candidate.layout;
        let old = candidate.entries;
        self.entries = self
            .layout
            .iter()
            .flat_map(|section| &section.items)
            .map(|item| {
                old.iter()
                    .find(|entry| entry.item == *item)
                    .expect("known sidebar entry")
                    .clone()
            })
            .collect();
        Ok(())
    }
}

fn default_layout() -> Vec<SidebarSection> {
    use Destination::*;
    vec![
        SidebarSection {
            category: SidebarCategory::Workspace,
            items: vec![Providers, Models, Notebooks, Plugins],
        },
        SidebarSection {
            category: SidebarCategory::Routers,
            items: vec![Stats, ApiKeys],
        },
        SidebarSection {
            category: SidebarCategory::Organization,
            items: vec![Members],
        },
        SidebarSection {
            category: SidebarCategory::Conversations,
            items: vec![Conversations],
        },
    ]
}

impl Default for SidebarPreferences {
    fn default() -> Self {
        let items = [
            Destination::Providers,
            Destination::Models,
            Destination::Notebooks,
            Destination::Plugins,
            Destination::Stats,
            Destination::ApiKeys,
            Destination::Members,
            Destination::Conversations,
        ];
        Self {
            badge_style: BadgeStyle::Dot,
            layout: default_layout(),
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
