//! Test-only stepping loop; the library intentionally has no full-forward API.
use adapter::{
    codec::TranslateStep,
    error::TranslationError,
    t2t::{
        Text2TextRequestTranslator,
        request::{decoder::RequestDecoder, encoder::RequestEncoder},
    },
};
use futures_io::{AsyncRead, AsyncWrite};
use std::{future::poll_fn, task::Poll};

pub async fn translate_request<
    S: AsyncRead,
    T: AsyncWrite,
    D: RequestDecoder,
    E: RequestEncoder,
>(
    source: S,
    decoder: D,
    target: T,
    encoder: E,
    buffer_bytes: usize,
    steps: usize,
) -> Result<(), TranslationError> {
    let mut translator = Text2TextRequestTranslator::new(buffer_bytes, steps);
    translator.set_source_stream(source)?;
    translator.set_target_stream(target)?;
    translator.set_decoder(decoder)?;
    translator.set_encoder(encoder)?;
    loop {
        match translator.translate()? {
            TranslateStep::OutputReady(_) => translator.flush().await?,
            TranslateStep::NeedInput => {
                translator.next().await?;
            }
            TranslateStep::Progress => {
                let mut yielded = false;
                poll_fn(|cx| {
                    if yielded {
                        Poll::Ready(())
                    } else {
                        yielded = true;
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }
                })
                .await;
            }
            TranslateStep::Finished => {
                translator.flush().await?;
                return Ok(());
            }
        }
    }
}
