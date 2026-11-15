use adapter::decoder::{Decoder, DecoderStatus};
use adapter::t2t::component::TextRequestComponent;
use adapter::t2t::copenai::request::CompatibleOpenAIRequestDecoder;

#[test]
fn emits_controls_in_source_order_across_chunks() {
    for limit in ["max_tokens", "max_completion_tokens"] {
        let body = format!(
            r#"{{"model":"x","parallel_tool_calls":false,"top_p":5e-1,"top_k":0,"messages":[{{"role":"user","content":"hi"}}],"{limit}":32768,"stream":true}}"#
        );
        for size in 1..=body.len() {
            let mut decoder = CompatibleOpenAIRequestDecoder::new();
            let mut components = Vec::new();
            for chunk in body.as_bytes().chunks(size) {
                let mut input = chunk;
                loop {
                    match decoder.next(&mut input).unwrap() {
                        DecoderStatus::Component(component) => components.push(component),
                        DecoderStatus::NeedInput => break,
                        DecoderStatus::Finished => panic!("unexpected finish"),
                    }
                }
            }
            decoder.finish().unwrap();
            assert_eq!(components.len(), 7);
            assert_eq!(
                components[1],
                TextRequestComponent::ParallelToolCalls(false)
            );
            assert_eq!(components[2], TextRequestComponent::TopP(0.5));
            assert_eq!(components[3], TextRequestComponent::TopK(0));
            assert_eq!(
                components[5],
                TextRequestComponent::MaxCompletionTokens(32768)
            );
            assert_eq!(components[6], TextRequestComponent::Stream(true));
        }
    }
}
