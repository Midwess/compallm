use futures_io::{AsyncRead, AsyncWrite};

use super::{decoder::ResponseDecoder, encoder::ResponseEncoder, event::ResponseEvent};
use crate::{
    codec::{DecodeStep, EncodeStep, TranslateStep},
    error::{CodecError, TranslationError},
    stream::{Codecs, Translator},
};

/// Explicitly configured, incrementally stepped response translation. Reader,
/// writer, and codec types stay fixed; both I/O allocations last until Drop.
/// All four components are required. Setters reject active attempts; the first
/// setter after completion/failure clears every old component. Configure four
/// fresh components for reuse. Canceling next/flush preserves resumable progress.
pub struct Text2TextResponseTranslator<S, T, D, E> {
    inner: Translator<S, T, ResponseCodecs<D, E>>,
}

impl<S: AsyncRead, T: AsyncWrite, D: ResponseDecoder, E: ResponseEncoder>
    Text2TextResponseTranslator<S, T, D, E>
{
    pub fn new(buffer_bytes: usize, steps_before_yield: usize) -> Self {
        Self {
            inner: Translator::new(
                buffer_bytes,
                steps_before_yield,
                ResponseCodecs {
                    decoder: None,
                    encoder: None,
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
        self.inner.codecs.decoder = Some(decoder);
        Ok(self)
    }
    pub fn set_encoder(&mut self, encoder: E) -> Result<&mut Self, TranslationError> {
        self.inner.configure()?;
        self.inner.codecs.encoder = Some(encoder);
        Ok(self)
    }

    pub async fn next(&mut self) -> Result<usize, TranslationError> {
        self.inner.next().await
    }
    pub fn translate(&mut self) -> Result<TranslateStep, TranslationError> {
        self.inner.translate()
    }
    /// Flush output only, never close. Resuming a canceled flush does not repeat
    /// bytes already written to the target.
    pub async fn flush(&mut self) -> Result<(), TranslationError> {
        self.inner.flush().await
    }
}

struct ResponseCodecs<D, E> {
    decoder: Option<D>,
    encoder: Option<E>,
}
impl<D: ResponseDecoder, E: ResponseEncoder> Codecs for ResponseCodecs<D, E> {
    type Event = ResponseEvent;
    fn has_decoder(&self) -> bool {
        self.decoder.is_some()
    }
    fn has_encoder(&self) -> bool {
        self.encoder.is_some()
    }
    fn clear(&mut self) {
        self.decoder = None;
        self.encoder = None;
    }
    fn decode(
        &mut self,
        input: &mut &[u8],
        eof: bool,
    ) -> Result<DecodeStep<ResponseEvent>, CodecError> {
        self.decoder
            .as_mut()
            .expect("validated decoder")
            .advance(input, eof)
    }
    fn accept(&mut self, event: ResponseEvent) -> Result<(), CodecError> {
        self.encoder
            .as_mut()
            .expect("validated encoder")
            .accept(event)
    }
    fn encode(&mut self, output: &mut [u8]) -> Result<EncodeStep, CodecError> {
        self.encoder
            .as_mut()
            .expect("validated encoder")
            .advance(output)
    }
    fn finish(&mut self) -> Result<(), CodecError> {
        self.encoder.as_mut().expect("validated encoder").finish()
    }
}
