use super::{ComponentDeserializer, deserialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StreamOptions {
    pub include_usage: Option<bool>,
}

deserialize!(StreamOptions, value, {
    let mut map = ComponentDeserializer::object(value)?;
    let include_usage = ComponentDeserializer::optional(&mut map, "include_usage")?;
    ComponentDeserializer::done(map)?;
    Ok(Self { include_usage })
});
