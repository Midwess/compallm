use adapter::decoder::{Decoder, DecoderStatus};
use adapter::t2t::component::{StreamOptions, TextRequestComponent};
use adapter::t2t::copenai::request::CompatibleOpenAIRequestDecoder;

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
                let mut decoder = CompatibleOpenAIRequestDecoder::new();
                let mut options = Vec::new();
                for chunk in body.as_bytes().chunks(size) {
                    let mut input = chunk;
                    loop {
                        match decoder.next(&mut input).unwrap() {
                            DecoderStatus::Component(TextRequestComponent::StreamOptions(
                                value,
                            )) => options.push(value),
                            DecoderStatus::Component(_) => {}
                            DecoderStatus::NeedInput => break,
                            DecoderStatus::Finished => panic!("unexpected finish"),
                        }
                    }
                }
                decoder.finish().unwrap();
                assert_eq!(options, vec![expected]);
            }
        }
    }
}
