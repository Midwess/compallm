#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stop<S = String> {
    Null,
    Text(S),
    Array(Vec<S>),
    ArrayView(crate::json::JsonView<S>),
}

impl<S> Stop<S> {
    pub fn map_text<T>(self, mut convert: impl FnMut(S) -> T) -> Stop<T> {
        match self {
            Self::Null => Stop::Null,
            Self::Text(text) => Stop::Text(convert(text)),
            Self::Array(texts) => Stop::Array(texts.into_iter().map(convert).collect()),
            Self::ArrayView(view) => Stop::ArrayView(view.map_text(convert)),
        }
    }
}
