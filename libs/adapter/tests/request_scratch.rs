use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::{Cell, RefCell},
    future::Future,
    io,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll, Waker},
};

use adapter::{
    codec::{DecodeStep, EncodeStep, TranslateStep},
    error::CodecError,
    t2t::{
        Text2TextRequestTranslator,
        anthropic::request::AnthropicRequestEncoder,
        copenai::{CompatibleOpenAIRequestDecoder, CompatibleOpenAIRequestEncoder},
        openai::request::{
            OpenAICompletionRequestDecoder, OpenAICompletionRequestEncoder,
            OpenAIResponseRequestDecoder, OpenAIResponseRequestEncoder,
        },
        request::{
            RequestScratch, decoder::RequestDecoder, encoder::RequestEncoder, event::RequestEvent,
        },
    },
};
use futures_io::AsyncWrite;
use futures_util::io::Cursor;
use serde_json::Value;

thread_local! {
    static COUNT_ALLOCATIONS: Cell<bool> = const { Cell::new(false) };
    static ALLOCATIONS: Cell<usize> = const { Cell::new(0) };
}

struct CountingAllocator;

fn count_allocation() {
    if COUNT_ALLOCATIONS.try_with(Cell::get).unwrap_or(false) {
        let _ = ALLOCATIONS.try_with(|count| count.set(count.get() + 1));
    }
}

// Forward the exact allocation contract to System; counting is thread-local.
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        count_allocation();
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        unsafe { System.dealloc(pointer, layout) }
    }
    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        count_allocation();
        unsafe { System.realloc(pointer, layout, size) }
    }
}

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

fn run<F: Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let mut context = Context::from_waker(Waker::noop());
    for _ in 0..10_000 {
        if let Poll::Ready(value) = future.as_mut().poll(&mut context) {
            return value;
        }
    }
    panic!("future did not complete");
}

#[derive(Default)]
struct Observations {
    address: Cell<usize>,
    capacity: Cell<usize>,
    fragments: Cell<usize>,
    calls: Cell<usize>,
}

struct Inspect<D> {
    decoder: D,
    observations: Rc<Observations>,
}

impl<D: RequestDecoder> RequestDecoder for Inspect<D> {
    fn advance(
        &mut self,
        input: &mut &[u8],
        eof: bool,
        scratch: &mut RequestScratch,
    ) -> Result<DecodeStep<RequestEvent>, CodecError> {
        let address = scratch.text().as_ptr() as usize;
        self.observations.capacity.set(scratch.text().len());
        assert_eq!(scratch.deferred_capacity(), scratch.text().len());
        if self.observations.address.get() == 0 {
            self.observations.address.set(address);
        }
        assert_eq!(
            self.observations.address.get(),
            address,
            "scratch allocation moved"
        );
        self.observations
            .calls
            .set(self.observations.calls.get() + 1);
        let step = self.decoder.advance(input, eof, scratch)?;
        if matches!(step, DecodeStep::Event(RequestEvent::TextFragment(_))) {
            self.observations
                .fragments
                .set(self.observations.fragments.get() + 1);
        }
        Ok(step)
    }
}

