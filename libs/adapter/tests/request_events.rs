#[path = "support/drive.rs"]
mod drive;
#[path = "support/request.rs"]
mod support;

use adapter::{
    error::{CodecError, StreamError, TranslationError},
    t2t::request::event::RequestEvent,
};
use bytes::Bytes;
use futures_util::{TryStreamExt, io::Cursor, stream};
use support::{MessageDecoder, ModelEncoder};

#[tokio::test]
async fn provider_encoding_rejects_invalid_lengths_and_premature_completion() {
    struct BrokenEncoder(u8);
    impl adapter::t2t::request::encoder::RequestEncoder for BrokenEncoder {
        fn accept(&mut self, _: RequestEvent) -> Result<(), CodecError> {
            unreachable!()
        }
        fn advance(
            &mut self,
            _: &mut &[u8],
            output: &mut [u8],
        ) -> Result<adapter::codec::EncodeStep, CodecError> {
            Ok(match self.0 {
                0 => adapter::codec::EncodeStep::Wrote(0),
                1 => adapter::codec::EncodeStep::Wrote(output.len() + 1),
                _ => adapter::codec::EncodeStep::Finished,
            })
        }
        fn finish(&mut self) -> Result<(), CodecError> {
            unreachable!()
        }
    }
    for mode in 0..3 {
        assert!(matches!(
            drive::translate_request(
                Cursor::new(Vec::new()),
                MessageDecoder::default(),
                Cursor::new(Vec::new()),
                BrokenEncoder(mode),
                2,
                3,
            )
            .await,
            Err(TranslationError::Stream(StreamError::Codec(
                CodecError::Contract(_)
            )))
        ));
    }
}

#[tokio::test]
async fn retained_encoded_chunks_are_independent_across_byte_forwarding_calls() {
    let mut retained = Vec::new();
    for model in ["a", "b"] {
        let mut output = Cursor::new(Vec::new());
        drive::translate_request(
            Cursor::new(b"hello".to_vec()),
            MessageDecoder::default(),
            &mut output,
            ModelEncoder::new(model),
            2,
            3,
        )
        .await
        .unwrap();
        retained.push(output.into_inner());
    }
    for (output, expected) in retained.iter().zip([b"a:hello!", b"b:hello!"]) {
        assert_eq!(output, expected);
    }
}

#[tokio::test]
async fn byte_source_failure_does_not_finalize_encoder_or_close_target() {
    let source = stream::iter([
        Ok(Bytes::from_static(b"partial")),
        Err("broken byte source"),
    ]);
    let mut bytes = Cursor::new(Vec::new());
    assert!(matches!(
        drive::translate_request(
                source.map_err(std::io::Error::other).into_async_read(),
                MessageDecoder::default(),
                &mut bytes,
                ModelEncoder::new("a"), 2, 3,
            )
            .await,
        Err(TranslationError::Stream(StreamError::Input(error))) if error.to_string() == "broken byte source"
    ));
    assert_eq!(bytes.into_inner(), b"a:partial");
}
