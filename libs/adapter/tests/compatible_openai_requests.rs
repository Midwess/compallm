use adapter::{
    codec::{DecodeStep, EncodeStep, TranslateStep},
    error::CodecError,
    t2t::{
        Text2TextRequestTranslator, Text2TextTranslatorConfiguration,
        copenai::request::{
            CompatibleOpenAIRequestDecoder as Decoder, CompatibleOpenAIRequestEncoder as Encoder,
        },
        openai::request::{OpenAICompletionRequestDecoder, OpenAICompletionRequestEncoder},
        request::{
            RequestScratch,
            decoder::RequestDecoder,
            encoder::RequestEncoder,
            event::{RequestEvent, RequestField as F, Structure as S},
        },
    },
};
use futures_util::io::Cursor;
use serde_json::{Value, json};

fn fixture() -> Vec<u8> {
    serde_json::to_vec(&json!({
    "model":"source", "stream":false,"max_tokens":123,"parallel_tool_calls":false,"top_p":0.85,"top_k":40,"stop":["END","\nUser:","😀"],
    "response_format":{"type":"json_schema","json_schema":{"name":"answer","strict":true,"schema":{"type":"object","properties":{"answer":{"type":"string"}},"required":["answer"],"additionalProperties":false}}},
    "provider":{"require_parameters":true},
    "tools":[{"type":"function","function":{"name":"weather","description":"Look up weather","strict":true,
        "parameters":{"type":"object","properties":{"city":{"type":"string","description":"A city"}},"required":["city"],"additionalProperties":false}}}],
    "tool_choice":{"type":"function","function":{"name":"weather"}},
    "messages":[{"role":"developer","content":"Use the weather tool."},
        {"role":"user","content":[{"type":"text","text":"Weather in 東京?"}]},
        {"role":"assistant","content":null,"tool_calls":[{"id":"call_1","type":"function","function":{"name":"weather","arguments":"{\"city\":\"東京\"}"}},{"id":"call_2","type":"function","function":{"name":"weather","arguments":"{\"city\":\"Paris\"}"}}],
            "reasoning_details":[{"type":"reasoning.summary","summary":"Compare both cities","format":"openai-responses-v1","id":"rs_1","index":0},
                {"type":"reasoning.text","text":"<returned text>","signature":"<signature>","format":"anthropic-claude-v1","index":1},
                {"type":"reasoning.encrypted","data":"<opaque>\0😀","id":null,"format":"openai-responses-v1","index":2}]},
        {"role":"tool","tool_call_id":"call_1","content":"rain\n12°C"},
        {"role":"tool","tool_call_id":"call_2","content":[{"type":"text","text":"clear"}]},
        {"role":"assistant","reasoning_content":"Compare results.","content":"Tokyo is rainy; Paris is clear."}]
})).unwrap()
}

fn drain(
    encoder: &mut dyn RequestEncoder,
    text: &mut &[u8],
    out: &mut Vec<u8>,
    size: usize,
) -> Result<bool, CodecError> {
    let mut buffer = vec![0xa5; size];
    loop {
        match encoder.advance(text, &mut buffer)? {
            EncodeStep::Wrote(len) => {
                assert!((1..=size.min(4096)).contains(&len));
                out.extend_from_slice(&buffer[..len]);
            }
            EncodeStep::NeedEvent => {
                assert!(text.is_empty());
                return Ok(false);
            }
            EncodeStep::Finished => {
                assert!(text.is_empty());
                return Ok(true);
            }
        }
    }
}
fn transcode(
    chunks: impl IntoIterator<Item = impl AsRef<[u8]>>,
    fragment: usize,
    size: usize,
) -> Result<Value, CodecError> {
    let mut decoder = Decoder::new();
    let mut scratch = RequestScratch::new(fragment, 0)?;
    let mut encoder = Encoder::new(
        "target\"😀",
        Text2TextTranslatorConfiguration {
            max_output_tokens: 777,
        },
    )?;
    let mut output = Vec::new();
    assert!(!drain(&mut encoder, &mut &b""[..], &mut output, size)?);
    for chunk in chunks {
        let mut input = chunk.as_ref();
        loop {
            match decoder.advance(&mut input, false, &mut scratch)? {
                DecodeStep::Event(event) => {
                    let mut text = match event {
                        RequestEvent::TextFragment(len) => {
                            assert!(len <= fragment);
                            &scratch.text()[..len]
                        }
                        _ => &[],
                    };
                    encoder.accept(event)?;
                    assert!(!drain(&mut encoder, &mut text, &mut output, size)?);
                }
                DecodeStep::Progress => {}
                DecodeStep::NeedInput => {
                    assert!(input.is_empty());
                    break;
                }
                DecodeStep::Finished => panic!("unvalidated EOF"),
            }
        }
    }
    loop {
        match decoder.advance(&mut &b""[..], true, &mut scratch)? {
            DecodeStep::Event(event) => {
                let mut text = match event {
                    RequestEvent::TextFragment(len) => &scratch.text()[..len],
                    _ => &[],
                };
                encoder.accept(event)?;
                drain(&mut encoder, &mut text, &mut output, size)?;
            }
            DecodeStep::Finished => break,
            _ => panic!("EOF must finish or fail"),
        }
    }
    encoder.finish()?;
    assert!(drain(&mut encoder, &mut &b""[..], &mut output, size)?);
    Ok(serde_json::from_slice(&output)
        .unwrap_or_else(|error| panic!("{error}: {}", String::from_utf8_lossy(&output))))
}
fn expected(body: &[u8]) -> Value {
    let mut value: Value = serde_json::from_slice(body).unwrap();
    let limit = value
        .as_object_mut()
        .unwrap()
        .remove("max_tokens")
        .or_else(|| value.get("max_completion_tokens").cloned())
        .unwrap_or(json!(777));
    value["max_completion_tokens"] = limit;
    value["stream"] = json!(false);
    value["model"] = json!("target\"😀");
    value
}

