pub mod decoder;
pub mod encoder;
pub mod event;
mod json;
mod scratch;
pub mod translator;

pub use scratch::RequestScratch;
pub use translator::Text2TextRequestTranslator;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Format {
    Chat,
    Responses,
    Anthropic,
}

const MODEL_BYTES: usize = 1024;