#[test]
fn production_translation_allocates_no_payloads_and_reuses_scratch_across_attempts() {
    let text = "quote\" slash\\ null\0 unicode \u{1f600}".repeat(4096);
    // Large role-first text streams; short role-late text fits translator scratch.
    let source = format!(
        r#"{{"model":"source","messages":[{{"role":"user","content":{}}}],"response_format":{{"type":"json_schema","json_schema":{{"name":"answer","strict":true,"schema":{{"type":"object","properties":{{"answer":{{"type":"string","description":{}}}}},"required":["answer"],"additionalProperties":false}}}}}},"stream":true,"top_p":8.5e-1,"stop":["END","\nUser:","\uD83D\uDE00"]}}"#,
        serde_json::to_string(&text).unwrap(),
        serde_json::to_string(&text).unwrap()
    );
    let role_late = r#"{"model":"source","messages":[{"content":"hi","role":"user"}]}"#;
    let output = PausingWriter::default();
    output.0.borrow_mut().bytes = Vec::with_capacity(source.len() * 2);
    output.0.borrow_mut().max_write = Some(usize::MAX);
    let observations = Rc::new(Observations::default());
    let mut translator = Text2TextRequestTranslator::new(3, 7);
    for source in [source.as_str(), role_late, source.as_str()] {
        output.0.borrow_mut().bytes.clear();
        translator
            .set_source_stream(Cursor::new(source.as_bytes()))
            .unwrap()
            .set_target_stream(output.clone())
            .unwrap()
            .set_decoder(Inspect {
                decoder: OpenAICompletionRequestDecoder::default(),
                observations: observations.clone(),
            })
            .unwrap()
            .set_encoder(AnthropicRequestEncoder::new("target", Default::default()).unwrap())
            .unwrap();
        ALLOCATIONS.with(|value| value.set(0));
        COUNT_ALLOCATIONS.with(|value| value.set(true));
        loop {
            match translator.translate().unwrap() {
                TranslateStep::NeedInput => {
                    run(translator.next()).unwrap();
                }
                TranslateStep::OutputReady(_) => run(translator.flush()).unwrap(),
                TranslateStep::Progress => {}
                TranslateStep::Finished => {
                    run(translator.flush()).unwrap();
                    break;
                }
            }
        }
        COUNT_ALLOCATIONS.with(|value| value.set(false));
        assert_eq!(ALLOCATIONS.with(Cell::get), 0);
    }
    drop(translator);
    assert!(observations.fragments.get() > 1000);
    let value: Value = serde_json::from_slice(&output.0.borrow().bytes).unwrap();
    assert_eq!(value["messages"][0]["content"], text);
    assert_eq!(value["max_tokens"], 32768);
    assert_eq!(value["stream"], true);
    assert!(value.get("top_p").is_none());
    assert_eq!(
        value["output_config"]["format"]["schema"]["properties"]["answer"]["description"],
        text
    );
    assert_eq!(
        value["stop_sequences"],
        serde_json::json!(["END", "\nUser:", "😀"])
    );
}

#[test]
fn compatible_tools_and_reasoning_allocate_no_payloads_during_translation() {
    let large = "quote\" escape\\ unicode😀".repeat(8192);
    let mut expected = serde_json::json!({
        "response_format":{"type":"json_schema","json_schema":{"name":"answer","strict":true,"schema":{"type":"object","properties":{"answer":{"type":"string","description":large}},"required":["answer"],"additionalProperties":false}}},
        "provider":{"require_parameters":true},
        "model":"source","stream":true,"stream_options":{"include_usage":true},"top_p":0.85,"top_k":40,"stop":["END","😀"],"tools":[{"type":"function","function":{"name":"f","parameters":{"description":large}}}],
        "messages":[{"role":"assistant","content":null,"tool_calls":[{"id":"id","type":"function","function":{"name":"f","arguments":large}}],
            "reasoning_details":[{"type":"reasoning.encrypted","data":large,"format":"unknown"}]},
            {"role":"tool","tool_call_id":"id","content":large}]
    });
    let source = expected.to_string();
    let writer = PausingWriter::default();
    writer.0.borrow_mut().bytes = Vec::with_capacity(source.len() * 2);
    writer.0.borrow_mut().max_write = Some(usize::MAX);
    let mut translator = Text2TextRequestTranslator::new(7, 64);
    translator
        .set_source_stream(Cursor::new(source.as_bytes()))
        .unwrap()
        .set_target_stream(writer.clone())
        .unwrap()
        .set_decoder(CompatibleOpenAIRequestDecoder::new())
        .unwrap()
        .set_encoder(CompatibleOpenAIRequestEncoder::new("target", Default::default()).unwrap())
        .unwrap();
    ALLOCATIONS.with(|count| count.set(0));
    COUNT_ALLOCATIONS.with(|count| count.set(true));
    loop {
        match translator.translate().unwrap() {
            TranslateStep::NeedInput => {
                run(translator.next()).unwrap();
            }
            TranslateStep::OutputReady(_) => run(translator.flush()).unwrap(),
            TranslateStep::Progress => {}
            TranslateStep::Finished => {
                run(translator.flush()).unwrap();
                break;
            }
        }
    }
    COUNT_ALLOCATIONS.with(|count| count.set(false));
    assert_eq!(ALLOCATIONS.with(Cell::get), 0);
    expected["model"] = serde_json::json!("target");
    expected["stream"] = serde_json::json!(true);
    expected["max_completion_tokens"] = serde_json::json!(32768);
    assert_eq!(
        serde_json::from_slice::<Value>(&writer.0.borrow().bytes).unwrap(),
        expected
    );
}

