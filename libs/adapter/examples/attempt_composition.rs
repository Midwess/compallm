//! Compile-only lifecycle example: supply your own replay backend and codecs.
//! No provider codecs, HTTP clients, or retry policy are implemented here.

use adapter::{
    replay::ReplayStore,
    t2t::{
        Text2TextRequestTranslator, Text2TextResponseTranslator,
        request::{
            decoder::RequestDecoder, encoder::RequestEncoder, translator::RequestTranslatorError,
        },
        response::{decoder::ResponseDecoder, encoder::ResponseEncoder},
    },
};
use bytes::Bytes;
use futures_core::Stream;

/// A fixed OpenAI source decoder factory can be paired with an Anthropic target
/// encoder first, then an OpenAI target encoder for fallback. These encoders may
/// have different concrete Rust types. The store is owned inside the translator.
pub async fn request_attempts<S, R, F, D, A, B, I>(
    input: S,
    store: R,
    openai_decoder_factory: F,
    anthropic_encoder: A,
    openai_encoder: B,
) -> Result<(), RequestTranslatorError<I, R::Error>>
where
    S: Stream<Item = Result<Bytes, I>>,
    R: ReplayStore,
    F: FnMut() -> D,
    D: RequestDecoder,
    A: RequestEncoder,
    B: RequestEncoder,
{
    let mut translator = Text2TextRequestTranslator::new(input, openai_decoder_factory, store);

    // Retains the complete original upload before creating the first body.
    let first_body = translator.forward(anthropic_encoder).await?;
    // Send first_body using the Anthropic HTTP client here.
    // If eligible failure occurs before response commitment: cancel/join HTTP
    // attempt A and release its body before requesting another attempt.
    drop(first_body);

    let fallback_body = translator.forward(openai_encoder).await?;
    // Send fallback_body using the OpenAI HTTP client here. It starts at byte 0
    // with a fresh source decoder; no prior encoder state is reused.
    drop(fallback_body);

    // Also arrange close on error paths; Drop cannot await remote cleanup.
    translator.close().await
}

/// Keep the client encoder factory fixed while changing the provider decoder.
/// The supervisor must not silently switch responses after client commitment.
pub fn response_attempts<SA, SB, DA, DB, F, E, IA, IB>(
    anthropic_body: SA,
    anthropic_decoder: DA,
    openai_body: SB,
    openai_decoder: DB,
    openai_encoder_factory: F,
) where
    SA: Stream<Item = Result<Bytes, IA>>,
    SB: Stream<Item = Result<Bytes, IB>>,
    DA: ResponseDecoder,
    DB: ResponseDecoder,
    F: FnMut() -> E,
    E: ResponseEncoder,
{
    let mut translator = Text2TextResponseTranslator::new(openai_encoder_factory);
    let first = translator.start_attempt(anthropic_body, anthropic_decoder);
    // Inspect/forward under the supervisor's commitment gate.
    drop(first);
    let fallback = translator.start_attempt(openai_body, openai_decoder);
    drop(fallback);
}

fn main() {}
