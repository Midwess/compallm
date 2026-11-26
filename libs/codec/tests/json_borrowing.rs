use codec::decoder::{Decoder, DecoderStatus};
use codec::json::{JsonDecoder, JsonToken};
use serde::Deserialize;

#[derive(Deserialize)]
struct BorrowedFields<'a> {
    first: &'a str,
    second: &'a str,
}

#[test]
fn borrows_complete_captured_strings_and_token_keys_from_input() {
    let source = br#"{"first":"alpha","second":"omega"}"#;
    let mut input = source.as_slice();
    let mut decoder = JsonDecoder::new();
    let DecoderStatus::Component(value) = decoder
        .parse_next::<BorrowedFields<'_>>(&mut input, true)
        .expect("capture complete object")
    else {
        panic!("expected captured object");
    };
    let source_range = source.as_ptr() as usize..source.as_ptr() as usize + source.len();
    assert!(source_range.contains(&(value.first.as_ptr() as usize)));
    assert!(source_range.contains(&(value.second.as_ptr() as usize)));
    decoder.finish().expect("finish captured object");

    let mut input = source.as_slice();
    let mut decoder = JsonDecoder::new();
    assert!(matches!(
        decoder.next(&mut input, true).expect("open object"),
        DecoderStatus::Component(JsonToken::ObjectStart(None))
    ));
    let DecoderStatus::Component(JsonToken::ValueString(Some(key), text)) =
        decoder.next(&mut input, true).expect("read keyed string")
    else {
        panic!("expected keyed string");
    };
    assert!(source_range.contains(&(key.as_ptr() as usize)));
    assert!(source_range.contains(&(text.as_ptr() as usize)));
}

#[test]
fn retains_fragmented_and_escaped_strings_across_input_buffer_reuse() {
    let source = br#"{"fi\u0072st":"alpha\n\uD83E\uDD80","second":"omega"}"#;
    for chunk_size in [1, 7, 16] {
        let mut decoder = JsonDecoder::new();
        let mut buffer = [0; 16];
        let mut retained = None;
        for chunk in source.chunks(chunk_size) {
            buffer[..chunk.len()].copy_from_slice(chunk);
            let mut input = &buffer[..chunk.len()];
            match decoder
                .parse_next::<BorrowedFields<'_>>(&mut input, false)
                .expect("capture fragmented object")
            {
                DecoderStatus::NeedInput => {}
                DecoderStatus::Component(value) => {
                    retained = Some((value.first.to_owned(), value.second.to_owned()));
                }
                DecoderStatus::Finished => panic!("unexpected capture completion"),
            }
            buffer.fill(b'!');
        }
        assert!(matches!(
            decoder.next(&mut &b""[..], true).expect("validate EOF"),
            DecoderStatus::Finished
        ));
        decoder.finish().expect("finish fragmented object");
        let (first, second) = retained.expect("retained captured strings");
        assert_eq!(first, "alpha\n\u{1f980}");
        assert_eq!(second, "omega");
    }
}