#[test]
fn compatible_argument_fragments_survive_canceled_partial_flush() {
    let source=br#"{"model":"source","messages":[{"role":"assistant","tool_calls":[{"id":"c","type":"function","function":{"name":"f","arguments":"quote\"\n\uD83D\uDE00"}}],"reasoning_details":[{"type":"reasoning.encrypted","data":"opaque"}]}]}"#;
    let writer = PausingWriter::default();
    let observations = Rc::new(Observations::default());
    let mut translator = Text2TextRequestTranslator::new(3, 64);
    translator
        .set_source_stream(Cursor::new(source))
        .unwrap()
        .set_target_stream(writer.clone())
        .unwrap()
        .set_decoder(Inspect {
            decoder: CompatibleOpenAIRequestDecoder::new(),
            observations: observations.clone(),
        })
        .unwrap()
        .set_encoder(CompatibleOpenAIRequestEncoder::new("target", Default::default()).unwrap())
        .unwrap();
    let mut canceled = false;
    loop {
        match translator.translate().unwrap() {
            TranslateStep::NeedInput => {
                run(translator.next()).unwrap();
            }
            TranslateStep::Progress => {}
            TranslateStep::OutputReady(_) => {
                if !canceled
                    && writer
                        .0
                        .borrow()
                        .bytes
                        .windows(13)
                        .any(|window| window == b"\"arguments\":\"")
                {
                    let calls = observations.calls.get();
                    let writes = writer.0.borrow().writes;
                    writer.0.borrow_mut().block_after = Some(writes + 1);
                    {
                        let mut flush = std::pin::pin!(translator.flush());
                        let mut cx = Context::from_waker(Waker::noop());
                        assert!(flush.as_mut().poll(&mut cx).is_pending());
                    }
                    assert!(translator.translate().is_err());
                    assert_eq!(observations.calls.get(), calls);
                    writer.0.borrow_mut().block_after = None;
                    canceled = true;
                }
                run(translator.flush()).unwrap();
            }
            TranslateStep::Finished => {
                run(translator.flush()).unwrap();
                break;
            }
        }
    }
    assert!(canceled);
    let mut expected: Value = serde_json::from_slice(source).unwrap();
    expected["model"] = serde_json::json!("target");
    expected["stream"] = serde_json::json!(false);
    expected["max_completion_tokens"] = serde_json::json!(32768);
    assert_eq!(
        serde_json::from_slice::<Value>(&writer.0.borrow().bytes).unwrap(),
        expected
    );
}

#[derive(Default)]
struct WriterState {
    bytes: Vec<u8>,
    writes: usize,
    block_after: Option<usize>,
    max_write: Option<usize>,
}

#[derive(Clone, Default)]
struct PausingWriter(Rc<RefCell<WriterState>>);

impl AsyncWrite for PausingWriter {
    fn poll_write(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
        bytes: &[u8],
    ) -> Poll<io::Result<usize>> {
        let mut state = self.0.borrow_mut();
        if state.block_after.is_some_and(|limit| state.writes >= limit) {
            return Poll::Pending;
        }
        let len = bytes.len().min(state.max_write.unwrap_or(1));
        state.bytes.extend_from_slice(&bytes[..len]);
        state.writes += 1;
        Poll::Ready(Ok(len))
    }
    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
    fn poll_close(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        panic!("caller owns closure");
    }
}

#[test]
fn pending_decoded_text_survives_partial_canceled_flush_and_split_escapes() {
    let text = "\0\"\\\n\u{1f600}tail";
    let source = format!(
        r#"{{"model":"source","input":[{{"role":"user","content":{}}}],"text":{{"format":{{"type":"json_schema","name":"answer","strict":true,"schema":{{"type":"object","properties":{{"answer":{{"type":"string","description":{}}}}},"required":["answer"],"additionalProperties":false}}}}}},"max_output_tokens":99}}"#,
        serde_json::to_string(text).unwrap(),
        serde_json::to_string(text).unwrap()
    );
    let writer = PausingWriter::default();
    let observations = Rc::new(Observations::default());
    let mut translator = Text2TextRequestTranslator::new(3, 64);
    translator
        .set_source_stream(Cursor::new(source.as_bytes()))
        .unwrap()
        .set_target_stream(writer.clone())
        .unwrap()
        .set_decoder(Inspect {
            decoder: OpenAIResponseRequestDecoder::new(),
            observations: observations.clone(),
        })
        .unwrap()
        .set_encoder(AnthropicRequestEncoder::new("target", Default::default()).unwrap())
        .unwrap();
    let mut canceled = false;
    loop {
        match translator.translate().unwrap() {
            TranslateStep::NeedInput => {
                run(translator.next()).unwrap();
            }
            TranslateStep::Progress => {}
            TranslateStep::OutputReady(_) => {
                if !canceled
                    && writer
                        .0
                        .borrow()
                        .bytes
                        .windows(9)
                        .any(|bytes| bytes == b"\"schema\":")
                {
                    let calls = observations.calls.get();
                    let writes = writer.0.borrow().writes;
                    writer.0.borrow_mut().block_after = Some(writes + 1);
                    {
                        let mut flush = std::pin::pin!(translator.flush());
                        let mut context = Context::from_waker(Waker::noop());
                        assert!(flush.as_mut().poll(&mut context).is_pending());
                    }
                    assert!(translator.translate().is_err());
                    assert!(run(translator.next()).is_err());
                    assert_eq!(observations.calls.get(), calls);
                    writer.0.borrow_mut().block_after = None;
                    canceled = true;
                }
                run(translator.flush()).unwrap();
            }
            TranslateStep::Finished => {
                run(translator.flush()).unwrap();
                break;
            }
        }
    }
    assert!(canceled);
    let value: Value = serde_json::from_slice(&writer.0.borrow().bytes).unwrap();
    assert_eq!(value["messages"][0]["content"], text);
    assert_eq!(value["max_tokens"], 99);
    assert_eq!(
        value["output_config"]["format"]["schema"]["properties"]["answer"]["description"],
        text
    );
}

