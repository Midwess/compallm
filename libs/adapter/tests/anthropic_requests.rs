use adapter::{
    codec::{DecodeStep, EncodeStep, TranslateStep},
    error::CodecError,
    t2t::{
        Text2TextRequestTranslator, Text2TextTranslatorConfiguration,
        anthropic::request::{AnthropicRequestDecoder, AnthropicRequestEncoder},
        openai::request::{OpenAICompletionRequestDecoder, OpenAICompletionRequestEncoder},
        request::{
            RequestScratch,
            decoder::RequestDecoder,
            encoder::RequestEncoder,
            event::{RequestEvent, Role},
        },
    },
};
use bytes::Bytes;
use futures_util::io::Cursor;
use serde_json::{Value, json};

const BODY: &[u8] = br#"{"system":"guide\n\uD83D\uDE00","stop_sequences":["END","\nUser:","\uD83D\uDE00"],"messages":[{"content":"hi \"there\"","role":"user"},{"role":"assistant","content":""}],"max_tokens":123,"stream":false,"model":"source"}"#;

fn configuration(max_output_tokens: u32) -> Text2TextTranslatorConfiguration {
    Text2TextTranslatorConfiguration { max_output_tokens }
}

fn decode(
    chunks: impl IntoIterator<Item = impl AsRef<[u8]>>,
    fragment: usize,
    deferred: usize,
) -> Result<Vec<RequestEvent<Bytes>>, CodecError> {
    let mut decoder = AnthropicRequestDecoder::new();
    let mut scratch = RequestScratch::new(fragment, deferred)?;
    let mut events = Vec::new();
    for chunk in chunks {
        let mut input = chunk.as_ref();
        loop {
            let before = input.len();
            match decoder.advance(&mut input, false, &mut scratch)? {
                DecodeStep::Event(event) => {
                    if let RequestEvent::TextFragment(bytes) = &event {
                        assert!(*bytes <= fragment);
                    }
                    events
                        .push(event.map_text(|len| Bytes::copy_from_slice(&scratch.text()[..len])));
                }
                DecodeStep::Progress => assert!(input.len() < before),
                DecodeStep::NeedInput => {
                    assert!(input.is_empty());
                    break;
                }
                DecodeStep::Finished => panic!("EOF not supplied"),
            }
        }
    }
    loop {
        match decoder.advance(&mut &b""[..], true, &mut scratch)? {
            DecodeStep::Event(event) => {
                events.push(event.map_text(|len| Bytes::copy_from_slice(&scratch.text()[..len])))
            }
            DecodeStep::Finished => return Ok(events),
            _ => panic!("EOF must finish or fail"),
        }
    }
}

fn drain(
    encoder: &mut dyn RequestEncoder,
    text: &mut &[u8],
    output: &mut Vec<u8>,
    size: usize,
) -> Result<bool, CodecError> {
    let mut buffer = vec![0; size];
    loop {
        match encoder.advance(text, &mut buffer)? {
            EncodeStep::Wrote(len) => {
                assert!(len > 0 && len <= size.min(4096));
                output.extend_from_slice(&buffer[..len]);
            }
            EncodeStep::NeedEvent => return Ok(false),
            EncodeStep::Finished => return Ok(true),
        }
    }
}

fn encode(
    encoder: &mut dyn RequestEncoder,
    events: Vec<RequestEvent<Bytes>>,
    size: usize,
) -> Result<Value, CodecError> {
    let mut output = Vec::new();
    assert!(!drain(encoder, &mut &b""[..], &mut output, size)?);
    for event in events {
        let text = match &event {
            RequestEvent::TextFragment(bytes) => bytes.as_ref(),
            _ => &[],
        };
        encoder.accept(event.clone().map_text(|bytes| bytes.len()))?;
        assert!(!drain(encoder, &mut &text[..], &mut output, size)?);
    }
    encoder.finish()?;
    assert!(drain(encoder, &mut &b""[..], &mut output, size)?);
    Ok(serde_json::from_slice(&output).unwrap())
}

