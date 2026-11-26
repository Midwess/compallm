use codec::encoder::{Encoder, EncoderStatus};
use codec::json::JsonEncoder;
use codec::llm::text::component::{Content, Message, Role, TextRequestComponent};
use codec::llm::text::copenai::request::CompatibleOpenAIRequestEncoder;

#[test]
fn resumes_messages_and_closes_the_array_before_scalars() {
    for size in [1, 2, 16, 4096] {
        let mut encoder =
            CompatibleOpenAIRequestEncoder::new(JsonEncoder::new(), Default::default());
        let message = |role, text| Message {
            role,
            content: Some(Content::Text(text)),
            tool_calls: None,
            tool_call_id: None,
            reasoning: None,
            reasoning_content: None,
            reasoning_details: None,
        };
        let mut components = [
            TextRequestComponent::Model("model"),
            TextRequestComponent::Message(message(Role::User, "hello\nworld")),
            TextRequestComponent::Message(message(Role::Assistant, "reply")),
            TextRequestComponent::Stream(true),
        ];
        let mut output = vec![0; size];
        let mut encoded = Vec::new();
        for component in &mut components {
            loop {
                let status = encoder.next(Some(component), &mut output).unwrap();
                let (written, complete) = match status {
                    EncoderStatus::NeedOutput { written } => (written, false),
                    EncoderStatus::ComponentFinished { written } => (written, true),
                    EncoderStatus::Finished { .. } => panic!("request not finalized"),
                };
                encoded.extend_from_slice(&output[..written]);
                if complete {
                    break;
                }
            }
        }
        assert_eq!(
            std::str::from_utf8(&encoded).unwrap(),
            r#"{"model":"model","messages":[{"role":"user","content":"hello\nworld"},{"role":"assistant","content":"reply"}],"stream":true"#
        );
    }
}
