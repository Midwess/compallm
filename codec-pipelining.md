# Codec pipelining context

## Goal and status

Compose purpose-specific processing stages in Rust. Each stage consumes input incrementally and passes an event downstream as soon as it is available. Do not collect the request's components before processing them.

The intended flow is:

```text
source bytes
    -> JSON tokenizer
    -> compatible OpenAI request parser
    -> TextRequestComponent
    -> target request encoder
    -> output bytes
```

Pipelining, source adapters, token-consuming parser extraction, and the fluent methods shown below are proposals. They are not implemented APIs. This note records the discussion for subsequent design and implementation; it does not authorize additional code changes or Wiki publication.

Current code lives in `libs/codec`, renamed from `libs/adapter`. Relevant files:

- `libs/codec/src/decoder.rs`: borrowing-capable byte decoder trait and progress statuses.
- `libs/codec/src/json/decoder.rs`: incremental JSON tokenizer.
- `libs/codec/src/json/component.rs`: borrowed JSON tokens.
- `libs/codec/src/llm/text/copenai/request/decoder/mod.rs`: compatible request routing, assembly, and conversion.
- `libs/codec/src/llm/text/copenai/request/decoder/value.rs`: owned JSON value builder.
- `libs/codec/src/llm/text/component/request/mod.rs`: request component enum and custom component deserialization.
- `libs/codec/src/encoder.rs`: currently an empty `RequestEncoder` marker trait, not an incremental encoding contract.

## Existing decoder contract

The current trait accepts a byte-slice cursor and an explicit source-EOF flag:

```rust
pub trait Decoder {
    type Component<'a>
    where
        Self: 'a;

    type Error;

    fn next<'a, 'input: 'a>(
        &'a mut self,
        input: &mut &'input [u8],
        eof: bool,
    ) -> Result<DecoderStatus<Self::Component<'a>>, Self::Error>;

    fn finish(self) -> Result<(), Self::Error>;
}

pub enum DecoderStatus<E> {
    Component(E),
    NeedInput,
    Finished,
}
```

`JsonDecoder` emits `JsonToken<'a>`. A token may borrow the source bytes or the decoder's reusable scratch. `CompatibleOpenAIRequestDecoder` emits owned `TextRequestComponent` values and currently owns a `JsonDecoder` internally.

The tokenizer handles partial strings/scalars, UTF-8 and escape decoding. It emits punctuation and scalar tokens; it does not independently validate the complete object/array grammar. The request parser and value builder own the structural checks they require.

The generic associated type allows temporary borrowed output. It also means this trait is not directly usable as `dyn Decoder`. Generic wrappers can use static dispatch without boxing each event.

## Process one component at a time

If a source buffer contains ten components, a pull pipeline should:

1. Decode enough tokens to finish the first component.
2. Pass that component to the next stage or return it to the caller.
3. Preserve the unread byte position and parser state.
4. Resume only when downstream is ready for another component.

No intermediate `Vec<TextRequestComponent>`, unbounded channel, or background task is required. Several tokens may be consumed before one component is ready. A message cannot be emitted as a completed owned message until its necessary fields and closing boundary have been processed.

Reading a chunk may bring bytes for several future components into the input buffer. That read-ahead is different from constructing and retaining those components.

## Byte arrays and streams

Byte slices are sufficient for the core API. An async stream or reader is optional.

For a complete immutable source, a proposed slice adapter could look like this:

```rust,ignore
let mut pipeline = JsonDecoder::new()
    .source(bytes.as_slice())
    .pipe(CompatibleOpenAIRequestParser::new());

while let Some(component) = pipeline.next()? {
    handle(component);
}
```

The adapter owns the decoder and the unread slice cursor, borrowing the source for the pipeline's lifetime. It knows the slice is the complete source and drives EOF validation after draining it.

For externally supplied chunks, preserve the existing cursor-based convention:

```rust,ignore
let mut pipeline = JsonDecoder::new()
    .pipe(CompatibleOpenAIRequestParser::new());

let mut input = chunk.as_slice();
loop {
    match pipeline.next(&mut input, eof)? {
        DecoderStatus::Component(component) => handle(component),
        DecoderStatus::NeedInput => break,
        DecoderStatus::Finished => break,
    }
}
```

`eof` means no bytes will follow this supplied suffix. It does not mean merely that the current chunk has ended. After the final component, the caller continues driving the decoder to validate the remaining suffix and observe completion.

