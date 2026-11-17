use codec::decoder::{Decoder, DecoderStatus};
use codec::llm::text::component::{Content, Role, TextRequestComponent};
use codec::llm::text::copenai::request::CompatibleOpenAIRequestDecoder;

#[test]
fn assembles_calls_and_results_across_chunks() {
    let body = br#"{"model":"x","messages":[{"tool_calls":[{"function":{"arguments":"{\"city\":\"Paris\"}","name":"weather"},"type":"function","id":"a"},{"id":"b","type":"function","function":{"name":"clock","arguments":""}}],"role":"assistant","content":null},{"content":"sunny","tool_call_id":"a","role":"tool"},{"role":"assistant","tool_calls":[{"id":"c","type":"function","function":{"name":"clock","arguments":"{}"}}]}]}"#;
    for size in 1..=body.len() {
        let mut decoder = CompatibleOpenAIRequestDecoder::new(codec::json::JsonDecoder::new());
        let mut messages = Vec::new();
        for chunk in body.chunks(size) {
            let mut input = chunk;
            loop {
                match decoder.next(&mut input, false).unwrap() {
                    DecoderStatus::Component(TextRequestComponent::Message(message)) => {
                        messages.push(message)
                    }
                    DecoderStatus::Component(_) => {}
                    DecoderStatus::NeedInput => break,
                    DecoderStatus::Finished => panic!("unexpected finish"),
                }
            }
        }
        let mut input = &b""[..];
        assert_eq!(
            decoder.next(&mut input, true).unwrap(),
            DecoderStatus::Finished
        );
        decoder.finish().unwrap();
        assert_eq!(messages.len(), 3);
        assert_eq!(messages[0].content, Some(Content::Null));
        let calls = messages[0].tool_calls.as_ref().unwrap();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].id, "a");
        assert_eq!(calls[0].function.name, "weather");
        assert_eq!(calls[0].function.arguments, "{\"city\":\"Paris\"}");
        assert_eq!(calls[1].id, "b");
        assert_eq!(calls[1].function.arguments, "");
        assert_eq!(messages[1].role, Role::Tool);
        assert_eq!(messages[1].tool_call_id.as_deref(), Some("a"));
        assert_eq!(messages[1].content, Some(Content::Text("sunny".into())));
        assert_eq!(messages[2].content, None);
        assert_eq!(messages[2].tool_calls.as_ref().unwrap()[0].id, "c");
    }
}
