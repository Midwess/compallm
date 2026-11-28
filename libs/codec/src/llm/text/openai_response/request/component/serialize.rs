use crate::json::JsonView;
use serde::{Serialize, Serializer, ser::SerializeMap};

fn member<'a>(view: JsonView<&'a str>, name: &str) -> Option<JsonView<&'a str>> {
    let mut scratch = [0; 128];
    view.members()?
        .find_map(|(key, value)| (key.decode_string(&mut scratch).ok()? == name).then_some(value))
}

pub(in crate::llm::text::openai_response::request) struct RawCall<'a>(pub JsonView<&'a str>);
impl Serialize for RawCall<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::Error;
        let function =
            member(self.0, "function").ok_or_else(|| S::Error::custom("missing function"))?;
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("type", "function_call")?;
        map.serialize_entry(
            "call_id",
            &member(self.0, "id").ok_or_else(|| S::Error::custom("missing id"))?,
        )?;
        map.serialize_entry(
            "name",
            &member(function, "name").ok_or_else(|| S::Error::custom("missing name"))?,
        )?;
        let arguments =
            member(function, "arguments").ok_or_else(|| S::Error::custom("missing arguments"))?;
        if arguments.as_str().starts_with('"') {
            map.serialize_entry("arguments", &arguments)?;
        } else {
            map.serialize_entry("arguments", arguments.as_str())?;
        }
        map.end()
    }
}

pub(super) struct FlatTool<'a>(pub JsonView<&'a str>);
impl Serialize for FlatTool<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::Error;
        let function = member(self.0, "function").unwrap_or(self.0);
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("type", "function")?;
        map.serialize_entry(
            "name",
            &member(function, "name").ok_or_else(|| S::Error::custom("missing name"))?,
        )?;
        for name in ["description", "parameters", "strict"] {
            if let Some(value) = member(function, name) {
                map.serialize_entry(name, &value)?;
            }
        }
        map.end()
    }
}