An async reader adapter adds input-buffer ownership and refilling:

```rust,ignore
let mut pipeline = JsonDecoder::new()
    .source(reader)
    .pipe(CompatibleOpenAIRequestParser::new());

while let Some(component) = pipeline.next().await? {
    handle(component);
}
```

These slice, chunk, and reader forms are alternative facade examples. Their exact constructors, method names, return types, and source bounds remain to be designed. In particular, an async `Stream<Item = owned chunk>` and an `AsyncRead` source need different input adapters. Async input operations belong in those source adapters; token processing can remain synchronous.

## Typed stages

Different boundaries have different input types:

| Stage | Input | Output |
| --- | --- | --- |
| Byte source | Slice, reader, or chunk source | Byte suffix |
| JSON tokenizer | Bytes | `JsonToken<'a>` |
| Compatible request parser | JSON tokens | `TextRequestComponent` |
| Target request encoder | Request components | Encoded byte fragments |
| Output sink | Encoded bytes | I/O progress or completion |

Keep the existing `Decoder` for byte-to-event decoding. A token consumer needs a separate event-processing contract. A minimal proposed shape is:

```rust
pub trait EventStage<I> {
    type Output;
    type Error;

    fn accept(
        &mut self,
        input: I,
    ) -> Result<Option<Self::Output>, Self::Error>;
}
```

An implementation for `CompatibleOpenAIRequestParser` would accept `JsonToken<'a>` for any temporary token lifetime and return owned `TextRequestComponent` values. `None` means that the token was consumed but no completed component is ready. It does not mean that the source ended.

A specialized wrapper could express that downstream requirement as:

```rust,ignore
S: for<'a> EventStage<
    JsonToken<'a>,
    Output = TextRequestComponent,
    Error = DecoderError,
>
```

This bound describes a consumer that works for any temporary token lifetime. It does not by itself solve generic composition between arbitrary borrowing output families.

To enable an external JSON-to-request pipe, extract the token-consuming logic from `CompatibleOpenAIRequestDecoder`. The standalone byte decoder can remain a wrapper around the tokenizer and parser if callers still need it. Do not feed JSON tokens into the existing byte-oriented `Decoder::next` method or instantiate a new request parser for each token.

`EventStage` above only covers zero-or-one output per input. It is not a complete generic pipeline contract. A stage that produces multiple events, or still has output after consuming an input, needs a drain/advance operation or an explicit progress state. EOF validation also needs its own operation.

## Pipe execution and borrowed-token handoff

One implementation shape is a wrapper owning both stages:

```rust
struct Pipe<Upstream, Downstream> {
    upstream: Upstream,
    downstream: Downstream,
}
```

The JSON-to-request wrapper's core loop would follow this structure:

```rust,ignore
loop {
    match upstream.next(input, eof)? {
        DecoderStatus::Component(token) => {
            if let Some(component) = downstream.accept(token)? {
                return Ok(DecoderStatus::Component(component));
            }
        }
        DecoderStatus::NeedInput => {
            return Ok(DecoderStatus::NeedInput);
        }
        DecoderStatus::Finished => {
            // Validate and drain downstream before reporting completion.
            downstream.finish()?;
            return Ok(DecoderStatus::Finished);
        }
    }
}
```

This is control-flow pseudocode, not a complete generic implementation. Error conversion, ownership of consuming finalizers, pending output, terminal state, and work budgets are omitted. A generic lending wrapper also needs lifetime bounds that compile for its chosen output family.

The downstream parser consumes the token before the upstream decoder advances again. It may inspect borrowed text immediately or move an owned copy into assembly state. It must not retain a borrow into reusable upstream scratch for later calls.

The proposed `EventStage<I>` uses a fixed output type. It is adequate for the JSON-token-to-owned-component boundary. Further stages that themselves lend temporary output would need another lifetime-dependent output contract or a callback/drain interface.

## Ordered transformations and encoding

Sequential composition means that a downstream stage consumes its immediate upstream's output:

```rust,ignore
let mut pipeline = Source::bytes(bytes.as_slice())
    .pipe(JsonDecoder::new())
    .pipe(CompatibleOpenAIRequestParser::new())
    .pipe(AnthropicRequestEncoder::new());
```

