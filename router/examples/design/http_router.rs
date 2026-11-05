//! Expected HTTP Router usage for review; NOT a compiling/runnable example.
//!
//! Existing SEC shape: App, Plugin, add_processors, async_processor, Shared,
//! Owned, Res/ResMut, ContextWriter, dependency ordering, run_stage_async.
//! Proposed Router APIs: installing HttpDriver as a shared resource, HTTP input /
//! exchange helpers, provider clients, persistence, and replay storage.
//! Nested under examples/design so Cargo does not discover it as a build target.
//! This file does not implement provider, authorization, replay, or HTTP behavior.

use compallm_core::model::Model;
use router::{prelude::*, resources::RouterModel};
use sec::prelude::*;
use std::{io, num::NonZeroUsize, sync::Arc};
use tokio::net::TcpStream;

fn create_router(
    model: Model,
    authorization: Arc<AuthorizationService>,
    providers: Arc<ProviderRegistry>,
    messages: Arc<MessageStore>,
    replay: Arc<ReplayStore>,
) -> Router {
    let mut router = Router::new(model);
    router.set_max_parallelism(NonZeroUsize::new(4).unwrap());
    router.add_plugins((
        HttpPlugin,
        AuthorizationPlugin {
            service: authorization,
        },
        ProviderPlugin {
            registry: providers,
        },
    ));
    // Optional capabilities: omit these registrations to disable them for new executions.
    router.add_plugins((
        ReplayPlugin { store: replay },
        PersistencePlugin { store: messages },
    ));
    router
}

struct HttpPlugin;
impl Plugin for HttpPlugin {
    fn build(&self, app: &mut App) {
        // Proposed dispatch integration: Router's connection task obtains this
        // concrete driver resource and calls drive(stream, execution).
        app.insert_shared_resource(Arc::new(HttpDriver));
        app.add_processors(BodyStarted, initialize_attempt_parser);
        app.add_processors(RequestSectionReceived, normalize_request_section);
        app.add_processors(RequestEnded, finish_request_encoding);
        app.add_processors(RequestFinished, release_request_state);
    }
}

struct AuthorizationPlugin {
    service: Arc<AuthorizationService>,
}
impl Plugin for AuthorizationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_shared_resource(self.service.clone());
        app.add_processors(
            HeaderReceived,
            (
                prepare_access_request,
                async_processor(authorize_request)
                    .after(prepare_access_request)
                    .run_if(needs_authorization),
                require_authorized.after(authorize_request),
            ),
        );
    }
}

fn prepare_access_request(
    header: Res<RequestHeader>,
    model: Res<RouterModel>,
    mut writer: ContextWriter,
) -> io::Result<()> {
    writer.insert_resource(AccessRequest::from_header(&header, model.0.id)?);
    Ok(())
}

async fn authorize_request(
    request: Owned<AccessRequest>,
    service: Shared<AuthorizationService>,
    mut writer: ContextWriter,
) -> io::Result<()> {
    // Owned input and a shared service handle: no hecs borrow crosses await.
    let authorization = service
        .authenticate_and_authorize(request.into_inner())
        .await?;
    writer.insert_resource(authorization);
    Ok(())
}

struct ProviderPlugin {
    registry: Arc<ProviderRegistry>,
}
impl Plugin for ProviderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_shared_resource(self.registry.clone());
        // Consult shared readiness; don't probe every upstream on every request.
        app.add_processors(BodyStarted, select_provider);
        app.add_processors(
            RequestSettingsReceived,
            (
                validate_provider_capabilities,
                map_reasoning_effort,
                prepare_provider_request,
                async_processor(open_provider_request),
            )
                .chain(),
        );
        app.add_processors(
            AttemptInputReceived,
            (map_request_section, async_processor(write_provider_section)).chain(),
        );
        app.add_processors(
            ProviderHeaderReceived,
            (
                validate_provider_status,
                map_response_headers,
                async_processor(write_client_headers),
            )
                .chain(),
        );
        app.add_processors(
            ProviderEventReceived,
            (
                normalize_provider_event,
                map_client_event.after(normalize_provider_event),
                async_processor(write_client_event).after(map_client_event),
            ),
        );
        app.add_processors(
            ProviderFailed,
            (stop_failed_attempt, evaluate_fallback).chain(),
        );
    }
}

