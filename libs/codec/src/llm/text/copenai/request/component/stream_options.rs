use crate::llm::text::component::StreamOptions;

#[derive(serde::Deserialize)]
#[serde(remote = "StreamOptions", deny_unknown_fields)]
pub(super) struct StreamFields {
    include_usage: Option<bool>,
}