The proposed final stage is an encoder because it produces the target protocol's JSON bytes. It is not another byte parser. This example does not establish an Anthropic constructor or compatibility policy; target model configuration and unsupported-component handling still need contracts.

Encoding may take many output-buffer fills for one component. A proposed encoder therefore needs operations resembling:

```rust,ignore
encoder.accept(component)?;
encoder.write_into(&mut output_buffer)?;
```

The driver repeats output advancement and writes until the encoder requests another component. It must not overwrite pending output or accept another component prematurely. A component may also need no immediate output because some target settings are emitted during finalization.

A possible high-level loop is:

```rust,ignore
while let Some(component) = pipeline.next().await? {
    encoder.accept(component)?;
    encoder.flush_into(&mut writer).await?;
}
```

`flush_into` here means an adapter that drains the encoder's pending bytes. It is not the existing empty encoder trait, nor does it specify whether the writer's transport flush is called after every component. End-of-request encoding and final output drainage still follow successful input validation.

Compatibility is separate from composition. Define whether a target rejects, maps, or explicitly omits each unsupported component. Do not silently drop tools or reasoning merely because a fluent chain connects the types.

## Side branches and parallel execution

A raw-byte recorder belongs before JSON decoding:

```text
source bytes ------+----> request-byte recorder
                   |
                   +----> JsonDecoder
                              -> compatible request parser
                              -> target request encoder
```

The recorder needs original bytes, including whitespace, escapes, and field order. JSON tokens cannot reconstruct an exact byte-for-byte replay of the request.

Prefer separate names for transformations and side branches:

```rust,ignore
let mut pipeline = Source::bytes(bytes.as_slice())
    .tee(RecordRequestBytes::new())
    .pipe(JsonDecoder::new())
    .pipe(CompatibleOpenAIRequestParser::new())
    .pipe(AnthropicRequestEncoder::new());
```

- `.pipe(stage)` transforms the main path.
- `.tee(handler)` observes the current boundary's data while preserving the main path.
- An inline tap consumes the borrow before the source buffer is reused; no queue or spawned task is inherently needed.
- Calling two stages in sequence does not make them concurrent.

The initially suggested spelling `.pipe_parralel(...)` should not obscure whether an operation is fan-out, a tap, or a transformation. JSON decoding, request parsing, and encoding have data dependencies; they must preserve dependency order.

A genuinely concurrent side branch could be explicit:

```rust,ignore
let mut pipeline = Source::bytes(bytes.as_slice())
    .tee_concurrent(
        RecordRequestBytes::new(),
        BranchOptions { max_in_flight: 2 },
    )
    .pipe(JsonDecoder::new())
    .pipe(CompatibleOpenAIRequestParser::new());
```

This is proposed syntax. A concurrent branch requires bounds on retained bytes as well as chunk count, error policy, cancellation, and completion behavior. Concurrency may use cooperatively polled futures; it does not inherently require separate threads or tasks. If separate transforming branches must rejoin, their output types and join ordering also need explicit design.

The source must tap each newly admitted byte range once. Do not record the same unread suffix again on every pull from the decoder.

### Parallel borrowing and storage

Both branches may borrow a complete immutable source while it remains alive. Scoped concurrent operations can also share a borrowed streaming chunk if both finish before the buffer is reused.

An independently advancing recorder cannot retain a slice into a buffer that the reader overwrites. Choose among waiting for the branch, shared owned chunks such as `Bytes`/`Arc`, or copying into separately owned storage. Shared ownership avoids another payload copy only when the original storage already supports that ownership model.

If recording falls behind, select an explicit policy: backpressure, fail the request, or disable recording/replay. Queueing without a bound is not acceptable. Recording retained original bytes is itself a storage operation with capacity, lifetime, and cleanup requirements; pipeline syntax does not make replay free or guarantee exactly-once provider execution.

## Memory, copying, and backpressure

Static pipe wrappers and synchronous borrowed-token handoff need no per-event heap allocation or intermediate queue. Overall memory still includes:

- Source and output buffers.
- JSON token scratch for fragmented or escaped strings.
- The current request component under assembly.
- Encoder state and pending output.
- Bounded retained data for any concurrent branch or replay store.

The current compatible parser constructs owned `serde_json::Value` data, then consumes it into request component types. It buffers one message at a time and whole values for fields such as `tools` or schemas. Strings and keys are copied into owned values. Custom deserialization can rebuild maps while moving string ownership. Pipelining does not remove those allocations or impose a byte bound on a large component.

