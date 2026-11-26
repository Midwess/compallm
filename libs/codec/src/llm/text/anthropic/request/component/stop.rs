use crate::llm::text::component::Stop;
use compallm_serde_rules::borrowed::Text;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct StopSequences<'a>(#[serde(borrow)] Vec<Text<'a>>);

impl<'a> From<StopSequences<'a>> for Stop<&'a str> {
    fn from(value: StopSequences<'a>) -> Self {
        Self::Array(value.0.into_iter().map(|Text(text)| text).collect())
    }
}