fn expected() -> Value {
    json!({"model":"target\"\\", "max_tokens":123, "stream":false,"stop_sequences":["END","\nUser:","😀"],
        "system":[{"type":"text","text":"guide\n\u{1f600}"}], "messages":[
            {"role":"user","content":"hi \"there\""},
            {"role":"assistant","content":""}]})
}

#[test]
fn round_trip_at_every_split_and_one_byte_transport() {
    for split in 0..=BODY.len() {
        let events = decode([&BODY[..split], &BODY[split..]], 1, 100).unwrap();
        let mut encoder = AnthropicRequestEncoder::new("target\"\\", configuration(456)).unwrap();
        assert_eq!(encode(&mut encoder, events, 1).unwrap(), expected());
    }
    let events = decode(BODY.chunks(1), 3, 100).unwrap();
    for size in [1, 2, 7, 4097] {
        let mut encoder = AnthropicRequestEncoder::new("target\"\\", configuration(456)).unwrap();
        assert_eq!(
            encode(&mut encoder, events.clone(), size).unwrap(),
            expected()
        );
    }
}

#[test]
fn translates_anthropic_to_openai() {
    let mut encoder =
        OpenAICompletionRequestEncoder::new("openai-target", Default::default()).unwrap();
    let output = encode(&mut encoder, decode(BODY.chunks(1), 2, 100).unwrap(), 1).unwrap();
    assert_eq!(
        output,
        json!({"model":"openai-target","stream":false,"max_completion_tokens":123,"stop":["END","\nUser:","😀"],"messages":[
        {"role":"system","content":"guide\n\u{1f600}"},
        {"role":"user","content":"hi \"there\""},
        {"role":"assistant","content":""}]})
    );
}

#[test]
fn optional_system_empty_text_and_root_field_order() {
    for system in [None, Some(""), Some("raw \u{1f600}\0")] {
        for fields in [
            ["model", "max_tokens", "system", "messages"],
            ["system", "messages", "max_tokens", "model"],
            ["max_tokens", "model", "system", "messages"],
        ] {
            let mut source = json!({"model":"m","max_tokens":u32::MAX,
                "messages":[{"role":"user","content":""},{"role":"user","content":"next"}]});
            if let Some(system) = system {
                source["system"] = json!(system);
            }
            let members: Vec<_> = fields
                .into_iter()
                .filter_map(|field| {
                    source
                        .get(field)
                        .map(|value| format!("\"{field}\":{value}"))
                })
                .collect();
            let body = format!("{{{}}} \n\t", members.join(","));
            let events = decode(body.as_bytes().chunks(1), 1, 4).unwrap();
            let mut encoder = AnthropicRequestEncoder::new("m", configuration(u32::MAX)).unwrap();
            source["stream"] = json!(false);
            if let Some(system) = system {
                source["system"] = if system.is_empty() {
                    json!("")
                } else {
                    json!([{"type":"text","text":system}])
                };
            } else {
                source["system"] = json!("");
            }
            assert_eq!(encode(&mut encoder, events, 1).unwrap(), source);
        }
    }
}

#[test]
fn decoder_requires_eof_and_emits_system_fragments_before_full_input() {
    let mut decoder = AnthropicRequestDecoder::new();
    let mut scratch = RequestScratch::new(2, 100).unwrap();
    let mut input = &br#"{"system":"abcdef"#[..];
    assert!(matches!(
        decoder.advance(&mut input, false, &mut scratch).unwrap(),
        DecodeStep::Event(RequestEvent::MessageStarted { role: Role::System })
    ));
    assert!(matches!(
        decoder.advance(&mut input, false, &mut scratch).unwrap(),
        DecodeStep::Event(RequestEvent::TextStarted)
    ));
    assert!(matches!(
        decoder.advance(&mut input, false, &mut scratch).unwrap(),
        DecodeStep::Event(RequestEvent::TextFragment(2))
    ));
    assert_eq!(&scratch.text()[..2], b"ab");
    assert!(!input.is_empty());

    let mut decoder = AnthropicRequestDecoder::default();
    scratch.reset();
    let mut input = BODY;
    loop {
        match decoder.advance(&mut input, false, &mut scratch).unwrap() {
            DecodeStep::Event(_) | DecodeStep::Progress => {}
            DecodeStep::NeedInput => break,
            DecodeStep::Finished => panic!("EOF required"),
        }
    }
    assert!(matches!(
        decoder.advance(&mut input, true, &mut scratch).unwrap(),
        DecodeStep::Finished
    ));
    assert!(matches!(
        decoder.advance(&mut input, true, &mut scratch).unwrap(),
        DecodeStep::Finished
    ));
    assert!(decoder.advance(&mut &b"x"[..], true, &mut scratch).is_err());
}

