//! Explicit translator state-machine tests, shared by request and response APIs.
use std::{
    cell::RefCell,
    future::Future,
    io,
    marker::PhantomPinned,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll, Waker},
};

use adapter::{
    codec::{DecodeStep, EncodeStep, TranslateStep},
    error::{CodecError, StreamError, TranslationError},
    t2t::{
        Text2TextRequestTranslator, Text2TextResponseTranslator,
        request::{
            RequestScratch, decoder::RequestDecoder, encoder::RequestEncoder, event::RequestEvent,
        },
        response::{decoder::ResponseDecoder, encoder::ResponseEncoder, event::ResponseEvent},
    },
};
use bytes::{Buf, Bytes};
use futures_io::{AsyncRead, AsyncWrite};
use futures_util::io::Cursor;

#[derive(Default)]
struct ReadState {
    data: Vec<u8>,
    offset: usize,
    polls: usize,
    drops: usize,
    pending: bool,
    fail: bool,
    invalid_length: bool,
    buffers: Vec<(usize, usize)>,
}
struct Reader {
    state: Rc<RefCell<ReadState>>,
    _pinned: PhantomPinned,
}
impl Reader {
    fn new(data: &[u8]) -> Self {
        Self {
            state: Rc::new(RefCell::new(ReadState {
                data: data.to_vec(),
                ..ReadState::default()
            })),
            _pinned: PhantomPinned,
        }
    }
}
impl Drop for Reader {
    fn drop(&mut self) {
        self.state.borrow_mut().drops += 1;
    }
}
impl AsyncRead for Reader {
    fn poll_read(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
        output: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let mut state = self.state.borrow_mut();
        state.polls += 1;
        state.buffers.push((output.as_ptr() as usize, output.len()));
        if state.pending {
            return Poll::Pending;
        }
        if state.fail {
            return Poll::Ready(Err(io::Error::other("read failed")));
        }
        if state.invalid_length {
            return Poll::Ready(Ok(output.len() + 1));
        }
        let len = output.len().min(state.data.len() - state.offset);
        output[..len].copy_from_slice(&state.data[state.offset..state.offset + len]);
        state.offset += len;
        Poll::Ready(Ok(len))
    }
}

#[derive(Default)]
struct WriteState {
    bytes: Vec<u8>,
    writes: usize,
    flushes: usize,
    closes: usize,
    max_write: Option<usize>,
    block_after: Option<usize>,
    block_flush: bool,
    fail_write: bool,
    fail_flush: bool,
    zero: bool,
    invalid_length: bool,
    buffers: Vec<(usize, usize)>,
}
#[derive(Clone)]
struct Writer {
    state: Rc<RefCell<WriteState>>,
    _pinned: PhantomPinned,
}
impl Default for Writer {
    fn default() -> Self {
        Self {
            state: Rc::default(),
            _pinned: PhantomPinned,
        }
    }
}
impl Writer {
    fn bytes(&self) -> Vec<u8> {
        self.state.borrow().bytes.clone()
    }
}
impl AsyncWrite for Writer {
    fn poll_write(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
        bytes: &[u8],
    ) -> Poll<io::Result<usize>> {
        let mut state = self.state.borrow_mut();
        if state.block_after.is_some_and(|count| state.writes >= count) {
            return Poll::Pending;
        }
        if state.fail_write {
            return Poll::Ready(Err(io::Error::other("write failed")));
        }
        if state.zero {
            return Poll::Ready(Ok(0));
        }
        if state.invalid_length {
            return Poll::Ready(Ok(bytes.len() + 1));
        }
        let len = state.max_write.unwrap_or(bytes.len()).min(bytes.len());
        state.buffers.push((bytes.as_ptr() as usize, bytes.len()));
        state.bytes.extend_from_slice(&bytes[..len]);
        state.writes += 1;
        Poll::Ready(Ok(len))
    }
    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        let mut state = self.state.borrow_mut();
        state.flushes += 1;
        if state.block_flush {
            return Poll::Pending;
        }
        if state.fail_flush {
            return Poll::Ready(Err(io::Error::other("flush failed")));
        }
        Poll::Ready(Ok(()))
    }
    fn poll_close(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.state.borrow_mut().closes += 1;
        Poll::Ready(Ok(()))
    }
}

