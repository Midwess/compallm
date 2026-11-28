use crate::llm::text::component::Stop;
use serde::{Deserialize, Deserializer, Serialize, de::Error};

pub(in crate::llm::text::anthropic::request) struct Stops<'a>(pub Stop<&'a str>);

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
            if count > 32 || text.is_empty() || bytes > 4096 {
                return Err(D::Error::custom("invalid stop sequences"));
            }
        }
        Ok(Stops(Stop::ArrayView(view)))
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub(in crate::llm::text::anthropic::request) enum StopOutput<'a, S: AsRef<str>> {
    Empty([&'a str; 0]),
    Text([&'a str; 1]),
    Array(&'a [S]),
    View(&'a crate::json::JsonView<S>),
}
impl<'a, S: AsRef<str>> TryFrom<&'a Stop<S>> for StopOutput<'a, S> {
    type Error = &'static str;

    fn try_from(value: &'a Stop<S>) -> Result<Self, Self::Error> {
        let mut count = 0;
        let mut bytes = 0;
        let mut check = |text: &str| {
            count += 1;
            bytes += text.len();
            if count > 32 || bytes > 4096 || text.is_empty() {
                Err("invalid stop sequences")
            } else {
                Ok(())
            }
        };
        match value {
            Stop::Null => {}
            Stop::Text(text) => check(text.as_ref())?,
            Stop::Array(values) => {
                for text in values {
                    check(text.as_ref())?;
                }
            }
            Stop::ArrayView(view) => {
                let mut scratch = [0; 4096];
                let view = view.borrowed();
                for item in view.items().ok_or("expected stop array")? {
                    check(
                        item.decode_string(&mut scratch)
                            .map_err(|_| "invalid stop text")?,
                    )?;
                }
            }
        }
        Ok(match value {
            Stop::Null => Self::Empty([]),
            Stop::Text(text) => Self::Text([text.as_ref()]),
            Stop::Array(values) => Self::Array(values),
            Stop::ArrayView(view) => Self::View(view),
        })
    }
}