fn select_provider(
    providers: Shared<ProviderRegistry>,
    history: Res<AttemptHistory>,
    model: Res<RouterModel>,
    mut writer: ContextWriter,
) -> io::Result<()> {
    let instance = providers.select_ready(model.0.id, &history)?;
    writer.insert_resource(SelectedProvider(instance));
    Ok(())
}

struct ReplayPlugin {
    store: Arc<ReplayStore>,
}
impl Plugin for ReplayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_shared_resource(self.store.clone());
        app.add_processors(BodyStarted, ensure_request_capture);
        app.add_processors(ReplayRequested, async_processor(open_replay_input));
        app.add_processors(RequestFinished, async_processor(release_replay_input));
        // Capture belongs to the input adapter: it retains original decoded input
        // before mutations, including header, settings, sections, and unread tail.
        // Replayed input is never captured as another copy of the request.
    }
}

struct PersistencePlugin {
    store: Arc<MessageStore>,
}
impl Plugin for PersistencePlugin {
    fn build(&self, app: &mut App) {
        app.insert_shared_resource(self.store.clone());
        app.add_processors(
            RequestSectionReceived,
            async_processor(save_user_message)
                .after(normalize_request_section)
                .run_if(is_new_complete_user_message),
        );
        app.add_processors(
            ProviderEventReceived,
            collect_response.after(normalize_provider_event),
        );
        app.add_processors(ResponseFinished, async_processor(save_response));
    }
}

// The driver explicitly triggers every moment. SEC has no HTTP lifecycle,
// automatic transition, rewind operation, or special replay scheduling primitive.
struct HttpDriver;
impl HttpDriver {
    async fn drive(&self, stream: TcpStream, mut execution: Execution) -> Result<(), HttpFailure> {
        // Proposed input adapter: live source or capture-backed source when enabled.
        // The adapter parses protocol boundaries incrementally with bounded queues.
        let mut input = HttpInput::from_client(stream, &execution).await?;
        execution.resources_mut().insert(RequestIdentity::new());
        execution.resources_mut().insert(AttemptHistory::default());

        loop {
            execution.resources_mut().insert(AttemptState::new());
            let outcome = self.process_once(&mut execution, &mut input).await;
            match outcome {
                Ok(()) => {
                    execution.run_stage_async(RequestFinished).await?;
                    return Ok(());
                }
                Err(failure) if failure.is_provider_failure() => {
                    execution.resources_mut().insert(failure);
                    execution.run_stage_async(ProviderFailed).await?;
                    let allowed = execution.resources().get::<FallbackDecision>()?.allowed();
                    if !allowed {
                        execution.run_stage_async(RequestFinished).await?;
                        return Err(HttpFailure::fallback_exhausted());
                    }
                    // ReplayPlugin replaces ReplayInput with a fresh reader.
                    // This is a resource operation; it doesn't rewind execution.
                    execution.run_stage_async(ReplayRequested).await?;
                    let replay = { execution.resources().get::<ReplayInput>()?.clone() };
                    input = replay.open_reader().await?;
                    // Explicitly call the SAME sequence again with different input.
                }
                Err(failure) => {
                    // Rejected credentials, malformed input, and persistence errors
                    // do not automatically qualify for provider fallback.
                    execution.resources_mut().insert(failure);
                    execution.run_stage_async(RequestFinished).await?;
                    return Err(HttpFailure::request_failed());
                }
            }
        }
    }

