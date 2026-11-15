use codec::decoder::{Decoder, DecoderStatus};
use codec::llm::text::component::*;
use codec::llm::text::copenai::request::CompatibleOpenAIRequestDecoder;

#[test]
fn decodes_full_request_with_16_byte_input_buffer() {
    let request = r#"{
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

    let mut decoder = CompatibleOpenAIRequestDecoder::new();
    let mut components = Vec::new();
    let mut buffer = [0_u8; 16];
    for chunk in request.as_bytes().chunks(buffer.len()) {
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

    assert_eq!(components, vec![
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
    ]);
}