#[derive(Clone, Copy, Default)]
enum BadDecoder {
    #[default]
    None,
    EarlyFinish,
    NoProgress,
    NeedWithData,
    WrongSuffix,
    GrowingSlice,
    NeedAtEof,
    LateEof,
}
#[derive(Default)]
struct Decoder {
    bad: BadDecoder,
    buffers: Rc<RefCell<Vec<(usize, usize)>>>,
}
macro_rules! decoder {
    ($contract:ident, $event:ident) => {
        impl $contract for Decoder {
            fn advance(
                &mut self,
                input: &mut &[u8],
                eof: bool,
            ) -> Result<DecodeStep<$event>, CodecError> {
                if matches!(self.bad, BadDecoder::EarlyFinish) {
                    return Ok(DecodeStep::Finished);
                }
                if matches!(self.bad, BadDecoder::NoProgress) {
                    return Ok(DecodeStep::Progress);
                }
                if !input.is_empty() {
                    self.buffers
                        .borrow_mut()
                        .push((input.as_ptr() as usize, input.len()));
                    if matches!(self.bad, BadDecoder::NeedWithData) {
                        return Ok(DecodeStep::NeedInput);
                    }
                    if matches!(self.bad, BadDecoder::WrongSuffix) {
                        *input = b"x";
                        return Ok(DecodeStep::Progress);
                    }
                    if matches!(self.bad, BadDecoder::GrowingSlice) {
                        *input = b"larger than the input";
                        return Ok(DecodeStep::Progress);
                    }
                    let len = input.len().min(2);
                    let event = $event::TextFragment(Bytes::copy_from_slice(&input[..len]));
                    input.advance(len);
                    return Ok(DecodeStep::Event(event));
                }
                if eof {
                    if matches!(self.bad, BadDecoder::NeedAtEof) {
                        return Ok(DecodeStep::NeedInput);
                    }
                    if matches!(self.bad, BadDecoder::LateEof) {
                        return Err(CodecError::InvalidInput("incomplete document"));
                    }
                    Ok(DecodeStep::Finished)
                } else {
                    Ok(DecodeStep::NeedInput)
                }
            }
        }
    };
}
decoder!(ResponseDecoder, ResponseEvent);

impl RequestDecoder for Decoder {
    fn advance(
        &mut self,
        input: &mut &[u8],
        eof: bool,
        scratch: &mut RequestScratch,
    ) -> Result<DecodeStep<RequestEvent>, CodecError> {
        Ok(match ResponseDecoder::advance(self, input, eof)? {
            DecodeStep::Event(ResponseEvent::TextFragment(bytes)) => {
                scratch.text_mut()[..bytes.len()].copy_from_slice(&bytes);
                DecodeStep::Event(RequestEvent::TextFragment(bytes.len()))
            }
            DecodeStep::Event(_) => unreachable!(),
            DecodeStep::Progress => DecodeStep::Progress,
            DecodeStep::NeedInput => DecodeStep::NeedInput,
            DecodeStep::Finished => DecodeStep::Finished,
        })
    }
}

#[derive(Clone, Copy, Default)]
enum BadEncoder {
    #[default]
    None,
    Zero,
    Oversized,
    EarlyFinish,
    NeedAfterFinish,
    Accept,
    Finish,
}
#[derive(Default)]
struct Encoder {
    pending: Bytes,
    request_pending: usize,
    finished: bool,
    bad: BadEncoder,
    buffers: Rc<RefCell<Vec<(usize, usize)>>>,
}
macro_rules! encoder {
    ($contract:ident, $event:ident) => {
        impl $contract for Encoder {
            fn accept(&mut self, event: $event) -> Result<(), CodecError> {
                if matches!(self.bad, BadEncoder::Accept) {
                    return Err(CodecError::Unsupported("event"));
                }
                assert!(self.pending.is_empty());
                let $event::TextFragment(bytes) = event else {
                    return Err(CodecError::Unsupported("text only"));
                };
                self.pending = bytes;
                Ok(())
            }
            fn advance(&mut self, output: &mut [u8]) -> Result<EncodeStep, CodecError> {
                self.buffers
                    .borrow_mut()
                    .push((output.as_ptr() as usize, output.len()));
                match self.bad {
                    BadEncoder::Zero => return Ok(EncodeStep::Wrote(0)),
                    BadEncoder::Oversized => return Ok(EncodeStep::Wrote(output.len() + 1)),
                    BadEncoder::EarlyFinish => return Ok(EncodeStep::Finished),
                    BadEncoder::NeedAfterFinish if self.finished => {
                        return Ok(EncodeStep::NeedEvent);
                    }
                    _ => {}
                }
                if !self.pending.is_empty() {
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
                if matches!(self.bad, BadEncoder::Finish) {
                    return Err(CodecError::Unsupported("finalization"));
                }
                assert!(!self.finished);
                self.finished = true;
                self.pending = Bytes::from_static(b"!");
                Ok(())
            }
        }
    };
}
encoder!(ResponseEncoder, ResponseEvent);

impl RequestEncoder for Encoder {
    fn accept(&mut self, event: RequestEvent) -> Result<(), CodecError> {
        ResponseEncoder::accept(self, ResponseEvent::TextFragment(Bytes::new()))?;
        let RequestEvent::TextFragment(len) = event else {
            return Err(CodecError::Unsupported("text only"));
        };
        self.request_pending = len;
        Ok(())
    }
    fn advance(&mut self, text: &mut &[u8], output: &mut [u8]) -> Result<EncodeStep, CodecError> {
        if matches!(
            self.bad,
            BadEncoder::None
                | BadEncoder::Accept
                | BadEncoder::Finish
                | BadEncoder::NeedAfterFinish
        ) && self.request_pending > 0
        {
            self.buffers
                .borrow_mut()
                .push((output.as_ptr() as usize, output.len()));
            let len = self.request_pending.min(output.len());
            output[..len].copy_from_slice(&text[..len]);
            text.advance(len);
            self.request_pending -= len;
            Ok(EncodeStep::Wrote(len))
        } else {
            ResponseEncoder::advance(self, output)
        }
    }
    fn finish(&mut self) -> Result<(), CodecError> {
        ResponseEncoder::finish(self)
    }
}

fn run<F: Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let mut cx = Context::from_waker(Waker::noop());
    for _ in 0..1000 {
        if let Poll::Ready(output) = future.as_mut().poll(&mut cx) {
            return output;
        }
    }
    panic!("operation did not finish");
}

