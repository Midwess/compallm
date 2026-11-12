use adapter::{
    codec::{DecodeStep, EncodeStep, TranslateStep},
    error::{CodecError, TranslationError},
    t2t::{
        Text2TextRequestTranslator,
        anthropic::request::{AnthropicRequestDecoder, AnthropicRequestEncoder},
        copenai::request::{CompatibleOpenAIRequestDecoder, CompatibleOpenAIRequestEncoder},
        openai::request::{
            OpenAICompletionRequestDecoder, OpenAICompletionRequestEncoder,
            OpenAIResponseRequestDecoder, OpenAIResponseRequestEncoder,
        },
        request::{
            RequestScratch,
            decoder::RequestDecoder,
            encoder::RequestEncoder,
            event::{RequestEvent, Role},
        },
    },
};
use futures_util::io::Cursor;
use serde_json::{Value, json};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Api {
    Chat,
    Responses,
    Anthropic,
    Compatible,
}
const ALL: [Api; 4] = [Api::Chat, Api::Responses, Api::Anthropic, Api::Compatible];
impl Api {
    fn chat(self) -> bool {
        matches!(self, Self::Chat | Self::Compatible)
    }
    fn decoder(self) -> Box<dyn RequestDecoder> {
        match self {
            Self::Chat => Box::new(OpenAICompletionRequestDecoder::new()),
            Self::Responses => Box::new(OpenAIResponseRequestDecoder::new()),
            Self::Anthropic => Box::new(AnthropicRequestDecoder::new()),
            Self::Compatible => Box::new(CompatibleOpenAIRequestDecoder::new()),
        }
    }
    fn encoder(self) -> Box<dyn RequestEncoder> {
        let configuration = Default::default();
        match self {
            Self::Chat => {
                Box::new(OpenAICompletionRequestEncoder::new("target", configuration).unwrap())
            }
            Self::Responses => {
                Box::new(OpenAIResponseRequestEncoder::new("target", configuration).unwrap())
            }
            Self::Anthropic => {
                Box::new(AnthropicRequestEncoder::new("target", configuration).unwrap())
            }
            Self::Compatible => {
                Box::new(CompatibleOpenAIRequestEncoder::new("target", configuration).unwrap())
            }
        }
    }
    fn body(self, settings: &str, before: bool) -> String {
        let (body, tokens) = match self {
            Self::Responses => (
                r#""input":[{"role":"user","content":"hi"}]"#,
                r#""max_output_tokens":73"#,
            ),
            Self::Anthropic => (
                r#""messages":[{"role":"user","content":"hi"}]"#,
                r#""max_tokens":73"#,
            ),
            _ => (
                r#""messages":[{"role":"user","content":"hi"}]"#,
                r#""max_completion_tokens":73"#,
            ),
        };
        let sampling = if self == Self::Anthropic {
            ""
        } else {
            r#""top_p":0.85"#
        };
        let members = if before {
            [settings, sampling, tokens, body]
        } else {
            [body, tokens, sampling, settings]
        };
        format!(
            r#"{{"model":"source",{}}}"#,
            members
                .into_iter()
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
    fn token(self) -> &'static str {
        match self {
            Self::Responses => "max_output_tokens",
            Self::Anthropic => "max_tokens",
            _ => "max_completion_tokens",
        }
    }
}

async fn translate(source: Api, target: Api, body: &str) -> Result<Value, TranslationError> {
    let mut output = Cursor::new(Vec::new());
    let mut translator = Text2TextRequestTranslator::new(1, 1);
    translator
        .set_source_stream(Cursor::new(body.as_bytes()))?
        .set_target_stream(&mut output)?
        .set_decoder(source.decoder())?
        .set_encoder(target.encoder())?;
    loop {
        match translator.translate()? {
            TranslateStep::NeedInput => {
                translator.next().await?;
            }
            TranslateStep::OutputReady(_) => translator.flush().await?,
            TranslateStep::Progress => tokio::task::yield_now().await,
            TranslateStep::Finished => {
                translator.flush().await?;
                break;
            }
        }
    }
    drop(translator);
    let output: Value = serde_json::from_slice(output.get_ref()).unwrap();
    assert_eq!(output["model"], "target");
    assert_eq!(output[target.token()], 73);
    let input: Value = serde_json::from_str(body).unwrap();
    let expected = if target == Api::Anthropic {
        None
    } else {
        input.get("top_p").and_then(Value::as_f64)
    };
    assert_eq!(output.get("top_p").and_then(Value::as_f64), expected);
    if expected.is_none() {
        assert!(output.get("top_p").is_none());
    }
    Ok(output)
}
fn targets(source: Api) -> Vec<Api> {
    if source == Api::Compatible {
        vec![Api::Compatible]
    } else {
        ALL.to_vec()
    }
}

#[tokio::test]
async fn stream_booleans_before_or_after_messages_map_across_every_supported_pair() {
    for source in ALL {
        for target in targets(source) {
            for value in [true, false] {
                for before in [true, false] {
                    for top_p in ["0", "0.85", "1", "9e-1", "5e-324", "null"] {
                        let body = source
                            .body(&format!(r#""stream":{value}"#), before)
                            .replace(r#""top_p":0.85"#, &format!(r#""top_p":{top_p}"#));
                        let output = translate(source, target, &body).await.unwrap();
                        assert_eq!(output["stream"], value, "{source:?} to {target:?}");
                        assert!(output.get("stream_options").is_none());
                    }
                }
            }
        }
    }
}

#[tokio::test]
async fn chat_usage_options_preserve_bools_and_omit_the_chat_only_field_on_native_targets() {
    for source in [Api::Chat, Api::Compatible] {
        for target in targets(source) {
            for value in [true, false] {
                for before in [true, false] {
                    for settings in [
                        format!(r#""stream":true,"stream_options":{{"include_usage":{value}}}"#),
                        format!(r#""stream_options":{{"include_usage":{value}}},"stream":true"#),
                    ] {
                        let output = translate(source, target, &source.body(&settings, before))
                            .await
                            .unwrap();
                        assert_eq!(output["stream"], true);
                        if target.chat() {
                            assert_eq!(output["stream_options"], json!({"include_usage":value}));
                        } else {
                            assert!(output.get("stream_options").is_none());
                        }
                    }
                }
            }
        }
    }
}

#[tokio::test]
async fn missing_null_and_empty_options_normalize_without_inventing_usage_preferences() {
    for source in ALL {
        for target in targets(source) {
            let output = translate(source, target, &source.body("", false))
                .await
                .unwrap();
            assert_eq!(output["stream"], false);
            assert!(output.get("stream_options").is_none());
            if !source.chat() {
                continue;
            }
            for (settings, enabled) in [
                (r#""stream_options":null"#, false),
                (r#""stream":false,"stream_options":null"#, false),
                (r#""stream":true,"stream_options":null"#, true),
                (r#""stream_options":{},"stream":true"#, true),
            ] {
                let output = translate(source, target, &source.body(settings, false))
                    .await
                    .unwrap();
                assert_eq!(output["stream"], enabled);
                assert!(output.get("stream_options").is_none());
            }
        }
    }
}

#[tokio::test]
async fn malformed_duplicate_and_conflicting_stream_fields_are_rejected() {
    for source in ALL {
        for settings in [
            r#""stream":null"#,
            r#""stream":0"#,
            r#""stream":"true"#,
            r#""stream":tru"#,
            r#""stream":true,"stream":false"#,
            r#""stream":false,"str\u0065am":false"#,
            r#""stream_options":{}"#,
            r#""stream":false,"stream_options":{}"#,
            r#""stream":false,"stream_options":{"include_usage":false}"#,
            r#""stream":true,"stream_options":{"include_usage":null}"#,
            r#""stream":true,"stream_options":{"include_usage":"true"}"#,
            r#""stream":true,"stream_options":{"include_usage":true,"include_usage":false}"#,
            r#""stream":true,"stream_options":{"include_obfuscation":false}"#,
            r#""stream":true,"stream_options":{},"stream_options":null"#,
            r#""stream":true,"stream_options":true"#,
            r#""stream":true,"stream_options":{"include_usage":true,}"#,
        ] {
            assert!(
                translate(source, source, &source.body(settings, false))
                    .await
                    .is_err(),
                "accepted {source:?}: {settings}"
            );
        }
        if !source.chat() {
            assert!(
                translate(
                    source,
                    source,
                    &source.body(r#""stream":true,"stream_options":null"#, true)
                )
                .await
                .is_err()
            );
        }
    }
}

fn native_events(api: Api, body: &str) -> Result<Vec<RequestEvent>, CodecError> {
    let mut decoder = api.decoder();
    let mut scratch = RequestScratch::new(1, 0)?;
    let mut input = body.as_bytes();
    let mut events = Vec::new();
    loop {
        match decoder.advance(&mut input, true, &mut scratch)? {
            DecodeStep::Event(event) => events.push(event),
            DecodeStep::Finished => return Ok(events),
            _ => {}
        }
    }
}

#[test]
fn native_streaming_events_resolve_once_at_root_completion_without_resetting_tokens() {
    for source in [Api::Chat, Api::Responses, Api::Anthropic] {
        for before in [true, false] {
            let body = source.body(r#""stream":true"#, before);
            let events = native_events(source, &body).unwrap();
            assert_eq!(
                events
                    .iter()
                    .filter(|e| matches!(e, RequestEvent::Streaming { .. }))
                    .count(),
                1
            );
            assert_eq!(
                events.last(),
                Some(&RequestEvent::Streaming {
                    stream: true,
                    include_usage: None
                })
            );
            assert_eq!(
                events
                    .iter()
                    .filter(|e| matches!(e, RequestEvent::Configuration(_)))
                    .count(),
                1
            );
            let events = native_events(source, &source.body("", before)).unwrap();
            assert!(
                !events
                    .iter()
                    .any(|e| matches!(e, RequestEvent::Streaming { .. }))
            );
        }
    }
}

fn drain(encoder: &mut dyn RequestEncoder, output: &mut Vec<u8>) -> Result<bool, CodecError> {
    loop {
        let mut byte = [0];
        match encoder.advance(&mut &b""[..], &mut byte)? {
            EncodeStep::Wrote(len) => {
                assert_eq!(len, 1);
                output.push(byte[0]);
            }
            EncodeStep::NeedEvent => return Ok(false),
            EncodeStep::Finished => return Ok(true),
        }
    }
}

#[test]
fn direct_encoders_guard_stream_settings_and_keep_them_independent_of_token_configuration() {
    for target in ALL {
        for include_usage in [Some(true), Some(false)] {
            let mut encoder = target.encoder();
            let mut output = Vec::new();
            drain(&mut *encoder, &mut output).unwrap();
            let result = encoder.accept(RequestEvent::Streaming {
                stream: false,
                include_usage,
            });
            // Compatible encoding selects its legacy-event mode on first accept.
            assert!(result.is_err() || drain(&mut *encoder, &mut output).is_err());
        }
        let streaming = RequestEvent::Streaming {
            stream: true,
            include_usage: Some(true),
        };
        for first in [true, false] {
            let mut encoder = target.encoder();
            let mut output = Vec::new();
            drain(&mut *encoder, &mut output).unwrap();
            let settings = [
                streaming.clone(),
                RequestEvent::Configuration(adapter::t2t::Text2TextTranslatorConfiguration {
                    max_output_tokens: 19,
                }),
            ];
            for event in if first {
                settings
            } else {
                [settings[1].clone(), settings[0].clone()]
            } {
                encoder.accept(event).unwrap();
                drain(&mut *encoder, &mut output).unwrap();
            }
            encoder.accept(RequestEvent::TopP(0.8)).unwrap();
            drain(&mut *encoder, &mut output).unwrap();
            for event in [
                RequestEvent::MessageStarted { role: Role::User },
                RequestEvent::TextStarted,
                RequestEvent::TextEnded,
                RequestEvent::MessageEnded,
            ] {
                encoder.accept(event).unwrap();
                drain(&mut *encoder, &mut output).unwrap();
            }
            encoder.finish().unwrap();
            assert!(drain(&mut *encoder, &mut output).unwrap());
            let output: Value = serde_json::from_slice(&output).unwrap();
            assert_eq!(output["stream"], true);
            assert_eq!(output[target.token()], 19);
            if target == Api::Anthropic {
                assert!(output.get("top_p").is_none());
            } else {
                assert_eq!(output["top_p"].as_f64(), Some(0.8));
            }
        }
        let mut encoder = target.encoder();
        let mut output = Vec::new();
        drain(&mut *encoder, &mut output).unwrap();
        encoder.accept(streaming.clone()).unwrap();
        drain(&mut *encoder, &mut output).unwrap();
        assert!(encoder.accept(streaming.clone()).is_err());
        let mut encoder = target.encoder();
        let mut output = Vec::new();
        drain(&mut *encoder, &mut output).unwrap();
        encoder
            .accept(RequestEvent::MessageStarted { role: Role::User })
            .unwrap();
        drain(&mut *encoder, &mut output).unwrap();
        assert!(encoder.accept(streaming).is_err());
    }
}

#[tokio::test]
async fn stop_settings_map_between_chat_and_anthropic_without_changing_other_settings() {
    let sequences = json!(["<END>", "\nUser:", "quote\" slash\\ null\0", "😀"]);
    for source in [Api::Chat, Api::Anthropic, Api::Compatible] {
        for target in targets(source)
            .into_iter()
            .filter(|target| *target != Api::Responses)
        {
            for before in [true, false] {
                let key = if source == Api::Anthropic {
                    "stop_sequences"
                } else {
                    "stop"
                };
                let settings = format!(r#""{key}":{sequences},"stream":true"#);
                let output = translate(source, target, &source.body(&settings, before))
                    .await
                    .unwrap();
                let target_key = if target == Api::Anthropic {
                    "stop_sequences"
                } else {
                    "stop"
                };
                assert_eq!(output[target_key], sequences, "{source:?} to {target:?}");
                assert!(
                    output
                        .get(if target_key == "stop" {
                            "stop_sequences"
                        } else {
                            "stop"
                        })
                        .is_none()
                );
                assert_eq!(output["stream"], true);
            }
        }
    }
    for source in [Api::Chat, Api::Compatible] {
        for target in targets(source)
            .into_iter()
            .filter(|target| *target != Api::Responses)
        {
            let output = translate(
                source,
                target,
                &source.body(r#""stop":"\uD83D\uDE00\nUser:""#, false),
            )
            .await
            .unwrap();
            let value = if source == Api::Compatible {
                json!("😀\nUser:")
            } else {
                json!(["😀\nUser:"])
            };
            assert_eq!(
                output[if target == Api::Anthropic {
                    "stop_sequences"
                } else {
                    "stop"
                }],
                value
            );
        }
    }
}

#[tokio::test]
async fn responses_rejects_active_stops_but_disabled_stops_leave_defaults_unchanged() {
    for source in [Api::Chat, Api::Anthropic] {
        let key = if source == Api::Anthropic {
            "stop_sequences"
        } else {
            "stop"
        };
        let error = translate(
            source,
            Api::Responses,
            &source.body(&format!(r#""{key}":["END"]"#), false),
        )
        .await
        .unwrap_err();
        assert!(matches!(
            error,
            TranslationError::Stream(adapter::error::StreamError::Codec(CodecError::Unsupported(
                _
            )))
        ));
        let output = translate(
            source,
            Api::Responses,
            &source.body(&format!(r#""{key}":[]"#), true),
        )
        .await
        .unwrap();
        assert!(output.get("stop").is_none());
        assert!(output.get("stop_sequences").is_none());
    }
    for target in ALL {
        let output = translate(Api::Chat, target, &Api::Chat.body(r#""stop":null"#, false))
            .await
            .unwrap();
        assert!(output.get("stop").is_none());
        assert!(output.get("stop_sequences").is_none());
    }
    let sequences = vec!["END"; 5];
    let body = Api::Anthropic.body(&format!(r#""stop_sequences":{}"#, json!(sequences)), false);
    let output = translate(Api::Anthropic, Api::Anthropic, &body)
        .await
        .unwrap();
    assert_eq!(output["stop_sequences"], json!(sequences));
    assert!(matches!(
        translate(Api::Anthropic, Api::Chat, &body).await,
        Err(TranslationError::Stream(
            adapter::error::StreamError::Codec(CodecError::LimitExceeded(_))
        ))
    ));
}

#[tokio::test]
async fn schema_output_formats_map_across_supported_pairs_before_and_after_messages() {
    let schema = json!({"type":"object","properties":{"answer":{"type":"string","description":"UTF-8 😀","minLength":0,"maxLength":1024},"score":{"type":"number","minimum":-10,"maximum":100,"multipleOf":0.5}},"required":["answer"],"additionalProperties":false});
    for source in ALL {
        for target in targets(source) {
            for before in [true, false] {
                let format = match source {
                    Api::Chat | Api::Compatible => {
                        json!({"response_format":{"type":"json_schema","json_schema":{"name":"answer","description":"An answer","strict":true,"schema":schema}}})
                    }
                    Api::Responses => {
                        json!({"text":{"format":{"type":"json_schema","name":"answer","description":"An answer","strict":true,"schema":schema}}})
                    }
                    Api::Anthropic => {
                        json!({"output_config":{"format":{"type":"json_schema","schema":schema}}})
                    }
                };
                let members = format
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(|(key, value)| format!("\"{key}\":{value}"))
                    .collect::<Vec<_>>()
                    .join(",");
                let mut body = source.body(&members, before);
                if source == Api::Anthropic {
                    body = body.replacen(r#""messages":"#, r#""system":"guide","messages":"#, 1);
                }
                let output = translate(source, target, &body).await.unwrap();
                let format = match target {
                    Api::Chat | Api::Compatible => &output["response_format"]["json_schema"],
                    Api::Responses => &output["text"]["format"],
                    Api::Anthropic => &output["output_config"]["format"],
                };
                assert_eq!(format["schema"], schema, "{source:?} to {target:?}");
                if target != Api::Anthropic {
                    assert_eq!(format["strict"], true);
                    assert_eq!(
                        format["name"],
                        if source == Api::Anthropic {
                            "response"
                        } else {
                            "answer"
                        }
                    );
                }
            }
        }
    }
    let settings =
        format!(r#""output_config":{{"format":{{"type":"json_schema","schema":{schema}}}}}"#);
    let body = Api::Anthropic.body(&settings, true).replacen(
        r#""model":"source","#,
        r#""model":"source","system":"guide","#,
        1,
    );
    for target in ALL {
        let result = translate(Api::Anthropic, target, &body).await;
        if target == Api::Anthropic {
            assert!(result.is_ok());
        } else {
            assert!(matches!(
                result,
                Err(TranslationError::Stream(
                    adapter::error::StreamError::Codec(CodecError::Unsupported(_))
                ))
            ));
        }
    }
}

#[tokio::test]
async fn json_object_mode_is_preserved_and_not_downgraded_for_anthropic() {
    for source in [Api::Chat, Api::Responses, Api::Compatible] {
        for target in targets(source) {
            for mode in ["text", "json_object"] {
                let settings = if source == Api::Responses {
                    format!(r#""text":{{"format":{{"type":"{mode}"}}}}"#)
                } else {
                    format!(r#""response_format":{{"type":"{mode}"}}"#)
                };
                let result = translate(source, target, &source.body(&settings, false)).await;
                if target == Api::Anthropic && mode == "json_object" {
                    assert!(matches!(
                        result,
                        Err(TranslationError::Stream(
                            adapter::error::StreamError::Codec(CodecError::Unsupported(_))
                        ))
                    ));
                } else {
                    let output = result.unwrap();
                    if mode == "text" && source != Api::Compatible {
                        assert!(output.get("response_format").is_none());
                        assert!(output.get("text").is_none());
                        assert!(output.get("output_config").is_none());
                        continue;
                    }
                    assert_eq!(
                        if target == Api::Responses {
                            &output["text"]["format"]["type"]
                        } else {
                            &output["response_format"]["type"]
                        },
                        mode
                    );
                }
            }
        }
    }
}
