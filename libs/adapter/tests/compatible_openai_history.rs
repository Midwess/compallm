use adapter::decoder::{Decoder as RequestDecoder, DecoderStatus as RequestDecoderStatus};
use adapter::t2t::component::{Content, ReasoningDetailContent, Role, TextRequestComponent};
use adapter::t2t::copenai::request::CompatibleOpenAIRequestDecoder;

#[test]
fn assembles_tool_and_reasoning_history_across_chunks() {
    let body = br#"{"model":"x","messages":[{"tool_calls":[{"function":{"arguments":"{\"x\":1}","name":"lookup"},"id":"call1","type":"function"}],"reasoning":"","reasoning_details":[{"summary":"plan","type":"reasoning.summary"},{"type":"reasoning.text","text":"think\n","signature":null,"id":null,"format":"opaque","index":0},{"type":"reasoning.encrypted","data":"secret"}],"content":null,"role":"assistant"},{"tool_call_id":"call1","content":"done","role":"tool"},{"role":"assistant","reasoning_content":"answer"}]}"#;
    for size in 1..=body.len() {
        let mut decoder = CompatibleOpenAIRequestDecoder::new();
        let mut messages = Vec::new();
        for chunk in body.chunks(size) {
            let mut input = chunk;
            loop {
                match decoder.next(&mut input).unwrap() {
                    RequestDecoderStatus::Component(TextRequestComponent::Message(message)) => {
                        messages.push(message)
                    }
                    RequestDecoderStatus::Component(_) => {}
                    RequestDecoderStatus::NeedInput => break,
                    RequestDecoderStatus::Finished => panic!("finished before EOF"),
                }
            }
        }
        decoder.finish().unwrap();
        assert_eq!(messages.len(), 3);
        let assistant = &messages[0];
        assert_eq!(assistant.content, Some(Content::Null));
        assert_eq!(assistant.reasoning.as_deref(), Some(""));
        let call = &assistant.tool_calls.as_ref().unwrap()[0];
        assert_eq!(call.id, "call1");
        assert_eq!(call.function.name, "lookup");
        assert_eq!(call.function.arguments, "{\"x\":1}");
        let details = assistant.reasoning_details.as_ref().unwrap();
        assert_eq!(details.len(), 3);
        assert_eq!(
            details[0].content,
            ReasoningDetailContent::Summary {
                summary: "plan".into()
            }
        );
        assert_eq!(
            details[1].content,
            ReasoningDetailContent::Text {
                text: "think\n".into(),
                signature: None
            }
        );
        assert_eq!(details[1].index, Some(0));
        assert_eq!(
            details[2].content,
            ReasoningDetailContent::Encrypted {
                data: "secret".into()
            }
        );
        assert_eq!(messages[1].role, Role::Tool);
        assert_eq!(messages[1].tool_call_id.as_deref(), Some("call1"));
        assert_eq!(messages[1].content, Some(Content::Text("done".into())));
        assert_eq!(messages[2].reasoning_content.as_deref(), Some("answer"));
    }
}
