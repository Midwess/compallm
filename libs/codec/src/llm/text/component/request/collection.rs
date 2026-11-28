use crate::json::JsonView;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Collection<T, S = String> {
    Owned(Vec<T>),
    Single(T),
    View(JsonView<S>),
}

impl<T, S: AsRef<str>> Collection<T, S> {
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Owned(values) => values.is_empty(),
            Self::Single(_) => false,
            Self::View(view) => view
                .borrowed()
                .items()
                .is_none_or(|mut items| items.next().is_none()),
        }
    }
}

impl<T, S> Collection<T, S> {
    pub fn map<U, R>(
        self,
        mut convert: impl FnMut(T) -> U,
        text: impl FnMut(S) -> R,
    ) -> Collection<U, R> {
        match self {
            Self::Single(value) => Collection::Single(convert(value)),
            Self::Owned(values) => Collection::Owned(values.into_iter().map(convert).collect()),
            Self::View(view) => Collection::View(view.map_text(text)),
        }
    }
}

impl<T, S> From<Vec<T>> for Collection<T, S> {
    fn from(values: Vec<T>) -> Self {
        Self::Owned(values)
    }
}
