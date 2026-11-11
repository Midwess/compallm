use std::{future::poll_fn, io, pin::Pin, task::Poll};

use futures_io::{AsyncRead, AsyncWrite};

use crate::{
    codec::{DecodeStep, EncodeStep, TranslateStep},
    error::{CodecError, StreamError, TranslationError},
};

pub(crate) const DEFAULT_BUFFER_BYTES: usize = 16 * 1024;

/// Private bridge sharing state transitions without unifying event contracts.
pub(crate) trait Codecs {
    type Event;
    fn has_decoder(&self) -> bool;
    fn has_encoder(&self) -> bool;
    fn clear(&mut self);
    fn decode(
        &mut self,
        input: &mut &[u8],
        eof: bool,
    ) -> Result<DecodeStep<Self::Event>, CodecError>;
    fn accept(&mut self, event: Self::Event) -> Result<(), CodecError>;
    fn encode(&mut self, output: &mut [u8]) -> Result<EncodeStep, CodecError>;
    fn finish(&mut self) -> Result<(), CodecError>;
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Configuring,
    Active,
    Finished,
    Failed,
}

pub(crate) struct Translator<S, T, C> {
    source: Option<Pin<Box<S>>>,
    target: Option<Pin<Box<T>>>,
    pub(crate) codecs: C,
    input: Vec<u8>,
    output: Vec<u8>,
    input_start: usize,
    input_end: usize,
    output_len: usize,
    output_written: usize,
    eof: bool,
    finishing: bool,
    need_input: bool,
    flush_pending: bool,
    phase: Phase,
    steps_before_yield: usize,
}

impl<S: AsyncRead, T: AsyncWrite, C: Codecs> Translator<S, T, C> {
    pub(crate) fn new(mut buffer_bytes: usize, mut steps_before_yield: usize, codecs: C) -> Self {
        if buffer_bytes == 0 {
            buffer_bytes = DEFAULT_BUFFER_BYTES;
        }
        if steps_before_yield == 0 {
            steps_before_yield = 64;
        }
        Self {
            source: None,
            target: None,
            codecs,
            input: vec![0; buffer_bytes],
            output: vec![0; buffer_bytes],
            input_start: 0,
            input_end: 0,
            output_len: 0,
            output_written: 0,
            eof: false,
            finishing: false,
            need_input: true,
            flush_pending: false,
            phase: Phase::Configuring,
            steps_before_yield,
        }
    }

    pub(crate) fn configure(&mut self) -> Result<(), TranslationError> {
        match self.phase {
            Phase::Active => {
                return Err(TranslationError::InvalidState(
                    "cannot replace an active attempt",
                ));
            }
            Phase::Finished if self.flush_pending => {
                return Err(TranslationError::InvalidState(
                    "resume the pending flush before reconfiguration",
                ));
            }
            Phase::Finished | Phase::Failed => {
                self.source = None;
                self.target = None;
                self.codecs.clear();
                self.input_start = 0;
                self.input_end = 0;
                self.output_len = 0;
                self.output_written = 0;
                self.eof = false;
                self.finishing = false;
                self.need_input = true;
                self.flush_pending = false;
                self.phase = Phase::Configuring;
            }
            Phase::Configuring => {}
        }
        Ok(())
    }

    pub(crate) fn set_source(&mut self, source: S) -> Result<(), TranslationError> {
        self.configure()?;
        self.source = Some(Box::pin(source));
        Ok(())
    }

    pub(crate) fn set_target(&mut self, target: T) -> Result<(), TranslationError> {
        self.configure()?;
        self.target = Some(Box::pin(target));
        Ok(())
    }

    fn ready(&mut self) -> Result<(), TranslationError> {
        if self.phase == Phase::Failed {
            return Err(TranslationError::InvalidState(
                "failed attempt requires fresh configuration",
            ));
        }
        if self.source.is_none() {
            return Err(TranslationError::MissingComponent("source_stream"));
        }
        if self.target.is_none() {
            return Err(TranslationError::MissingComponent("target_stream"));
        }
        if !self.codecs.has_decoder() {
            return Err(TranslationError::MissingComponent("decoder"));
        }
        if !self.codecs.has_encoder() {
            return Err(TranslationError::MissingComponent("encoder"));
        }
        if self.phase == Phase::Configuring {
            self.phase = Phase::Active;
        }
        Ok(())
    }

    fn fail(&mut self, error: TranslationError) -> TranslationError {
        self.phase = Phase::Failed;
        error
    }

    fn no_pending_output(&self) -> Result<(), TranslationError> {
        if self.output_len != 0 || self.flush_pending {
            Err(TranslationError::InvalidState(
                "flush pending output before reading or translating",
            ))
        } else {
            Ok(())
        }
    }

