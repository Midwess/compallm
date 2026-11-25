#[derive(Debug, PartialEq)]
pub enum JsonToken<'a> {
    ObjectStart(Option<&'a str>),
    ObjectEnd,
    ArrayStart(Option<&'a str>),
    ArrayEnd,
    ValueString(Option<&'a str>, &'a str),
    ValueNumber(Option<&'a str>, serde_json::Number),
    ValueBoolean(Option<&'a str>, bool),
    ValueNull(Option<&'a str>),
}

impl<'a> JsonToken<'a> {
    pub fn key(&self) -> Option<&'a str> {
        match self {
            Self::ObjectStart(key)
            | Self::ArrayStart(key)
            | Self::ValueString(key, _)
            | Self::ValueNumber(key, _)
            | Self::ValueBoolean(key, _)
            | Self::ValueNull(key) => *key,
            Self::ObjectEnd | Self::ArrayEnd => None,
        }
    }
}
#[derive(Debug)]
pub(crate) enum StringReference {
    Input(std::ops::Range<usize>),
    Buffered,
}

#[derive(Debug)]
pub(crate) enum RetainedString {
    Capture(std::ops::Range<usize>),
    Decoded(std::ops::Range<usize>),
    Literal(&'static str),
}

pub(crate) struct StringLayout {
    capture: (usize, usize),
    decoded: (usize, usize),
}

impl StringLayout {
    pub(crate) fn new(capture: &[u8], decoded: &[u8]) -> Self {
        Self {
            capture: (capture.as_ptr() as usize, capture.len()),
            decoded: (decoded.as_ptr() as usize, decoded.len()),
        }
    }

    pub(crate) fn locate(&self, value: &str) -> Option<RetainedString> {
        if value.is_empty() {
            return Some(RetainedString::Literal(""));
        }
        for ((base, length), decoded) in [(self.capture, false), (self.decoded, true)] {
            if let Some(start) = (value.as_ptr() as usize).checked_sub(base)
                && let Some(end) = start.checked_add(value.len())
                && end <= length
            {
                return Some(if decoded {
                    RetainedString::Decoded(start..end)
                } else {
                    RetainedString::Capture(start..end)
                });
            }
        }
        None
    }
}

impl StringReference {
    pub(crate) fn new(value: &str, input: &[u8]) -> Self {
        if let Some(start) = (value.as_ptr() as usize).checked_sub(input.as_ptr() as usize)
            && let Some(end) = start.checked_add(value.len())
            && end <= input.len()
        {
            return Self::Input(start..end);
        }
        Self::Buffered
    }
}
