use codec::decoder::{Decoder, DecoderStatus};
use codec::encoder::{Encoder, EncoderStatus};
use codec::json::{JsonDecoder, JsonEncoder};
use codec::llm::text::component::TextRequestComponent;
use codec::llm::text::openai::request::completions::{
    decoder::OpenAICompletionRequestDecoder, encoder::OpenAICompletionRequestEncoder,
};
use codec::llm::text::openai::request::responses::{
    decoder::OpenAIResponseRequestDecoder, encoder::OpenAIResponseRequestEncoder,
};
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
