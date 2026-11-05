//! Proposed Router usage for review, NOT an implemented or compiling example.
//! This nested file is outside Cargo's automatically discovered example targets.
//! Names, signatures, replay limits, and failure/durability policies are proposals.
//! No Bevy dependency is intended.

use compallm_core::model::Model;
use router::prelude::*;
use std::sync::Arc;
use tokio::net::TcpStream;

// Target ownership:
// Router: Model, immutable stage definitions, shared services/provider instances.
// Request: authorization, parsed sections, replay log, persistence identity.
// Attempt: selected provider, provider mapping, replay cursor, response decoder.
// Event: one owned request section or response event, with sequence/attempt IDs.
// Executable stages and processor state are connection-local and built lazily.

fn create_router(
    model: Model,
    providers: Arc<ProviderRegistry>,
    authorization: Arc<AuthorizationService>,
    messages: Arc<MessageStore>,
    replay: Option<ReplayConfig>,
) -> Router {
    let mut router = Router::new(model);

    // The registry owns pooled clients and shared provider readiness.
    // Readiness refreshes share in-flight work; each request still checks eligibility.
    router.insert_shared_resource(providers);
    router.insert_shared_resource(authorization);
    router.insert_shared_resource(messages);
    router.add_plugins(HttpProcessingPlugin);
    if let Some(config) = replay {
        router.add_plugins(ReplayPlugin { config });
    }

    // Success gates the transition to BodyStarted. Merely running "after" an
    // authorization processor must never grant access when it returned rejection.
    router.add_processors(
        HeaderReceived,
        (
            read_credentials
                .pipe(authenticate_api_key)
                .pipe(authorize_model)
                .pipe(store_authorization)
                .run_if(needs_authorization),
            validate_request_headers,
        ),
    );

    router.add_processors(BodyStarted, initialize_request_parser);

    // A section is meaningful parsed data, not an arbitrary TCP read.
    // Retain provider-independent data before any attempt consumes that section.
    router.add_processors(
        RequestSectionReceived,
        (
            normalize_request_section,
            save_user_message
                .after(normalize_request_section)
                .run_if(is_new_complete_user_message),
        ),
    );

    // The runtime begins an attempt only when required request settings are known.
    // This need not wait for all input if the upstream protocol supports streaming.
    router.add_processors(
        AttemptStarted,
        (
            select_available_provider,
            prepare_provider_mapping,
            open_provider_request,
        )
            .chain(),
    );

    // Initial and replay inputs both feed this same provider-specific mapping.
    // Effort/settings mapping is prepared for the selected provider above.
    router.add_processors(
        AttemptInputReceived,
        map_request_section.pipe(write_provider_input),
    );

    router.add_processors(
        ProviderHeaderReceived,
        (
            inspect_provider_status,
            map_response_headers,
            write_client_headers,
        )
            .chain(),
    );

    // One complete protocol event, assembled across body chunks when necessary.
    // Reject stale attempt IDs before any client output or persistence effects.
    router.add_processors(
        ProviderEventReceived,
        (
            reject_stale_attempt_event,
            normalize_provider_event.after(reject_stale_attempt_event),
            collect_response.after(normalize_provider_event),
            map_client_event
                .pipe(write_client_event)
                .after(normalize_provider_event),
        ),
    );

    // evaluate_fallback records a decision. The HTTP driver explicitly runs
    // the next stage after this one returns; the stage runner never restarts itself.
    router.add_processors(
        ProviderFailed,
        (
            stop_failed_attempt,
            evaluate_fallback.after(stop_failed_attempt),
            prepare_next_attempt
                .after(evaluate_fallback)
                .run_if(fallback_allowed),
        ),
    );

    router.add_processors(ResponseFinished, finalize_response.pipe(save_response));
    router.add_processors(RequestFinished, finalize_request);
    router
}

async fn use_router(router: &mut Router, stream: TcpStream) -> Result<()> {
    // The caller has already obtained the stream and selected its model Router.
    router.on_connection(stream)?;
    while let Some(result) = router.join_next().await {
        result??;
    }
    Ok(())
}

// Example typed parameters: select from shared readiness, but record the choice
// in this attempt's Resources. This is a lookup, not a fresh probe per request.
fn select_available_provider(
    providers: Shared<ProviderRegistry>,
    request: Res<RequestRequirements>,
    history: Res<AttemptHistory>,
    mut writer: ContextWriter,
) -> Result<()> {
    let provider = providers.select_eligible(&request, &history)?;
    writer.insert_resource(SelectedProvider(provider));
    Ok(())
}

// Async processors receive owned values and shared service handles.
// Resource borrows must end before awaiting network or database operations.
async fn authenticate_api_key(
    credentials: In<Credentials>,
    service: Shared<AuthorizationService>,
) -> Result<AuthenticatedCaller> {
    service.authenticate(credentials.into_inner()).await
}

async fn save_user_message(
    message: In<CompleteUserMessage>,
    store: Shared<MessageStore>,
) -> Result<()> {
    // Stable request/message identity prevents duplicate writes.
    // A replay may trigger the same stage again; its condition and stable identity
    // prevent saving the same user message twice.
    store.save_user_message(message.into_inner()).await
}

