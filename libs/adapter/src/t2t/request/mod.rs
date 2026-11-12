pub mod decoder;
pub mod encoder;
pub mod event;
pub(crate) mod json;
mod scratch;
pub(crate) mod structure;
pub mod translator;

pub use scratch::RequestScratch;
pub use translator::Text2TextRequestTranslator;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Format {
    Chat,
    Responses,
    Anthropic,
}

impl Format {
    pub(crate) fn streaming_suffix(
        self,
        stream: bool,
        include_usage: Option<bool>,
    ) -> &'static [u8] {
        match (
            stream,
            if self == Self::Chat {
                include_usage
            } else {
                None
            },
        ) {
            (false, _) => b",\"stream\":false}",
            (true, None) => b",\"stream\":true}",
            (true, Some(true)) => b",\"stream\":true,\"stream_options\":{\"include_usage\":true}}",
            (true, Some(false)) => {
                b",\"stream\":true,\"stream_options\":{\"include_usage\":false}}"
            }
        }
    }
}

pub(crate) const MODEL_BYTES: usize = 1024;