#[tokio::test]
async fn translates_openai_to_anthropic_through_driver() {
    let source = br#"{"model":"logical","messages":[{"role":"system","content":"guide"},{"role":"user","content":"hi"}]}"#;
    let mut translator = Text2TextRequestTranslator::new(1, 1);
    let mut output = Cursor::new(Vec::new());
    translator
        .set_source_stream(Cursor::new(source))
        .unwrap()
        .set_target_stream(&mut output)
        .unwrap()
        .set_decoder(OpenAICompletionRequestDecoder::new())
        .unwrap()
        .set_encoder(AnthropicRequestEncoder::new("claude-target", configuration(789)).unwrap())
        .unwrap();
    loop {
        match translator.translate().unwrap() {
            TranslateStep::NeedInput => {
                translator.next().await.unwrap();
            }
            TranslateStep::OutputReady(_) => translator.flush().await.unwrap(),
            TranslateStep::Progress => tokio::task::yield_now().await,
            TranslateStep::Finished => {
                translator.flush().await.unwrap();
                break;
            }
        }
    }
    drop(translator);
    assert_eq!(
        serde_json::from_slice::<Value>(output.get_ref()).unwrap(),
        json!({
        "model":"claude-target","max_tokens":789,"stream":false,"system":[{"type":"text","text":"guide"}],
        "messages":[{"role":"user","content":"hi"}]})
    );
}

#[test]
fn rejects_invalid_or_unsupported_requests_at_single_byte_boundaries() {
    let valid = json!({"model":"m","max_tokens":1,"messages":[{"role":"user","content":"x"}]});
    for key in ["model", "max_tokens", "messages"] {
        let mut body = valid.clone();
        body.as_object_mut().unwrap().remove(key);
        assert!(decode([body.to_string()], 1, 10).is_err(), "missing {key}");
    }
    for budget in [
        json!(0),
        json!(-1),
        json!(1.5),
        json!("1"),
        json!(null),
        json!(4294967296_u64),
    ] {
        let mut body = valid.clone();
        body["max_tokens"] = budget;
        assert!(decode(body.to_string().as_bytes().chunks(1), 1, 10).is_err());
    }
    for body in [
        r#"{"model":"m","max_tokens":01,"messages":[{"role":"user","content":"x"}]}"#,
        r#"{"model":"m","max_tokens":1e2,"messages":[{"role":"user","content":"x"}]}"#,
        r#"{"model":"m","max_tokens":1,"max_tokens":2,"messages":[{"role":"user","content":"x"}]}"#,
        r#"{"model":"m","max_tokens":1,"messages":[{"role":"user","content":"x"}],"system":"late"}"#,
        r#"{"model":"m","max_tokens":1,"messages":[{"role":"system","content":"x"}]}"#,
        r#"{"model":"m","max_tokens":1,"messages":[{"role":"user","content":[]}]}"#,
        r#"{"model":"m","max_tokens":1,"messages":[]}"#,
        r#"{"model":"m","max_tokens":1,"system":"x","messages":[]}"#,
        r#"{"model":"m","max_tokens":1,"messages":[{"role":"user","content":"\uD800"}]}"#,
        r#"{"model":"m","max_tokens":1,"messages":[{"role":"user","content":"\uDC00"}]}"#,
    ] {
        assert!(decode(body.as_bytes().chunks(1), 1, 10).is_err(), "{body}");
    }
    for (key, value) in [
        ("stream", json!("true")),
        ("stream", json!(null)),
        ("tools", json!([])),
        ("temperature", json!(1)),
    ] {
        let mut body = valid.clone();
        body[key] = value;
        assert!(decode(body.to_string().as_bytes().chunks(1), 1, 10).is_err());
    }
    for end in 0..BODY.len() {
        assert!(decode([&BODY[..end]], 1, 100).is_err(), "truncation {end}");
    }
    let mut trailing = BODY.to_vec();
    trailing.push(b'x');
    assert!(decode(trailing.chunks(1), 1, 100).is_err());
    let mut invalid_utf8 =
        br#"{"model":"m","max_tokens":1,"messages":[{"role":"user","content":""#.to_vec();
    invalid_utf8.extend_from_slice(b"\xff\"}]}");
    assert!(decode(invalid_utf8.chunks(1), 1, 10).is_err());
}