macro_rules! translator_tests {
    ($module:ident, $translator:ident) => {
        mod $module {
            use super::*;
            type Subject = $translator<Reader, Writer, Decoder, Encoder>;
            fn configure(subject: &mut Subject, reader: Reader, writer: Writer, decoder: Decoder, encoder: Encoder) {
                subject.set_source_stream(reader).unwrap().set_target_stream(writer).unwrap().set_decoder(decoder).unwrap().set_encoder(encoder).unwrap();
            }
            fn drive(subject: &mut Subject) -> Result<(), TranslationError> {
                for _ in 0..1000 {
                    match subject.translate()? {
                        TranslateStep::NeedInput => { run(subject.next())?; }
                        TranslateStep::OutputReady(_) => run(subject.flush())?,
                        TranslateStep::Progress => {}
                        TranslateStep::Finished => { run(subject.flush())?; return Ok(()); }
                    }
                }
                panic!("translation did not finish");
            }
            fn output_ready(subject: &mut Subject) -> usize {
                for _ in 0..1000 {
                    match subject.translate().unwrap() {
                        TranslateStep::OutputReady(len) => return len,
                        TranslateStep::NeedInput => { run(subject.next()).unwrap(); }
                        TranslateStep::Progress => {}
                        TranslateStep::Finished => panic!("expected output"),
                    }
                }
                panic!("no output");
            }

            #[test]
            fn setters_are_lazy_and_all_components_are_required() {
                let mut subject = Subject::new(2, 3);
                assert!(matches!(subject.translate(), Err(TranslationError::MissingComponent("source_stream"))));
                assert!(matches!(run(subject.next()), Err(TranslationError::MissingComponent("source_stream"))));
                let reader = Reader::new(b"hello"); let state = reader.state.clone();
                subject.set_source_stream(reader).unwrap();
                assert!(matches!(run(subject.flush()), Err(TranslationError::MissingComponent("target_stream"))));
                subject.set_target_stream(Writer::default()).unwrap();
                assert!(matches!(subject.translate(), Err(TranslationError::MissingComponent("decoder"))));
                subject.set_decoder(Decoder::default()).unwrap();
                assert!(matches!(subject.translate(), Err(TranslationError::MissingComponent("encoder"))));
                subject.set_encoder(Encoder::default()).unwrap();
                assert_eq!(state.borrow().polls, 0);
                assert_eq!(run(subject.next()).unwrap(), 2);
            }

            #[test]
            fn caller_drives_many_chunks_and_flush_never_closes() {
                let mut subject = Subject::new(2, 1); let reader = Reader::new(b"hello world"); let state = reader.state.clone(); let writer = Writer::default();
                configure(&mut subject, reader, writer.clone(), Decoder::default(), Encoder::default());
                drive(&mut subject).unwrap();
                assert_eq!(writer.bytes(), b"hello world!");
                assert!(writer.state.borrow().flushes > 1); assert_eq!(writer.state.borrow().closes, 0);
                let reads = state.borrow().polls;
                assert_eq!(run(subject.next()).unwrap(), 0);
                assert_eq!(subject.translate().unwrap(), TranslateStep::Finished);
                run(subject.flush()).unwrap(); assert_eq!(state.borrow().polls, reads);
            }

            #[test]
            fn next_translate_and_flush_have_separate_responsibilities() {
                let mut subject = Subject::new(2, 64); let reader = Reader::new(b"hello"); let state = reader.state.clone(); let writer = Writer::default();
                configure(&mut subject, reader, writer.clone(), Decoder::default(), Encoder::default());
                assert_eq!(run(subject.next()).unwrap(), 2);
                assert!(writer.bytes().is_empty());
                assert_eq!(subject.translate().unwrap(), TranslateStep::OutputReady(2));
                assert_eq!(state.borrow().polls, 1); assert!(writer.bytes().is_empty());
                run(subject.flush()).unwrap(); assert_eq!(writer.bytes(), b"he"); assert_eq!(state.borrow().polls, 1);
                drive(&mut subject).unwrap(); assert_eq!(writer.bytes(), b"hello!");
            }

            #[test]
            fn pending_input_and_output_cannot_be_overwritten() {
                let mut subject = Subject::new(2, 64); let reader = Reader::new(b"hello"); let state = reader.state.clone(); let writer = Writer::default();
                configure(&mut subject, reader, writer.clone(), Decoder::default(), Encoder::default());
                run(subject.next()).unwrap();
                assert!(matches!(run(subject.next()), Err(TranslationError::InvalidState(_))));
                assert_eq!(subject.translate().unwrap(), TranslateStep::OutputReady(2));
                assert!(matches!(run(subject.next()), Err(TranslationError::InvalidState(_))));
                assert!(matches!(subject.translate(), Err(TranslationError::InvalidState(_))));
                assert_eq!(state.borrow().polls, 1);
                run(subject.flush()).unwrap(); drive(&mut subject).unwrap(); assert_eq!(writer.bytes(), b"hello!");
            }

            #[test]
            fn active_setters_reject_replacement_without_corrupting_progress() {
                let mut subject = Subject::new(2, 64); let writer = Writer::default();
                configure(&mut subject, Reader::new(b"original"), writer.clone(), Decoder::default(), Encoder::default());
                run(subject.next()).unwrap();
                assert!(subject.set_source_stream(Reader::new(b"wrong")).is_err());
                assert!(subject.set_target_stream(Writer::default()).is_err());
                assert!(subject.set_decoder(Decoder::default()).is_err());
                assert!(subject.set_encoder(Encoder::default()).is_err());
                drive(&mut subject).unwrap(); assert_eq!(writer.bytes(), b"original!");
            }

            #[test]
            fn canceled_next_resumes_with_the_same_stored_reader() {
                let mut subject = Subject::new(2, 64); let reader = Reader::new(b"hello"); let state = reader.state.clone(); state.borrow_mut().pending = true; let writer = Writer::default();
                configure(&mut subject, reader, writer.clone(), Decoder::default(), Encoder::default());
                { let mut next = std::pin::pin!(subject.next()); assert!(next.as_mut().poll(&mut Context::from_waker(Waker::noop())).is_pending()); }
                assert_eq!(state.borrow().drops, 0); assert_eq!(state.borrow().offset, 0);
                state.borrow_mut().pending = false; run(subject.next()).unwrap(); drive(&mut subject).unwrap(); assert_eq!(writer.bytes(), b"hello!");
                assert!(state.borrow().buffers.iter().all(|buffer| *buffer == state.borrow().buffers[0]));
                drop(subject); assert_eq!(state.borrow().drops, 1);
            }

            #[test]
            fn canceled_partial_flush_resumes_without_duplicate_bytes() {
                let mut subject = Subject::new(2, 64); let writer = Writer::default();
                writer.state.borrow_mut().max_write = Some(1); writer.state.borrow_mut().block_after = Some(1);
                configure(&mut subject, Reader::new(b"hello"), writer.clone(), Decoder::default(), Encoder::default());
                assert_eq!(output_ready(&mut subject), 2);
                { let mut flush = std::pin::pin!(subject.flush()); assert!(flush.as_mut().poll(&mut Context::from_waker(Waker::noop())).is_pending()); }
                assert_eq!(writer.bytes(), b"h");
                assert!(subject.translate().is_err()); assert!(run(subject.next()).is_err()); assert!(subject.set_encoder(Encoder::default()).is_err());
                writer.state.borrow_mut().block_after = None; run(subject.flush()).unwrap(); assert_eq!(writer.bytes(), b"he");
                drive(&mut subject).unwrap(); assert_eq!(writer.bytes(), b"hello!");
                let buffers = &writer.state.borrow().buffers;
                assert_eq!(buffers[1], (buffers[0].0 + 1, 1));
            }

            #[test]
            fn canceled_target_flush_retries_flush_but_not_writes() {
                let mut subject = Subject::new(2, 64); let writer = Writer::default(); writer.state.borrow_mut().block_flush = true;
                configure(&mut subject, Reader::new(b"hello"), writer.clone(), Decoder::default(), Encoder::default()); output_ready(&mut subject);
                { let mut flush = std::pin::pin!(subject.flush()); assert!(flush.as_mut().poll(&mut Context::from_waker(Waker::noop())).is_pending()); }
                let writes = writer.state.borrow().writes; assert_eq!(writer.bytes(), b"he"); assert!(subject.translate().is_err());
                writer.state.borrow_mut().block_flush = false; run(subject.flush()).unwrap(); assert_eq!(writer.state.borrow().writes, writes);
                drive(&mut subject).unwrap(); assert_eq!(writer.bytes(), b"hello!");
            }

            #[test]
            fn finished_pending_flush_blocks_reconfiguration_until_resumed() {
                let mut subject = Subject::new(2, 64); let writer = Writer::default();
                configure(&mut subject, Reader::new(b"hello"), writer.clone(), Decoder::default(), Encoder::default()); drive(&mut subject).unwrap();
                writer.state.borrow_mut().block_flush = true;
                { let mut flush = std::pin::pin!(subject.flush()); assert!(flush.as_mut().poll(&mut Context::from_waker(Waker::noop())).is_pending()); }
                assert!(subject.set_decoder(Decoder::default()).is_err()); assert!(subject.translate().is_err());
                writer.state.borrow_mut().block_flush = false; run(subject.flush()).unwrap();
                subject.set_decoder(Decoder::default()).unwrap();
                assert!(matches!(subject.translate(), Err(TranslationError::MissingComponent("source_stream"))));
            }

            #[test]
            fn explicit_progress_bounds_translation_without_hidden_io() {
                let mut subject = Subject::new(2, 1); let reader = Reader::new(b"hello"); let state = reader.state.clone(); let writer = Writer::default();
                configure(&mut subject, reader, writer.clone(), Decoder::default(), Encoder::default()); run(subject.next()).unwrap();
                assert_eq!(subject.translate().unwrap(), TranslateStep::Progress);
                assert!(matches!(run(subject.next()), Err(TranslationError::InvalidState(_))));
                assert_eq!(state.borrow().polls, 1); assert!(writer.bytes().is_empty());
                assert_eq!(subject.translate().unwrap(), TranslateStep::OutputReady(2));
                run(subject.flush()).unwrap(); drive(&mut subject).unwrap();
            }

            #[test]
            fn encoder_header_can_require_multiple_outputs_before_input() {
                let mut subject = Subject::new(2, 64); let reader = Reader::new(b"hello"); let state = reader.state.clone(); let writer = Writer::default();
                configure(&mut subject, reader, writer.clone(), Decoder::default(), Encoder { pending: Bytes::from_static(b"HEAD"), ..Encoder::default() });
                assert_eq!(subject.translate().unwrap(), TranslateStep::OutputReady(2)); run(subject.flush()).unwrap();
                assert_eq!(subject.translate().unwrap(), TranslateStep::OutputReady(2)); run(subject.flush()).unwrap();
                assert_eq!(state.borrow().polls, 0); assert_eq!(subject.translate().unwrap(), TranslateStep::NeedInput);
                drive(&mut subject).unwrap(); assert_eq!(writer.bytes(), b"HEADhello!");
            }

            #[test]
            fn buffer_addresses_survive_completion_errors_and_reconfiguration() {
                let mut subject = Subject::new(8, 64);
                let mut original_input = None; let mut original_output = None;
                for outcome in 0..5 {
                    let data = if outcome == 3 { b"".as_slice() } else { b"fresh".as_slice() };
                    let reader = Reader::new(data); let reads = reader.state.clone(); reads.borrow_mut().fail = outcome == 1;
                    let writer = Writer::default(); writer.state.borrow_mut().fail_write = outcome == 2;
                    let decoder = Decoder::default(); let decoder_buffers = decoder.buffers.clone();
                    let encoder = Encoder::default(); let encoder_buffers = encoder.buffers.clone();
                    configure(&mut subject, reader, writer.clone(), decoder, encoder);
                    let result = drive(&mut subject);
                    if outcome == 1 || outcome == 2 { assert!(result.is_err()); } else { result.unwrap(); let mut expected = data.to_vec(); expected.push(b'!'); assert_eq!(writer.bytes(), expected); }
                    let input = reads.borrow().buffers[0]; let output = encoder_buffers.borrow()[0];
                    assert_eq!(input.1, 8); assert_eq!(output.1, 8); assert_ne!(input.0, output.0);
                    if let Some(previous) = original_input { assert_eq!(previous, input); }
                    if let Some(previous) = original_output { assert_eq!(previous, output); }
                    original_input = Some(input); original_output = Some(output);
                    assert!(decoder_buffers.borrow().iter().all(|(pointer, len)| *pointer >= input.0 && pointer + len <= input.0 + input.1));
                    assert!(writer.state.borrow().buffers.iter().all(|(pointer, _)| *pointer == output.0));
                }
            }

            #[test]
            fn first_setter_after_error_clears_every_old_component() {
                let mut subject = Subject::new(2, 64); let reader = Reader::new(b"old"); let old = reader.state.clone(); old.borrow_mut().fail = true;
                configure(&mut subject, reader, Writer::default(), Decoder::default(), Encoder::default());
                assert!(drive(&mut subject).is_err()); assert!(subject.translate().is_err()); assert!(run(subject.flush()).is_err()); assert!(run(subject.next()).is_err());
                subject.set_encoder(Encoder::default()).unwrap(); assert_eq!(old.borrow().drops, 1);
                assert!(matches!(subject.translate(), Err(TranslationError::MissingComponent("source_stream"))));
                subject.set_source_stream(Reader::new(b"fresh")).unwrap();
                assert!(matches!(subject.translate(), Err(TranslationError::MissingComponent("target_stream"))));
                let writer = Writer::default(); subject.set_target_stream(writer.clone()).unwrap();
                assert!(matches!(subject.translate(), Err(TranslationError::MissingComponent("decoder"))));
                subject.set_decoder(Decoder::default()).unwrap(); drive(&mut subject).unwrap(); assert_eq!(writer.bytes(), b"fresh!");
            }

            #[test]
            fn eof_validation_failure_does_not_emit_finalization() {
                let mut subject = Subject::new(2, 64); let writer = Writer::default();
                configure(&mut subject, Reader::new(b"partial"), writer.clone(), Decoder { bad: BadDecoder::LateEof, ..Decoder::default() }, Encoder::default());
                assert!(matches!(drive(&mut subject), Err(TranslationError::Stream(StreamError::Codec(CodecError::InvalidInput(_))))));
                assert_eq!(writer.bytes(), b"partial"); assert_eq!(writer.state.borrow().closes, 0);
            }

            #[test]
            fn invalid_decoder_steps_fail_without_successful_completion() {
                for bad in [BadDecoder::EarlyFinish, BadDecoder::NoProgress, BadDecoder::NeedWithData, BadDecoder::WrongSuffix, BadDecoder::GrowingSlice, BadDecoder::NeedAtEof] {
                    let mut subject = Subject::new(2, 64); let writer = Writer::default();
                    let data = if matches!(bad, BadDecoder::NeedAtEof) { b"".as_slice() } else { b"original".as_slice() };
                    configure(&mut subject, Reader::new(data), writer.clone(), Decoder { bad, ..Decoder::default() }, Encoder::default());
                    assert!(matches!(drive(&mut subject), Err(TranslationError::Stream(StreamError::Codec(CodecError::Contract(_))))));
                    assert!(writer.bytes().is_empty()); assert_eq!(writer.state.borrow().closes, 0);
                }
            }

            #[test]
            fn invalid_encoder_steps_and_codec_errors_fail_explicitly() {
                for bad in [BadEncoder::Zero, BadEncoder::Oversized, BadEncoder::EarlyFinish, BadEncoder::NeedAfterFinish, BadEncoder::Accept, BadEncoder::Finish] {
                    let mut subject = Subject::new(2, 64); let writer = Writer::default();
                    configure(&mut subject, Reader::new(b"original"), writer.clone(), Decoder::default(), Encoder { bad, ..Encoder::default() });
                    assert!(matches!(drive(&mut subject), Err(TranslationError::Stream(StreamError::Codec(_)))));
                    assert_eq!(writer.state.borrow().closes, 0);
                }
            }

            #[test]
            fn invalid_read_lengths_and_writer_lengths_are_terminal() {
                let mut subject = Subject::new(2, 64); let reader = Reader::new(b"hello"); reader.state.borrow_mut().invalid_length = true;
                configure(&mut subject, reader, Writer::default(), Decoder::default(), Encoder::default());
                assert!(matches!(drive(&mut subject), Err(TranslationError::Stream(StreamError::Input(error))) if error.kind() == io::ErrorKind::InvalidData));
                for zero in [false, true] {
                    let writer = Writer::default(); writer.state.borrow_mut().zero = zero; writer.state.borrow_mut().invalid_length = !zero;
                    configure(&mut subject, Reader::new(b"hello"), writer.clone(), Decoder::default(), Encoder::default());
                    let expected = if zero { io::ErrorKind::WriteZero } else { io::ErrorKind::InvalidData };
                    assert!(matches!(drive(&mut subject), Err(TranslationError::Target(error)) if error.kind() == expected));
                    assert_eq!(writer.state.borrow().flushes, 0); assert_eq!(writer.state.borrow().closes, 0);
                }
            }

            #[test]
            fn flush_failure_does_not_close_and_can_start_a_fresh_attempt() {
                let mut subject = Subject::new(2, 64); let writer = Writer::default(); writer.state.borrow_mut().fail_flush = true;
                configure(&mut subject, Reader::new(b"hello"), writer.clone(), Decoder::default(), Encoder::default());
                assert!(matches!(drive(&mut subject), Err(TranslationError::Target(error)) if error.to_string() == "flush failed"));
                assert_eq!(writer.state.borrow().closes, 0);
                let fresh = Writer::default(); configure(&mut subject, Reader::new(b"fresh"), fresh.clone(), Decoder::default(), Encoder::default());
                drive(&mut subject).unwrap(); assert_eq!(fresh.bytes(), b"fresh!");
            }

            #[test]
            fn zero_configuration_uses_default_sizes_and_empty_input_finalizes() {
                let mut subject = Subject::new(0, 0); let reader = Reader::new(b""); let state = reader.state.clone();
                let encoder = Encoder::default(); let output = encoder.buffers.clone(); let writer = Writer::default();
                configure(&mut subject, reader, writer.clone(), Decoder::default(), encoder); drive(&mut subject).unwrap();
                assert_eq!(state.borrow().buffers[0].1, 16 * 1024); assert_eq!(output.borrow()[0].1, 16 * 1024); assert_eq!(writer.bytes(), b"!");
            }

            #[test]
            fn dropping_an_unpolled_step_keeps_components_until_translator_drop() {
                let mut subject = Subject::new(2, 64); let reader = Reader::new(b"hello"); let state = reader.state.clone();
                configure(&mut subject, reader, Writer::default(), Decoder::default(), Encoder::default());
                drop(subject.next()); assert_eq!(state.borrow().polls, 0); assert_eq!(state.borrow().drops, 0);
                drop(subject); assert_eq!(state.borrow().drops, 1);
            }

            #[test]
            fn ready_partial_writes_yield_and_resume_the_saved_offset() {
                let mut subject = Subject::new(16, 1); let writer = Writer::default(); writer.state.borrow_mut().max_write = Some(1);
                configure(&mut subject, Reader::new(b"hello"), writer.clone(), Decoder::default(), Encoder::default()); output_ready(&mut subject);
                { let mut flush = std::pin::pin!(subject.flush()); assert!(flush.as_mut().poll(&mut Context::from_waker(Waker::noop())).is_pending()); }
                assert_eq!(writer.bytes(), b"h"); run(subject.flush()).unwrap(); assert_eq!(writer.bytes(), b"he"); drive(&mut subject).unwrap(); assert_eq!(writer.bytes(), b"hello!");
            }
        }
    };
}
translator_tests!(request, Text2TextRequestTranslator);
translator_tests!(response, Text2TextResponseTranslator);

