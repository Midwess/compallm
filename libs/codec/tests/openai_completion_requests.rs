use codec::decoder::{Decoder, DecoderStatus};
use codec::encoder::{Encoder, EncoderStatus};
use codec::json::{JsonDecoder, JsonEncoder};
use codec::llm::text::component::TextRequestComponent;
use codec::llm::text::openai_completion::request::{
    OpenAICompletionRequestDecoder, OpenAICompletionRequestEncoder,
};
#[path = "support/allocation.rs"]
mod allocation;
use allocation::AllocationMeasurement;

const REQUEST: &[u8] = br#"{"model":"gpt-model","messages":[{"role":"system","content":"Help"},{"content":"hello\n\ud83e\udd80","role":"user"},{"role":"assistant","content":"Checking","tool_calls":[{"id":"call1","type":"function","function":{"name":"weather","arguments":"{\"city\":\"Boston\"}"}},{"id":"call2","type":"function","function":{"name":"weather","arguments":"{}"}}]},{"role":"tool","content":"sunny","tool_call_id":"call1"}],"tools":[{"type":"function","function":{"name":"weather","description":"Weather","parameters":{"type":"object","properties":{"city":{"type":"string"}}},"strict":true}}],"tool_choice":{"type":"function","function":{"name":"weather"}},"parallel_tool_calls":false,"max_completion_tokens":1024,"reasoning_effort":"high","top_p":0.7,"stop":["END"],"response_format":{"type":"json_schema","json_schema":{"name":"answer","strict":true,"schema":{"type":"object"}}},"stream":true,"stream_options":{"include_usage":true}}"#;

fn roundtrip<const N: usize>() {
    let measurement = AllocationMeasurement::start();
    let mut decoder = OpenAICompletionRequestDecoder::new(JsonDecoder::new());
    let mut encoder = OpenAICompletionRequestEncoder::new(JsonEncoder::new(), Default::default());
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
        "completion {N}-byte buffers: {count} components; input={} output={length}; construction={construction:?}; processing allocations={} reallocations={} requested_bytes={}",
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
fn borrows_message_text_and_normalizes_legacy_token_budget() {
    let source =
        br#"{"model":"gpt-model","messages":[{"role":"user","content":"hello"}],"max_tokens":42}"#;
    let range = source.as_ptr() as usize..source.as_ptr() as usize + source.len();
    let mut decoder = OpenAICompletionRequestDecoder::new(JsonDecoder::new());
    let mut encoder = OpenAICompletionRequestEncoder::new(JsonEncoder::new(), Default::default());
    let mut input = source.as_slice();
    let mut encoded = Vec::new();
    let mut output = [0; 4096];
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
                    _ => panic!("sufficient output space"),
                }
            }
            DecoderStatus::Finished => break,
            _ => panic!("complete source"),
        }
    }
    decoder.finish().unwrap();
    match encoder
        .next(None::<&mut TextRequestComponent>, &mut output)
        .unwrap()
    {
        EncoderStatus::Finished { written } => encoded.extend_from_slice(&output[..written]),
        _ => panic!("sufficient output space"),
    }
    encoder.finish().unwrap();
    assert_eq!(strings, 2);
    let value: serde_json::Value = serde_json::from_slice(&encoded).unwrap();
    assert_eq!(value["max_completion_tokens"], 42);
    assert!(value.get("max_tokens").is_none());
}

#[test]
fn translates_to_other_openai_protocol() {
    let mut decoder = OpenAICompletionRequestDecoder::new(JsonDecoder::new());
    let mut encoder = codec::llm::text::openai_response::request::OpenAIResponseRequestEncoder::new(
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
    assert_eq!(value["input"][3]["type"], "function_call");
    assert_eq!(value["input"][5]["type"], "function_call_output");
    assert_eq!(value["tools"][0]["name"], "weather");
    assert_eq!(value["text"]["format"]["name"], "answer");
    let mut target = codec::llm::text::openai_response::request::OpenAIResponseRequestDecoder::new(
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
