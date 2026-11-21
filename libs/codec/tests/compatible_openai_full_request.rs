use codec::decoder::{Decoder, DecoderStatus};
use codec::encoder::{Encoder, EncoderStatus};
use codec::llm::text::component::*;
use codec::llm::text::copenai::request::{
    CompatibleOpenAIRequestDecoder, CompatibleOpenAIRequestEncoder,
};
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

struct CountingAllocator;

#[derive(Clone, Copy, Debug, Default)]
struct AllocationCount {
    allocations: usize,
    reallocations: usize,
    requested_bytes: usize,
}

impl AllocationCount {
    fn since(self, before: Self) -> Self {
        Self {
            allocations: self.allocations - before.allocations,
            reallocations: self.reallocations - before.reallocations,
            requested_bytes: self.requested_bytes - before.requested_bytes,
        }
    }

    fn add(&mut self, other: Self) {
        self.allocations += other.allocations;
        self.reallocations += other.reallocations;
        self.requested_bytes += other.requested_bytes;
    }
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

const REQUEST: &str = r#"{
  "model": "gpt-4o",
  "messages": [
    {
      "role": "system",
      "content": "You are a helpful assistant."
    },
    {
      "role": "user",
      "content": "Can you check the weather and analyze the reasoning?"
    },
    {
      "role": "assistant",
      "content": "Let me check the weather for you.",
      "tool_calls": [
        {
          "id": "call_abc123",
          "type": "function",
          "function": {
            "name": "get_weather",
            "arguments": "{\"location\": \"Boston, MA\"}"
          }
        }
      ],
      "reasoning": "Thinking process summary...",
      "reasoning_content": "Detailed thinking content...",
      "reasoning_details": [
        {
          "type": "summary",
          "summary": "Step 1: Parse user location. Step 2: Call weather tool.",
          "id": "detail_01",
          "format": "text",
          "index": 0
        },
        {
          "type": "text",
          "text": "Internal token reasoning block.",
          "signature": "sig_xyz789"
        },
        {
          "type": "encrypted",
          "data": "base64_encoded_encrypted_reasoning_data_here=="
        }
      ]
    },
    {
      "role": "tool",
      "content": "{\"temperature\": 72, \"condition\": \"Sunny\"}",
      "tool_call_id": "call_abc123"
    }
  ],
  "tools": [
    {
      "type": "function",
      "function": {
        "name": "get_weather",
        "description": "Get the current weather for a given location.",
        "parameters": "{\"type\": \"object\", \"properties\": {\"location\": {\"type\": \"string\"}}, \"required\": [\"location\"]}",
        "strict": true
      }
    }
  ],
  "tool_choice": "auto",
  "parallel_tool_calls": true,
  "max_completion_tokens": 1500,
  "top_p": 1.0,
  "top_k": 50,
  "stop": ["<|endoftext|>", "User:"],
  "response_format": {
    "type": "json_schema",
    "json_schema": {
      "name": "weather_response",
      "description": "Schema for weather output",
      "strict": true,
      "schema": "{\"type\": \"object\", \"properties\": {\"temp\": {\"type\": \"number\"}}, \"required\": [\"temp\"]}"
    }
  },
  "provider": {
    "require_parameter": true
  },
  "stream": true,
  "stream_options": {
    "include_usage": true
  }
}"#;

#[test]
fn measures_full_request_decode_encode_allocations() {
    measure_full_request_allocations::<16>();
    measure_full_request_allocations::<{ 16 * 1024 }>();
}