#[test]
fn generic_translators_remain_send_when_explicit_component_types_are_send() {
    fn assert_send<T: Send>() {}
    type SendRequest = Text2TextRequestTranslator<
        Cursor<Vec<u8>>,
        Cursor<Vec<u8>>,
        Box<dyn RequestDecoder + Send>,
        Box<dyn RequestEncoder + Send>,
    >;
    type SendResponse = Text2TextResponseTranslator<
        Cursor<Vec<u8>>,
        Cursor<Vec<u8>>,
        Box<dyn ResponseDecoder + Send>,
        Box<dyn ResponseEncoder + Send>,
    >;
    assert_send::<SendRequest>();
    assert_send::<SendResponse>();
    fn future_is_send<F: Send>(_: F) {}
    let mut request = SendRequest::new(2, 64);
    future_is_send(request.next());
    future_is_send(request.flush());
    let mut response = SendResponse::new(2, 64);
    future_is_send(response.next());
    future_is_send(response.flush());
}

#[test]
fn explicit_boxed_codec_types_allow_heterogeneous_setter_values() {
    type Subject = Text2TextRequestTranslator<
        Cursor<Vec<u8>>,
        Writer,
        Box<dyn RequestDecoder>,
        Box<dyn RequestEncoder>,
    >;
    struct OtherDecoder(Decoder);
    impl RequestDecoder for OtherDecoder {
        fn advance(
            &mut self,
            input: &mut &[u8],
            eof: bool,
            scratch: &mut RequestScratch,
        ) -> Result<DecodeStep<RequestEvent>, CodecError> {
            RequestDecoder::advance(&mut self.0, input, eof, scratch)
        }
    }
    struct OtherEncoder(Encoder);
    impl RequestEncoder for OtherEncoder {
        fn accept(&mut self, event: RequestEvent) -> Result<(), CodecError> {
            RequestEncoder::accept(&mut self.0, event)
        }
        fn advance(
            &mut self,
            text: &mut &[u8],
            output: &mut [u8],
        ) -> Result<EncodeStep, CodecError> {
            RequestEncoder::advance(&mut self.0, text, output)
        }
        fn finish(&mut self) -> Result<(), CodecError> {
            RequestEncoder::finish(&mut self.0)
        }
    }
    let mut subject = Subject::new(2, 64);
    let first = Writer::default();
    subject
        .set_source_stream(Cursor::new(b"hello".to_vec()))
        .unwrap()
        .set_target_stream(first.clone())
        .unwrap()
        .set_decoder(Box::new(Decoder::default()))
        .unwrap()
        .set_encoder(Box::new(Encoder::default()))
        .unwrap();
    for _ in 0..1000 {
        match subject.translate().unwrap() {
            TranslateStep::NeedInput => {
                run(subject.next()).unwrap();
            }
            TranslateStep::OutputReady(_) => {
                run(subject.flush()).unwrap();
            }
            TranslateStep::Progress => {}
            TranslateStep::Finished => break,
        }
    }
    subject
        .set_decoder(Box::new(OtherDecoder(Decoder::default())))
        .unwrap();
    assert_eq!(first.bytes(), b"hello!");
    let second = Writer::default();
    subject
        .set_source_stream(Cursor::new(b"again".to_vec()))
        .unwrap()
        .set_target_stream(second.clone())
        .unwrap()
        .set_encoder(Box::new(OtherEncoder(Encoder {
            pending: Bytes::from_static(b"B:"),
            ..Encoder::default()
        })))
        .unwrap();
    let mut finished = false;
    for _ in 0..1000 {
        match subject.translate().unwrap() {
            TranslateStep::NeedInput => {
                run(subject.next()).unwrap();
            }
            TranslateStep::OutputReady(_) => {
                run(subject.flush()).unwrap();
            }
            TranslateStep::Progress => {}
            TranslateStep::Finished => {
                finished = true;
                break;
            }
        }
    }
    assert!(finished);
    assert_eq!(second.bytes(), b"B:again!");
}

