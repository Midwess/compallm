use super::values::Weekday;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CalendarPreferences {
    pub first_day: Weekday,
}

impl Default for CalendarPreferences {
    fn default() -> Self {
        Self {
            first_day: Weekday::Monday,
        }
    }
}
