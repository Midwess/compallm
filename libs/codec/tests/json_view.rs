use codec::decoder::DecoderStatus;
use codec::json::{JsonDecoder, JsonToken};

#[test]
fn views_complete_nested_values_directly_in_input() {
    let source = br#" {"role":"user","content":[{"type":"text","text":"hello\nworld"}],"flags":[true,null,17],"empty":{}} "#;
    let mut input = source.as_slice();
    let mut decoder = JsonDecoder::new();
    assert!(matches!(
        decoder.advance_value(&mut input, true).unwrap(),
        DecoderStatus::Component(())
    ));
    let view = decoder.view_current(source).unwrap();
    let range = source.as_ptr() as usize..source.as_ptr() as usize + source.len();
    assert!(range.contains(&(view.as_str().as_ptr() as usize)));
    let mut fields = view.members().unwrap();
    let (key, role) = fields.next().unwrap();
    assert_eq!(key.decode_string(&mut []).unwrap(), "role");
    assert_eq!(role.decode_string(&mut []).unwrap(), "user");
    let (_, content) = fields.next().unwrap();
    let part = content.items().unwrap().next().unwrap();
    let (_, text) = part.members().unwrap().nth(1).unwrap();
    let mut scratch = [0; 32];
    assert_eq!(text.decode_string(&mut scratch).unwrap(), "hello\nworld");
    assert!(range.contains(&(text.as_str().as_ptr() as usize)));
    let (_, flags) = fields.next().unwrap();
    let flags: Vec<_> = flags.items().unwrap().map(|value| value.as_str()).collect();
    assert_eq!(flags, ["true", "null", "17"]);
    assert!(fields.next().unwrap().1.members().unwrap().next().is_none());
    assert!(fields.next().is_none());
}

#[test]
fn views_fragmented_values_after_input_storage_is_reused() {
    let source = br#"[{"text":"escaped quote: \" and braces: } ]"},{"text":"\ud83e\udd80"}]"#;
    for size in [1, 7, source.len()] {
        let mut decoder = codec::json::JsonContainer::new(JsonDecoder::new());
        let mut buffer = vec![0; size];
        let mut entered = false;
        let mut values = Vec::new();
        for chunk in source.chunks(size) {
            buffer[..chunk.len()].copy_from_slice(chunk);
            let mut input = &buffer[..chunk.len()];
            loop {
                let original = input;
                if !entered {
                    match decoder.advance(&mut input, false).unwrap() {
                        DecoderStatus::Component(()) => {
                            assert_eq!(
                                decoder.current(original).unwrap(),
                                JsonToken::ArrayStart(None)
                            );
                            decoder.enter().unwrap();
                            entered = true;
                        }
                        DecoderStatus::NeedInput => break,
                        DecoderStatus::Finished => panic!("unexpected end"),
                    }
                } else {
                    match decoder.advance_value(&mut input, false).unwrap() {
                        DecoderStatus::Component(()) => {
                            let view = decoder.view_current(original).unwrap();
                            let (_, text) = view.members().unwrap().next().unwrap();
                            let mut scratch = [0; 64];
                            values.push(text.decode_string(&mut scratch).unwrap().to_owned());
                        }
                        DecoderStatus::NeedInput | DecoderStatus::Finished => break,
                    }
                }
            }
            buffer.fill(b'!');
        }
        assert_eq!(values, ["escaped quote: \" and braces: } ]", "\u{1f980}"]);
        assert!(matches!(
            decoder.advance(&mut &b""[..], true).unwrap(),
            DecoderStatus::Finished
        ));
        decoder.finish().unwrap();
    }
}
