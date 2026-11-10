use futures_io::{AsyncRead, AsyncWrite};

use super::{
    RequestScratch, decoder::RequestDecoder, encoder::RequestEncoder, event::RequestEvent,
};
use crate::{
    codec::{DecodeStep, EncodeStep, TranslateStep},
    error::{CodecError, TranslationError},
    stream::{Codecs, Translator},
};

/// Explicitly configured, incrementally stepped request translation. Reader,
/// writer, and codec types stay fixed; both I/O allocations last until Drop.
/// All four components are required. Setters reject active attempts; the first
/// setter after completion/failure clears every old component. Configure four
/// fresh components for reuse. Canceling next/flush preserves resumable progress.
/// Decoded fragment/deferral scratch is prepared by set_decoder and never grows
/// during processing. Pending text stays in scratch until encoding drains it.
pub struct Text2TextRequestTranslator<S, T, D, E> {
    inner: Translator<S, T, RequestCodecs<D, E>>,
}

impl<S: AsyncRead, T: AsyncWrite, D: RequestDecoder, E: RequestEncoder>
    Text2TextRequestTranslator<S, T, D, E>
{
    pub fn new(buffer_bytes: usize, steps_before_yield: usize) -> Self {
        Self {
            inner: Translator::new(
                buffer_bytes,
                steps_before_yield,
                RequestCodecs {
                    decoder: None,
                    encoder: None,
                    scratch: None,
                    text_len: 0,
                    text_consumed: 0,
                },
            ),
        }
    }

    pub fn set_source_stream(&mut self, source: S) -> Result<&mut Self, TranslationError> {
        self.inner.set_source(source)?;
        Ok(self)
    }
    /// Store the writer value. Pass a borrowed/pinned reference if the caller
    /// must retain ownership; no translator method calls poll_close.
    pub fn set_target_stream(&mut self, target: T) -> Result<&mut Self, TranslationError> {
        self.inner.set_target(target)?;
        Ok(self)
    }
    pub fn set_decoder(&mut self, decoder: D) -> Result<&mut Self, TranslationError> {
        self.inner.configure()?;
        let (fragment, deferred) = decoder.scratch_requirements();
        let codecs = &mut self.inner.codecs;
        match &mut codecs.scratch {
            Some(scratch) => scratch.prepare(fragment, deferred),
            None => RequestScratch::new(fragment, deferred)
                .map(|scratch| codecs.scratch = Some(scratch)),
        }
        .map_err(|error| TranslationError::Stream(error.into()))?;
        self.inner.codecs.decoder = Some(decoder);
        Ok(self)
    }
    pub fn set_encoder(&mut self, encoder: E) -> Result<&mut Self, TranslationError> {
        self.inner.configure()?;
        self.inner.codecs.encoder = Some(encoder);
        Ok(self)
    }

    /// Read one chunk. Zero records EOF; call translate to validate/finalize it.
    pub async fn next(&mut self) -> Result<usize, TranslationError> {
        self.inner.next().await
    }
    /// Produce bounded output or an explicit progress state without doing I/O.
    pub fn translate(&mut self) -> Result<TranslateStep, TranslationError> {
        self.inner.translate()
    }
    /// Write/flush pending output; never close the writer. Cancellation preserves
    /// the write offset and can be resumed by calling flush again.
    pub async fn flush(&mut self) -> Result<(), TranslationError> {
        self.inner.flush().await
    }
}

struct RequestCodecs<D, E> {
    decoder: Option<D>,
    encoder: Option<E>,
    scratch: Option<RequestScratch>,
    text_len: usize,
    text_consumed: usize,
}
impl<D: RequestDecoder, E: RequestEncoder> Codecs for RequestCodecs<D, E> {
    type Event = RequestEvent;
    fn has_decoder(&self) -> bool {
        self.decoder.is_some()
    }
    fn has_encoder(&self) -> bool {
        self.encoder.is_some()
    }
    fn clear(&mut self) {
        self.decoder = None;
        self.encoder = None;
        if let Some(scratch) = &mut self.scratch {
            scratch.reset();
        }
        self.text_len = 0;
        self.text_consumed = 0;
    }
    fn decode(
        &mut self,
        input: &mut &[u8],
        eof: bool,
    ) -> Result<DecodeStep<RequestEvent>, CodecError> {
        self.decoder.as_mut().expect("validated decoder").advance(
            input,
            eof,
            self.scratch.as_mut().expect("configured scratch"),
        )
    }
    fn accept(&mut self, event: RequestEvent) -> Result<(), CodecError> {
        self.text_len = match event {
            RequestEvent::TextFragment(len) => len,
            _ => 0,
        };
        self.text_consumed = 0;
        if self.text_len
            > self
                .scratch
                .as_ref()
                .expect("configured scratch")
                .text()
                .len()
        {
            return Err(CodecError::Contract(
                "decoder reported invalid fragment length",
            ));
        }
        self.encoder
            .as_mut()
            .expect("validated encoder")
            .accept(event)
    }
    fn encode(&mut self, output: &mut [u8]) -> Result<EncodeStep, CodecError> {
        let scratch = self.scratch.as_ref().expect("configured scratch");
        let text = &scratch.text()[self.text_consumed..self.text_len];
        let mut remaining = text;
        let step = self
            .encoder
            .as_mut()
            .expect("validated encoder")
            .advance(&mut remaining, output)?;
        if remaining.len() > text.len()
            || (!remaining.is_empty()
                && remaining.as_ptr() != text.as_ptr().wrapping_add(text.len() - remaining.len()))
        {
            return Err(CodecError::Contract("encoder did not retain a text suffix"));
        }
        self.text_consumed += text.len() - remaining.len();
        if matches!(step, EncodeStep::NeedEvent | EncodeStep::Finished)
            && self.text_consumed != self.text_len
        {
            return Err(CodecError::Contract("encoder abandoned pending text"));
        }
        Ok(step)
    }
    fn finish(&mut self) -> Result<(), CodecError> {
        self.encoder.as_mut().expect("validated encoder").finish()
    }
}