With complete unescaped tokens, JSON can borrow directly from the source. Fragmentation copies into reusable token storage; unescaping writes into decoded scratch. Transport bridges and schema serialization may add further copies.

To bound memory independently of message/tool/schema size, change the event vocabulary to fragments and consume each fragment before reusing its storage. That is a separate refactor from adding `.pipe()`.

Backpressure follows the pull order: finish handling pending output before asking for more components, finish using borrowed tokens before advancing JSON, and refill input only when its consumed region is safe to reuse. A driver should also bound ready work per scheduling turn so a large already-buffered request does not monopolize an async task.

## EOF, errors, and cancellation

- `NeedInput` is decoder progress, not asynchronous `Poll::Pending`. The reader adapter translates it into an input read when necessary.
- At real EOF, keep driving `next(..., true)` so a final scalar can be emitted while its scratch is still alive. Incomplete strings/scalars must fail.
- Validate the entire remaining input, including trailing bytes. Completing the root object does not prove source EOF.
- A token stage must validate its own required fields and container completion when upstream ends.
- Drain any stage's final output before consuming it through `finish(self)`. A consuming finalizer cannot return a token borrowed from its destroyed buffers.
- Preserve error provenance across source I/O, JSON, request parsing, encoding, output I/O, and recording. Do not require all layers to use identical errors just to compose them.
- Late validation may occur after bytes were sent downstream. Discard a failed attempt's partial target; do not treat it as a complete request or retry delivered output silently.
- Keep partial-write offsets and pending state in the adapter if canceled operations are resumable. Dropping a future does not roll back writes or other external effects.
- A chain holding a stage with consuming `finish` must own it or have an explicit ownership/take protocol. A borrowed generic wrapper cannot simply move out that stage.

Exact repeated-completion behavior, early `finish` rules, and branch shutdown semantics remain to be specified. Current codec methods reject some calls after `Finished`; the proposed adapters should not assume all stages are fused automatically.

## Rust patterns and references

The overall pattern is pipes and filters with typed adapters. Familiar Rust tools cover parts of it:

- [`futures::StreamExt`](https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html): synchronous `map`, asynchronous `then`, filtering, and other combinators for ordinary stream items. `chain` concatenates streams; it does not compose codecs as transforms.
- [`lending-iterator`](https://docs.rs/lending-iterator/latest/lending_iterator/): outputs whose lifetimes depend on borrowing their producer. Related to this decoder's GAT-based tokens, but not a complete async source/encoder driver.
- Synchronous callback or SAX-style processing: immediately feed each borrowed event to a consumer while retaining parsing state between calls.

An ordinary `Stream` has a fixed `Item` type and cannot directly express an item borrowing its own mutable reusable scratch for each poll. It can carry references into suitable external storage, or owned/shared items. Once the JSON-to-request wrapper yields owned `TextRequestComponent`, an ordinary `Stream` facade becomes possible. Avoid converting borrowed tokens to owned tokens solely to fit a stream combinator.

Use the existing Wiki for broader integration context:

- [Stream adaptation and replay](https://github.com/Midwess/compallm/wiki/Llm-router-stream-adaptation)
- [Compatible OpenAI requests](https://github.com/Midwess/compallm/wiki/Llm-router-compatible-openai-requests)
- [LLM Router](https://github.com/Midwess/compallm/wiki/Llm-router)

Those pages describe contracts that differ from the current local owned-component implementation. Do not infer that fluent adapters, fixed scratch memory, production replay storage, or Anthropic encoding exist in `libs/codec` from those descriptions.

## Remaining design choices

1. Extract a persistent JSON-token-consuming request parser while preserving a standalone byte-decoder facade if needed.
2. Choose finalization/draining contracts for zero, one, or multiple outputs per input.
3. Decide whether initial composition targets concrete types or requires runtime-selected stages; GAT borrowing prevents a straightforward `dyn Decoder` design.
4. Keep slice/chunk processing as the core and decide source-adapter naming and reader/stream support.
5. Define an incremental encoder contract, output ownership, target configuration, and unsupported-feature policy.
6. Define recording bounds, backpressure/failure policy, ordering, and cancellation before implementing concurrency.
7. Decide whether component-sized buffering is acceptable or fragment events are required.
8. Validate borrowing bounds and progress semantics in concrete implementations before promising a fully generic `.pipe()` API.
