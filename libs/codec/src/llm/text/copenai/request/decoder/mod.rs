mod state;

use crate::decoder::{Decoder, DecoderError, DecoderStatus};
use crate::json::JsonDecoder;
use crate::llm::text::component::TextRequestComponent;
use state::RequestState;

#[derive(Debug)]
pub struct CompatibleOpenAIRequestDecoder {
    json: JsonDecoder,
    state: RequestState,
}

impl CompatibleOpenAIRequestDecoder {
    pub fn new(json: JsonDecoder) -> Self {
        Self {
            json,
            state: RequestState::default(),
        }
    }
}

impl Decoder for CompatibleOpenAIRequestDecoder {
    type Component<'a> = TextRequestComponent;
    type Error = DecoderError;

    fn next<'a, 'input: 'a>(
        &'a mut self,
        input: &mut &'input [u8],
        eof: bool,
    ) -> Result<DecoderStatus<Self::Component<'a>>, Self::Error> {
        self.state.ensure_active()?;
        let result = (|| loop {
            match self
                .json
                .next(input, eof)
                .map_err(|_| RequestState::invalid())?
            {
                DecoderStatus::Component(token) => {
                    if let Some(component) = self.state.accept(token)? {
                        return Ok(DecoderStatus::Component(component));
                    }
                }
                DecoderStatus::NeedInput => return Ok(DecoderStatus::NeedInput),
                DecoderStatus::Finished => {
                    self.state.finish()?;
                    return Ok(DecoderStatus::Finished);
                }
            }
        })();
        if result.is_err() {
            self.state.fail();
        }
        result
    }

    fn finish(self) -> Result<(), Self::Error> {
        self.state.ensure_complete()?;
        self.json.finish().map_err(|_| RequestState::invalid())
    }
}
