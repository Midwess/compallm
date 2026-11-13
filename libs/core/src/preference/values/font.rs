use serde::{Deserialize, Serialize};

/// A reusable font choice whose supported values depend on its context.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Font<Family, Size> {
    pub family: Family,
    pub size: Size,
}
