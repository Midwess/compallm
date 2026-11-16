use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Destination {
    Chat,
    Models,
    Providers,
    Notebooks,
    Plugins,
    Stats,
    ApiKeys,
    Members,
    Conversations,
}
