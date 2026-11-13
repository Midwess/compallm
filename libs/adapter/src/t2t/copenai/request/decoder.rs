use crate::decoder::{RequestDecoder, RequestDecoderError, RequestDecoderStatus};

use super::state::CompatibleOpenAIDecoderState;

#[derive(Default)]
pub struct CompatibleOpenAIRequestDecoder;

impl CompatibleOpenAIRequestDecoder {
    pub fn new() -> Self {
        Self
    }
}

impl RequestDecoder<CompatibleOpenAIDecoderState> for CompatibleOpenAIRequestDecoder {
    fn next(&mut self, _input: &mut &[u8]) -> Result<RequestDecoderStatus, RequestDecoderError> {
        todo!()
    }

    fn get_component(&self) -> Option<CompatibleOpenAIDecoderState> {
        todo!()
    }

    fn finish(&mut self) -> Result<(), RequestDecoderError> {
        todo!()
    }
}