#[test]
fn tools_results_and_all_reasoning_forms_round_trip_at_every_split() {
    let body = fixture();
    let wanted = expected(&body);
    for split in 0..=body.len() {
        assert_eq!(
            transcode([&body[..split], &body[split..]], 3, 1).unwrap(),
            wanted,
            "split {split}"
        );
    }
    assert_eq!(transcode(body.chunks(1), 1, 1).unwrap(), wanted);
}

#[test]
fn field_order_text_nulls_and_tool_selection_forms_are_preserved() {
    for choice in [
        json!("none"),
        json!("auto"),
        json!("required"),
        json!({"type":"function","function":{"name":"f"}}),
    ] {
        for reasoning in [
            json!(null),
            json!(""),
            json!("summary? no, ordinary reasoning text\0😀"),
        ] {
            let body=serde_json::to_vec(&json!({"model":"source","tool_choice":choice,"tools":[],"top_k":0,"messages":[
                {"role":"system","content":""},{"role":"assistant","content":"x","reasoning":reasoning}]})).unwrap();
            assert_eq!(transcode(body.chunks(1), 1, 1).unwrap(), expected(&body));
        }
    }
    let body=br#"{"messages":[{"tool_calls":[{"function":{"arguments":"not valid JSON is still a string","name":"f"},"type":"function","id":"unchanged.id"}],"role":"assistant"}],"max_completion_tokens":99,"model":"source"}"#;
    assert_eq!(transcode(body.chunks(1), 2, 1).unwrap(), expected(body));
}

#[test]
fn rejects_malformed_unsupported_and_duplicate_fields() {
    let invalid = [
        "{}",
        r#"{"model":"m","messages":[]}"#,
        r#"{"model":"m","messages":[{"role":"tool","content":"x"}]}"#,
        r#"{"model":"m","messages":[{"role":"tool","tool_call_id":"id","content":null}]}"#,
        r#"{"model":"m","messages":[{"role":"user","content":"x","tool_call_id":"id"}]}"#,
        r#"{"model":"m","messages":[{"role":"user","content":"x","reasoning":"y"}]}"#,
        r#"{"model":"m","messages":[{"role":"assistant","content":null}]}"#,
        r#"{"model":"m","messages":[{"role":"assistant","tool_calls":[]}]}"#,
        r#"{"model":"m","messages":[{"role":"assistant","tool_calls":[{"id":"c","type":"function","function":{"name":"f","arguments":{}}}]}]}"#,
        r#"{"model":"m","messages":[{"role":"assistant","tool_calls":[{"id":"","type":"function","function":{"name":"f","arguments":"{}"}}]}]}"#,
        r#"{"model":"m","messages":[{"role":"assistant","content":"x","reasoning":"a","reasoning_content":"a"}]}"#,
        r#"{"model":"m","messages":[{"role":"assistant","content":"x","reasoning_details":[{"type":"reasoning.text"}]}]}"#,
        r#"{"model":"m","messages":[{"role":"assistant","content":"x","reasoning_details":[{"type":"reasoning.summary","summary":"s","data":"blob"}]}]}"#,
        r#"{"model":"m","messages":[{"role":"user","content":[{"type":"image_url","image_url":"x"}]}]}"#,
        r#"{"model":"m","messages":[{"role":"user","role":"user","content":"x"}]}"#,
        r#"{"model":"m","messages":[{"role":"user","content":"x"}],"stream":"true"}"#,
        r#"{"model":"m","messages":[{"role":"user","content":"x"}],"max_tokens":1,"max_completion_tokens":1}"#,
        r#"{"model":"m","messages":[{"role":"user","content":"x"}],"tools":[{"type":"web_search"}]}"#,
        r#"{"model":"m","messages":[{"role":"user","content":"x"}],"tools":[{"type":"function","function":{"name":"f","parameters":true}}]}"#,
        r#"{"model":"m","messages":[{"role":"user","content":"x"}],"tool_choice":"surprise"}"#,
        r#"{"model":"m","messages":[{"role":"user","content":"x"}],"temperature":1}"#,
    ];
    for body in invalid {
        assert!(
            transcode(body.as_bytes().chunks(1), 1, 1).is_err(),
            "accepted {body}"
        );
    }
    for number in ["0", "-1", "1.0", "1e2", "01", "4294967296", "null"] {
        let body = format!(
            r#"{{"model":"m","messages":[{{"role":"user","content":"x"}}],"max_tokens":{number}}}"#
        );
        assert!(
            transcode(body.as_bytes().chunks(1), 1, 1).is_err(),
            "accepted {body}"
        );
    }
}

