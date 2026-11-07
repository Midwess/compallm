//! Tests for the example's deliberately limited translation and replay logic.

#[path = "../examples/axum_forward_support/codecs.rs"]
mod codecs;
#[path = "../examples/axum_forward_support/store.rs"]
mod store;

use std::{convert::Infallible, num::NonZeroU32};

use adapter::{
    replay::ReplayStore,
    t2t::{Text2TextRequestStream, Text2TextRequestTranslator},
};
use bytes::Bytes;
use codecs::{AnthropicTextEncoder, OpenAiTextDecoder, TargetProperties};
use futures_util::{StreamExt, stream};
use serde_json::json;
use store::DiskReplayStore;

fn properties() -> TargetProperties {
    TargetProperties {
        model: "target-model".into(),
        max_tokens: NonZeroU32::new(256).unwrap(),
    }
}

async fn translate(chunks: Vec<Bytes>) -> Result<serde_json::Value, ()> {
    let input = stream::iter(chunks.into_iter().map(Ok::<_, Infallible>));
    let mut output = Text2TextRequestStream::new(
        input,
        OpenAiTextDecoder::default(),
        AnthropicTextEncoder::new(&properties()),
    );
    let mut bytes = Vec::new(); // Small test fixture only, never the example handler.
    while let Some(chunk) = output.next().await {
        bytes.extend_from_slice(&chunk.map_err(|_| ())?);
    }
    serde_json::from_slice(&bytes).map_err(|_| ())
}

#[tokio::test]
async fn arbitrary_chunk_boundaries_preserve_messages_and_override_model() {
    let input = br#"{"messages":[{"content":"hi \"there\" \uD83D\uDE00","role":"user"},{"role":"assistant","content":"ok"}],"stream":false,"model":"source"}"#;
    let expected = json!({
        "model":"target-model", "max_tokens":256, "stream":false,
        "messages":[
            {"role":"user","content":[{"type":"text","text":"hi \"there\" 😀"}]},
            {"role":"assistant","content":[{"type":"text","text":"ok"}]}
        ]
    });
    for split in 0..=input.len() {
        assert_eq!(
            translate(vec![
                Bytes::copy_from_slice(&input[..split]),
                Bytes::copy_from_slice(&input[split..])
            ])
            .await
            .unwrap(),
            expected
        );
    }
    assert_eq!(
        translate(
            input
                .iter()
                .map(|byte| Bytes::copy_from_slice(&[*byte]))
                .collect()
        )
        .await
        .unwrap(),
        expected
    );
}

#[tokio::test]
async fn rejects_unsupported_incomplete_duplicate_and_trailing_data() {
    for body in [
        r#"{"model":"x","messages":[]}"#,
        r#"{"model":"x","messages":[{"role":"system","content":"x"}]}"#,
        r#"{"model":"x","messages":[{"role":"user","content":[]}]}"#,
        r#"{"model":"x","messages":[{"role":"user","content":"x","name":"x"}]}"#,
        r#"{"model":"x","messages":[{"role":"user","role":"user","content":"x"}]}"#,
        r#"{"model":"x","messages":[{"role":"user","content":"x"}],"stream":true}"#,
        r#"{"model":"x","messages":[{"role":"user","content":"x"}],"tools":[]}"#,
        r#"{"model":"x","messages":[{"role":"user","content":"x"}],"model":"y"}"#,
        r#"{"messages":[{"role":"user","content":"x"}]}"#,
        r#"{"model":"x","messages":[{"role":"user","content":"x"},]}"#,
        r#"{"model":"x","messages":[{"role":"user","content":"x"}],}"#,
        r#"{"model":"x","messages":[{"role":"user","content":"x"}]} {}"#,
        r#"{"model":"x","messages":[{"role":"user","content":"x"}]"#,
    ] {
        assert!(
            translate(vec![Bytes::copy_from_slice(body.as_bytes())])
                .await
                .is_err(),
            "accepted {body}"
        );
    }
}

#[tokio::test]
async fn large_individual_messages_are_rejected_instead_of_accumulating() {
    let body = format!(
        r#"{{"model":"x","messages":[{{"role":"user","content":"{}"}}]}}"#,
        "x".repeat(64 * 1024)
    );
    assert!(translate(vec![Bytes::from(body)]).await.is_err());
}

#[tokio::test]
async fn disk_replay_supports_fresh_attempts_and_reader_lifetime_after_cleanup() {
    let body =
        Bytes::from_static(br#"{"model":"source","messages":[{"role":"user","content":"hello"}]}"#);
    let input = stream::iter([Ok::<_, Infallible>(body)]);
    let mut translator = Text2TextRequestTranslator::new(
        input,
        OpenAiTextDecoder::default,
        DiskReplayStore::new().await.unwrap(),
    );
    for _ in 0..2 {
        let mut attempt = translator
            .forward(AnthropicTextEncoder::new(&properties()))
            .await
            .unwrap();
        let mut output = Vec::new();
        while let Some(chunk) = attempt.next().await {
            output.extend_from_slice(&chunk.unwrap());
        }
        let output: serde_json::Value = serde_json::from_slice(&output).unwrap();
        assert_eq!(output["model"], "target-model");
    }
    translator.close().await.unwrap();

    let mut store = DiskReplayStore::new().await.unwrap();
    store.append(Bytes::from_static(b"original")).await.unwrap();
    store.seal().await.unwrap();
    let mut reader = store.open_reader().await.unwrap();
    store.abort().await.unwrap();
    assert_eq!(
        reader.next().await.unwrap().unwrap(),
        b"original".as_slice()
    );
    assert!(reader.next().await.is_none());
}

#[tokio::test]
async fn replay_rejects_oversized_uploads() {
    let mut store = DiskReplayStore::new().await.unwrap();
    assert!(
        store
            .append(Bytes::from(vec![b'x'; 8 * 1024 * 1024 + 1]))
            .await
            .is_err()
    );
    store.abort().await.unwrap();
}
