use codec::decoder::{Decoder, DecoderStatus};
use codec::llm::text::component::{StreamOptions, TextRequestComponent};
use codec::llm::text::copenai::request::CompatibleOpenAIRequestDecoder;

#[test]
fn preserves_stream_options_across_chunks() {
    for (value, expected) in [
        ("null", None),
        (
            "{}",
            Some(StreamOptions {
                include_usage: None,
            }),
        ),
        (
            r#"{"include_usage":true}"#,
            Some(StreamOptions {
                include_usage: Some(true),
            }),
        ),
        (
            r#"{"include_usage":false}"#,
            Some(StreamOptions {
                include_usage: Some(false),
            }),
        ),
    ] {
        for before_messages in [true, false] {
            let messages = r#""messages":[{"role":"user","content":"hi"}]"#;
            let body = if before_messages {
                format!(r#"{{"model":"x","stream_options":{value},{messages}}}"#)
            } else {
                format!(r#"{{"model":"x",{messages},"stream_options":{value}}}"#)
            };
            for size in 1..=body.len() {
                let mut decoder =
                    CompatibleOpenAIRequestDecoder::new(codec::json::JsonDecoder::new());
                let mut options = Vec::new();
                for chunk in body.as_bytes().chunks(size) {
                    let mut input = chunk;
                    loop {
                        match decoder.next(&mut input, false).unwrap() {
                            DecoderStatus::Component(TextRequestComponent::StreamOptions(
                                value,
                            )) => options.push(value),
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
                assert_eq!(options, vec![expected]);
            }
        }
    }
}
