use std::convert::Infallible;

use adapter::{
    codec::{DecodeStep, EncodeStep, TranslateStep},
    error::CodecError,
    t2t::{
        Text2TextRequestTranslator,
        openai::request::{
            ChatCompletionsRequestDecoder, ChatCompletionsRequestEncoder, ResponsesRequestDecoder,
            ResponsesRequestEncoder,
        },
        request::{
            decoder::RequestDecoder,
            encoder::RequestEncoder,
            event::{RequestEvent, Role},
        },
    },
};
use bytes::Bytes;
use futures_util::{TryStreamExt, io::Cursor, stream};
use serde_json::{Value, json};

#[derive(Clone, Copy, Debug)]
enum Api {
    Chat,
    Responses,
}
const APIS: [Api; 2] = [Api::Chat, Api::Responses];

fn decoder(api: Api, fragment: usize, deferred: usize) -> Box<dyn RequestDecoder> {
    match api {
        Api::Chat => Box::new(ChatCompletionsRequestDecoder::new(fragment, deferred).unwrap()),
        Api::Responses => Box::new(ResponsesRequestDecoder::new(fragment, deferred).unwrap()),
    }
}
fn encoder(api: Api, model: &str, fragment: usize) -> Box<dyn RequestEncoder> {
    match api {
        Api::Chat => Box::new(ChatCompletionsRequestEncoder::new(model, fragment).unwrap()),
        Api::Responses => Box::new(ResponsesRequestEncoder::new(model, fragment).unwrap()),
    }
}

fn decode<'a>(
    api: Api,
    chunks: impl IntoIterator<Item = &'a [u8]>,
    fragment: usize,
    deferred: usize,
) -> Result<Vec<RequestEvent>, CodecError> {
    let mut decoder = decoder(api, fragment, deferred);
    let mut events = Vec::new();
    for mut input in chunks {
        loop {
            let before = input.len();
            match decoder.advance(&mut input, false)? {
                DecodeStep::Event(event) => {
                    if let RequestEvent::TextFragment(bytes) = &event {
                        assert!(bytes.len() <= fragment);
                    }
                    events.push(event);
                }
                DecodeStep::Progress => assert!(input.len() < before),
                DecodeStep::NeedInput => {
                    assert!(input.is_empty());
                    break;
                }
                DecodeStep::Finished => panic!("unvalidated EOF must not finish"),
            }
        }
    }
    let mut input = &b""[..];
    for _ in 0..10000 {
        match decoder.advance(&mut input, true)? {
            DecodeStep::Event(event) => events.push(event),
            DecodeStep::Finished => return Ok(events),
            _ => panic!("EOF must finalize or reject"),
        }
    }
    panic!("decoder did not finish")
}

fn messages(events: &[RequestEvent]) -> Vec<(Role, Vec<u8>)> {
    let mut result = Vec::new();
    let mut text = false;
    let mut message = false;
    for event in events {
        match event {
            RequestEvent::MessageStarted { role } => {
                assert!(!message);
                result.push((*role, Vec::new()));
                message = true;
            }
            RequestEvent::TextStarted => {
                assert!(message && !text);
                text = true;
            }
            RequestEvent::TextFragment(bytes) => {
                assert!(text);
                result.last_mut().unwrap().1.extend_from_slice(bytes);
            }
            RequestEvent::TextEnded => {
                assert!(text);
                text = false;
            }
            RequestEvent::MessageEnded => {
                assert!(message && !text);
                message = false;
            }
        }
    }
    assert!(!message && !text);
    result
}

fn drain(
    encoder: &mut dyn RequestEncoder,
    output: &mut Vec<u8>,
    size: usize,
) -> Result<bool, CodecError> {
    let mut buffer = vec![0xa5; size];
    for _ in 0..10000 {
        match encoder.advance(&mut buffer)? {
            EncodeStep::Wrote(len) => {
                assert!(len > 0 && len <= size);
                output.extend_from_slice(&buffer[..len]);
            }
            EncodeStep::NeedEvent => return Ok(false),
            EncodeStep::Finished => return Ok(true),
        }
    }
    panic!("encoder failed to drain")
}

