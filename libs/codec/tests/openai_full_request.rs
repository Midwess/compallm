use codec::decoder::{Decoder, DecoderStatus};
use codec::encoder::{Encoder, EncoderStatus};
use codec::json::{JsonDecoder, JsonEncoder};
use codec::llm::text::component::{
    Reasoning, ReasoningEffort, ResponseFormat, Role, Stop, TextRequestComponent, ToolChoice,
};
use codec::llm::text::copenai::request::decoder::CompatibleOpenAIRequestDecoder;
use codec::llm::text::openai::request::completions::{
    decoder::OpenAICompletionRequestDecoder, encoder::OpenAICompletionRequestEncoder,
};
use codec::llm::text::openai::request::responses::{
    decoder::OpenAIResponseRequestDecoder, encoder::OpenAIResponseRequestEncoder,
};
use serde_json::{Value, json};
use std::fmt::Debug;

const COMPLETION_REQUEST: &str = r#"{
  "model": "gpt-4o",
  "messages": [
    {"role": "system", "content": "You are a helpful assistant."},
    {"role": "user", "content": "Can you check the weather and analyze the reasoning?"},
    {"role": "system", "content": "Use Fahrenheit for temperatures in the weather response."},
    {
      "role": "assistant",
      "content": "Let me check the weather for you.",
      "tool_calls": [{
        "id": "call_abc123",
        "type": "function",
        "function": {"name": "get_weather", "arguments": "{\"location\": \"Boston, MA\"}"}
      }]
    },
    {"role": "tool", "tool_call_id": "call_abc123", "content": "{\"temperature\": 72, \"condition\": \"Sunny\"}"}
  ],
  "tools": [{
    "type": "function",
    "function": {
      "name": "get_weather",
      "description": "Get the current weather for a given location.",
      "parameters": {
        "type": "object",
        "properties": {"location": {"type": "string"}},
        "required": ["location"],
        "additionalProperties": false
      },
      "strict": true
    }
  }],
  "tool_choice": "auto",
  "parallel_tool_calls": true,
  "max_completion_tokens": 1500,
  "top_p": 1.0,
  "stop": ["<|endoftext|>", "User:"],
  "response_format": {
    "type": "json_schema",
    "json_schema": {
      "name": "weather_response",
      "description": "Schema for weather output",
      "strict": true,
      "schema": {
        "type": "object",
        "properties": {"temp": {"type": "number"}},
        "required": ["temp"],
        "additionalProperties": false
      }
    }
  },
  "stream": true,
  "stream_options": {"include_usage": true}
}"#;

const RESPONSE_REQUEST: &str = r#"{
  "model": "gpt-4o",
  "input": [
    {"type": "message", "role": "system", "content": "You are a helpful assistant."},
    {"type": "message", "role": "user", "content": "Can you check the weather and analyze the reasoning?"},
    {"type": "message", "role": "system", "content": "Use Fahrenheit for temperatures in the weather response."},
    {"type": "message", "role": "assistant", "content": "Let me check the weather for you."},
    {"type": "function_call", "call_id": "call_abc123", "name": "get_weather", "arguments": "{\"location\": \"Boston, MA\"}"},
    {"type": "function_call_output", "call_id": "call_abc123", "output": "{\"temperature\": 72, \"condition\": \"Sunny\"}"}
  ],
  "tools": [{
    "type": "function",
    "name": "get_weather",
    "description": "Get the current weather for a given location.",
    "parameters": {
      "type": "object",
      "properties": {"location": {"type": "string"}},
      "required": ["location"],
      "additionalProperties": false
    },
    "strict": true
  }],
  "tool_choice": "auto",
  "parallel_tool_calls": true,
  "max_output_tokens": 1500,
  "top_p": 1.0,
  "text": {
    "format": {
      "type": "json_schema",
      "name": "weather_response",
      "description": "Schema for weather output",
      "strict": true,
      "schema": {
        "type": "object",
        "properties": {"temp": {"type": "number"}},
        "required": ["temp"],
        "additionalProperties": false
      }
    }
  },
  "stream": true
}"#;