#[test]
fn direct_codecs_use_supplied_scratch_and_reject_incorrect_pending_text_lengths() {
    assert!(RequestScratch::new(usize::MAX, 1).is_err());
    assert!(RequestScratch::new(0, 0).is_err());
    for (fragment, deferred) in [(1, 4), (4, 1)] {
        let mut scratch = RequestScratch::new(fragment, deferred).unwrap();
        let mut decoder = OpenAICompletionRequestDecoder::new();
        assert!(matches!(
            decoder
                .advance(&mut &b"{"[..], false, &mut scratch)
                .unwrap(),
            DecodeStep::Progress
        ));
        assert!(decoder.advance(&mut &b""[..], true, &mut scratch).is_err());
    }
    let mut encoder = AnthropicRequestEncoder::new("target", Default::default()).unwrap();
    let mut empty = &b""[..];
    for event in [
        RequestEvent::MessageStarted {
            role: adapter::t2t::request::event::Role::User,
        },
        RequestEvent::TextStarted,
        RequestEvent::TextFragment(4),
    ] {
        while !matches!(
            encoder.advance(&mut empty, &mut [0; 32]).unwrap(),
            EncodeStep::NeedEvent
        ) {}
        encoder.accept(event).unwrap();
    }
    assert!(encoder.advance(&mut &b"abc"[..], &mut [0; 32]).is_err());
    assert!(encoder.advance(&mut &b"abcd"[..], &mut [0; 32]).is_err());
}

#[test]
fn translator_buffer_size_bounds_fragments_and_exact_role_late_deferral() {
    for buffer_bytes in [1, 3, 64, 0] {
        let capacity = if buffer_bytes == 0 {
            16 * 1024
        } else {
            buffer_bytes
        };
        let observations = Rc::new(Observations::default());
        let mut translator = Text2TextRequestTranslator::new(buffer_bytes, 64);
        for length in [capacity, capacity + 1] {
            let content = "x".repeat(length);
            let source = format!(
                r#"{{"model":"source","messages":[{{"content":"{content}","role":"user"}}]}}"#
            );
            translator
                .set_source_stream(Cursor::new(source.into_bytes()))
                .unwrap()
                .set_target_stream(Cursor::new(Vec::new()))
                .unwrap()
                .set_decoder(Inspect {
                    decoder: OpenAICompletionRequestDecoder::new(),
                    observations: observations.clone(),
                })
                .unwrap()
                .set_encoder(AnthropicRequestEncoder::new("target", Default::default()).unwrap())
                .unwrap();
            let result = loop {
                match translator.translate() {
                    Ok(TranslateStep::NeedInput) => {
                        run(translator.next()).unwrap();
                    }
                    Ok(TranslateStep::OutputReady(_)) => run(translator.flush()).unwrap(),
                    Ok(TranslateStep::Progress) => {}
                    Ok(TranslateStep::Finished) => {
                        run(translator.flush()).unwrap();
                        break Ok(());
                    }
                    Err(error) => break Err(error),
                }
            };
            assert_eq!(observations.capacity.get(), capacity);
            if length == capacity {
                assert!(result.is_ok());
                assert_eq!(observations.fragments.get(), 1);
            } else {
                assert!(matches!(
                    result,
                    Err(adapter::error::TranslationError::Stream(
                        adapter::error::StreamError::Codec(CodecError::LimitExceeded(_))
                    ))
                ));
            }
        }
    }
}

