// Anthropic-native weather request for offline codec round trips.
// Thinking signatures and redacted data are fixture placeholders, not API-valid
// reasoning history. Live replay requires original Anthropic-issued
// blocks preserved unchanged and in order. Sampling overrides are omitted with thinking.
pub const REQUEST: &str = r#"{
  "model": "claude-opus-4-8",
  "system": "You are a helpful assistant.",
  "messages": [
    {
      "role": "user",
      "content": "Can you check the weather and analyze the reasoning?"
    },
    {
      "role": "system",
      "content": "Use Fahrenheit for temperatures in the weather response."
    },
    {
      "role": "assistant",
      "content": [
        {
          "type": "thinking",
          "thinking": "Step 1: Parse user location. Step 2: Call weather tool.",
          "signature": "fixture-only-anthropic-signature"
        },
        {
          "type": "redacted_thinking",
          "data": "Zml4dHVyZS1vbmx5LXJlZGFjdGVkLXRoaW5raW5n"
        },
        {
          "type": "text",
          "text": "Let me check the weather for you."
        },
        {
          "type": "tool_use",
          "id": "call_abc123",
          "name": "get_weather",
          "input": {
            "location": "Boston, MA"
          }
        }
      ]
    },
    {
      "role": "user",
      "content": [
        {
          "type": "tool_result",
          "tool_use_id": "call_abc123",
          "content": "{\"temperature\": 72, \"condition\": \"Sunny\"}"
        }
      ]
    }
  ],
  "tools": [
    {
      "name": "get_weather",
      "description": "Get the current weather for a given location.",
      "input_schema": {
        "type": "object",
        "properties": {
          "location": {
            "type": "string"
          }
        },
        "required": ["location"],
        "additionalProperties": false
      },
      "strict": true
    }
  ],
  "tool_choice": {
    "type": "auto",
    "disable_parallel_tool_use": false
  },
  "max_tokens": 1500,
  "thinking": {
    "type": "adaptive"
  },
  "stop_sequences": ["<|endoftext|>", "User:"],
  "output_config": {
    "format": {
      "type": "json_schema",
      "schema": {
        "type": "object",
        "properties": {
          "temp": {
            "type": "number"
          }
        },
        "required": ["temp"],
        "additionalProperties": false
      }
    }
  },
  "stream": true
}"#;

use codec::decoder::{Decoder, DecoderStatus};
use codec::encoder::{Encoder, EncoderStatus};
use codec::json::{JsonDecoder, JsonEncoder};
use codec::llm::text::anthropic::request::decoder::AnthropicRequestDecoder;
use codec::llm::text::anthropic::request::encoder::{
    AnthropicRequestEncoder, AnthropicRequestEncoderConfiguration,
};
use codec::llm::text::component::TextRequestComponent;
use codec::llm::text::copenai::request::decoder::CompatibleOpenAIRequestDecoder;
use serde_json::{Value, json};
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

struct CountingAllocator;

thread_local! {
    static ALLOCATIONS: Cell<Option<usize>> = const { Cell::new(None) };
}

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            ALLOCATIONS.with(|count| count.set(count.get().map(|value| value + 1)));
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let pointer = unsafe { System.alloc_zeroed(layout) };
        if !pointer.is_null() {
            ALLOCATIONS.with(|count| count.set(count.get().map(|value| value + 1)));
        }
        pointer
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        let pointer = unsafe { System.realloc(pointer, layout, size) };
        if !pointer.is_null() {
            ALLOCATIONS.with(|count| count.set(count.get().map(|value| value + 1)));
        }
        pointer
    }
}

