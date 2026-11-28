use codec::decoder::{Decoder, DecoderStatus};
use codec::json::JsonDecoder;
use codec::llm::text::component::TextRequestComponent;
use codec::llm::text::copenai::request::CompatibleOpenAIRequestDecoder;

#[test]
fn borrows_scalar_and_message_text_from_complete_input() {
    let source = br#"{"model":"model","messages":[{"role":"user","content":"hello"}]}"#;
    let range = source.as_ptr() as usize..source.as_ptr() as usize + source.len();
    let mut input = source.as_slice();
    let mut decoder = CompatibleOpenAIRequestDecoder::new(JsonDecoder::new());
    let mut strings = 0;
    loop {
        match decoder.next(&mut input, true).unwrap() {
            DecoderStatus::Component(component) => {
                component.map_text(|text| {
                    assert!(range.contains(&(text.as_ptr() as usize)));
                    strings += 1;
                });
            }
            DecoderStatus::Finished => break,
            DecoderStatus::NeedInput => panic!("complete source"),
        }
    }
    assert_eq!(strings, 2);
    decoder.finish().unwrap();
}

#[test]
fn decodes_nested_messages_across_reused_input_chunks() {
    let source = br#"{"model":"model","messages":[{"role":"user","content":[{"type":"text","text":"hello\nworld"}]}],"tools":[],"stop":["END"],"stream":true}"#;
    let mut expected = None;
    for chunk_size in [1, 7, 16, source.len()] {
        let mut decoder = CompatibleOpenAIRequestDecoder::new(JsonDecoder::new());
        let mut buffer = vec![0; chunk_size];
        let mut components = Vec::new();
        for chunk in source.chunks(chunk_size) {
            buffer[..chunk.len()].copy_from_slice(chunk);
            let mut input = &buffer[..chunk.len()];
            loop {
                match decoder.next(&mut input, false).unwrap() {
                    DecoderStatus::Component(component) => components.push(component.into_owned()),
                    DecoderStatus::NeedInput => {
                        assert!(input.is_empty());
                        break;
                    }
                    DecoderStatus::Finished => panic!("EOF not supplied"),
                }
            }
            buffer.fill(b'!');
        }
        assert!(matches!(
            decoder.next(&mut &b""[..], true).unwrap(),
            DecoderStatus::Finished
        ));
        decoder.finish().unwrap();
        assert_eq!(components.len(), 5);
        assert!(matches!(&components[2], TextRequestComponent::Tools(tools) if tools.is_empty()));
        assert_eq!(components[4], TextRequestComponent::Stream(true));
        use codec::encoder::{Encoder, EncoderStatus};
        let mut encoder = codec::llm::text::copenai::request::CompatibleOpenAIRequestEncoder::new(
            codec::json::JsonEncoder::new(),
            Default::default(),
        );
        let mut output = vec![0; chunk_size];
        let mut encoded = Vec::new();
        for component in &mut components {
            loop {
                match encoder.next(Some(component), &mut output).unwrap() {
                    EncoderStatus::NeedOutput { written } => {
                        encoded.extend_from_slice(&output[..written])
                    }
                    EncoderStatus::ComponentFinished { written } => {
                        encoded.extend_from_slice(&output[..written]);
                        break;
                    }
                    EncoderStatus::Finished { .. } => panic!("unexpected end"),
                }
            }
        }
        loop {
            match encoder
                .next(None::<&mut TextRequestComponent>, &mut output)
                .unwrap()
            {
                EncoderStatus::NeedOutput { written } => {
                    encoded.extend_from_slice(&output[..written])
                }
                EncoderStatus::Finished { written } => {
                    encoded.extend_from_slice(&output[..written]);
                    break;
                }
                EncoderStatus::ComponentFinished { .. } => panic!("unexpected component"),
            }
        }
        encoder.finish().unwrap();
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&encoded).unwrap(),
            serde_json::from_slice::<serde_json::Value>(source).unwrap()
        );
        if let Some(expected) = &expected {
            assert_eq!(&components, expected);
        } else {
            expected = Some(components);
        }
    }
}
