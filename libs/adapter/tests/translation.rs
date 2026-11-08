//! Tests for the deliberately limited translation and replay logic.

#[path = "support/codecs.rs"]
mod codecs;
#[path = "support/drive.rs"]
mod drive;
#[path = "support/store.rs"]
mod store;

use std::{convert::Infallible, num::NonZeroU32};

use adapter::replay::{Replay, ReplayStore};
use bytes::Bytes;
use codecs::{AnthropicTextEncoder, OpenAiTextDecoder, TargetProperties};
use futures_util::{StreamExt, TryStreamExt, io::Cursor, stream};
use serde_json::json;
use store::DiskReplayStore;

fn properties() -> TargetProperties {
    TargetProperties {
        model: "target-model".into(),
        max_tokens: NonZeroU32::new(256).unwrap(),
    }
}

// Small fixtures only: preserve upload chunk boundaries in the replay reader so
// the decoder is still exercised at every split, rather than disk read sizes.
#[derive(Default)]
struct ChunkReplayStore(Vec<Bytes>);

impl ReplayStore for ChunkReplayStore {
    type Error = Infallible;
    type Reader = stream::Iter<std::vec::IntoIter<Result<Bytes, Infallible>>>;

    async fn append(&mut self, chunk: Bytes) -> Result<(), Self::Error> {
        self.0.push(chunk);
        Ok(())
    }

    async fn seal(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn open_reader(&mut self) -> Result<Self::Reader, Self::Error> {
        Ok(stream::iter(
            self.0.iter().cloned().map(Ok).collect::<Vec<_>>(),
        ))
    }

    async fn abort(&mut self) -> Result<(), Self::Error> {
        self.0.clear();
        Ok(())
    }
}

async fn translate(chunks: Vec<Bytes>) -> Result<serde_json::Value, ()> {
    let input = stream::iter(chunks.into_iter().map(Ok::<_, Infallible>));
    let mut replay = Replay::new(input, ChunkReplayStore::default());
    let reader = replay
        .open_reader()
        .await
        .map_err(|_| ())?
        .map_err(std::io::Error::other)
        .into_async_read();
    let mut bytes = Cursor::new(Vec::new()); // Small test fixture only.
    let result = drive::translate_request(
        reader,
        OpenAiTextDecoder::default(),
        &mut bytes,
        AnthropicTextEncoder::new(&properties()),
        16 * 1024,
        64,
    )
    .await
    .map_err(|_| ())
    .and_then(|()| serde_json::from_slice(bytes.get_ref()).map_err(|_| ()));
    replay.close().await.map_err(|_| ())?;
    result
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
    let mut replay = Replay::new(input, DiskReplayStore::new().await.unwrap());
    for _ in 0..2 {
        let reader = replay.open_reader().await.unwrap().into_async_read();
        let mut output = Cursor::new(Vec::new());
        drive::translate_request(
            reader,
            OpenAiTextDecoder::default(),
            &mut output,
            AnthropicTextEncoder::new(&properties()),
            16 * 1024,
            64,
        )
        .await
        .unwrap();
        let output: serde_json::Value = serde_json::from_slice(output.get_ref()).unwrap();
        assert_eq!(output["model"], "target-model");
    }
    replay.close().await.unwrap();

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