fn measure_full_request_allocations<const BUFFER_SIZE: usize>() {
    let mut encoded = [0_u8; 16 * 1024];
    let mut encoded_length = 0;
    let mut output = [0_u8; BUFFER_SIZE];
    let measurement = AllocationMeasurement::start();
    let json_decoder = codec::json::JsonDecoder::new();
    let json_decoder_construction = measurement.snapshot();
    let mut decoder = CompatibleOpenAIRequestDecoder::new(json_decoder);
    let request_decoder_construction = measurement.snapshot().since(json_decoder_construction);
    let before = measurement.snapshot();
    let json_encoder = codec::json::JsonEncoder::new();
    let json_encoder_construction = measurement.snapshot().since(before);
    let before = measurement.snapshot();
    let mut encoder = CompatibleOpenAIRequestEncoder::new(json_encoder);
    let request_encoder_construction = measurement.snapshot().since(before);
    let construction = measurement.snapshot();
    let mut component_count = 0;
    let mut decoding = AllocationCount::default();
    let mut encoding = AllocationCount::default();
    let mut pending_decode = AllocationCount::default();
    let mut per_component = [(AllocationCount::default(), AllocationCount::default()); 16];

    for chunk in REQUEST.as_bytes().chunks(BUFFER_SIZE) {
        let mut input = chunk;
        loop {
            let before = measurement.snapshot();
            let status = decoder.next(&mut input, false).expect("decode component");
            let count = measurement.snapshot().since(before);
            decoding.add(count);
            pending_decode.add(count);
            match status {
                DecoderStatus::Component(mut component) => {
                    per_component[component_count].0 = pending_decode;
                    pending_decode = AllocationCount::default();
                    let component_index = component_count;
                    component_count += 1;
                    loop {
                        let before = measurement.snapshot();
                        let status = encoder
                            .next(Some(&mut component), &mut output)
                            .expect("encode decoded component");
                        let count = measurement.snapshot().since(before);
                        encoding.add(count);
                        per_component[component_index].1.add(count);
                        let (written, complete) = match status {
                            EncoderStatus::NeedOutput { written } => (written, false),
                            EncoderStatus::ComponentFinished { written } => (written, true),
                            EncoderStatus::Finished { .. } => {
                                panic!("premature encoder completion")
                            }
                        };
                        encoded[encoded_length..encoded_length + written]
                            .copy_from_slice(&output[..written]);
                        encoded_length += written;
                        if complete {
                            break;
                        }
                    }
                }
                DecoderStatus::NeedInput => break,
                DecoderStatus::Finished => panic!("premature decoder completion"),
            }
        }
    }
    let mut input = &b""[..];
    let before = measurement.snapshot();
    let decoder_status = decoder.next(&mut input, true).expect("decode EOF");
    decoder.finish().expect("finish decoding");
    let decoder_finalization = measurement.snapshot().since(before);
    let before = measurement.snapshot();
    loop {
        let status = encoder.next(None, &mut output).expect("finalize encoding");
        let (written, complete) = match status {
            EncoderStatus::NeedOutput { written } => (written, false),
            EncoderStatus::Finished { written } => (written, true),
            EncoderStatus::ComponentFinished { .. } => panic!("unexpected component"),
        };
        encoded[encoded_length..encoded_length + written].copy_from_slice(&output[..written]);
        encoded_length += written;
        if complete {
            break;
        }
    }
    encoder.finish().expect("finish encoding");
    let encoder_finalization = measurement.snapshot().since(before);
    let total = measurement.snapshot();
    drop(measurement);

    assert_eq!(decoder_status, DecoderStatus::Finished);
    let expected = request_components();
    assert_eq!(component_count, expected.len());
    assert_eq!(decode_request(&encoded[..encoded_length]), expected);
    println!(
        "{BUFFER_SIZE}-byte input/output buffers; {component_count} components; {encoded_length} encoded bytes"
    );
    println!("construction: {construction:?}");
    println!("  JsonDecoder: {json_decoder_construction:?}");
    println!("  request decoder: {request_decoder_construction:?}");
    println!("  JsonEncoder: {json_encoder_construction:?}");
    println!("  request encoder: {request_encoder_construction:?}");
    println!("decoding (including JSON): {decoding:?}");
    println!("encoding (including JSON): {encoding:?}");
    println!("decoder finalization: {decoder_finalization:?}");
    println!("encoder finalization: {encoder_finalization:?}");
    for (index, (decode, encode)) in per_component[..component_count].iter().enumerate() {
        println!(
            "component {}: decode={decode:?}, encode={encode:?}",
            index + 1
        );
    }
    println!("trailing decode work after last component: {pending_decode:?}");
    println!(
        "processing: allocations={}, reallocations={}, requested_bytes={}",
        total.allocations - construction.allocations,
        total.reallocations - construction.reallocations,
        total.requested_bytes - construction.requested_bytes
    );
    println!("total: {total:?}");
}

#[test]
fn decodes_full_request_with_16_byte_input_buffer() {
    assert_eq!(decode_request(REQUEST.as_bytes()), request_components());
}

fn decode_request(request: &[u8]) -> Vec<TextRequestComponent> {
    let mut decoder = CompatibleOpenAIRequestDecoder::new(codec::json::JsonDecoder::new());
    let mut components = Vec::new();
    let mut buffer = [0_u8; 16];
    for chunk in request.chunks(buffer.len()) {
        buffer[..chunk.len()].copy_from_slice(chunk);
        let mut input = &buffer[..chunk.len()];
        loop {
            match decoder
                .next(&mut input, false)
                .expect("decode full request")
            {
                DecoderStatus::Component(component) => components.push(component),
                DecoderStatus::NeedInput => {
                    assert!(input.is_empty());
                    break;
                }
                DecoderStatus::Finished => panic!("finished before source EOF"),
            }
        }
    }
    let mut input = &b""[..];
    assert_eq!(
        decoder.next(&mut input, true).unwrap(),
        DecoderStatus::Finished
    );
    decoder.finish().expect("complete request at EOF");
    components
}

