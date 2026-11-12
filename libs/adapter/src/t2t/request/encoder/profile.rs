use super::{Encoder, RequestEncoder, structured::Structured};
use crate::{
    codec::EncodeStep,
    error::CodecError,
    t2t::{
        Text2TextTranslatorConfiguration,
        request::{Format, event::RequestEvent},
    },
};

pub(crate) struct ProfileEncoder<'model> {
    inner: Encoder<'model>,
    structured: Option<Structured>,
    input_kind: Option<bool>,
    ready: bool,
    failed: bool,
}

impl<'model> ProfileEncoder<'model> {
    pub fn new(
        format: Format,
        model: &'model str,
        configuration: Text2TextTranslatorConfiguration,
    ) -> Result<Self, CodecError> {
        Ok(Self {
            inner: Encoder::new(format, model, configuration)?,
            structured: None,
            input_kind: None,
            ready: false,
            failed: false,
        })
    }

    pub fn with_mid_conversation_system(mut self, supported: bool) -> Self {
        self.inner = self.inner.with_mid_conversation_system(supported);
        self
    }
}

impl RequestEncoder for ProfileEncoder<'_> {
    fn accept(&mut self, event: RequestEvent) -> Result<(), CodecError> {
        let result = (|| {
            if self.failed || !self.ready {
                return Err(CodecError::Contract("accept requires NeedEvent"));
            }
            self.ready = false;
            if self.input_kind.is_none() {
                let structured = matches!(event, RequestEvent::Structure(_));
                self.input_kind = Some(structured);
                if structured {
                    self.structured = Some(Structured::new());
                }
            }
            if let Some(state) = &mut self.structured {
                state.accept(event, &mut self.inner)
            } else {
                self.inner.accept(event)
            }
        })();
        if result.is_err() {
            self.failed = true;
        }
        result
    }

    fn advance(&mut self, text: &mut &[u8], output: &mut [u8]) -> Result<EncodeStep, CodecError> {
        if self.failed {
            return Err(CodecError::Contract("encoder previously failed"));
        }
        if output.is_empty() {
            self.failed = true;
            return Err(CodecError::Contract(
                "encoder output slice must not be empty",
            ));
        }
        let result = if let Some(state) = &mut self.structured {
            state.advance(text, output, &mut self.inner)
        } else {
            self.inner.advance(text, output)
        };
        self.ready = matches!(&result, Ok(EncodeStep::NeedEvent));
        if result.is_err() {
            self.failed = true;
        }
        result
    }

    fn finish(&mut self) -> Result<(), CodecError> {
        let result = (|| {
            if self.failed || !self.ready {
                return Err(CodecError::Contract("finish requires NeedEvent"));
            }
            if let Some(state) = &self.structured {
                state.finish()?;
            }
            self.inner.finish()
        })();
        self.ready = false;
        if result.is_err() {
            self.failed = true;
        }
        result
    }
}