    async fn process_once(
        &self,
        execution: &mut Execution,
        input: &mut HttpInput,
    ) -> Result<(), HttpFailure> {
        let header = input.read_header().await?;
        execution.resources_mut().insert(header);
        execution.run_stage_async(HeaderReceived).await?;
        execution.run_stage_async(BodyStarted).await?;

        let settings = input.read_required_settings().await?;
        execution.resources_mut().insert(settings);
        execution.run_stage_async(RequestSettingsReceived).await?;

        // Proposed concrete transport helper: bounded readiness events from the
        // request parser and provider reader. An early provider rejection can
        // arrive while client input is still being received/uploaded.
        let attempt = { execution.resources().get::<ProviderAttempt>()?.clone() };
        // Exchange owns transport handles; it must not borrow Execution while
        // the driver inserts event resources and runs processors.
        let mut exchange = HttpExchange::start(input, attempt).await?;
        while let Some(event) = exchange.next_event().await? {
            match event {
                HttpEvent::RequestSection(section) => {
                    execution.resources_mut().insert(section);
                    execution.run_stage_async(RequestSectionReceived).await?;
                    execution
                        .run_stage_async(AttemptInputReceived)
                        .await
                        .map_err(HttpFailure::classify_provider_stage)?;
                }
                HttpEvent::RequestEnded => {
                    execution.run_stage_async(RequestEnded).await?;
                }
                HttpEvent::ProviderHeader { attempt, header } => {
                    if !is_current_attempt(execution, attempt)? {
                        continue;
                    }
                    execution.resources_mut().insert(header);
                    execution
                        .run_stage_async(ProviderHeaderReceived)
                        .await
                        .map_err(HttpFailure::classify_provider_stage)?;
                }
                HttpEvent::ProviderEvent { attempt, event } => {
                    if !is_current_attempt(execution, attempt)? {
                        continue;
                    }
                    // A complete event, not an arbitrary TCP/HTTP body chunk.
                    execution.resources_mut().insert(event);
                    execution
                        .run_stage_async(ProviderEventReceived)
                        .await
                        .map_err(HttpFailure::classify_provider_stage)?;
                }
                HttpEvent::ProviderFailed { attempt, error } => {
                    if !is_current_attempt(execution, attempt)? {
                        continue;
                    }
                    return Err(HttpFailure::provider(error));
                }
                HttpEvent::ResponseEnded { attempt } => {
                    if !is_current_attempt(execution, attempt)? {
                        continue;
                    }
                    execution.run_stage_async(ResponseFinished).await?;
                    return Ok(());
                }
                HttpEvent::ClientDisconnected => return Err(HttpFailure::cancelled()),
            }
        }
        Err(HttpFailure::incomplete_response())
    }
}

// Example async mapping output writer. The ordered bounded writer owns the socket.
async fn write_provider_section(
    section: Owned<EncodedSection>,
    attempt: Owned<ProviderAttempt>,
) -> io::Result<()> {
    attempt.writer.write(section.into_inner()).await
}

// Policies still to settle before implementing transport/plugins:
// - A protocol may need the complete request before it can start inference;
//   read_required_settings/start must honor that, without unlimited RAM buffering.
// - Input capture survives failed attempts, can spill to disk, and enforces limits.
// - Fallback requires replay capability, an eligible failure, remaining attempt /
//   time budget, and no client response commitment that forbids replacement.
// - Stage-error classification must distinguish upstream failures from caller
//   disconnects, invalid mapping, and persistence failures; stage name alone
//   cannot make every error eligible for fallback.
// - Stable request/message IDs prevent duplicate persistence across invocations.
// - Authorization reuse/revalidation follows an explicit API-key lifecycle policy.
// - A fresh attempt resets provider parser/mapping state explicitly; Local values
//   and Resources are not restored merely because a stage runs again.
// - Await one stage before replacing its event Resources. Parallel processors
//   within that invocation still obey access/dependency metadata and wave limits.
// - Stop a failed exchange before starting the next; ignore its late attempt IDs.
// - Cancellation/failure must settle persistence and finalization deliberately.
// - Actual async SEC-stage cancellation invalidates it: drop the Execution;
//   ordinary provider failure returned normally can still trigger another stage.
