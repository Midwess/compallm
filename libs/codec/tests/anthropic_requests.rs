use codec::decoder::{Decoder, DecoderStatus};
use codec::encoder::{Encoder, EncoderStatus};
use codec::json::{JsonDecoder, JsonEncoder};
use codec::llm::text::anthropic::request::{AnthropicRequestDecoder, AnthropicRequestEncoder};
use codec::llm::text::component::TextRequestComponent;
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

struct CountingAllocator;

#[derive(Clone, Copy, Debug, Default)]
struct AllocationCount {
    allocations: usize,
    reallocations: usize,
    requested_bytes: usize,
}

thread_local! {
    static ALLOCATION_COUNT: Cell<Option<AllocationCount>> = const { Cell::new(None) };
}

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

fn record_allocation(bytes: usize, reallocation: bool) {
    let _ = ALLOCATION_COUNT.try_with(|count| {
        if let Some(mut value) = count.get() {
            if reallocation {
                value.reallocations += 1;
            } else {
                value.allocations += 1;
            }
            value.requested_bytes += bytes;
            count.set(Some(value));
        }
    });
}

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            record_allocation(layout.size(), false);
        }
        pointer
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let pointer = unsafe { System.alloc_zeroed(layout) };
        if !pointer.is_null() {
            record_allocation(layout.size(), false);
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        unsafe {
            System.dealloc(pointer, layout);
        }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        let pointer = unsafe { System.realloc(pointer, layout, size) };
        if !pointer.is_null() {
            record_allocation(size, true);
        }
        pointer
    }
}

struct AllocationMeasurement;

impl AllocationMeasurement {
    fn start() -> Self {
        ALLOCATION_COUNT.with(|count| {
            assert!(count.get().is_none());
            count.set(Some(AllocationCount::default()));
        });
        Self
    }

    fn snapshot(&self) -> AllocationCount {
        ALLOCATION_COUNT.with(|count| count.get().unwrap())
    }
}

impl Drop for AllocationMeasurement {
    fn drop(&mut self) {
        ALLOCATION_COUNT.with(|count| count.set(None));
    }
}

const REQUEST: &[u8] = br#"{"model":"claude-model","system":"Be helpful\nplease","messages":[{"content":"hello \ud83e\udd80","role":"user"},{"role":"assistant","content":"Hi"}],"max_tokens":1024,"stop_sequences":["END","\nSTOP"],"stream":true}"#;

fn translate<const N: usize>() {
    let mut decoder = AnthropicRequestDecoder::new(JsonDecoder::new());
    let mut encoder = AnthropicRequestEncoder::new(JsonEncoder::new(), Default::default());
    let mut encoded = [0; 4096];
    let mut length = 0;
    let mut output = [0; N];
    let mut buffer = [0; N];
    let measurement = AllocationMeasurement::start();
    let mut count = 0;
    for chunk in REQUEST.chunks(N) {
        buffer[..chunk.len()].copy_from_slice(chunk);
        let mut input = &buffer[..chunk.len()];
        loop {
            match decoder.next(&mut input, false).unwrap() {
                DecoderStatus::Component(mut component) => {
                    count += 1;
                    loop {
                        let status = encoder.next(Some(&mut component), &mut output).unwrap();
                        let (written, done) = match status {
                            EncoderStatus::NeedOutput { written } => (written, false),
                            EncoderStatus::ComponentFinished { written } => (written, true),
                            _ => panic!("unexpected finish"),
                        };
                        encoded[length..length + written].copy_from_slice(&output[..written]);
                        length += written;
                        if done {
                            break;
                        }
                    }
                }
                DecoderStatus::NeedInput => {
                    assert!(input.is_empty());
                    break;
                }
                _ => panic!("finished without EOF"),
            }
        }
        buffer.fill(b'!');
    }
    assert!(matches!(
        decoder.next(&mut &b""[..], true).unwrap(),
        DecoderStatus::Finished
    ));
    decoder.finish().unwrap();
    loop {
        let status = encoder
            .next(None::<&mut TextRequestComponent>, &mut output)
            .unwrap();
        let (written, done) = match status {
            EncoderStatus::NeedOutput { written } => (written, false),
            EncoderStatus::Finished { written } => (written, true),
            _ => panic!("unexpected component"),
        };
        encoded[length..length + written].copy_from_slice(&output[..written]);
        length += written;
        if done {
            break;
        }
    }
    encoder.finish().unwrap();
    let allocations = measurement.snapshot();
    drop(measurement);
    assert_eq!(allocations.allocations + allocations.reallocations, 0);
    assert_eq!(count, 7);
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&encoded[..length]).unwrap(),
        serde_json::from_slice::<serde_json::Value>(REQUEST).unwrap()
    );
    println!(
        "{N}-byte buffers: {count} components, {length} output bytes; processing: {allocations:?}"
    );
}

#[test]
fn translates_borrowed_requests_without_processing_allocations() {
    translate::<1>();
    translate::<16>();
    translate::<16384>();
    translate::<32768>();
    translate::<65536>();
}

#[test]
fn borrows_complete_input_and_translates_to_compatible_openai() {
    let source = br#"{"model":"claude","max_tokens":100,"system":"help","messages":[{"role":"user","content":"hello"}]}"#;
    let range = source.as_ptr() as usize..source.as_ptr() as usize + source.len();
    let mut decoder = AnthropicRequestDecoder::new(JsonDecoder::new());
    let mut encoder = codec::llm::text::copenai::request::CompatibleOpenAIRequestEncoder::new(
        JsonEncoder::new(),
        Default::default(),
    );
    let mut input = source.as_slice();
    let mut output = [0; 4096];
    let mut encoded = Vec::new();
    let mut strings = 0;
    loop {
        match decoder.next(&mut input, true).unwrap() {
            DecoderStatus::Component(component) => {
                let mut component = component.map_text(|text| {
                    assert!(range.contains(&(text.as_ptr() as usize)));
                    strings += 1;
                    text
                });
                match encoder.next(Some(&mut component), &mut output).unwrap() {
                    EncoderStatus::ComponentFinished { written } => {
                        encoded.extend_from_slice(&output[..written])
                    }
                    _ => panic!("unexpected status"),
                }
            }
            DecoderStatus::Finished => break,
            _ => panic!("complete input"),
        }
    }
    decoder.finish().unwrap();
    match encoder
        .next(None::<&mut TextRequestComponent>, &mut output)
        .unwrap()
    {
        EncoderStatus::Finished { written } => encoded.extend_from_slice(&output[..written]),
        _ => panic!("unexpected status"),
    }
    encoder.finish().unwrap();
    assert_eq!(strings, 3);
    let value: serde_json::Value = serde_json::from_slice(&encoded).unwrap();
    assert_eq!(value["messages"][0]["role"], "system");
    assert_eq!(value["messages"][1]["content"], "hello");
    assert_eq!(value["max_tokens"], 100);
}
