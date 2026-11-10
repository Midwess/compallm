use adapter::{
    codec::{DecodeStep, EncodeStep},
    error::CodecError,
    t2t::request::{
        RequestScratch,
        decoder::RequestDecoder,
        encoder::RequestEncoder,
        event::{RequestEvent, Role},
    },
};
use bytes::{Buf, Bytes};

#[derive(Default)]
pub struct MessageDecoder {
    state: u8,
}

impl RequestDecoder for MessageDecoder {
    fn advance(
        &mut self,
        input: &mut &[u8],
        eof: bool,
        scratch: &mut RequestScratch,
    ) -> Result<DecodeStep<RequestEvent>, CodecError> {
        let event = match self.state {
            0 => {
                self.state = 1;
                RequestEvent::MessageStarted { role: Role::User }
            }
            1 => {
                self.state = 2;
                RequestEvent::TextStarted
            }
            2 if !input.is_empty() => {
                let len = input.len().min(2);
                scratch.text_mut()[..len].copy_from_slice(&input[..len]);
                input.advance(len);
                RequestEvent::TextFragment(len)
            }
            2 if eof => {
                self.state = 3;
                RequestEvent::TextEnded
            }
            2 => return Ok(DecodeStep::NeedInput),
            3 => {
                self.state = 4;
                RequestEvent::MessageEnded
            }
            _ => return Ok(DecodeStep::Finished),
        };
        Ok(DecodeStep::Event(event))
    }
}

pub struct ModelEncoder {
    pending: Bytes,
    text_len: usize,
    finished: bool,
}

impl ModelEncoder {
    pub fn new(model: &str) -> Self {
        Self {
            pending: Bytes::from(format!("{model}:")),
            text_len: 0,
            finished: false,
        }
    }
}

impl RequestEncoder for ModelEncoder {
    fn accept(&mut self, event: RequestEvent) -> Result<(), CodecError> {
        assert!(self.pending.is_empty());
        if let RequestEvent::TextFragment(len) = event {
            self.text_len = len;
        }
        Ok(())
    }

    fn advance(&mut self, text: &mut &[u8], output: &mut [u8]) -> Result<EncodeStep, CodecError> {
        if self.text_len > 0 {
            let len = self.text_len.min(output.len());
            output[..len].copy_from_slice(&text[..len]);
            text.advance(len);
            self.text_len -= len;
            Ok(EncodeStep::Wrote(len))
        } else if !self.pending.is_empty() {
            let len = self.pending.len().min(output.len());
            output[..len].copy_from_slice(&self.pending[..len]);
            self.pending.advance(len);
            Ok(EncodeStep::Wrote(len))
        } else if self.finished {
            Ok(EncodeStep::Finished)
        } else {
            Ok(EncodeStep::NeedEvent)
        }
    }

    fn finish(&mut self) -> Result<(), CodecError> {
        assert!(!self.finished);
        self.finished = true;
        self.pending = Bytes::from_static(b"!");
        Ok(())
    }
}