const COMPATIBLE_REQUEST: &str = r#"{
  "model": "gpt-4o",
  "messages": [
    {"role": "user", "content": "Can you check the weather?"},
    {
      "role": "assistant",
      "content": "Let me check the weather for you.",
      "tool_calls": [{
        "id": "call_abc123",
        "type": "function",
        "function": {"name": "get_weather", "arguments": "{\"location\": \"Boston, MA\"}"}
      }]
    },
    {"role": "tool", "tool_call_id": "call_abc123", "content": "{\"temperature\": 72}"}
  ],
  "tools": [{
    "type": "function",
    "function": {
      "name": "get_weather",
      "description": "Get the current weather for a given location.",
      "parameters": "{\"type\": \"object\", \"properties\": {\"location\": {\"type\": \"string\"}}}",
      "strict": true
    }
  }],
  "tool_choice": "auto",
  "parallel_tool_calls": true,
  "max_completion_tokens": 1500,
  "top_p": 1.0,
  "stream": true
}"#;

#[test]
fn round_trips_openai_completion_full_request() {
    for size in [1, 16, 16 * 1024] {
        round_trip(
            COMPLETION_REQUEST,
            OpenAICompletionRequestDecoder::new(JsonDecoder::new()),
            OpenAICompletionRequestEncoder::new(JsonEncoder::new(), Default::default()),
            size,
        );
    }
}

#[test]
fn round_trips_openai_response_full_request() {
    for size in [1, 16, 16 * 1024] {
        round_trip(
            RESPONSE_REQUEST,
            OpenAIResponseRequestDecoder::new(JsonDecoder::new()),
            OpenAIResponseRequestEncoder::new(JsonEncoder::new(), Default::default()),
            size,
        );
    }
}

#[test]
fn decodes_openai_completion_request_fields() {
    let components = decode(
        COMPLETION_REQUEST.as_bytes(),
        OpenAICompletionRequestDecoder::new(JsonDecoder::new()),
    );
    assert_eq!(
        components[0],
        TextRequestComponent::Model("gpt-4o".to_owned())
    );
    let TextRequestComponent::Message(assistant) = &components[4] else {
        panic!("expected an assistant message");
    };
    assert_eq!(assistant.role, Role::Assistant);
    let calls = assistant.tool_calls.as_ref().expect("assistant tool calls");
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].id, "call_abc123");
    assert_eq!(calls[0].function.name, "get_weather");
    let TextRequestComponent::Message(result) = &components[5] else {
        panic!("expected a tool message");
    };
    assert_eq!(result.role, Role::Tool);
    assert_eq!(result.tool_call_id.as_deref(), Some("call_abc123"));
    let TextRequestComponent::Tools(tools) = &components[6] else {
        panic!("expected tool declarations");
    };
    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0].function.name, "get_weather");
    assert_eq!(tools[0].function.strict, Some(true));
    assert_eq!(
        stored_json(
            tools[0]
                .function
                .parameters
                .as_deref()
                .expect("tool parameters")
        ),
        json!({
            "type": "object",
            "properties": {"location": {"type": "string"}},
            "required": ["location"],
            "additionalProperties": false
        })
    );
    assert_eq!(
        components[7],
        TextRequestComponent::ToolChoice(ToolChoice::Auto)
    );
    assert_eq!(components[8], TextRequestComponent::ParallelToolCalls(true));
    assert_eq!(components[9], TextRequestComponent::MaxTokens(1500));
    assert_eq!(
        components[11],
        TextRequestComponent::Stop(Stop::Array(vec![
            "<|endoftext|>".to_owned(),
            "User:".to_owned()
        ]))
    );
    let TextRequestComponent::ResponseFormat(ResponseFormat::JsonSchema(schema)) = &components[12]
    else {
        panic!("expected a JSON schema response format");
    };
    assert_eq!(schema.name, "weather_response");
    assert_eq!(schema.strict, Some(true));
    assert_eq!(
        stored_json(&schema.schema),
        json!({
            "type": "object",
            "properties": {"temp": {"type": "number"}},
            "required": ["temp"],
            "additionalProperties": false
        })
    );
    assert_eq!(components[13], TextRequestComponent::Stream(true));
}