async fn write_provider_input(
    section: In<EncodedProviderSection>,
    attempt: Shared<AttemptWriter>,
) -> Result<()> {
    // One ordered writer per attempt; await capacity instead of accumulating data.
    attempt.write(section.into_inner()).await
}

// Plugins install resources and processors. There is no configure_replay method
// and no lifecycle rewind/restart primitive in Router or Stage.
struct HttpProcessingPlugin;
impl Plugin for HttpProcessingPlugin {
    fn build(&self, router: &mut Router) {
        router.set_driver(HttpDriver);
    }
}

struct ReplayPlugin {
    config: ReplayConfig,
}
impl Plugin for ReplayPlugin {
    fn build(&self, router: &mut Router) {
        router.insert_shared_resource(self.config.clone());
        router.add_processors(BodyStarted, ensure_request_buffer);
        router.add_processors(
            RequestSectionReceived,
            retain_request_section
                .after(normalize_request_section)
                .run_if(is_original_input),
        );
        router.add_processors(ReplayRequested, provide_buffered_input);
        router.add_processors(RequestFinished, release_request_buffer);
    }
}

// The HTTP driver owns sequencing. Neither stages nor their labels decide when
// to run. The same function emits the same moments for initial and replay input.
struct HttpDriver;
impl HttpDriver {
    async fn process(
        &mut self,
        connection: &mut Connection,
        mut input: RequestInput,
    ) -> Result<()> {
        loop {
            // Fresh attempt-owned state is application data, not a scheduler rewind.
            connection.resources.insert(AttemptState::new());
            match self.process_once(connection, &mut input).await {
                Ok(()) => {
                    connection.run_stage(RequestFinished).await?;
                    return Ok(());
                }
                Err(failure) if failure.is_provider_failure() => {
                    connection.resources.insert(failure);
                    connection.run_stage(ProviderFailed).await?;
                    if !connection.resources.get::<FallbackDecision>()?.allowed() {
                        connection.run_stage(RequestFinished).await?;
                        return Err(connection.take_failure());
                    }
                    // ReplayPlugin supplies a new reader of retained input.
                    // The next loop calls exactly the same processing function.
                    connection.run_stage(ReplayRequested).await?;
                    input = connection
                        .resources
                        .take::<ReplayInput>()?
                        .into_request_input();
                }
                Err(failure) => {
                    // Authorization, parsing, or persistence errors are not
                    // automatically eligible for provider fallback.
                    connection.resources.insert(failure);
                    connection.run_stage(RequestFinished).await?;
                    return Err(connection.take_failure());
                }
            }
        }
    }

    async fn process_once(
        &mut self,
        connection: &mut Connection,
        input: &mut RequestInput,
    ) -> Result<()> {
        connection.resources.insert(input.header().await?);
        connection.run_stage(HeaderReceived).await?;
        connection.run_stage(BodyStarted).await?;

        // Parse and retain the settings needed to determine provider eligibility.
        connection.resources.insert(input.requirements().await?);
        connection.run_stage(AttemptStarted).await?;

        while let Some(section) = input.next_section().await? {
            connection.resources.insert(section);
            connection.run_stage(RequestSectionReceived).await?;
            connection.run_stage(AttemptInputReceived).await?;
        }
        connection.finish_provider_input().await?;

        connection
            .resources
            .insert(connection.read_provider_header().await?);
        connection.run_stage(ProviderHeaderReceived).await?;
        while let Some(event) = connection.next_provider_event().await? {
            connection.resources.insert(event);
            connection.run_stage(ProviderEventReceived).await?;
        }
        connection.run_stage(ResponseFinished).await
    }
}

// ReplayConfig shape; values are supplied by configuration, not decided here.
#[derive(Clone)]
struct ReplayConfig {
    memory_limit: usize,
    total_limit: usize,
    spill_directory: std::path::PathBuf,
}

// Replay flow:
// initial input -> explicit HTTP driver stage calls -> retained request input
// buffered input -> the SAME explicit HTTP driver stage calls -> fallback provider
//
// Retain provider-independent headers/settings/sections, not provider A's bytes.
// On replay, run provider-specific mapping again for the new selected provider.
// Repeated stage calls do not restore Resources or Local state automatically.
// Attempt setup resets its own parser/mapping state explicitly where required.
// Request identity, authorization result, history, and persistence identities survive.
// Conditions/idempotency prevent duplicate authorization effects and message writes.
//
// Concurrency contract:
// - chain: completion/success dependency, not just invocation order.
// - pipe: forwards one processor's successful owned output into the next.
// - after: dependency within the SAME stage invocation.
// - run_if: explicit state gate, including failed/skipped dependency behavior.
// - compatible processors may overlap; shared mutation needs access coordination.
// - event sequence IDs preserve wire ordering even when transformations overlap.
// - bounded queues apply backpressure to provider, caller, and persistence work.
//
// Plugin/driver policies still to settle:
// - fallback failures, attempt limits, time budget, and replay retention limits.
// - commitment point after which client response replacement is forbidden.
// - buffers can spill beyond memory_limit, but total retention remains bounded.
// - live input capture while a fallback reader catches up is owned by ReplayPlugin.
// - missing replay capability means consumed input cannot simply be read again.
// - late failed-attempt events, side effects, persistence, cancellation, and charging.