#[test]
fn borrowed_components_work_without_static_bounds_and_only_the_caller_closes() {
    macro_rules! check {
        ($translator:ident) => {{
            let body = b"borrowed".to_vec();
            let mut target = Box::pin(Writer::default());
            let observed = target.as_ref().get_ref().clone();
            {
                let mut subject = $translator::new(2, 64);
                subject
                    .set_source_stream(Cursor::new(body.as_slice()))
                    .unwrap()
                    .set_target_stream(target.as_mut())
                    .unwrap()
                    .set_decoder(Decoder::default())
                    .unwrap()
                    .set_encoder(Encoder::default())
                    .unwrap();
                let mut finished = false;
                for _ in 0..1000 {
                    match subject.translate().unwrap() {
                        TranslateStep::NeedInput => {
                            run(subject.next()).unwrap();
                        }
                        TranslateStep::OutputReady(_) => {
                            run(subject.flush()).unwrap();
                        }
                        TranslateStep::Progress => {}
                        TranslateStep::Finished => {
                            run(subject.flush()).unwrap();
                            finished = true;
                            break;
                        }
                    }
                }
                assert!(finished);
                assert_eq!(observed.bytes(), b"borrowed!");
                assert_eq!(observed.state.borrow().closes, 0);
            }
            run(std::future::poll_fn(|cx| target.as_mut().poll_close(cx))).unwrap();
            assert_eq!(observed.state.borrow().closes, 1);
        }};
    }
    check!(Text2TextRequestTranslator);
    check!(Text2TextResponseTranslator);
}