#[test]
fn decodes_openai_response_request_fields() {
    let components = decode(
        RESPONSE_REQUEST.as_bytes(),
        OpenAIResponseRequestDecoder::new(JsonDecoder::new()),
    );
    assert_eq!(
        components[0],
        TextRequestComponent::Model("gpt-4o".to_owned())
    );
    let TextRequestComponent::Message(call) = &components[5] else {
        panic!("expected a function call message");
    };
    assert_eq!(call.role, Role::Assistant);
    let calls = call.tool_calls.as_ref().expect("function call");
    assert_eq!(calls[0].id, "call_abc123");
    assert_eq!(calls[0].function.name, "get_weather");
    let TextRequestComponent::Message(result) = &components[6] else {
        panic!("expected a function call output message");
    };
    assert_eq!(result.role, Role::Tool);
    assert_eq!(result.tool_call_id.as_deref(), Some("call_abc123"));
    let TextRequestComponent::Tools(tools) = &components[7] else {
        panic!("expected tool declarations");
    };
    assert_eq!(tools[0].function.name, "get_weather");
    assert_eq!(tools[0].function.strict, Some(true));
    assert_eq!(
        stored_json(
            tools[0]
                .function
                .parameters
                .as_deref()
                .expect("tool parameters")
        ),
        json!({
            "type": "object",
            "properties": {"location": {"type": "string"}},
            "required": ["location"],
            "additionalProperties": false
        })
    );
    let TextRequestComponent::ResponseFormat(ResponseFormat::JsonSchema(schema)) = &components[12]
    else {
        panic!("expected a JSON schema response format");
    };
    assert_eq!(schema.name, "weather_response");
}

#[test]
fn round_trips_native_reasoning_effort() {
    let response = r#"{"model": "gpt-4o", "input": [{"type": "message", "role": "user", "content": "hi"}], "reasoning": {"effort": "medium"}}"#;
    let components = decode(
        response.as_bytes(),
        OpenAIResponseRequestDecoder::new(JsonDecoder::new()),
    );
    assert_eq!(
        components[2],
        TextRequestComponent::Reasoning(Reasoning {
            effort: Some(ReasoningEffort::Medium),
            ..Reasoning::default()
        })
    );
    let encoded = encode(
        components,
        OpenAIResponseRequestEncoder::new(JsonEncoder::new(), Default::default()),
    );
    assert_eq!(
        serde_json::from_slice::<Value>(&encoded).expect("encoded request"),
        serde_json::from_str::<Value>(response).expect("source request")
    );

    let completion = r#"{"model": "gpt-4o", "messages": [{"role": "user", "content": "hi"}], "reasoning_effort": "high"}"#;
    let components = decode(
        completion.as_bytes(),
        OpenAICompletionRequestDecoder::new(JsonDecoder::new()),
    );
    assert_eq!(
        components[2],
        TextRequestComponent::Reasoning(Reasoning {
            effort: Some(ReasoningEffort::High),
            ..Reasoning::default()
        })
    );
    let encoded = encode(
        components,
        OpenAICompletionRequestEncoder::new(JsonEncoder::new(), Default::default()),
    );
    assert_eq!(
        serde_json::from_slice::<Value>(&encoded).expect("encoded request"),
        serde_json::from_str::<Value>(completion).expect("source request")
    );
}

#[test]
fn translates_compatible_request_to_openai_response() {
    let components = decode(
        COMPATIBLE_REQUEST.as_bytes(),
        CompatibleOpenAIRequestDecoder::new(JsonDecoder::new()),
    );
    let encoded = encode(
        components,
        OpenAIResponseRequestEncoder::new(JsonEncoder::new(), Default::default()),
    );
    assert_eq!(
        serde_json::from_slice::<Value>(&encoded).expect("encoded request"),
        json!({
            "model": "gpt-4o",
            "input": [
                {"type": "message", "role": "user", "content": "Can you check the weather?"},
                {"type": "message", "role": "assistant", "content": "Let me check the weather for you."},
                {"type": "function_call", "call_id": "call_abc123", "name": "get_weather", "arguments": "{\"location\": \"Boston, MA\"}"},
                {"type": "function_call_output", "call_id": "call_abc123", "output": "{\"temperature\": 72}"}
            ],
            "tools": [{
                "type": "function",
                "name": "get_weather",
                "description": "Get the current weather for a given location.",
                "parameters": {"type": "object", "properties": {"location": {"type": "string"}}},
                "strict": true
            }],
            "tool_choice": "auto",
            "parallel_tool_calls": true,
            "max_output_tokens": 1500,
            "top_p": 1.0,
            "stream": true
        })
    );
}

