use codec::decoder::{Decoder as RequestDecoder, DecoderStatus as RequestDecoderStatus};
use codec::llm::text::component::{Content, Message, Role, TextRequestComponent};
use codec::llm::text::copenai::request::CompatibleOpenAIRequestDecoder;

#[test]
fn emits_components_across_arbitrary_chunks() {
    let body = br#"{"model":"provider/model","messages":[{"content":"hello\n\uD83D\uDE00","role":"user"},{"role":"assistant","content":"yes"}],"stream":true}"#;
    for size in 1..=body.len() {
        let mut decoder = CompatibleOpenAIRequestDecoder::new(codec::json::JsonDecoder::new());
        let mut components = Vec::new();
        for chunk in body.chunks(size) {
            let mut input = chunk;
            loop {
                match decoder.next(&mut input, false).unwrap() {
                    RequestDecoderStatus::Component(component) => components.push(component),
                    RequestDecoderStatus::NeedInput => {
                        assert!(input.is_empty());
                        break;
                    }
                    RequestDecoderStatus::Finished => panic!("unexpected finish"),
                }
            }
        }
        assert_eq!(
            components,
            vec![
                TextRequestComponent::Model("provider/model".into()),
                TextRequestComponent::Message(Message {
                    role: Role::User,
                    content: Some(Content::Text("hello\n😀".into())),
                    tool_calls: None,
                    tool_call_id: None,
                    reasoning: None,
                    reasoning_content: None,
                    reasoning_details: None
                }),
                TextRequestComponent::Message(Message {
                    role: Role::Assistant,
                    content: Some(Content::Text("yes".into())),
                    tool_calls: None,
                    tool_call_id: None,
                    reasoning: None,
                    reasoning_content: None,
                    reasoning_details: None
                }),
                TextRequestComponent::Stream(true),
            ]
        );
        let mut input = &b""[..];
        assert_eq!(
            decoder.next(&mut input, true).unwrap(),
            RequestDecoderStatus::Finished
        );
        decoder.finish().unwrap();
    }
}

#[test]
fn returns_early_and_rejects_invalid_or_incomplete_requests() {
    let mut decoder = CompatibleOpenAIRequestDecoder::new(codec::json::JsonDecoder::new());
    let mut input = &br#"{"model":"x","messages":[]}"#[..];
    assert_eq!(
        decoder.next(&mut input, false).unwrap(),
        RequestDecoderStatus::Component(TextRequestComponent::Model("x".into()))
    );
    assert_eq!(input, br#","messages":[]}"#);
    assert!(decoder.next(&mut input, false).is_err());

    for body in [
        &br#"{"model":"x","messages":[{"role":"unknown","content":"x"}]}"#[..],
        &br#"{"model":"x","messages":[{"role":"user","content":null}]}"#[..],
        &br#"{"model":"x","messages":[{"role":"user"}]}"#[..],
        &br#"{"model":"x","model":"y"}"#[..],
        &br#"{"model":"x","messages":[{"role":"user","content":"x",}]}"#[..],
        &br#"{"model":"x","messages":[{"role":"user","content":"x"}]} {}"#[..],
    ] {
        let mut decoder = CompatibleOpenAIRequestDecoder::new(codec::json::JsonDecoder::new());
        let mut input = body;
        loop {
            match decoder.next(&mut input, false) {
                Ok(RequestDecoderStatus::Component(_)) => {}
                Err(_) => break,
                other => panic!("unexpected result: {other:?}"),
            }
        }
    }
    let mut decoder = CompatibleOpenAIRequestDecoder::new(codec::json::JsonDecoder::new());
    let mut input = &br#"{"model":"x""#[..];
    assert!(matches!(
        decoder.next(&mut input, false),
        Ok(RequestDecoderStatus::Component(_))
    ));
    assert_eq!(
        decoder.next(&mut input, false).unwrap(),
        RequestDecoderStatus::NeedInput
    );
    assert!(decoder.finish().is_err());
}