const COMPATIBLE_REQUEST: &str = r#"{
  "model": "claude-sonnet-4-5-20250929",
  "messages": [
    {"role": "system", "content": "You are a helpful assistant."},
    {"role": "user", "content": "Weather in Boston? 🦀"},
    {"role": "assistant", "content": "Checking weather.",
     "reasoning": "Unsigned reasoning",
     "reasoning_content": "More unsigned reasoning",
     "reasoning_details": [
       {"type": "summary", "summary": "Summary"},
       {"type": "text", "text": "Signed thinking", "signature": "fixture-signature"},
       {"type": "text", "text": "Unsigned thinking"},
       {"type": "encrypted", "data": "opaque-non-anthropic-data"}
     ],
     "tool_calls": [{"id": "call_weather", "type": "function",
       "function": {"name": "get_weather", "arguments": "{\"location\":\"Boston\"}"}}]},
    {"role": "tool", "tool_call_id": "call_weather", "content": "{\"temperature\":72}"}
  ],
  "tools": [{"type": "function", "function": {
    "name": "get_weather", "description": "Weather lookup",
    "parameters": "{\"type\":\"object\",\"properties\":{\"location\":{\"type\":\"string\"}},\"required\":[\"location\"]}",
    "strict": true
  }}],
  "tool_choice": "auto",
  "parallel_tool_calls": true,
  "max_tokens": 1500,
  "reasoning": {"enabled": true, "max_tokens": 1024, "exclude": false},
  "top_p": 0.9,
  "top_k": 50,
  "stop": ["END", "User:"],
  "response_format": {"type": "json_schema", "json_schema": {
    "name": "weather", "description": "Weather output", "strict": true,
    "schema": "{\"type\":\"object\",\"properties\":{\"temp\":{\"type\":\"number\"}}}"
  }},
  "provider": {"require_parameter": true},
  "stream": true,
  "stream_options": {"include_usage": true}
}"#;