#[test]
fn enforces_bounds_but_streams_long_role_first_content() {
    assert!(AnthropicRequestEncoder::new("m", configuration(0)).is_err());
    assert!(AnthropicRequestEncoder::new("", configuration(1)).is_err());
    assert!(AnthropicRequestEncoder::new(&"m".repeat(1025), configuration(1)).is_err());
    let text = "x".repeat(100_000);
    let body = format!(
        r#"{{"model":"m","max_tokens":4294967295,"messages":[{{"role":"user","content":"{text}"}}]}}"#
    );
    let events = decode(body.as_bytes().chunks(37), 13, 0).unwrap();
    let length: usize = events
        .iter()
        .map(|event| match event {
            RequestEvent::TextFragment(bytes) => bytes.len(),
            _ => 0,
        })
        .sum();
    assert_eq!(length, text.len());
    let late = br#"{"model":"m","max_tokens":1,"messages":[{"content":"ab","role":"user"}]}"#;
    assert!(matches!(
        decode(late.chunks(1), 1, 1),
        Err(CodecError::LimitExceeded(_))
    ));
    assert!(decode(late.chunks(1), 1, 2).is_ok());
    let model =
        json!({"model":"m".repeat(1025),"max_tokens":1,"messages":[{"role":"user","content":""}]})
            .to_string();
    assert!(matches!(
        decode([model], 1, 0),
        Err(CodecError::LimitExceeded(_))
    ));
}

#[test]
fn rejects_misordered_events_and_invalid_utf8() {
    let message = |role| {
        vec![
            RequestEvent::MessageStarted { role },
            RequestEvent::TextStarted,
            RequestEvent::TextEnded,
            RequestEvent::MessageEnded,
        ]
    };
    for events in [
        vec![],
        message(Role::System),
        [message(Role::User), message(Role::System)].concat(),
        vec![RequestEvent::TextStarted],
        vec![
            RequestEvent::MessageStarted { role: Role::User },
            RequestEvent::TextStarted,
            RequestEvent::TextEnded,
            RequestEvent::TextStarted,
        ],
        vec![
            RequestEvent::MessageStarted { role: Role::User },
            RequestEvent::TextStarted,
            RequestEvent::TextFragment(Bytes::from_static(b"\xff")),
        ],
        vec![
            RequestEvent::MessageStarted { role: Role::User },
            RequestEvent::TextStarted,
            RequestEvent::TextFragment(Bytes::from_static(b"\xc3")),
            RequestEvent::TextEnded,
        ],
        vec![
            RequestEvent::MessageStarted { role: Role::User },
            RequestEvent::TextStarted,
            RequestEvent::TextFragment(Bytes::from_static(b"too long")),
        ],
    ] {
        let mut encoder = AnthropicRequestEncoder::new("m", configuration(1)).unwrap();
        assert!(encode(&mut encoder, events, 1).is_err());
        assert!(encoder.advance(&mut &b""[..], &mut [0]).is_err());
    }
    let mut encoder = AnthropicRequestEncoder::new("m", configuration(1)).unwrap();
    assert!(
        encoder
            .accept(RequestEvent::MessageStarted { role: Role::User })
            .is_err()
    );
    let mut decoder = AnthropicRequestDecoder::new();
    let mut scratch = RequestScratch::new(1, 0).unwrap();
    assert!(
        decoder
            .advance(&mut &b"!"[..], false, &mut scratch)
            .is_err()
    );
    assert!(decoder.advance(&mut &b""[..], true, &mut scratch).is_err());
}
