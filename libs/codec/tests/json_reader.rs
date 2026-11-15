use codec::decoder::{Decoder, DecoderStatus};
use codec::json::{JsonDecodeError, JsonDecoder, JsonToken};

#[test]
fn reuses_decoding_storage_and_handles_json_escapes() {
    let mut decoder = JsonDecoder::new();
    let mut input = &br#""long\ntext\twith\rspace\b\f\/\\\"\u0061\uD83D\uDE00" "hi\n""#[..];
    let pointer = match decoder.next(&mut input, false).unwrap() {
        DecoderStatus::Component(JsonToken::String(text)) => {
            assert_eq!(text, "long\ntext\twith\rspace\u{8}\u{c}/\\\"a😀");
            text.as_ptr()
        }
        event => panic!("unexpected event: {event:?}"),
    };
    match decoder.next(&mut input, false).unwrap() {
        DecoderStatus::Component(JsonToken::String(text)) => {
            assert_eq!(text, "hi\n");
            assert_eq!(text.as_ptr(), pointer);
        }
        event => panic!("unexpected event: {event:?}"),
    }
}

#[test]
fn rejects_invalid_escapes_and_surrogates() {
    for body in [
        &br#""\x""#[..],
        &br#""\uDC00""#[..],
        &br#""\uD800a""#[..],
        &br#""\uD800\u0061""#[..],
        &br#""\uZZZZ""#[..],
        &b"\"\xff\\n\""[..],
    ] {
        let mut decoder = JsonDecoder::new();
        let mut input = body;
        assert!(matches!(
            decoder.next(&mut input, false),
            Err(JsonDecodeError::InvalidToken)
        ));
        assert!(matches!(
            decoder.next(&mut input, false),
            Err(JsonDecodeError::InvalidState)
        ));
    }
}

#[test]
fn borrows_complete_tokens_and_preserves_remaining_input() {
    let body = br#"["hello",12,true,null]"#;
    let mut input = body.as_slice();
    let mut decoder = JsonDecoder::new();
    assert_eq!(
        decoder.next(&mut input, false).unwrap(),
        DecoderStatus::Component(JsonToken::ArrayStart)
    );
    match decoder.next(&mut input, false).unwrap() {
        DecoderStatus::Component(JsonToken::String(text)) => {
            assert_eq!(text, "hello");
            assert_eq!(text.as_ptr(), body[2..].as_ptr());
        }
        event => panic!("unexpected event: {event:?}"),
    }
    assert_eq!(input, b",12,true,null]");
    for expected in [
        JsonToken::Comma,
        JsonToken::Number("12"),
        JsonToken::Comma,
        JsonToken::Boolean(true),
        JsonToken::Comma,
        JsonToken::Null,
        JsonToken::ArrayEnd,
    ] {
        assert_eq!(
            decoder.next(&mut input, false).unwrap(),
            DecoderStatus::Component(expected)
        );
    }
    assert_eq!(
        decoder.next(&mut input, false).unwrap(),
        DecoderStatus::NeedInput
    );
    assert_eq!(
        decoder.next(&mut &b""[..], true).unwrap(),
        DecoderStatus::Finished
    );
    decoder.finish().unwrap();
}

#[test]
fn retains_fragmented_tokens_and_decodes_escapes() {
    let body = br#"{"model":"provider-model","text":"h\u00e9\uD83D\uDE00","n":-12.5e2}"#;
    let expected = ["provider-model", "hé😀"];
    for size in 1..=body.len() {
        let mut decoder = JsonDecoder::new();
        let mut values = Vec::new();
        let mut number = None;
        for chunk in body.chunks(size) {
            let mut input = chunk;
            loop {
                match decoder.next(&mut input, false).unwrap() {
                    DecoderStatus::Component(JsonToken::String(text))
                        if expected.contains(&text) =>
                    {
                        values.push(text.to_owned())
                    }
                    DecoderStatus::Component(JsonToken::Number(text)) => {
                        number = Some(text.to_owned())
                    }
                    DecoderStatus::Component(_) => {}
                    DecoderStatus::NeedInput => {
                        assert!(input.is_empty());
                        break;
                    }
                    DecoderStatus::Finished => panic!("unexpected EOF"),
                }
            }
        }
        assert_eq!(values, expected);
        assert_eq!(number.as_deref(), Some("-12.5e2"));
        assert_eq!(
            decoder.next(&mut &b""[..], true).unwrap(),
            DecoderStatus::Finished
        );
        decoder.finish().unwrap();
    }
    let mut decoder = JsonDecoder::new();
    assert_eq!(
        decoder.next(&mut &b"12"[..], false).unwrap(),
        DecoderStatus::NeedInput
    );
    assert_eq!(
        decoder.next(&mut &b""[..], true).unwrap(),
        DecoderStatus::Component(JsonToken::Number("12"))
    );
    decoder.finish().unwrap();
    let mut decoder = JsonDecoder::new();
    assert_eq!(
        decoder.next(&mut &b"\"unfinished"[..], false).unwrap(),
        DecoderStatus::NeedInput
    );
    assert!(matches!(
        decoder.finish(),
        Err(JsonDecodeError::IncompleteToken)
    ));
}
