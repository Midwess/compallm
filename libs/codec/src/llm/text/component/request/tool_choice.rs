#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ToolChoice<S = String> {
    Auto,
    None,
    Required,
    Function(S),
}

impl<S> ToolChoice<S> {
    pub fn map_text<T>(self, convert: impl FnOnce(S) -> T) -> ToolChoice<T> {
        match self {
            Self::Auto => ToolChoice::Auto,
            Self::None => ToolChoice::None,
            Self::Required => ToolChoice::Required,
            Self::Function(name) => ToolChoice::Function(convert(name)),
        }
    }
}