#[test]
fn requires_eof_valid_unicode_and_valid_json_punctuation() {
    let body = br#"{"model":"m","messages":[{"role":"user","content":"\uD83D\uDE00"}]}"#;
    for end in 0..body.len() {
        assert!(transcode([&body[..end]], 2, 1).is_err(), "prefix {end}");
    }
    for body in [
        r#"{"model":"m" "messages":[]}"#,
        r#"{"model":"m","messages":[{"role":"user","content":"x",}]}"#,
        r#"{"model":"m","messages":[{"role":"user","content":"x"},]}"#,
        r#"{"model":"m","messages":[{"role":"user","content":"x"}],}"#,
        r#"{"model":"m","messages":[{"role":"user","content":"\uD800"}]}"#,
        r#"{"model":"m","messages":[{"role":"user","content":"x"}]}{}"#,
    ] {
        assert!(
            transcode(body.as_bytes().chunks(1), 2, 1).is_err(),
            "accepted {body}"
        );
    }
    let bad = b"{\"model\":\"m\",\"messages\":[{\"role\":\"user\",\"content\":\"\xff\"}]}";
    assert!(transcode(bad.chunks(1), 1, 1).is_err());
}

#[tokio::test]
async fn translator_integrates_and_legacy_text_can_target_the_compatible_encoder() {
    let body = fixture();
    let mut output = Cursor::new(Vec::new());
    let mut translator = Text2TextRequestTranslator::new(1, 1);
    translator
        .set_source_stream(Cursor::new(&body))
        .unwrap()
        .set_target_stream(&mut output)
        .unwrap()
        .set_decoder(Decoder::default())
        .unwrap()
        .set_encoder(Encoder::new("target\"😀", Default::default()).unwrap())
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
        expected(&body)
    );
    let body = r#"{"model":"m","messages":[{"role":"user","content":"hi😀"}]}"#.as_bytes();
    let mut decoder = OpenAICompletionRequestDecoder::new();
    let mut scratch = RequestScratch::new(1, 0).unwrap();
    let mut input = body;
    let mut encoder = Encoder::new(
        "target\"😀",
        Text2TextTranslatorConfiguration {
            max_output_tokens: 777,
        },
    )
    .unwrap();
    let mut out = Vec::new();
    drain(&mut encoder, &mut &b""[..], &mut out, 1).unwrap();
    loop {
        match decoder.advance(&mut input, true, &mut scratch).unwrap() {
            DecodeStep::Event(event) => {
                let mut text = match event {
                    RequestEvent::TextFragment(len) => &scratch.text()[..len],
                    _ => &[],
                };
                encoder.accept(event).unwrap();
                drain(&mut encoder, &mut text, &mut out, 1).unwrap();
            }
            DecodeStep::Finished => break,
            _ => {}
        }
    }
    encoder.finish().unwrap();
    assert!(drain(&mut encoder, &mut &b""[..], &mut out, 1).unwrap());
    assert_eq!(
        serde_json::from_slice::<Value>(&out).unwrap(),
        expected(body)
    );
}

