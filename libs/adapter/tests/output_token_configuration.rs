use adapter::{
    codec::{DecodeStep, EncodeStep, TranslateStep},
    t2t::{
        Text2TextRequestTranslator, Text2TextTranslatorConfiguration,
        anthropic::request::{AnthropicRequestDecoder, AnthropicRequestEncoder},
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
use serde_json::Value;

#[derive(Clone, Copy, Debug)]
enum Api {
    Chat,
    Responses,
    Anthropic,
}

const APIS: [Api; 3] = [Api::Chat, Api::Responses, Api::Anthropic];

impl Api {
    fn token_field(self) -> &'static str {
        match self {
            Self::Chat => "max_completion_tokens",
            Self::Responses => "max_output_tokens",
            Self::Anthropic => "max_tokens",
        }
    }

    fn body(self, fields: &str, before_messages: bool) -> String {
        let messages = match self {
            Self::Responses => r#""input":[{"role":"user","content":"hello"}]"#,
            _ => r#""messages":[{"role":"user","content":"hello"}]"#,
        };
        if fields.is_empty() {
            format!(r#"{{"model":"source",{messages}}}"#)
        } else if before_messages {
            format!(r#"{{"model":"source",{fields},{messages}}}"#)
        } else {
            format!(r#"{{"model":"source",{messages},{fields}}}"#)
        }
    }

    fn decoder(self) -> Box<dyn RequestDecoder> {
        match self {
            Self::Chat => Box::new(OpenAICompletionRequestDecoder::new()),
            Self::Responses => Box::new(OpenAIResponseRequestDecoder::new()),
            Self::Anthropic => Box::new(AnthropicRequestDecoder::new()),
        }
    }

    fn encoder(self, configuration: Text2TextTranslatorConfiguration) -> Box<dyn RequestEncoder> {
        self.try_encoder(configuration).unwrap()
    }

    fn try_encoder(
        self,
        configuration: Text2TextTranslatorConfiguration,
    ) -> Result<Box<dyn RequestEncoder>, adapter::error::CodecError> {
        Ok(match self {
            Self::Chat => Box::new(OpenAICompletionRequestEncoder::new(
                "target",
                configuration,
            )?),
            Self::Responses => {
                Box::new(OpenAIResponseRequestEncoder::new("target", configuration)?)
            }
            Self::Anthropic => Box::new(AnthropicRequestEncoder::new("target", configuration)?),
        })
    }
}

async fn translate(
    source: Api,
    target: Api,
    body: &str,
    configuration: Text2TextTranslatorConfiguration,
) -> Value {
    let mut writer = Cursor::new(Vec::new());
    let mut translator = Text2TextRequestTranslator::new(1, 1);
    translator
        .set_source_stream(Cursor::new(body.as_bytes()))
        .unwrap()
        .set_target_stream(&mut writer)
        .unwrap()
        .set_decoder(source.decoder())
        .unwrap()
        .set_encoder(target.encoder(configuration))
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
    let output: Value = serde_json::from_slice(writer.get_ref()).unwrap();
    let text_field = match target {
        Api::Responses => "input",
        _ => "messages",
    };
    assert_eq!(output[text_field][0]["content"], "hello");
    assert_eq!(output["model"], "target");
    for field in ["max_tokens", "max_completion_tokens", "max_output_tokens"] {
        if field != target.token_field() {
            assert!(output.get(field).is_none(), "wrong token field {field}");
        }
    }
    output
}

#[tokio::test]
async fn all_nine_protocol_pairs_preserve_explicit_limits_at_one_byte_boundaries() {
    for source in APIS {
        for target in APIS {
            for limit in [1, 123, 32768, u32::MAX] {
                for before in [true, false] {
                    let body =
                        source.body(&format!(r#""{}":{limit}"#, source.token_field()), before);
                    let output = translate(
                        source,
                        target,
                        &body,
                        Text2TextTranslatorConfiguration {
                            max_output_tokens: 456,
                        },
                    )
                    .await;
                    assert_eq!(
                        output[target.token_field()],
                        limit,
                        "{source:?} to {target:?}"
                    );
                }
            }
        }
    }
}

#[tokio::test]
async fn missing_openai_limits_use_32k_default_or_custom_fallback_for_every_target() {
    assert_eq!(
        Text2TextTranslatorConfiguration::default().max_output_tokens,
        32 * 1024
    );
    for source in [Api::Chat, Api::Responses] {
        for target in APIS {
            for configuration in [
                Text2TextTranslatorConfiguration::default(),
                Text2TextTranslatorConfiguration {
                    max_output_tokens: 789,
                },
            ] {
                let output =
                    translate(source, target, &source.body("", false), configuration).await;
                assert_eq!(
                    output[target.token_field()],
                    configuration.max_output_tokens
                );
            }
        }
    }
}

#[tokio::test]
async fn legacy_chat_limit_maps_to_canonical_field_for_every_target() {
    for target in APIS {
        let output = translate(
            Api::Chat,
            target,
            &Api::Chat.body(r#""max_tokens":77"#, false),
            Default::default(),
        )
        .await;
        assert_eq!(output[target.token_field()], 77);
    }
}

fn decode(body: &str, api: Api) -> Result<Vec<RequestEvent>, adapter::error::CodecError> {
    let mut decoder = api.decoder();
    let mut scratch = RequestScratch::new(1, 0)?;
    let mut events = Vec::new();
    for mut input in body.as_bytes().chunks(1) {
        loop {
            match decoder.advance(&mut input, false, &mut scratch)? {
                DecodeStep::Event(event) => events.push(event),
                DecodeStep::Progress => {}
                DecodeStep::NeedInput => {
                    assert!(input.is_empty());
                    break;
                }
                DecodeStep::Finished => panic!("EOF required"),
            }
        }
    }
    loop {
        match decoder.advance(&mut &b""[..], true, &mut scratch)? {
            DecodeStep::Event(event) => events.push(event),
            DecodeStep::Finished => return Ok(events),
            _ => panic!("must finish or fail at EOF"),
        }
    }
}

#[test]
fn configuration_events_preserve_source_position_and_are_not_invented_for_defaults() {
    for api in APIS {
        for before in [true, false] {
            let body = api.body(&format!(r#""{}":42"#, api.token_field()), before);
            let events = decode(&body, api).unwrap();
            let configurations: Vec<_> = events
                .iter()
                .enumerate()
                .filter(|(_, event)| matches!(event, RequestEvent::Configuration(_)))
                .collect();
            assert_eq!(configurations.len(), 1);
            let (index, event) = configurations[0];
            assert_eq!(
                *event,
                RequestEvent::Configuration(Text2TextTranslatorConfiguration {
                    max_output_tokens: 42
                })
            );
            assert_eq!(index, if before { 0 } else { events.len() - 1 });
        }
    }
    for api in [Api::Chat, Api::Responses] {
        assert!(
            decode(&api.body("", false), api)
                .unwrap()
                .iter()
                .all(|event| !matches!(event, RequestEvent::Configuration(_)))
        );
    }
    assert!(decode(&Api::Anthropic.body("", false), Api::Anthropic).is_err());
}

#[test]
fn rejects_bad_limits_duplicate_fields_and_conflicting_chat_aliases() {
    for api in APIS {
        assert!(
            api.try_encoder(Text2TextTranslatorConfiguration {
                max_output_tokens: 0
            })
            .is_err()
        );
        for value in [
            "0",
            "-1",
            "1.5",
            "1e2",
            "null",
            "true",
            "\"1\"",
            "[]",
            "{}",
            "01",
            "4294967296",
            "99999999999999999999999999999999",
        ] {
            let body = api.body(&format!(r#""{}":{value}"#, api.token_field()), false);
            assert!(decode(&body, api).is_err(), "accepted {body}");
        }
        let field = api.token_field();
        let body = api.body(&format!(r#""{field}":5,"{field}":5"#), false);
        assert!(decode(&body, api).is_err());
    }
    for fields in [
        r#""max_tokens":5,"max_completion_tokens":5"#,
        r#""max_completion_tokens":5,"max_tokens":9"#,
    ] {
        assert!(decode(&Api::Chat.body(fields, true), Api::Chat).is_err());
    }
}

fn need_event(encoder: &mut dyn RequestEncoder) {
    loop {
        match encoder.advance(&mut &b""[..], &mut [0]).unwrap() {
            EncodeStep::Wrote(_) => {}
            EncodeStep::NeedEvent => break,
            EncodeStep::Finished => panic!("not finalized"),
        }
    }
}

#[test]
fn encoder_rejects_duplicate_zero_and_mid_message_configuration() {
    let event = RequestEvent::Configuration(Text2TextTranslatorConfiguration {
        max_output_tokens: 5,
    });
    for api in APIS {
        let mut encoder = api.encoder(Default::default());
        need_event(&mut *encoder);
        encoder.accept(event.clone()).unwrap();
        need_event(&mut *encoder);
        assert!(encoder.accept(event.clone()).is_err());

        let mut encoder = api.encoder(Default::default());
        need_event(&mut *encoder);
        assert!(
            encoder
                .accept(RequestEvent::Configuration(
                    Text2TextTranslatorConfiguration {
                        max_output_tokens: 0
                    }
                ))
                .is_err()
        );

        let mut encoder = api.encoder(Default::default());
        need_event(&mut *encoder);
        encoder
            .accept(RequestEvent::MessageStarted { role: Role::User })
            .unwrap();
        need_event(&mut *encoder);
        assert!(encoder.accept(event.clone()).is_err());
    }
}