    pub(crate) async fn next(&mut self) -> Result<usize, TranslationError> {
        self.ready()?;
        self.no_pending_output()?;
        if self.eof {
            return Ok(0);
        }
        if self.input_start != self.input_end || !self.need_input {
            return Err(TranslationError::InvalidState(
                "translate until NeedInput before reading again",
            ));
        }
        let source = self.source.as_mut().expect("validated source");
        let read = poll_fn(|cx| source.as_mut().poll_read(cx, &mut self.input)).await;
        let len = match read {
            Ok(len) if len <= self.input.len() => len,
            Ok(_) => {
                return Err(self.fail(TranslationError::Stream(StreamError::Input(
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "source reported invalid read length",
                    ),
                ))));
            }
            Err(error) => {
                return Err(self.fail(TranslationError::Stream(StreamError::Input(error))));
            }
        };
        self.input_start = 0;
        self.input_end = len;
        self.eof = len == 0;
        self.need_input = false;
        Ok(len)
    }

    pub(crate) fn translate(&mut self) -> Result<TranslateStep, TranslationError> {
        self.ready()?;
        self.no_pending_output()?;
        if self.phase == Phase::Finished {
            return Ok(TranslateStep::Finished);
        }
        self.need_input = false;
        macro_rules! codec {
            ($call:expr) => {
                match $call {
                    Ok(value) => value,
                    Err(error) => return Err(self.fail(TranslationError::Stream(error.into()))),
                }
            };
        }
        macro_rules! contract {
            ($message:literal) => {
                return Err(self.fail(TranslationError::Stream(
                    CodecError::Contract($message).into(),
                )))
            };
        }
        for _ in 0..self.steps_before_yield {
            self.output.fill(0);
            match codec!(self.codecs.encode(&mut self.output)) {
                EncodeStep::Wrote(len) => {
                    if len == 0 || len > self.output.len() {
                        contract!("encoder reported invalid output length");
                    }
                    self.output_len = len;
                    self.output_written = 0;
                    self.flush_pending = true;
                    return Ok(TranslateStep::OutputReady(len));
                }
                EncodeStep::Finished => {
                    if !self.finishing {
                        contract!("encoder finished before source validation");
                    }
                    self.phase = Phase::Finished;
                    return Ok(TranslateStep::Finished);
                }
                EncodeStep::NeedEvent if self.finishing => {
                    contract!("encoder requested an event after finish");
                }
                EncodeStep::NeedEvent => {}
            }
            let input = &self.input[self.input_start..self.input_end];
            let mut remaining = input;
            let decoded = self.codecs.decode(&mut remaining, self.eof);
            if remaining.len() > input.len()
                || (!remaining.is_empty()
                    && remaining.as_ptr()
                        != input.as_ptr().wrapping_add(input.len() - remaining.len()))
            {
                contract!("decoder did not retain an input suffix");
            }
            let consumed = input.len() - remaining.len();
            self.input_start += consumed;
            match codec!(decoded) {
                DecodeStep::Event(event) => codec!(self.codecs.accept(event)),
                DecodeStep::Progress if consumed == 0 => {
                    contract!("decoder reported progress without consuming bytes");
                }
                DecodeStep::Progress => {}
                DecodeStep::Finished => {
                    if !self.eof || self.input_start != self.input_end {
                        contract!("decoder finished before consuming transport EOF");
                    }
                    codec!(self.codecs.finish());
                    self.finishing = true;
                }
                DecodeStep::NeedInput => {
                    if self.input_start != self.input_end || self.eof {
                        contract!("decoder requested input with unconsumed bytes or at EOF");
                    }
                    self.need_input = true;
                    return Ok(TranslateStep::NeedInput);
                }
            }
        }
        Ok(TranslateStep::Progress)
    }

    pub(crate) async fn flush(&mut self) -> Result<(), TranslationError> {
        self.ready()?;
        self.flush_pending = true;
        let mut writes = 0;
        while self.output_written < self.output_len {
            let target = self.target.as_mut().expect("validated target");
            let bytes = &self.output[self.output_written..self.output_len];
            let written = poll_fn(|cx| target.as_mut().poll_write(cx, bytes)).await;
            let written = match written {
                Ok(0) => {
                    return Err(self.fail(TranslationError::Target(io::Error::new(
                        io::ErrorKind::WriteZero,
                        "target wrote no bytes",
                    ))));
                }
                Ok(len) if len <= self.output_len - self.output_written => len,
                Ok(_) => {
                    return Err(self.fail(TranslationError::Target(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "target reported invalid write length",
                    ))));
                }
                Err(error) => return Err(self.fail(TranslationError::Target(error))),
            };
            self.output_written += written;
            writes += 1;
            if writes == self.steps_before_yield {
                Self::yield_once().await;
                writes = 0;
            }
        }
        let target = self.target.as_mut().expect("validated target");
        if let Err(error) = poll_fn(|cx| target.as_mut().poll_flush(cx)).await {
            return Err(self.fail(TranslationError::Target(error)));
        }
        self.output_len = 0;
        self.output_written = 0;
        self.flush_pending = false;
        Ok(())
    }

    async fn yield_once() {
        let mut yielded = false;
        poll_fn(|cx| {
            if yielded {
                Poll::Ready(())
            } else {
                yielded = true;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        })
        .await
    }
}