fn encode(
    api: Api,
    events: Vec<RequestEvent>,
    size: usize,
    fragment: usize,
) -> Result<Vec<u8>, CodecError> {
    let mut encoder = encoder(api, "target\"\\🙂", fragment);
    let mut output = Vec::new();
    assert!(!drain(&mut *encoder, &mut output, size)?);
    for event in events {
        encoder.accept(event)?;
        assert!(!drain(&mut *encoder, &mut output, size)?);
    }
    encoder.finish()?;
    assert!(drain(&mut *encoder, &mut output, size)?);
    Ok(output)
}

fn fixture(api: Api) -> Vec<u8> {
    let body: &[u8] = match api {
        Api::Chat => br#"{"messages":[{"content":"hi \"there\" \uD83D\uDE00\/\b\f\n\r\t\u0000","role":"user"},{"role":"system","content":""},{"role":"assistant","content":"raw text"}],"stream":false,"model":"client"}"#,
        Api::Responses => br#"{"input":[{"content":"hi \"there\" \uD83D\uDE00\/\b\f\n\r\t\u0000","type":"message","role":"user"},{"role":"system","content":""},{"role":"assistant","content":"raw text"}],"stream":false,"model":"client"}"#,
    };
    body.to_vec()
}

#[test]
fn both_decoders_preserve_events_at_every_input_split_and_single_byte_chunks() {
    for api in APIS {
        let body = fixture(api);
        let expected = vec![
            (
                Role::User,
                "hi \"there\" 😀/\u{8}\u{c}\n\r\t\0".as_bytes().to_vec(),
            ),
            (Role::System, vec![]),
            (Role::Assistant, b"raw text".to_vec()),
        ];
        for split in 0..=body.len() {
            let events = decode(api, [&body[..split], &body[split..]], 3, 64).unwrap();
            assert_eq!(messages(&events), expected, "{api:?} split {split}");
        }
        assert_eq!(
            messages(&decode(api, body.chunks(1), 1, 64).unwrap()),
            expected
        );
    }
}

#[test]
fn all_four_format_pairings_round_trip_through_one_byte_encoder_output() {
    for source in APIS {
        for target in APIS {
            for fragment in [1, 3, 8] {
                for output_size in 1..=8 {
                    let original = fixture(source);
                    let events = decode(source, original.chunks(1), fragment, 64).unwrap();
                    let expected = messages(&events);
                    let output = encode(target, events, output_size, fragment).unwrap();
                    let value: Value = serde_json::from_slice(&output).unwrap();
                    assert_eq!(value["model"], "target\"\\🙂");
                    assert_eq!(value["stream"], false);
                    let key = match target {
                        Api::Chat => "messages",
                        Api::Responses => "input",
                    };
                    assert_eq!(value[key].as_array().unwrap().len(), 3);
                    if matches!(target, Api::Responses) {
                        assert_eq!(value["input"][0]["type"], "message");
                    }
                    assert_eq!(
                        messages(&decode(target, output.chunks(1), fragment, 64).unwrap()),
                        expected
                    );
                }
            }
        }
    }
}

#[test]
fn responses_string_input_is_a_user_message_including_empty_and_escaped_text() {
    for text in ["", "hello\nworld", "café 🙂", "\0\t\"\\"] {
        let body = serde_json::to_vec(&json!({"model":"client", "input":text})).unwrap();
        assert_eq!(
            messages(&decode(Api::Responses, body.chunks(1), 1, 0).unwrap()),
            vec![(Role::User, text.as_bytes().to_vec())]
        );
    }
}