#[test]
fn all_encoders_consume_large_fragments_without_a_separate_limit() {
    fn drain(encoder: &mut dyn RequestEncoder, text: &mut &[u8], output: &mut Vec<u8>) -> bool {
        let mut buffer = [0xa5; 8192];
        loop {
            match encoder.advance(text, &mut buffer).unwrap() {
                EncodeStep::Wrote(len) => {
                    assert!((1..=4096).contains(&len));
                    assert!(buffer[4096..].iter().all(|byte| *byte == 0xa5));
                    output.extend_from_slice(&buffer[..len]);
                }
                EncodeStep::NeedEvent => {
                    assert!(text.is_empty());
                    return false;
                }
                EncodeStep::Finished => {
                    assert!(text.is_empty());
                    return true;
                }
            }
        }
    }
    let content = "quote\" slash\\ null\0 unicode 😀".repeat(8192);
    let encoders: [Box<dyn RequestEncoder>; 3] = [
        Box::new(OpenAICompletionRequestEncoder::new("target", Default::default()).unwrap()),
        Box::new(OpenAIResponseRequestEncoder::new("target", Default::default()).unwrap()),
        Box::new(AnthropicRequestEncoder::new("target", Default::default()).unwrap()),
    ];
    for mut encoder in encoders {
        let mut output = Vec::new();
        assert!(!drain(&mut *encoder, &mut &b""[..], &mut output));
        for event in [
            RequestEvent::MessageStarted {
                role: adapter::t2t::request::event::Role::User,
            },
            RequestEvent::TextStarted,
            RequestEvent::TextFragment(content.as_bytes()),
            RequestEvent::TextEnded,
            RequestEvent::MessageEnded,
        ] {
            let mut text = match event {
                RequestEvent::TextFragment(text) => text,
                _ => &[],
            };
            encoder.accept(event.map_text(|text| text.len())).unwrap();
            assert!(!drain(&mut *encoder, &mut text, &mut output));
        }
        encoder.finish().unwrap();
        assert!(drain(&mut *encoder, &mut &b""[..], &mut output));
        let output: Value = serde_json::from_slice(&output).unwrap();
        let messages = output
            .get("messages")
            .or_else(|| output.get("input"))
            .unwrap();
        assert_eq!(messages[0]["content"], content);
    }
}

#[test]
fn translator_rejects_invalid_fragment_lengths_and_abandoned_or_replaced_text() {
    struct Decoder {
        oversized: bool,
    }
    impl RequestDecoder for Decoder {
        fn advance(
            &mut self,
            input: &mut &[u8],
            eof: bool,
            scratch: &mut RequestScratch,
        ) -> Result<DecodeStep<RequestEvent>, CodecError> {
            if let Some((&byte, rest)) = input.split_first() {
                scratch.text_mut()[0] = byte;
                *input = rest;
                Ok(DecodeStep::Event(RequestEvent::TextFragment(
                    if self.oversized { 2 } else { 1 },
                )))
            } else if eof {
                Ok(DecodeStep::Finished)
            } else {
                Ok(DecodeStep::NeedInput)
            }
        }
    }
    struct Encoder {
        mode: u8,
        pending: bool,
    }
    impl RequestEncoder for Encoder {
        fn accept(&mut self, _: RequestEvent) -> Result<(), CodecError> {
            self.pending = true;
            Ok(())
        }
        fn advance(
            &mut self,
            text: &mut &[u8],
            output: &mut [u8],
        ) -> Result<EncodeStep, CodecError> {
            if !self.pending || self.mode == 0 {
                return Ok(EncodeStep::NeedEvent);
            }
            if self.mode == 1 {
                *text = b"unrelated";
            } else {
                *text = b"z";
            }
            output[0] = b'x';
            Ok(EncodeStep::Wrote(1))
        }
        fn finish(&mut self) -> Result<(), CodecError> {
            panic!("invalid codecs must not finalize");
        }
    }
    for (oversized, mode) in [(true, 0), (false, 0), (false, 1), (false, 2)] {
        let mut translator = Text2TextRequestTranslator::new(1, 64);
        translator
            .set_source_stream(Cursor::new(b"x"))
            .unwrap()
            .set_target_stream(Cursor::new(Vec::new()))
            .unwrap()
            .set_decoder(Decoder { oversized })
            .unwrap()
            .set_encoder(Encoder {
                mode,
                pending: false,
            })
            .unwrap();
        run(translator.next()).unwrap();
        assert!(matches!(
            translator.translate(),
            Err(adapter::error::TranslationError::Stream(
                adapter::error::StreamError::Codec(CodecError::Contract(_))
            ))
        ));
    }
}
