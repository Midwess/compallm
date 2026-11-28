use codec::decoder::{Decoder, DecoderStatus};
use codec::encoder::{Encoder, EncoderStatus};
use codec::json::{JsonDecoder, JsonEncoder};
use codec::llm::text::component::TextRequestComponent;
use codec::llm::text::openai_response::request::{
    OpenAIResponseRequestDecoder, OpenAIResponseRequestEncoder,
};
#[path = "support/allocation.rs"]
mod allocation;
use allocation::AllocationMeasurement;

const REQUEST: &[u8] = br#"{"model":"gpt-model","input":[{"role":"system","content":"Help"},{"content":"hello\n\ud83e\udd80","role":"user"},{"type":"function_call","call_id":"call1","name":"weather","arguments":"{\"city\":\"Boston\"}"},{"type":"function_call_output","call_id":"call1","output":"sunny"}],"tools":[{"type":"function","name":"weather","description":"Weather","parameters":{"type":"object","properties":{"city":{"type":"string"}}},"strict":true}],"tool_choice":{"type":"function","name":"weather"},"parallel_tool_calls":false,"max_output_tokens":1024,"reasoning":{"effort":"high"},"top_p":0.7,"text":{"format":{"type":"json_schema","name":"answer","strict":true,"schema":{"type":"object"}}},"stream":true}"#;

fn roundtrip<const N: usize>() {
    let measurement = AllocationMeasurement::start();
    let mut decoder = OpenAIResponseRequestDecoder::new(JsonDecoder::new());
    let mut encoder = OpenAIResponseRequestEncoder::new(JsonEncoder::new(), Default::default());
    let construction = measurement.snapshot();
    let mut encoded = [0; 16384];
    let mut length = 0;
    let mut input_buffer = [0; N];
    let mut output = [0; N];
    let mut count = 0;
    for chunk in REQUEST.chunks(N) {
        input_buffer[..chunk.len()].copy_from_slice(chunk);
        let mut input = &input_buffer[..chunk.len()];
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
        input_buffer.fill(b'!');
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
    let total = measurement.snapshot();
    drop(measurement);
    assert_eq!(total.allocations - construction.allocations, 0);
    assert_eq!(total.reallocations - construction.reallocations, 0);
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&encoded[..length]).unwrap(),
        serde_json::from_slice::<serde_json::Value>(REQUEST).unwrap()
    );
    println!(
        "response {N}-byte buffers: {count} components; input={} output={length}; construction={construction:?}; processing allocations={} reallocations={} requested_bytes={}",
        REQUEST.len(),
        total.allocations - construction.allocations,
        total.reallocations - construction.reallocations,
        total.requested_bytes - construction.requested_bytes
    );
}
#[test]
fn roundtrips_with_zero_processing_allocations() {
    roundtrip::<1>();
    roundtrip::<16>();
    roundtrip::<16384>();
    roundtrip::<32768>();
    roundtrip::<65536>();
}

#[test]
fn borrows_string_input_as_a_completed_user_message() {
    use codec::llm::text::component::{Content, Role};
    let source = br#"{"model":"gpt-model","input":"hello"}"#;
    let range = source.as_ptr() as usize..source.as_ptr() as usize + source.len();
    let mut decoder = OpenAIResponseRequestDecoder::new(JsonDecoder::new());
    let mut input = source.as_slice();
    let mut count = 0;
    loop {
        match decoder.next(&mut input, true).unwrap() {
            DecoderStatus::Component(component) => {
                if let TextRequestComponent::Message(message) = &component {
                    assert_eq!(message.role, Role::User);
                    assert_eq!(message.content, Some(Content::Text("hello")));
                }
                component.map_text(|text| {
                    assert!(range.contains(&(text.as_ptr() as usize)));
                    count += 1;
                });
            }
            DecoderStatus::Finished => break,
            _ => panic!("complete source"),
        }
    }
    decoder.finish().unwrap();
    assert_eq!(count, 2);
}

#[test]
fn translates_to_other_openai_protocol() {
    let mut decoder = OpenAIResponseRequestDecoder::new(JsonDecoder::new());
    let mut encoder =
        codec::llm::text::openai_completion::request::OpenAICompletionRequestEncoder::new(
            JsonEncoder::new(),
            Default::default(),
        );
    let mut input = REQUEST;
    let mut output = [0; 1];
    let mut encoded = Vec::new();
    loop {
        match decoder.next(&mut input, true).unwrap() {
            DecoderStatus::Component(mut component) => loop {
                let status = encoder.next(Some(&mut component), &mut output).unwrap();
                let (written, done) = match status {
                    EncoderStatus::NeedOutput { written } => (written, false),
                    EncoderStatus::ComponentFinished { written } => (written, true),
                    _ => panic!("unexpected finish"),
                };
                encoded.extend_from_slice(&output[..written]);
                if done {
                    break;
                }
            },
            DecoderStatus::Finished => break,
            _ => panic!("complete input"),
        }
    }
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
        encoded.extend_from_slice(&output[..written]);
        if done {
            break;
        }
    }
    encoder.finish().unwrap();
    let value: serde_json::Value = serde_json::from_slice(&encoded).unwrap();
    assert_eq!(value["model"], "gpt-model");
    assert_eq!(
        value["messages"][2]["tool_calls"][0]["function"]["name"],
        "weather"
    );
    assert_eq!(value["messages"][3]["role"], "tool");
    assert_eq!(value["tools"][0]["function"]["name"], "weather");
    assert_eq!(value["response_format"]["json_schema"]["name"], "answer");
    let mut target =
        codec::llm::text::openai_completion::request::OpenAICompletionRequestDecoder::new(
            JsonDecoder::new(),
        );
    let mut input = encoded.as_slice();
    loop {
        match target.next(&mut input, true).unwrap() {
            DecoderStatus::Component(_) => (),
            DecoderStatus::Finished => break,
            _ => panic!("complete output"),
        }
    }
    target.finish().unwrap();
}
