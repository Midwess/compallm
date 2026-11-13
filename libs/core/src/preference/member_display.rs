use super::values::NamingFormat;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MemberDisplayPreferences {
    pub name_format: NamingFormat,
}

impl Default for MemberDisplayPreferences {
    fn default() -> Self {
        Self {
            name_format: NamingFormat::FullName,
        }
    }
}