#[test]
fn rejects_unsupported_openai_request_fields() {
    let request = r#"{"model": "gpt-4o", "input": [{"type": "message", "role": "user", "content": "hi"}], "temperature": 0.5}"#;
    let mut decoder = OpenAIResponseRequestDecoder::new(JsonDecoder::new());
    let mut input = request.as_bytes();
    let mut failure = None;
    loop {
        match decoder.next(&mut input, false) {
            Ok(DecoderStatus::Component(_)) => {}
            Ok(DecoderStatus::NeedInput) => break,
            Ok(DecoderStatus::Finished) => break,
            Err(error) => {
                failure = Some(error);
                break;
            }
        }
    }
    assert!(failure.is_some(), "expected an explicit decoding failure");
}

fn stored_json(value: &str) -> Value {
    serde_json::from_str(value).expect("stored JSON text")
}

fn decode<D>(request: &[u8], mut decoder: D) -> Vec<TextRequestComponent>
where
    D: for<'a> Decoder<Component<'a> = TextRequestComponent>,
    D::Error: Debug,
{
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
        decoder.next(&mut input, true).expect("decode EOF"),
        DecoderStatus::Finished
    );
    decoder.finish().expect("complete request at EOF");
    components
}

fn encode<E>(components: Vec<TextRequestComponent>, mut encoder: E) -> Vec<u8>
where
    E: Encoder<TextRequestComponent>,
    E::Error: Debug,
{
    let mut encoded = Vec::new();
    let mut output = vec![0_u8; 4096];
    for mut component in components {
        loop {
            match encoder
                .next(Some(&mut component), &mut output)
                .expect("encode component")
            {
                EncoderStatus::NeedOutput { written } => {
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
        match encoder.next(None, &mut output).expect("finalize encoding") {
            EncoderStatus::NeedOutput { written } => {
                encoded.extend_from_slice(&output[..written]);
            }
            EncoderStatus::Finished { written } => {
                encoded.extend_from_slice(&output[..written]);
                break;
            }
            EncoderStatus::ComponentFinished { .. } => panic!("component during finalization"),
        }
    }
    encoder.finish().expect("finish encoding");
    encoded
}

fn round_trip<D, E>(request: &str, mut decoder: D, mut encoder: E, size: usize)
where
    D: for<'a> Decoder<Component<'a> = TextRequestComponent>,
    D::Error: Debug,
    E: Encoder<TextRequestComponent>,
    E::Error: Debug,
{
    let mut output = vec![0; size];
    let mut encoded = Vec::new();
    let mut component_count = 0;
    for chunk in request.as_bytes().chunks(size) {
        let mut input = chunk;
        loop {
            match decoder.next(&mut input, false).expect("decode request") {
                DecoderStatus::Component(mut component) => {
                    component_count += 1;
                    loop {
                        match encoder
                            .next(Some(&mut component), &mut output)
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
                            EncoderStatus::Finished { .. } => {
                                panic!("finished before finalization")
                            }
                        }
                    }
                }
                DecoderStatus::NeedInput => {
                    assert!(input.is_empty());
                    break;
                }
                DecoderStatus::Finished => panic!("finished before source EOF"),
            }
        }
    }
    assert_eq!(
        decoder.next(&mut &b""[..], true).expect("decode EOF"),
        DecoderStatus::Finished
    );
    decoder.finish().expect("finish decoding");
    assert!(component_count > 0);
    loop {
        match encoder.next(None, &mut output).expect("finalize encoding") {
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
    encoder.finish().expect("finish encoding");
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&encoded).expect("encoded request JSON"),
        serde_json::from_str::<serde_json::Value>(request).unwrap(),
        "buffer size {size}",
    );
}
