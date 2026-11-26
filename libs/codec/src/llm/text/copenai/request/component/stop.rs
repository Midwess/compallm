use crate::llm::text::component::Stop;
use serde::{Deserialize, Deserializer, Serialize, de::Error};

#[derive(Serialize)]
#[serde(remote = "Stop", untagged)]
pub(in crate::llm::text::copenai::request) enum StopWire<S: AsRef<str>> {
    Null,
    Text(S),
    Array(Vec<S>),
    ArrayView(crate::json::JsonView<S>),
}

pub(in crate::llm::text::copenai::request) struct Stops<'a>(pub Stop<&'a str>);

impl<'de> Deserialize<'de> for Stops<'de> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let view = crate::json::JsonView::deserialize_array(deserializer)?;
        let mut count = 0;
        let mut bytes = 0;
        let mut scratch = [0; 4096];
        for value in view
            .items()
            .ok_or_else(|| D::Error::custom("expected stop array"))?
        {
            count += 1;
            let text = value
                .decode_string(&mut scratch)
                .map_err(D::Error::custom)?;
            bytes += text.len();
            if count > 4 || text.is_empty() || bytes > 4096 {
                return Err(D::Error::custom("invalid stop sequences"));
            }
        }
        Ok(Stops(Stop::ArrayView(view)))
    }
}
