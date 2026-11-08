#[path = "support/request.rs"]
mod support;

use std::convert::Infallible;

use adapter::{
    error::{CodecError, StreamError},
    t2t::{
        Text2TextRequestTranslator,
        request::{RequestForwardError, event::RequestEvent},
    },
};
use bytes::Bytes;
use futures_util::{sink::unfold, stream};
use support::{MessageDecoder, ModelEncoder};

#[tokio::test]
async fn provider_encoding_rejects_invalid_lengths_and_premature_completion() {
    struct BrokenEncoder(u8);
    impl adapter::t2t::request::encoder::RequestEncoder for BrokenEncoder {
        fn accept(&mut self, _: RequestEvent) -> Result<(), CodecError> {
            unreachable!()
        }
        fn advance(&mut self, output: &mut [u8]) -> Result<adapter::codec::EncodeStep, CodecError> {
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
    let mut translator = Text2TextRequestTranslator::new(2, 3);
    for mode in 0..3 {
        assert!(matches!(
            translator
                .forward(
                    stream::pending::<Result<Bytes, Infallible>>(),
                    MessageDecoder::default,
                    futures_util::sink::drain(),
                    BrokenEncoder(mode),
                )
                .await,
            Err(RequestForwardError::Stream(StreamError::Codec(
                CodecError::Contract(_)
            )))
        ));
    }
}

#[tokio::test]
async fn retained_encoded_chunks_are_independent_across_byte_forwarding_calls() {
    let mut translator = Text2TextRequestTranslator::new(2, 3);
    let mut retained = Vec::new();
    for model in ["a", "b"] {
        let mut chunks = Vec::new();
        let sink = unfold(&mut chunks, |chunks, chunk: Bytes| async move {
            chunks.push(chunk);
            Ok::<_, Infallible>(chunks)
        });
        translator
            .forward(
                stream::iter([Ok::<_, Infallible>(Bytes::from_static(b"hello"))]),
                MessageDecoder::default,
                sink,
                ModelEncoder::new(model),
            )
            .await
            .unwrap();
        retained.push(chunks);
    }
    for (chunks, expected) in retained.iter().zip([b"a:hello!", b"b:hello!"]) {
        assert_eq!(
            chunks
                .iter()
                .flat_map(|chunk| chunk.iter().copied())
                .collect::<Vec<_>>(),
            expected
        );
    }
}

#[tokio::test]
async fn byte_source_failure_does_not_finalize_encoder_or_close_target() {
    let source = stream::iter([
        Ok(Bytes::from_static(b"partial")),
        Err("broken byte source"),
    ]);
    let mut translator = Text2TextRequestTranslator::new(2, 3);
    let mut bytes = Vec::new();
    let sink = unfold(&mut bytes, |bytes, chunk: Bytes| async move {
        bytes.extend_from_slice(&chunk);
        Ok::<_, Infallible>(bytes)
    });
    assert!(matches!(
        translator
            .forward(
                source,
                MessageDecoder::default,
                sink,
                ModelEncoder::new("a")
            )
            .await,
        Err(RequestForwardError::Stream(StreamError::Input(
            "broken byte source"
        )))
    ));
    // Failure does not flush a sink's accepted-but-buffered final chunk.
    assert!(bytes.starts_with(b"a:part"));
    assert!(b"a:partial".starts_with(&bytes));
}