fn translate(
    request: &str,
    input_size: usize,
    output_size: usize,
    configuration: AnthropicRequestEncoderConfiguration,
) -> Value {
    let mut decoder = CompatibleOpenAIRequestDecoder::new(JsonDecoder::new());
    let mut encoder = AnthropicRequestEncoder::new(JsonEncoder::new(), configuration);
    let mut output = vec![0; output_size];
    let mut encoded = Vec::new();
    let mut encode_component = |component: &mut TextRequestComponent| loop {
        match encoder
            .next(Some(component), &mut output)
            .expect("encode decoded component")
        {
            EncoderStatus::NeedOutput { written } => {
                assert!(written > 0);
                encoded.extend_from_slice(&output[..written]);
            }
            EncoderStatus::ComponentFinished { written } => {
                encoded.extend_from_slice(&output[..written]);
                break;
            }
            EncoderStatus::Finished { .. } => panic!("encoder finished before EOF"),
        }
    };
    for chunk in request.as_bytes().chunks(input_size) {
        let mut input = chunk;
        loop {
            match decoder
                .next(&mut input, false)
                .expect("decode compatible request")
            {
                DecoderStatus::Component(mut component) => encode_component(&mut component),
                DecoderStatus::NeedInput => {
                    assert!(input.is_empty());
                    break;
                }
                DecoderStatus::Finished => panic!("decoder finished before EOF"),
            }
        }
    }
    let mut input = &b""[..];
    loop {
        match decoder.next(&mut input, true).expect("decode EOF") {
            DecoderStatus::Component(mut component) => encode_component(&mut component),
            DecoderStatus::Finished => break,
            DecoderStatus::NeedInput => panic!("decoder needs input after EOF"),
        }
    }
    decoder.finish().expect("finish decoder");
    loop {
        match encoder
            .next(None, &mut output)
            .expect("finalize Anthropic request")
        {
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
    encoder.finish().expect("finish encoder");
    serde_json::from_slice(&encoded).expect("Anthropic JSON")
}

fn round_trip_anthropic(request: &str, input_size: usize, output_size: usize) -> Value {
    let mut decoder = AnthropicRequestDecoder::new(JsonDecoder::new());
    let mut components = Vec::new();
    for chunk in request.as_bytes().chunks(input_size) {
        let mut input = chunk;
        loop {
            let status = decoder.next(&mut input, false).unwrap_or_else(|error| {
                panic!(
                    "decode Anthropic request near {:?}: {error:?}",
                    String::from_utf8_lossy(&input[..input.len().min(80)])
                )
            });
            match status {
                DecoderStatus::Component(component) => components.push(component),
                DecoderStatus::NeedInput => {
                    assert!(input.is_empty());
                    break;
                }
                DecoderStatus::Finished => panic!("decoder finished before EOF"),
            }
        }
    }
    let mut input = &b""[..];
    loop {
        match decoder
            .next(&mut input, true)
            .expect("decode Anthropic EOF")
        {
            DecoderStatus::Component(component) => components.push(component),
            DecoderStatus::Finished => break,
            DecoderStatus::NeedInput => panic!("decoder needs input after EOF"),
        }
    }
    decoder.finish().expect("finish Anthropic decoder");

    let mut encoder = AnthropicRequestEncoder::new(
        JsonEncoder::new(),
        AnthropicRequestEncoderConfiguration::default(),
    );
    let mut output = vec![0; output_size];
    let mut encoded = Vec::new();
    for component in &mut components {
        loop {
            match encoder
                .next(Some(component), &mut output)
                .expect("encode component")
            {
                EncoderStatus::NeedOutput { written } => {
                    encoded.extend_from_slice(&output[..written])
                }
                EncoderStatus::ComponentFinished { written } => {
                    encoded.extend_from_slice(&output[..written]);
                    break;
                }
                EncoderStatus::Finished { .. } => panic!("encoder finished before EOF"),
            }
        }
    }
    loop {
        match encoder.next(None, &mut output).expect("finalize request") {
            EncoderStatus::NeedOutput { written } => encoded.extend_from_slice(&output[..written]),
            EncoderStatus::Finished { written } => {
                encoded.extend_from_slice(&output[..written]);
                break;
            }
            EncoderStatus::ComponentFinished { .. } => panic!("component during finalization"),
        }
    }
    encoder.finish().expect("finish Anthropic encoder");
    serde_json::from_slice(&encoded).expect("round-trip JSON")
}

#[test]
fn measures_anthropic_request_encoding_allocations() {
    let mut decoder = CompatibleOpenAIRequestDecoder::new(JsonDecoder::new());
    let mut components = Vec::new();
    let mut input = COMPATIBLE_REQUEST.as_bytes();
    loop {
        match decoder.next(&mut input, true).expect("decode components") {
            DecoderStatus::Component(component) => components.push(component),
            DecoderStatus::Finished => break,
            DecoderStatus::NeedInput => panic!("complete input needs more bytes"),
        }
    }
    decoder.finish().expect("finish decoder");

    let mut output = [0_u8; 16 * 1024];
    ALLOCATIONS.with(|count| count.set(Some(0)));
    let mut encoder = AnthropicRequestEncoder::new(
        JsonEncoder::new(),
        AnthropicRequestEncoderConfiguration::default(),
    );
    for component in &mut components {
        assert!(matches!(
            encoder
                .next(Some(component), &mut output)
                .expect("encode component"),
            EncoderStatus::ComponentFinished { .. }
        ));
    }
    assert!(matches!(
        encoder.next(None, &mut output).expect("finalize request"),
        EncoderStatus::Finished { .. }
    ));
    encoder.finish().expect("finish encoder");
    let allocations = ALLOCATIONS.with(|count| count.replace(None).expect("measurement"));
    assert_eq!(allocations, 2, "unexpected Anthropic encoding allocations");
}

#[test]
fn measures_compatible_to_anthropic_allocations() {
    measure_compatible_to_anthropic_allocations::<16>();
    measure_compatible_to_anthropic_allocations::<{ 16 * 1024 }>();
}

fn measure_compatible_to_anthropic_allocations<const BUFFER_SIZE: usize>() {
    let mut input_buffer = [0_u8; BUFFER_SIZE];
    let mut output = [0_u8; BUFFER_SIZE];
    ALLOCATIONS.with(|count| count.set(Some(0)));
    let mut decoder = CompatibleOpenAIRequestDecoder::new(JsonDecoder::new());
    let mut encoder = AnthropicRequestEncoder::new(
        JsonEncoder::new(),
        AnthropicRequestEncoderConfiguration::default(),
    );
    let construction_allocations = ALLOCATIONS.with(|count| count.get().expect("measurement"));
    let mut components = 0;
    let mut encoded_bytes = 0;
    let mut pending_decode_allocations = 0;
    let mut rows = [None; 32];

    for chunk in COMPATIBLE_REQUEST.as_bytes().chunks(BUFFER_SIZE) {
        input_buffer[..chunk.len()].copy_from_slice(chunk);
        let mut input = &input_buffer[..chunk.len()];
        loop {
            let before = ALLOCATIONS.with(|count| count.get().expect("measurement"));
            let status = decoder.next(&mut input, false).expect("decode component");
            let after = ALLOCATIONS.with(|count| count.get().expect("measurement"));
            pending_decode_allocations += after - before;
            match status {
                DecoderStatus::Component(mut component) => {
                    components += 1;
                    let name = component_name(&component);
                    let before = ALLOCATIONS.with(|count| count.get().expect("measurement"));
                    loop {
                        match encoder
                            .next(Some(&mut component), &mut output)
                            .expect("encode component")
                        {
                            EncoderStatus::NeedOutput { written } => encoded_bytes += written,
                            EncoderStatus::ComponentFinished { written } => {
                                encoded_bytes += written;
                                break;
                            }
                            EncoderStatus::Finished { .. } => {
                                panic!("encoder finished before source EOF")
                            }
                        }
                    }
                    let after = ALLOCATIONS.with(|count| count.get().expect("measurement"));
                    rows[components - 1] = Some((name, pending_decode_allocations, after - before));
                    pending_decode_allocations = 0;
                }
                DecoderStatus::NeedInput => break,
                DecoderStatus::Finished => panic!("decoder finished before source EOF"),
            }
        }
    }
    let mut input = &b""[..];
    loop {
        let before = ALLOCATIONS.with(|count| count.get().expect("measurement"));
        let status = decoder.next(&mut input, true).expect("decode EOF");
        let after = ALLOCATIONS.with(|count| count.get().expect("measurement"));
        pending_decode_allocations += after - before;
        match status {
            DecoderStatus::Component(mut component) => {
                components += 1;
                let name = component_name(&component);
                let before = ALLOCATIONS.with(|count| count.get().expect("measurement"));
                loop {
                    match encoder
                        .next(Some(&mut component), &mut output)
                        .expect("encode final component")
                    {
                        EncoderStatus::NeedOutput { written } => encoded_bytes += written,
                        EncoderStatus::ComponentFinished { written } => {
                            encoded_bytes += written;
                            break;
                        }
                        EncoderStatus::Finished { .. } => {
                            panic!("encoder finished before finalization")
                        }
                    }
                }
                let after = ALLOCATIONS.with(|count| count.get().expect("measurement"));
                rows[components - 1] = Some((name, pending_decode_allocations, after - before));
                pending_decode_allocations = 0;
            }
            DecoderStatus::Finished => break,
            DecoderStatus::NeedInput => panic!("decoder needs input after EOF"),
        }
    }
    decoder.finish().expect("finish decoder");
    loop {
        match encoder.next(None, &mut output).expect("finalize encoder") {
            EncoderStatus::NeedOutput { written } => encoded_bytes += written,
            EncoderStatus::Finished { written } => {
                encoded_bytes += written;
                break;
            }
            EncoderStatus::ComponentFinished { .. } => panic!("unexpected component"),
        }
    }
    encoder.finish().expect("finish encoder");
    let allocations = ALLOCATIONS.with(|count| count.replace(None).expect("measurement"));
    println!(
        "{BUFFER_SIZE}-byte buffers: {components} components, {encoded_bytes} encoded bytes, {allocations} allocations/reallocations"
    );
    println!("| # | Component | Decode | Encode |");
    println!("|---:|---|---:|---:|");
    println!("| - | construction | - | {construction_allocations} |");
    for (index, row) in rows[..components].iter().enumerate() {
        let (component, decode, encode) = row.expect("component allocation row");
        println!("| {} | {component} | {decode} | {encode} |", index + 1);
    }
    println!("| - | trailing/finalization | {pending_decode_allocations} | - |");
    assert!(components > 0);
    assert!(encoded_bytes > 0);
}

fn component_name(component: &TextRequestComponent) -> &'static str {
    match component {
        TextRequestComponent::Model(_) => "Model",
        TextRequestComponent::Message(_) => "Message",
        TextRequestComponent::ToolsStarted => "ToolsStarted",
        TextRequestComponent::Tool(_) => "Tool",
        TextRequestComponent::ToolsEnded => "ToolsEnded",
        TextRequestComponent::ToolChoice(_) => "ToolChoice",
        TextRequestComponent::ParallelToolCalls(_) => "ParallelToolCalls",
        TextRequestComponent::MaxTokens(_) => "MaxTokens",
        TextRequestComponent::Reasoning(_) => "Reasoning",
        TextRequestComponent::TopP(_) => "TopP",
        TextRequestComponent::TopK(_) => "TopK",
        TextRequestComponent::Stop(_) => "Stop",
        TextRequestComponent::ResponseFormat(_) => "ResponseFormat",
        TextRequestComponent::Provider(_) => "Provider",
        TextRequestComponent::Stream(_) => "Stream",
        TextRequestComponent::StreamOptions(_) => "StreamOptions",
    }
}

#[test]
fn decodes_and_reencodes_anthropic_request_across_buffer_sizes() {
    for input_size in [16 * 1024, 16, 1] {
        for output_size in [16 * 1024, 16, 1] {
            let value = round_trip_anthropic(REQUEST, input_size, output_size);
            assert_eq!(value["model"], "claude-opus-4-8");
            assert_eq!(value["max_tokens"], 1500);
            assert_eq!(value["thinking"], json!({"type": "adaptive"}));
            assert_eq!(value["messages"][2]["content"][0]["type"], "thinking");
            assert_eq!(value["messages"][2]["content"][2]["type"], "tool_use");
            assert_eq!(value["messages"][3]["content"][0]["type"], "tool_result");
            assert_eq!(value["output_config"]["format"]["type"], "json_schema");
        }
    }
}

#[test]
fn translates_compatible_request_to_anthropic_across_buffer_sizes() {
    let expected = json!({
        "model": "claude-sonnet-4-5-20250929",
        "system": [{"type": "text", "text": "You are a helpful assistant."}],
        "messages": [
            {"role": "user", "content": [{"type": "text", "text": "Weather in Boston? 🦀"}]},
            {"role": "assistant", "content": [
                {"type": "thinking", "thinking": "Signed thinking", "signature": "fixture-signature"},
                {"type": "text", "text": "Checking weather."},
                {"type": "tool_use", "id": "call_weather", "name": "get_weather", "input": {"location": "Boston"}}
            ]},
            {"role": "user", "content": [{"type": "tool_result", "tool_use_id": "call_weather",
                "content": [{"type": "text", "text": "{\"temperature\":72}"}]}]}
        ],
        "tools": [{"name": "get_weather", "description": "Weather lookup", "input_schema": {
            "type": "object", "properties": {"location": {"type": "string"}}, "required": ["location"]
        }}],
        "tool_choice": {"type": "auto"},
        "max_tokens": 1500,
        "thinking": {"type": "enabled", "budget_tokens": 1024},
        "top_p": 0.9, "top_k": 50, "stop_sequences": ["END", "User:"],
        "output_config": {"format": {"type": "json_schema", "schema": {
            "type": "object", "properties": {"temp": {"type": "number"}}
        }}},
        "stream": true
    });
    for input_size in [1, 16, 16 * 1024] {
        for output_size in [1, 16, 16 * 1024] {
            assert_eq!(
                translate(
                    COMPATIBLE_REQUEST,
                    input_size,
                    output_size,
                    AnthropicRequestEncoderConfiguration::default(),
                ),
                expected,
                "input size {input_size}, output size {output_size}"
            );
        }
    }
}

#[test]
fn preserves_late_system_message_and_defaults_tokens_with_adaptive_effort() {
    let request = r#"{
        "model": "claude-opus-4-8",
        "messages": [
            {"role": "user", "content": "Hello"},
            {"role": "system", "content": "Be concise"},
            {"role": "assistant", "content": "Hi"}
        ],
        "reasoning": {"enabled": true, "effort": "low"},
        "response_format": {"type": "json_schema", "json_schema": {
            "name": "reply", "schema": "{\"type\":\"object\"}"
        }}
    }"#;
    let expected = json!({
        "model": "claude-opus-4-8", "system": "", "max_tokens": 32768,
        "messages": [
            {"role": "user", "content": [{"type": "text", "text": "Hello"}]},
            {"role": "system", "content": [{"type": "text", "text": "Be concise"}]},
            {"role": "assistant", "content": [{"type": "text", "text": "Hi"}]}
        ],
        "thinking": {"type": "adaptive"},
        "output_config": {"effort": "high", "format": {"type": "json_schema", "schema": {"type": "object"}}}
    });
    for size in [1, 16, 16 * 1024] {
        assert_eq!(
            translate(
                request,
                size,
                size,
                AnthropicRequestEncoderConfiguration {
                    reasoning_effort: Some(codec::llm::text::component::ReasoningEffort::High,),
                },
            ),
            expected
        );
    }
}