#[test]
fn bounded_role_late_deferral_uses_decoded_utf8_bytes_and_enforces_exact_limits() {
    for api in APIS {
        let key = match api {
            Api::Chat => "messages",
            Api::Responses => "input",
        };
        let body =
            format!(r#"{{"model":"m","{key}":[{{"content":"\uD83D\uDE00","role":"user"}}]}}"#)
                .into_bytes();
        assert_eq!(
            messages(&decode(api, body.chunks(1), 1, 4).unwrap()),
            vec![(Role::User, "😀".as_bytes().to_vec())]
        );
        assert!(matches!(
            decode(api, body.chunks(1), 1, 3),
            Err(CodecError::LimitExceeded(_))
        ));
        let empty =
            format!(r#"{{"model":"m","{key}":[{{"content":"","role":"user"}}]}}"#).into_bytes();
        assert_eq!(
            messages(&decode(api, empty.chunks(1), 1, 0).unwrap()),
            vec![(Role::User, vec![])]
        );
    }
}

#[test]
fn long_role_first_text_emits_fragments_early_without_collecting_the_message() {
    for api in APIS {
        let key = match api {
            Api::Chat => "messages",
            Api::Responses => "input",
        };
        let body = format!(
            r#"{{"model":"m","{key}":[{{"role":"user","content":"{}"}}]}}"#,
            "x".repeat(1024 * 1024)
        );
        let mut input = body.as_bytes();
        let mut decoder = decoder(api, 7, 0);
        let mut saw_fragment = false;
        for _ in 0..10 {
            if let DecodeStep::Event(RequestEvent::TextFragment(bytes)) =
                decoder.advance(&mut input, false).unwrap()
            {
                assert_eq!(bytes, b"xxxxxxx".as_slice());
                saw_fragment = true;
                break;
            }
        }
        assert!(saw_fragment);
        assert!(input.len() > body.len() - 128);
        let mut decoded_bytes = 7;
        loop {
            match decoder.advance(&mut input, false).unwrap() {
                DecodeStep::Event(RequestEvent::TextFragment(bytes)) => {
                    assert!(bytes.len() <= 7);
                    decoded_bytes += bytes.len();
                }
                DecodeStep::NeedInput => {
                    assert!(input.is_empty());
                    break;
                }
                DecodeStep::Finished => panic!("source EOF has not been provided"),
                _ => {}
            }
        }
        assert_eq!(decoded_bytes, 1024 * 1024);
        assert!(matches!(
            decoder.advance(&mut input, true).unwrap(),
            DecodeStep::Finished
        ));
        let late = format!(
            r#"{{"model":"m","{key}":[{{"content":"{}","role":"user"}}]}}"#,
            "x".repeat(1024 * 1024)
        );
        let mut input = late.as_bytes();
        let mut decoder = self::decoder(api, 7, 32);
        assert!(matches!(
            decoder.advance(&mut input, false),
            Err(CodecError::LimitExceeded(_))
        ));
        assert!(input.len() > late.len() - 128);
    }
}

#[test]
fn json_syntax_duplicates_unknown_fields_and_unsupported_semantics_are_rejected() {
    let invalid = [
        "{}",
        "[]",
        "null",
        r#"{"model":"m"}"#,
        r#"{"model":"","messages":[]}"#,
        r#"{"model":"m","messages":[]}"#,
        r#"{"model":"m","messages":[{}]}"#,
        r#"{"model":"m","messages":[{"role":"user"}]}"#,
        r#"{"model":"m","messages":[{"content":"x"}]}"#,
        r#"{"model":"m","model":"n","messages":[{"role":"user","content":"x"}]}"#,
        r#"{"model":"m","mo\u0064el":"n","messages":[{"role":"user","content":"x"}]}"#,
        r#"{"model":"m","messages":[{"role":"user","role":"user","content":"x"}]}"#,
        r#"{"model":"m","messages":[{"role":"user","content":"x","content":"y"}]}"#,
        r#"{"model":"m","messages":[{"role":"developer","content":"x"}]}"#,
        r#"{"model":"m","messages":[{"role":"tool","content":"x"}]}"#,
        r#"{"model":"m","messages":[{"role":"user","content":null}]}"#,
        r#"{"model":"m","messages":[{"role":"user","content":[{"type":"text","text":"x"}]}]}"#,
        r#"{"model":"m","messages":[{"role":"user","content":"x","name":"n"}]}"#,
        r#"{"model":"m","messages":[{"role":"user","content":"x"},]}"#,
        r#"{"model":"m","messages":[{"role":"user","content":"x",}]}"#,
        r#"{"model":"m","messages":[{"role":"user","content":"x"}],}"#,
        r#"{"model":"m","messages":[{"role":"user","content":"x"}],"stream":true}"#,
        r#"{"model":"m","messages":[{"role":"user","content":"x"}],"stream":null}"#,
        r#"{"model":"m","messages":[{"role":"user","content":"x"}],"stream":falsex}"#,
        r#"{"model":"m","messages":[{"role":"user","content":"x"}],"tools":[]}"#,
        r#"{"model":"m","messages":[{"role":"user","content":"x"}],"max_tokens":1}"#,
    ];
    for api in APIS {
        for body in invalid {
            let body = if matches!(api, Api::Responses) {
                body.replace("messages", "input")
            } else {
                body.to_owned()
            };
            assert!(
                decode(api, body.as_bytes().chunks(1), 2, 32).is_err(),
                "accepted {api:?}: {body}"
            );
        }
    }
    for body in [
        r#"{"model":"m","input":"x","instructions":"system"}"#,
        r#"{"model":"m","input":"x","previous_response_id":"id"}"#,
        r#"{"model":"m","input":[{"type":"function_call","role":"user","content":"x"}]}"#,
    ] {
        assert!(decode(Api::Responses, body.as_bytes().chunks(1), 2, 32).is_err());
    }
}

#[test]
fn every_truncated_prefix_and_non_whitespace_trailer_is_rejected() {
    for api in APIS {
        let body = fixture(api);
        for end in 0..body.len() {
            assert!(
                decode(api, body[..end].chunks(1), 3, 64).is_err(),
                "accepted truncation {api:?} {end}"
            );
        }
        for trailer in [b"{}".as_slice(), b"x", b"\0", b","] {
            let mut invalid = body.clone();
            invalid.extend_from_slice(trailer);
            assert!(decode(api, invalid.chunks(1), 3, 64).is_err());
        }
        let mut valid = body.clone();
        valid.extend_from_slice(b" \n\r\t");
        assert!(decode(api, valid.chunks(1), 3, 64).is_ok());
    }
}

#[test]
fn malformed_utf8_escapes_and_surrogates_are_rejected_at_every_boundary() {
    let bad: &[&[u8]] = &[
        b"\\uD800",
        b"\\uDC00",
        b"\\uD800\\u0041",
        b"\\uD800x",
        b"\\u123g",
        b"\\z",
        b"\n",
        &[0xff],
        &[0xc0, 0x80],
        &[0xe0, 0x80, 0x80],
        &[0xed, 0xa0, 0x80],
        &[0xf4, 0x90, 0x80, 0x80],
        &[0xe2, b'(', 0xa1],
        &[0xf0, 0x9f],
    ];
    for api in APIS {
        for text in bad {
            let key = match api {
                Api::Chat => "messages",
                Api::Responses => "input",
            };
            let mut body =
                format!(r#"{{"model":"m","{key}":[{{"role":"user","content":""#).into_bytes();
            body.extend_from_slice(text);
            body.extend_from_slice(br#""}]}"#);
            for size in 1..=4 {
                assert!(decode(api, body.chunks(size), 1, 32).is_err());
            }
        }
    }
}

#[test]
fn escaped_keys_roles_and_raw_utf8_are_recognized_without_changing_content() {
    for api in APIS {
        let key = match api {
            Api::Chat => r#"messa\u0067es"#,
            Api::Responses => r#"in\u0070ut"#,
        };
        let body = format!(
            r#"{{"mo\u0064el":"模型","{key}":[{{"ro\u006ce":"us\u0065r","con\u0074ent":"café🙂"}}]}}"#
        );
        assert_eq!(
            messages(&decode(api, body.as_bytes().chunks(1), 1, 0).unwrap()),
            vec![(Role::User, "café🙂".as_bytes().to_vec())]
        );
    }
}

#[test]
fn encoders_reject_invalid_order_oversized_fragments_and_incomplete_utf8() {
    for api in APIS {
        let cases = vec![
            vec![],
            vec![RequestEvent::TextEnded],
            vec![RequestEvent::MessageStarted { role: Role::User }],
            vec![
                RequestEvent::MessageStarted { role: Role::User },
                RequestEvent::MessageEnded,
            ],
            vec![
                RequestEvent::MessageStarted { role: Role::User },
                RequestEvent::TextStarted,
                RequestEvent::TextFragment(Bytes::from_static(b"long")),
            ],
            vec![
                RequestEvent::MessageStarted { role: Role::User },
                RequestEvent::TextStarted,
                RequestEvent::TextFragment(Bytes::from_static(&[0xff])),
            ],
            vec![
                RequestEvent::MessageStarted { role: Role::User },
                RequestEvent::TextStarted,
                RequestEvent::TextFragment(Bytes::from_static(&[0xf0])),
                RequestEvent::TextEnded,
            ],
            vec![
                RequestEvent::MessageStarted { role: Role::User },
                RequestEvent::TextStarted,
                RequestEvent::TextEnded,
                RequestEvent::TextStarted,
            ],
        ];
        for events in cases {
            assert!(encode(api, events, 1, 2).is_err());
        }
        let mut encoder = encoder(api, "target", 2);
        assert!(matches!(
            encoder.accept(RequestEvent::TextStarted),
            Err(CodecError::Contract(_))
        ));
        assert!(
            encoder.advance(&mut [0; 8]).is_err(),
            "errors must remain terminal"
        );
        let mut encoder = self::encoder(api, "target", 2);
        assert!(matches!(
            encoder.advance(&mut []),
            Err(CodecError::Contract(_))
        ));
    }
}

#[test]
fn configuration_model_bounds_and_advance_work_are_bounded() {
    assert!(ChatCompletionsRequestDecoder::new(0, 32).is_err());
    assert!(ResponsesRequestDecoder::new(0, 32).is_err());
    assert!(ChatCompletionsRequestEncoder::new("", 8).is_err());
    assert!(ResponsesRequestEncoder::new("", 8).is_err());
    assert!(ChatCompletionsRequestEncoder::new("model", 0).is_err());
    assert!(ResponsesRequestEncoder::new("model", 0).is_err());
    for api in APIS {
        let mut padded = vec![b' '; 8192];
        padded.extend_from_slice(&fixture(api));
        let mut input = padded.as_slice();
        let mut bounded = decoder(api, 8, 32);
        assert!(matches!(
            bounded.advance(&mut input, false).unwrap(),
            DecodeStep::Progress
        ));
        assert_eq!(padded.len() - input.len(), 4096);
        let key = match api {
            Api::Chat => "messages",
            Api::Responses => "input",
        };
        let body = format!(
            r#"{{"model":"{}","{key}":[{{"role":"user","content":"x"}}]}}"#,
            "m".repeat(1025)
        );
        assert!(matches!(
            decode(api, body.as_bytes().chunks(1), 8, 32),
            Err(CodecError::LimitExceeded(_))
        ));
        match api {
            Api::Chat => assert!(ChatCompletionsRequestEncoder::new("m".repeat(1025), 8).is_err()),
            Api::Responses => assert!(ResponsesRequestEncoder::new("m".repeat(1025), 8).is_err()),
        }
        let mut encoder = encoder(api, &"\0".repeat(1024), 8);
        let mut output = vec![0xa5; 16384];
        let EncodeStep::Wrote(len) = encoder.advance(&mut output).unwrap() else {
            panic!("expected model output");
        };
        assert_eq!(len, 4096);
        assert!(output[len..].iter().all(|byte| *byte == 0xa5));
    }
}

#[test]
fn encoder_validates_fragments_incrementally_instead_of_scanning_them_on_accept() {
    for api in APIS {
        let mut encoder = encoder(api, "target", 8192);
        let mut ignored = Vec::new();
        drain(&mut *encoder, &mut ignored, 16).unwrap();
        encoder
            .accept(RequestEvent::MessageStarted { role: Role::User })
            .unwrap();
        drain(&mut *encoder, &mut ignored, 16).unwrap();
        encoder.accept(RequestEvent::TextStarted).unwrap();
        drain(&mut *encoder, &mut ignored, 16).unwrap();
        let mut fragment = vec![b'x'; 8192];
        fragment[8191] = 0xff;
        encoder
            .accept(RequestEvent::TextFragment(Bytes::from(fragment)))
            .unwrap();
        let mut output = vec![0xa5; 16384];
        let EncodeStep::Wrote(len) = encoder.advance(&mut output).unwrap() else {
            panic!("expected first fragment prefix");
        };
        assert_eq!(len, 4096);
        assert!(output[..len].iter().all(|byte| *byte == b'x'));
        assert!(output[len..].iter().all(|byte| *byte == 0xa5));
        assert!(matches!(
            encoder.advance(&mut output),
            Err(CodecError::InvalidInput(_))
        ));
        assert!(matches!(
            encoder.advance(&mut output),
            Err(CodecError::Contract(_))
        ));
    }
}

#[tokio::test]
async fn concrete_codecs_work_through_the_explicit_translator_step_api() {
    for source in APIS {
        for target in APIS {
            let body = fixture(source);
            let chunks = body
                .chunks(1)
                .map(|chunk| Ok::<_, Infallible>(Bytes::copy_from_slice(chunk)))
                .collect::<Vec<_>>();
            let reader = stream::iter(chunks)
                .map_err(std::io::Error::other)
                .into_async_read();
            let mut writer = Cursor::new(Vec::new());
            let mut translator = Text2TextRequestTranslator::new(3, 2);
            translator
                .set_source_stream(reader)
                .unwrap()
                .set_target_stream(&mut writer)
                .unwrap()
                .set_decoder(decoder(source, 2, 64))
                .unwrap()
                .set_encoder(encoder(target, "selected-model", 2))
                .unwrap();
            for _ in 0..10000 {
                match translator.translate().unwrap() {
                    TranslateStep::OutputReady(_) => translator.flush().await.unwrap(),
                    TranslateStep::NeedInput => {
                        translator.next().await.unwrap();
                    }
                    TranslateStep::Progress => tokio::task::yield_now().await,
                    TranslateStep::Finished => {
                        translator.flush().await.unwrap();
                        break;
                    }
                }
            }
            assert_eq!(translator.translate().unwrap(), TranslateStep::Finished);
            drop(translator);
            let value: Value = serde_json::from_slice(writer.get_ref()).unwrap();
            assert_eq!(value["model"], "selected-model");
        }
    }
}

#[test]
fn surrogate_extremes_and_mixed_control_unicode_text_round_trip() {
    for api in APIS {
        let key = match api {
            Api::Chat => "messages",
            Api::Responses => "input",
        };
        let body = format!(
            r#"{{"model":"m","{key}":[{{"role":"user","content":"\uD800\uDC00\uDBFF\uDFFF"}}]}}"#
        );
        assert_eq!(
            messages(&decode(api, body.as_bytes().chunks(1), 1, 0).unwrap()),
            vec![(Role::User, "\u{10000}\u{10ffff}".as_bytes().to_vec())]
        );
        let mut seed = 17_u64;
        for count in 0..32 {
            let mut text = String::new();
            for _ in 0..count {
                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                let choices = [
                    '\0',
                    '\u{1}',
                    '\u{8}',
                    '\u{1f}',
                    '\t',
                    '\n',
                    '\r',
                    '"',
                    '\\',
                    '/',
                    'a',
                    'é',
                    '中',
                    '🙂',
                    '\u{10000}',
                    '\u{10ffff}',
                ];
                text.push(choices[(seed as usize) % choices.len()]);
            }
            let body =
                serde_json::to_vec(&json!({"model":"m", key:[{"content":text,"role":"user"}]}))
                    .unwrap();
            let events = decode(api, body.chunks(1), 1, 512).unwrap();
            let output = encode(api, events, 1, 1).unwrap();
            let parsed: Value = serde_json::from_slice(&output).unwrap();
            assert_eq!(parsed[key][0]["content"], text);
        }
    }
}

#[test]
fn exact_model_bound_and_message_field_permutations_remain_supported() {
    for api in APIS {
        let key = match api {
            Api::Chat => "messages",
            Api::Responses => "input",
        };
        let body = serde_json::to_vec(
            &json!({"model":"m".repeat(1024), key:[{"role":"user","content":"x"}]}),
        )
        .unwrap();
        assert!(decode(api, body.chunks(1), 2, 32).is_ok());
        let mut encoder = encoder(api, &"m".repeat(1024), 2);
        let mut output = Vec::new();
        assert!(!drain(&mut *encoder, &mut output, 7).unwrap());
        for event in decode(api, body.chunks(1), 2, 32).unwrap() {
            encoder.accept(event).unwrap();
            drain(&mut *encoder, &mut output, 7).unwrap();
        }
        encoder.finish().unwrap();
        assert!(drain(&mut *encoder, &mut output, 7).unwrap());
        let value: Value = serde_json::from_slice(&output).unwrap();
        assert_eq!(value["model"], "m".repeat(1024));
    }
    let fields = [
        r#""role":"user""#,
        r#""content":"x""#,
        r#""type":"message""#,
    ];
    for order in [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ] {
        let message = order.map(|index| fields[index]).join(",");
        let body = format!(r#"{{"input":[{{{message}}}],"model":"m"}}"#);
        assert_eq!(
            messages(&decode(Api::Responses, body.as_bytes().chunks(1), 1, 1).unwrap()),
            vec![(Role::User, b"x".to_vec())]
        );
    }
}

#[test]
fn decoder_eof_is_explicit_and_codec_errors_remain_terminal() {
    for api in APIS {
        let body = fixture(api);
        let mut decoder = decoder(api, 3, 64);
        let mut input = body.as_slice();
        loop {
            match decoder.advance(&mut input, false).unwrap() {
                DecodeStep::NeedInput => {
                    assert!(input.is_empty());
                    break;
                }
                DecodeStep::Finished => panic!("EOF not provided"),
                _ => {}
            }
        }
        assert!(matches!(
            decoder.advance(&mut input, true).unwrap(),
            DecodeStep::Finished
        ));
        assert!(matches!(
            decoder.advance(&mut input, true).unwrap(),
            DecodeStep::Finished
        ));
        assert!(decoder.advance(&mut b"{}".as_slice(), true).is_err());
        assert!(matches!(
            decoder.advance(&mut input, true),
            Err(CodecError::Contract(_))
        ));
        let mut decoder = self::decoder(api, 3, 64);
        let mut bad = b"not json".as_slice();
        assert!(decoder.advance(&mut bad, false).is_err());
        assert!(matches!(
            decoder.advance(&mut bad, false),
            Err(CodecError::Contract(_))
        ));
    }
}

#[test]
fn encoder_finish_requires_a_real_need_event_and_does_not_repeat() {
    for api in APIS {
        let mut early = encoder(api, "target", 3);
        assert!(matches!(early.finish(), Err(CodecError::Contract(_))));
        let mut early = encoder(api, "target", 3);
        // Draining the prefix with Wrote does not grant event readiness until
        // the caller actually observes NeedEvent.
        assert!(matches!(
            early.advance(&mut [0; 4096]).unwrap(),
            EncodeStep::Wrote(_)
        ));
        assert!(matches!(
            early.accept(RequestEvent::MessageStarted { role: Role::User }),
            Err(CodecError::Contract(_))
        ));
        let events = decode(api, [fixture(api).as_slice()], 3, 64).unwrap();
        let mut encoder = encoder(api, "target", 3);
        let mut output = Vec::new();
        drain(&mut *encoder, &mut output, 1).unwrap();
        for event in events {
            encoder.accept(event).unwrap();
            drain(&mut *encoder, &mut output, 1).unwrap();
        }
        encoder.finish().unwrap();
        drain(&mut *encoder, &mut output, 1).unwrap();
        assert!(encoder.finish().is_err());
        assert!(encoder.accept(RequestEvent::TextEnded).is_err());
    }
}