fn request_components() -> Vec<TextRequestComponent> {
    let message = |role, content: &str| Message {
        role,
        content: Some(Content::Text(content.into())),
        tool_calls: None,
        tool_call_id: None,
        reasoning: None,
        reasoning_content: None,
        reasoning_details: None,
    };
    let mut assistant = message(Role::Assistant, "Let me check the weather for you.");
    assistant.tool_calls = Some(vec![ToolCall {
        id: "call_abc123".into(),
        kind: ToolType::Function,
        function: FunctionCall {
            name: "get_weather".into(),
            arguments: r#"{"location": "Boston, MA"}"#.into(),
        },
    }]);
    assistant.reasoning = Some("Thinking process summary...".into());
    assistant.reasoning_content = Some("Detailed thinking content...".into());
    assistant.reasoning_details = Some(vec![
        ReasoningDetail {
            content: ReasoningDetailContent::Summary {
                summary: "Step 1: Parse user location. Step 2: Call weather tool.".into(),
            },
            id: Some("detail_01".into()),
            format: Some("text".into()),
            index: Some(0),
        },
        ReasoningDetail {
            content: ReasoningDetailContent::Text {
                text: "Internal token reasoning block.".into(),
                signature: Some("sig_xyz789".into()),
            },
            id: None,
            format: None,
            index: None,
        },
        ReasoningDetail {
            content: ReasoningDetailContent::Encrypted {
                data: "base64_encoded_encrypted_reasoning_data_here==".into(),
            },
            id: None,
            format: None,
            index: None,
        },
    ]);
    let mut result = message(Role::Tool, r#"{"temperature": 72, "condition": "Sunny"}"#);
    result.tool_call_id = Some("call_abc123".into());

    vec![
        TextRequestComponent::Model("gpt-4o".into()),
        TextRequestComponent::Message(message(Role::System, "You are a helpful assistant.")),
        TextRequestComponent::Message(message(Role::User, "Can you check the weather and analyze the reasoning?")),
        TextRequestComponent::Message(assistant),
        TextRequestComponent::Message(result),
        TextRequestComponent::Tools(vec![ToolDefinition {
            kind: ToolType::Function,
            function: FunctionDefinition {
                name: "get_weather".into(),
                description: Some("Get the current weather for a given location.".into()),
                parameters: Some(r#"{"type": "object", "properties": {"location": {"type": "string"}}, "required": ["location"]}"#.into()),
                strict: Some(true),
            },
        }]),
        TextRequestComponent::ToolChoice(ToolChoice::Auto),
        TextRequestComponent::ParallelToolCalls(true),
        TextRequestComponent::MaxCompletionTokens(1500),
        TextRequestComponent::TopP(1.0),
        TextRequestComponent::TopK(50),
        TextRequestComponent::Stop(Stop::Array(vec!["<|endoftext|>".into(), "User:".into()])),
        TextRequestComponent::ResponseFormat(ResponseFormat::JsonSchema(JsonSchema {
            name: "weather_response".into(),
            description: Some("Schema for weather output".into()),
            strict: Some(true),
            schema: r#"{"type": "object", "properties": {"temp": {"type": "number"}}, "required": ["temp"]}"#.into(),
        })),
        TextRequestComponent::Provider(Provider::RequireParameter(true)),
        TextRequestComponent::Stream(true),
        TextRequestComponent::StreamOptions(Some(StreamOptions { include_usage: Some(true) })),
    ]
}

#[test]
fn encodes_full_request_across_output_buffer_sizes() {
    for size in [1, 16, 16 * 1024] {
        let mut encoder = CompatibleOpenAIRequestEncoder::new(codec::json::JsonEncoder::new());
        let mut components = request_components();
        let mut output = vec![0_u8; size];
        let mut encoded = Vec::new();
        for component in &mut components {
            loop {
                match encoder
                    .next(Some(component), &mut output)
                    .expect("encode component")
                {
                    EncoderStatus::NeedOutput { written } => {
                        assert!(written > 0);
                        encoded.extend_from_slice(&output[..written]);
                    }
                    EncoderStatus::ComponentFinished { written } => {
                        encoded.extend_from_slice(&output[..written]);
                        break;
                    }
                    EncoderStatus::Finished { .. } => panic!("finished before finalization"),
                }
            }
        }
        loop {
            match encoder.next(None, &mut output).expect("finalize request") {
                EncoderStatus::NeedOutput { written } => {
                    assert!(written > 0);
                    encoded.extend_from_slice(&output[..written]);
                }
                EncoderStatus::Finished { written } => {
                    encoded.extend_from_slice(&output[..written]);
                    break;
                }
                EncoderStatus::ComponentFinished { .. } => panic!("component during finalization"),
            }
        }
        encoder.finish().expect("complete encoder");
        assert_eq!(
            decode_request(&encoded),
            components,
            "output buffer size {size}"
        );
    }
}