#[test]
fn encoders_reject_bad_events_and_standard_encoder_cannot_silently_drop_tools() {
    let mut encoder = Encoder::new("m", Default::default()).unwrap();
    assert!(
        encoder
            .accept(RequestEvent::Structure(S::ObjectStarted))
            .is_err()
    );
    let mut encoder = Encoder::new("m", Default::default()).unwrap();
    let mut out = Vec::new();
    drain(&mut encoder, &mut &b""[..], &mut out, 1).unwrap();
    encoder
        .accept(RequestEvent::Structure(S::ObjectStarted))
        .unwrap();
    drain(&mut encoder, &mut &b""[..], &mut out, 1).unwrap();
    assert!(
        encoder
            .accept(RequestEvent::Structure(S::Field(F::ToolCalls)))
            .is_err()
    );
    assert!(encoder.advance(&mut &b""[..], &mut [0]).is_err());
    let mut encoder = OpenAICompletionRequestEncoder::new("m", Default::default()).unwrap();
    drain(&mut encoder, &mut &b""[..], &mut out, 1).unwrap();
    assert!(
        encoder
            .accept(RequestEvent::Structure(S::ObjectStarted))
            .is_err()
    );
}

#[test]
fn schema_numbers_keys_and_nesting_stream_without_value_collection() {
    let key = "schema-key-😀".repeat(1024);
    let mut schema = format!(
        r#"{{"{}":{{"enum":[null,true,false,-12.5e+2,0,1e-30,"\u0000\uD83D\uDE00"]}}}}"#,
        key
    );
    for _ in 0..20 {
        schema = format!(r#"{{"nested":{schema}}}"#);
    }
    let body = format!(
        r#"{{"model":"m","tools":[{{"type":"function","function":{{"name":"f","parameters":{schema}}}}}],"messages":[{{"role":"user","content":"go"}}]}}"#
    );
    assert_eq!(
        transcode(body.as_bytes().chunks(1), 7, 1).unwrap(),
        expected(body.as_bytes())
    );
    for number in ["01", "-01", "1.", "1e", "1e+", "1e--2", ".5", "+1"] {
        let body = format!(
            r#"{{"model":"m","tools":[{{"type":"function","function":{{"name":"f","parameters":{{"x":{number}}}}}}}],"messages":[{{"role":"user","content":"go"}}]}}"#
        );
        assert!(
            transcode(body.as_bytes().chunks(1), 1, 1).is_err(),
            "accepted {number}"
        );
    }
    let schema = format!("{}0{}", "[".repeat(64), "]".repeat(64));
    let body = format!(
        r#"{{"model":"m","tools":[{{"type":"function","function":{{"name":"f","parameters":{{"x":{schema}}}}}}}],"messages":[{{"role":"user","content":"go"}}]}}"#
    );
    assert!(matches!(
        transcode([body.as_bytes()], 3, 1),
        Err(CodecError::LimitExceeded(_))
    ));
}

#[test]
fn large_arguments_and_reasoning_emit_before_the_payload_has_been_read() {
    for field in [
        r#""tool_calls":[{"id":"c","type":"function","function":{"name":"f","arguments":""#,
        r#""reasoning_details":[{"type":"reasoning.encrypted","data":""#,
        r#""reasoning":""#,
    ] {
        let body = format!(
            r#"{{"model":"m","messages":[{{"role":"assistant",{field}{}"#,
            "x".repeat(1024 * 1024)
        );
        let mut input = body.as_bytes();
        let mut decoder = Decoder::new();
        let mut scratch = RequestScratch::new(7, 0).unwrap();
        let mut largest = 0;
        while !input.is_empty() {
            let before = input.len();
            match decoder.advance(&mut input, false, &mut scratch).unwrap() {
                DecodeStep::Event(RequestEvent::TextFragment(len)) => {
                    assert!(len <= 7);
                    if &scratch.text()[..len] == b"xxxxxxx" {
                        largest = len;
                        break;
                    }
                }
                DecodeStep::Progress => assert!(before - input.len() <= 4096),
                _ => {}
            }
        }
        assert_eq!(largest, 7);
        assert!(body.len() - input.len() < 256);
    }
}

#[test]
fn model_limits_number_boundaries_and_terminal_states_remain_explicit() {
    assert!(Encoder::new("", Default::default()).is_err());
    assert!(Encoder::new(&"m".repeat(1025), Default::default()).is_err());
    assert!(
        Encoder::new(
            "m",
            Text2TextTranslatorConfiguration {
                max_output_tokens: 0
            }
        )
        .is_err()
    );
    for len in [1024, 1025] {
        let body = format!(
            r#"{{"model":"{}","messages":[{{"role":"user","content":"x"}}],"max_tokens":4294967295}}"#,
            "m".repeat(len)
        );
        let result = transcode(body.as_bytes().chunks(1), 1, 1);
        if len == 1024 {
            assert_eq!(result.unwrap()["max_completion_tokens"], u32::MAX);
        } else {
            assert!(result.is_err());
        }
    }
    let mut decoder = Decoder::new();
    let mut scratch = RequestScratch::new(1, 0).unwrap();
    assert!(
        decoder
            .advance(&mut b"!".as_slice(), false, &mut scratch)
            .is_err()
    );
    assert!(matches!(
        decoder.advance(&mut b"".as_slice(), true, &mut scratch),
        Err(CodecError::Contract(_))
    ));
    let mut decoder = Decoder::new();
    let mut input =
        b"{\"model\":\"m\",\"messages\":[{\"role\":\"user\",\"content\":\"x\"}]}".as_slice();
    loop {
        if matches!(
            decoder.advance(&mut input, true, &mut scratch).unwrap(),
            DecodeStep::Finished
        ) {
            break;
        }
    }
    assert!(matches!(
        decoder.advance(&mut input, true, &mut scratch).unwrap(),
        DecodeStep::Finished
    ));
    assert!(
        decoder
            .advance(&mut b"x".as_slice(), true, &mut scratch)
            .is_err()
    );
    let mut encoder = Encoder::new("m", Default::default()).unwrap();
    assert!(encoder.advance(&mut b"".as_slice(), &mut []).is_err());
    assert!(matches!(
        encoder.advance(&mut b"".as_slice(), &mut [0]),
        Err(CodecError::Contract(_))
    ));
}

#[test]
fn large_payloads_and_large_output_windows_keep_per_call_work_bounded() {
    let text = "quote\" backslash\\ controls\0\n\t😀".repeat(8192);
    let body=serde_json::to_vec(&json!({"model":"m","messages":[{"role":"assistant","reasoning_details":[
        {"type":"reasoning.text","text":text,"signature":null,"format":"unknown","id":null,"index":u32::MAX}],
        "tool_calls":[{"id":"c","type":"function","function":{"name":"f","arguments":text}}]}],
        "tools":[{"type":"function","function":{"name":"f","strict":null,"parameters":{"description":text}}}]})).unwrap();
    for output_size in [2, 7, 4097, 16384] {
        assert_eq!(
            transcode([body.as_slice()], 65536, output_size).unwrap(),
            expected(&body)
        );
    }
}

#[test]
fn direct_encoder_checks_payload_lengths_utf8_and_completion() {
    let events = [S::ObjectStarted, S::Field(F::Model), S::StringStarted];
    for (announced, bytes) in [
        (2, b"x".as_slice()),
        (1, b"\xff".as_slice()),
        (1, b"\xf0".as_slice()),
    ] {
        let mut encoder = Encoder::new("target", Default::default()).unwrap();
        let mut out = Vec::new();
        drain(&mut encoder, &mut &b""[..], &mut out, 1).unwrap();
        for event in events {
            encoder.accept(RequestEvent::Structure(event)).unwrap();
            drain(&mut encoder, &mut &b""[..], &mut out, 1).unwrap();
        }
        encoder
            .accept(RequestEvent::TextFragment(announced))
            .unwrap();
        let result = drain(&mut encoder, &mut &bytes[..], &mut out, 1);
        if result.is_ok() {
            assert!(
                encoder
                    .accept(RequestEvent::Structure(S::StringEnded))
                    .is_err()
            );
        } else {
            assert!(encoder.advance(&mut &bytes[..], &mut [0]).is_err());
        }
        assert!(encoder.finish().is_err());
    }
    for body in [
        r#"{"model":"m","messages":[{"role":"assistant","reasoning":null}]}"#,
        r#"{"model":"m","messages":[{"role":"assistant","content":null,"reasoning_details":[]}]}"#,
    ] {
        assert!(transcode(body.as_bytes().chunks(1), 1, 1).is_err());
    }
}

#[test]
fn accepted_mutations_remain_valid_json_and_preserve_supported_values() {
    let original=br#"{"model":"m","tools":[{"type":"function","function":{"name":"f","parameters":{"x":[0,true,null]}}}],"messages":[{"role":"assistant","content":"x","reasoning_details":[{"type":"reasoning.summary","summary":"s"}]}]}"#;
    for index in 0..original.len() {
        for replacement in b"{}[],:\"\\01 \n\xff" {
            let mut body = original.to_vec();
            body[index] = *replacement;
            if let Ok(output) = transcode(body.chunks(3), 2, 3) {
                assert_eq!(output, expected(&body), "mutation {index} {replacement}");
            }
        }
    }
}
