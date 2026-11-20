//! Provider catalog migration plan.
//!
//! The registered implementation below applies the catalog snapshot and schema changes.
//! The plan records the scope; imported facts remain distinguished from verified fields.
//! Keep product documentation in the external Compallm Wiki; these notes guide this migration.
//!
//! Work: https://github.com/Midwess/compallm/issues/176
//! Parent: https://github.com/Midwess/compallm/issues/175
//!
//! ## Goal and current evidence
//!
//! Prepare provider/model defaults for authenticated discovery, search, and the existing UI.
//! The local catalog audit found 143 providers and 1,273 models. Of 511 models declaring
//! supported text operations, 346 fail current model-name validation, 356 lack context limits,
//! and 323 lack at least one required price. Only 25 distinct models across 12 providers pass
//! the generated configuration validator unchanged. Validation does not verify vendor facts.
//! There are 87 providers without selectable text models, 24 without model rows, 10 without
//! base URLs, and 137 without icon keys. The restricted database role cannot read templates.
//! These counts are an audit baseline; recompute them during implementation.
//!
//! ## Public model-info API enrichment
//!
//! public-api-enrichment.json records model-list/detail probes, catalog pagination, and the
//! additional endpoint inspections. Existing catalog identities remain stable. Venice type=all
//! supplies descriptions, capabilities, media billing units, and embedding input/vector limits.
//! Featherless pagination is inspected; all existing target IDs are on the first 1,000-row page,
//! so unrelated pages and models are not imported. OpenRouter limits become defaults only when
//! every listed endpoint publishes the same positive cap; route differences remain conditions.
//! Official OpenCode harness documentation supplies gateway prefixes and translated text APIs
//! for paid/Go models, plus a stable-session requirement for Go and client restrictions on free
//! chat. Historical IDs not in the current source retain an explicit availability warning.
//! Model-info APIs provide actual data when present. Authentication errors, 404/405 detail
//! routes, missing cache prices, and service-specific billing never become invented defaults.
//!
//! ## Public-source resolution review
//!
//! The user confirmed that no provider keys are available. Continue with public sources only;
//! do not use ambient platform credentials or infer account-specific contracts. The review
//! checks provider catalogs, individual endpoint records, official billing pages, embedded
//! public-page model records, and lifecycle schedules. resolution-ledger.json records each
//! remaining model/field pair with source attempts; absence of a published value is not zero.
//! document-research.json preserves document response hashes and exact parsed pricing labels.
//! OpenRouter endpoint rates may vary by provider, and a catalog without endpoints cannot
//! establish a usable rate. Official deprecation notices remove only verified retired IDs
//! from current text selections while retaining their historical records. Imported context
//! labels such as 1M/262K are not expanded into unverified exact limits. New public-source
//! statuses include unpublished, no_public_endpoint, migration_required, credit_based, and
//! deprecated. None of these labels makes an unknown numeric field resolved.
//!
//! ## Model-information retrieval metadata
//!
//! model_info_sources on both catalog tables records how to obtain further information,
//! separately from source_refs (provenance) and field_metadata (missing-value explanations).
//! Each source is api, documentation, repository, or manual, with access, evidence status,
//! topic/field coverage, and notes. API sources record GET and credential type without secrets.
//! Observed responses include JSON Pointer selection, exact model identity, field locations,
//! units, and known page pagination. Credential challenges retain an unknown response schema
//! and empty field coverage until inspected. A missing exact model may retain an identity-only
//! list lookup without asserting that the API supplies its specifications or prices.
//! model_info_constraints.sql installs immutable PostgreSQL checks for nested structure,
//! enumerations, method/auth consistency, URL restrictions, units, pointer syntax, mapping
//! coverage, and pagination. Typed Rust decoding checks the same contract before seeding.
//! Source arrays must contain 1–32 records. A source is descriptive metadata, never executable
//! SQL/JavaScript or automatic refresh behavior. Network admission and refresh scheduling
//! remain future runtime work. External registries contribute data/output references while
//! Compallm owns this metadata shape. model-info-sources-report.json records coverage.
//!
//! ## Zero-price evidence and authentication alternatives
//!
//! zero-price-audit.json records 260 unverified scalar zeros cleared across 108 models.
//! Original claims and URLs remain in field_metadata.zero_price_review; absent tariffs are
//! null. The 117 retained zero fields require zero_price_evidence with a verified source
//! covering that exact field and a nonblank explanation. Provenance does not prove that a
//! published tariff remains current. Non-scalar route/tier/schedule rates retain their own
//! existing evidence contracts and are outside this scalar audit.
//!
//! Authentication readiness is evaluated per method. One usable method must cover all model
//! operations without unimplemented adapter hooks; unknown optional methods do not block it.
//! provider_catalog_auth_readiness exposes each alternative independently. The aggregate view
//! chooses the best complete method and retains model/session requirements as blockers.
//!
//! ## Estimated price fallbacks
//!
//! User-authorized cross-provider estimates live in pricing_config.fallback as USD per
//! million tokens, with reference provider/model, source URLs, confidence, basis, and notes.
//! Provider-specific values and validated fetched values take precedence. Fallbacks preserve
//! exact-source recipes and unresolved requirements; they do not establish hosted availability,
//! cache support, provider billing, or complete readiness. Unknown cache cost may use the full
//! input rate as an estimate. Free/subscription and conditional route/tier/schedule tariffs keep
//! their existing billing contracts. Explicit zero estimates need matching fallback evidence.
//! gemini-fallback-research.json records the batch inventory and received research.
//! materialized-defaults-report.json records estimated defaults copied into formerly null
//! scalar columns, including matching-model limit estimates and attributed zero-price claims.
//! Exact-provider acquisition and unresolved facts remain distinct from populated estimates.
//!
//! ## Whole-catalog enrichment
//!
//! Research includes text models without selectable operations. Volcengine Ark uses the
//! configured Coding Plan product: missing specifications and estimated capability defaults
//! are researched independently of its subscription quota. pricing_config.cost_proxy retains
//! estimated USD/million-token equivalents without changing actual subscription billing.
//! Whole-catalog research preserves exact identities, source URLs, raw Gemini answers,
//! conditional provenance, unsupported transports, and existing nonnull values.
//! whole-catalog-research.json records results and distinguishes scalar defaults, cost proxies,
//! intentional billing nulls, and unresolved facts. A populated estimate is not proof of
//! account access, current hosted availability, or a verified deployment limit.
//!
//! ## Reviewed catalog removals
//!
//! Remove 39 reviewed identities with no literal/naming-variant mention in the pinned
//! 9Router/LiteLLM checkouts and missing applicable context, billing, pricing, or cache data.
//! Complete estimated defaults and references in either checkout protect records from this
//! removal set. Intentional subscription/media nulls are not treated as missing token tariffs.
//! removed-unreferenced-incomplete-models.json retains full removed rows, source revisions,
//! and per-model reasons. The final snapshot has 145 providers and 2,348 model records.
//! A further explicit user instruction removes 29 listed records with nonscalar prices:
//! 11 free, nine tiered, and nine audio/video tariffs. These rows have pricing information;
//! their removal is a catalog-scope decision, not a finding of missing or invalid prices.
//! removed-unreferenced-nonscalar-price-models.json retains their full rows and reasons.
//! The reversible migration preserves its original baseline and concurrent-edit protection.
//!
//! ## Applicable requirements and derived readiness
//!
//! catalog_semantics.sql separates provider auth methods/configuration and endpoint mode
//! from each model's service, billing model/unit, pricing mode/configuration, cache billing,
//! specification mode, field resolution plan, capabilities, and unresolved requirements.
//! Native enum columns and immutable CHECK functions reject inconsistent combinations.
//! No rule infers billing from OAuth or an API key. Subscription entitlement requires an
//! identified product and either complete plan/fee/quota/period/overage facts or named
//! account inputs. Cache unsupported and standard-input billing need exact source evidence.
//! Context tiers are contiguous nonoverlapping ranges from zero through an unbounded end;
//! each tier carries applicable rates. Unknown conditional pricing remains incomplete.
//!
//! Readiness is generated from the validated provider/model requirements. The read-only
//! provider_catalog_readiness view combines both rows, supported authentication placement,
//! and session-header requirements. Missing declarations remain incomplete; named account
//! requirements become needs_configuration; complete observed API recipes become needs_fetch.
//! Neither a source URL nor a reason alone makes a model ready. Readiness describes catalog
//! requirements, not account access, current credentials, or an implemented runtime fetcher.
//! Current text execution retains known transport, client, and catalog-presence blockers.
//!
//! Prices use unconstrained numeric storage with finite/nonnegative bounds matching Rust:
//! fewer than 10^12 USD/million, at most 18 decimal places, and at most 28 total digits.
//! Shared generated input validation preserves these bounds; no price is rounded on insert.
//! Four previously unrepresentable published prices are restored exactly. Original numeric
//! typmods and capability arrays are restored only after original rows during rollback.
//! semantics-backfill-report.json records the current classifications and effective readiness.
//!
//! ## Additional public catalogs
//!
//! three-catalog-investigation.json records LLM Gateway's public model API, Artificial
//! Analysis's public page datasets, and the LiteLLM model website's price-file revision.
//! LiteLLM 983c7ef20d73954ca150434d347c77c20f4d1a50 adds exact same-provider input/output
//! caps and explicit capability flags. Input caps remain separate from context. LLM Gateway
//! f49b08cb1867e7599fbbdcb37ea7bdc22d60caab confirms Alibaba's default Singapore endpoint;
//! its international Baidu endpoint is not the China Qianfan product in this catalog.
//! Gateway absent-price defaults serialize as zero and never establish free cache billing.
//! Standard Kimi/GLM token APIs do not supply coding-plan tariffs. Historical/deactivated
//! mappings do not establish current defaults. Context-tier prices retain their conditions.
//! Artificial Analysis provider-host records have explicit null cache prices in the reviewed
//! incomplete matches; median prices, rounded caps, and quantization variants are not copied.
//! Multi-provider API records remain review references until a provider selector can be
//! expressed without fragile array indices. This pass fills 30 numeric defaults and adds
//! 45 explicit capability records. Applicable requirements now determine readiness.
//!
//! ## Exact OpenRouter variant and route review
//!
//! openrouter-enrichment.json records the live model-list and exact-ID endpoint responses.
//! Model links.details may resolve a :free or :batch identifier to its paid canonical model;
//! endpoint pricing is used only when response data.id equals the requested upstream ID.
//! Explicit zero input/output rates on exact free IDs establish free token billing with
//! account limits and optional paid features kept separate. A false supports_implicit_caching
//! flag describes automatic caching only; it never establishes absence of explicit caching.
//! Cache-read omissions remain unknown. Context overrides become complete token intervals.
//! Fully priced route options require selecting a matching endpoint slug with fallbacks off;
//! per_route cache billing and typed allowed_values reject incomplete or inconsistent choices.
//! Four current synchronous text IDs are added. Batch-only variants remain research candidates,
//! without inventing synchronous endpoints. Historical IDs absent from the current catalog
//! retain provenance and explicit availability blockers rather than guessed successor aliases.
//!
//! ## Complete SiliconFlow public website collection
//!
//! siliconflow-enrichment.json records all 147 records in the public CMS chunk consumed
//! by the pricing page's five-row Load More lists. The decoder verifies 3,431,469 bytes
//! consumed through the final record. Exact api_model_name/api_endpoint keys produce 144
//! unique IDs; three duplicate CMS records retain their identity/status evidence. All IDs
//! are imported, including deprecated and non-text services without selectable text paths.
//! Actual stored kv_cache=false resolves unsupported caching; a blank price alone does not.
//! Public form_schema maxima supply exact output caps. Rounded K context labels remain
//! labeled source facts rather than invented exact context limits. Native rates preserve
//! per-image, per-video, input-token, and per-million UTF-8-byte units. UTC peak/off-peak
//! rates become validated daily schedules. MiniMax's ambiguous 512k boundary remains an
//! explicit requirement until its exact count is confirmed. API credential values, SEO
//! content, benchmark tables, and executable browser code are not copied into defaults.
//!
//! ## User-supplied Gemini research suggestions
//!
//! nim-gemini-suggestions.json preserves the user's 19 Gemini suggestions, including
//! rounded/ranged context labels and approximate or provider-variable price claims.
//! Matching NIM rows store them under field_metadata.gemini_suggestion and name a manual
//! review source. They are unverified suggestions, not NVIDIA-hosted prices or exact caps.
//! No authoritative model value or derived readiness is changed by recording suggestions.
//!
//! ## Sources and provenance
//!
//! Start with the 9Router repository for provider identities, model definitions, adapters,
//! endpoint construction, authentication, pricing, capabilities, effort variants, and artwork.
//! The initial source is https://github.com/decolua/9router at commit
//! 1855aa44473e88feac9d3f131b9c0c5fc9706b68. Supplemental exact-match metadata comes from
//! https://github.com/anomalyco/models.dev at 5c40e591a65246bcbf2e6a890d90516086ba72de.
//! LiteLLM adds exact-match prices and separate input/output limits at
//! https://github.com/BerriAI/litellm commit 60d05972aeeadc3fb21476f586ce3f4747887c9d.
//! litellm-comparison.json records candidate providers, source conflicts, and filled fields.
//! Its gateway endpoint table describes adapter support, not proof of a vendor-native protocol.
//! Verify and complete imported facts using official provider API/model/pricing documentation.
//! Official documentation takes precedence when sources conflict. Keep unknown facts explicit.
//! Import provenance alone does not establish that a field has been verified.
//!
//! Add structured source_refs to provider/model templates. Each reference records source kind
//! (repository or web), exact URL, repository revision where applicable, fields supplied or
//! verified, and verification revision. Use concrete references rather than "internet".
//! References contain kind, url, revision, fields, status, and an optional explanatory note.
//! Status is imported, estimate, or verified; imported facts are never promoted implicitly.
//! Record pricing conditions when
//! a single published rate does not cover all usage. Never invent universal prices or limits.
//!
//! ## Planned database and data changes
//!
//! | Action | Target | Change | Source |
//! | --- | --- | --- | --- |
//! | Rename/backfill | provider_templates.icon_key | logo_path: root-relative public asset path | 9Router brand mapping; downloaded authentic artwork |
//! | Add | Both template tables | Structured source_refs with field-level provenance | Pinned 9Router files; official documentation |
//! | Add | Both template tables | field_metadata explains unavailable and conditional values | Exact official catalogs, plan documentation, and recorded source failures |
//! | Rename | provider_model_templates.name | upstream_model_id, preserving the exact request identifier | 9Router definitions; official model catalogs |
//! | Add | Model templates | Optional display_name, falling back to upstream_model_id | 9Router labels; official model names |
//! | Add | Model templates | Separate nullable positive max_input_tokens and max_output_tokens | Exact models.dev/LiteLLM metadata; never infer total context |
//! | Add | Provider templates | Typed authentication defaults and supported methods, including operation differences | 9Router adapters; official authentication docs |
//! | Add | Provider templates | Fixed versus account/deployment-specific endpoint requirements | 9Router URL construction; official deployment docs |
//! | Correct | URLs, paths, protocols, headers | Verified defaults retaining base-path prefixes | 9Router adapters; official API references |
//! | Complete | Context limits and three prices | Explicit verified values; unknown remains null | 9Router metadata; official specifications/pricing |
//! | Correct | Capabilities and effort mappings | Declared supported features and accepted values | 9Router metadata/adapters; official request docs |
//! | Improve | Provider/model descriptions | Useful descriptions and configuration requirements | 9Router descriptions; official provider/model pages |
//! | Insert | Missing provider/model coverage | Reviewed supported entries absent from the snapshot | Reviewed 9Router snapshot; official catalogs |
//! | Merge/delete | Duplicate or obsolete catalog rows | Consolidate settings variants only after verifying meaning | 9Router variant logic; official identifiers/deprecations |
//! | Grant | Global template reads | Read access through the intended authenticated database path | Compallm authorization and existing restricted-role behavior |
//! | Keep | Existing schema | Stable catalog IDs, relationships, nullable defaults, exact numeric prices | Existing Compallm contracts |
//!
//! Preserve the composite model catalog identity (provider_template_id, id). Distinguish it
//! from the upstream request identifier and display label. Do not sanitize vendor identifiers
//! into different request values. Review DeepSeek's three deepseek-v4-pro variants, which
//! collide under current configured-model name uniqueness, while retaining real distinctions.
//!
//! Fifty model templates declare multiple text operations. Model operations, not provider
//! family hints, determine compatibility. Gemini and OpenRouter need specific coverage review;
//! an empty operation list must not become an implied supported text endpoint. Retain unsupported
//! providers with accurate limitations instead of deleting them solely because the UI lacks support.
//! Verify OpenAI, Anthropic, DeepSeek, Groq, and Mistral first, then complete the remaining catalog.
//! Missing context, prices, authentication, or endpoint facts must never receive guessed defaults.
//! Explicit zero means a verified free price; unsupported cache pricing needs separate semantics
//! from an unknown price and must not silently become zero. Catalog rates remain editable defaults.
//!
//! ## Logos and public assets
//!
//! Download authentic artwork for every catalog provider to portal/public/assets/providers/.
//! Prefer self-contained SVG; use suitable PNG/WebP when necessary. Brand aliases may share files.
//! Start with available 9Router assets, then official media kits/sites and established collections
//! such as LobeHub. Review the artwork, retain attribution/license notices, and inspect SVGs for
//! executable or external content. Do not fabricate logos or mark generic placeholders complete.
//! Keep unresolved artwork explicitly incomplete until sourced.
//!
//! Store paths such as /assets/providers/openai.svg, without an origin or filesystem prefix.
//! Consumers resolve new URL(logoPath, origin); assets must work in development and production.
//! Backfill every existing provider and verify referenced files before requiring logo_path.
//! Keep a companion asset manifest with exact source URLs/revisions, checksums, attribution,
//! and provider mappings. Ensure shared assets render legibly in light and dark themes and are
//! served in offline Storybook previews. Replace frontend brand-name guessing with supplied paths.
//! File downloads and manifest updates happen during development, outside migration execution.
//!
//! ## Companion implementation outside this migration
//!
//! - Update SeaORM models/conversions and the Rust input contract for legitimate bounded upstream
//!   identifiers, including punctuation. Preserve control-character restrictions and regenerate
//!   Genql/Zod contracts. Distinguish model identifiers from narrower effort-value rules.
//! - Expose flat GraphQL catalog queries and shared Payload/Collection outputs with description,
//!   logo path, defaults, supported operations, and missing configuration requirements.
//! - Derive readiness from authoritative metadata and validation; avoid a duplicated stored ready flag.
//! - Support case-insensitive partial provider-name/model-identifier search, operation filters,
//!   deterministic server pagination, and selected-provider model loading. Measure before adding
//!   search indexes; existing primary keys support identity and parent lookup.
//! - Authenticated organization members may browse global templates. Enforce Cedar/RLS for saved
//!   organization resources. Reuse existing database scopes; context/connection infrastructure
//!   changes require separate explicit permission. No authority comes from a catalog ID.
//! - Keep SeaORM queries in PostgreSQL and direct read-resolver projections. Preserve cancellation,
//!   request tracking, shared collection rendering, drafts, scroll, and unchanged record identities.
//! - Template selection copies editable defaults and creates nothing. Only explicitly enabled,
//!   valid configured models enter creation; catalog updates never overwrite saved connections.
//! - API keys retain authenticated encryption with fresh random nonces and a dedicated server
//!   environment secret. Rotation is separate. Custom headers are stored as supplied without
//!   special secret-header support; keep draft values out of URLs and all credentials out of logs.
//! - Optional explicit connection tests may incur charges and never gate saving. Confirm provider
//!   record deletion with the user; router processing and in-flight requests remain outside scope.
//!   Creation, editing, cloning, testing, and removal runtime work belongs to #177/#178.
//!
//! ## Local migration workflow and rollback
//!
//! The user authorized repeated local up/down cycles for this new migration while developing it.
//! Existing migrations, including the original template seeds, remain unchanged. Edit this new
//! migration only after rolling back its applied version. Check migration history first and reverse
//! only this migration; do not roll back unrelated migrations or reset the application database.
//! Once merged or shared for deployment, freeze it and make later corrections in new migrations.
//!
//! Embed explicit reviewed rows and reversible changes in Rust. Execution must work offline,
//! without fetching 9Router, downloading logos, starting an importer, or publishing the Wiki.
//! up() applies schema changes, catalog corrections, and required access changes atomically.
//! down() restores prior column names, values, deleted rows, constraints, and grants, and removes
//! only this migration's inserts. Preserve later/local edits; detect conflicting state and fail
//! safely rather than overwriting it. Capture the before-values needed for reversible updates.
//! Scope both directions to catalog changes; preserve organization providers, credentials, models,
//! router bindings, usage, and unrelated application data. Do not introduce generic timestamps.
//!
//! Ordered work: pin/research sources; review each provider and source mapping; obtain logos;
//! implement schema and explicit correction rows; implement down(); register; apply locally;
//! inspect catalog and generated-contract compatibility; roll back and verify restoration;
//! refine and repeat; finalize reviewed data and publish relevant Wiki updates with implementation.
//!
//! Completion checks cover source coverage, unique stable identities, legitimate upstream IDs,
//! operation/path/authentication consistency, exact prices, explicit unknowns, supported limits,
//! generated validation, logo file coverage, authenticated reads, and safe rollback/reapply.
//! Use approved tests directories for critical business checks. Storybook uses lint, type checks,
//! builds, and manual inspection without automated interaction runners. A structurally valid row
//! or available logo does not establish verified pricing, adapter support, or a working credential.
//!
//! References:
//! - https://github.com/Midwess/compallm/wiki/Provider-templates
//! - https://github.com/Midwess/compallm/wiki/Provider-contract
//! - https://github.com/Midwess/compallm/wiki/Database-schema-Provider-templates
//! - https://github.com/Midwess/compallm/wiki/Portal-validation
//!
//! ## Implemented snapshot and remaining limits
//!
//! after.rs embeds 145 providers and 2,416 models; before.rs retains the exact prior catalog.
//! Every provider references downloaded public artwork with a source/checksum manifest. Six
//! local/self-hosted software entries use sourced service artwork rather than an invented company logo.
//! No unsupported adapter is implemented by this migration. Unknown configuration values stay
//! null; official DeepSeek peak-price conditions are recorded in the corresponding source note.
//! scripts/verify-provider-catalog.mjs checks assets and generated configuration compatibility.
//! Repository and official web sources are recorded per field; most catalog facts remain imported,
//! so this snapshot does not claim universal vendor verification or successful upstream requests.
//! Runtime discovery/paging and configured-provider loading remain the rest of #176.
//! Additional LiteLLM provider candidates require identity, logo, endpoint, and adapter review;
//! they are recorded separately from the seeded providers. Existing verified values retain
//! precedence over imported LiteLLM estimates. Deprecation metadata removes affected entries
//! from selectable text configuration while retaining catalog identities and provenance.
//! investigation.json records official public-catalog requests and provider alias mappings.
//! Exact model metadata was recovered for Alibaba coding/token plans, Xiaomi Singapore token
//! plans, Volcengine Coding Plan, GLM coding plans, Kimi Code, Fireworks, and Together. Region
//! and product identities remain distinct. Subscription-included zero prices are not free token
//! rates: those values remain null with subscription_based metadata. Featherless model details
//! supply limits/prices beyond its paginated list. Exact model pages supply discounted cache
//! rates omitted by its API; API field absence never proves standard-input or free billing. BazaarLink uses the
//! documented api.bazaarlink.ai host. Public catalogs execute only during research; migration
//! execution remains offline. field_metadata also records unknown, account_specific, variable,
//! unsupported_precision, standard_input_rate, and conditional values for UI interpretation.

use sea_orm::{ConnectionTrait, prelude::Decimal, sea_query::Query};
use sea_orm_migration::prelude::*;
use serde_json::{Value, json};
use std::str::FromStr;

mod after;
mod before;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261117_000047_provider_catalog"
    }
}

const PROVIDERS: &str = "provider_templates";
const MODELS: &str = "provider_model_templates";
const PROVIDER_FIELDS: &[&str] = &[
    "id",
    "name",
    "description",
    "icon_key",
    "protocol",
    "base_url",
    "headers",
];
const MODEL_FIELDS: &[&str] = &[
    "provider_template_id",
    "id",
    "name",
    "description",
    "protocols",
    "paths",
    "capabilities",
    "max_context_tokens",
    "effort_mapping",
    "input_price",
    "output_price",
    "cached_price",
];
const PROVIDER_ADDITIONS: &[&str] = &[
    "source_refs",
    "auth_methods",
    "auth_config",
    "endpoint_mode",
    "endpoint_config",
    "field_metadata",
    "model_info_sources",
];
const MODEL_ADDITIONS: &[&str] = &[
    "source_refs",
    "display_name",
    "service_kind",
    "billing_model",
    "billing_unit",
    "pricing_mode",
    "pricing_config",
    "cache_billing",
    "specification_mode",
    "resolution_plan",
    "unresolved_fields",
    "max_input_tokens",
    "max_output_tokens",
    "field_metadata",
    "model_info_sources",
];

fn snapshot(data: &str) -> Result<Value, DbErr> {
    serde_json::from_str(data).map_err(|e| DbErr::Custom(e.to_string()))
}

fn fields(table: &str, upgraded: bool) -> Vec<String> {
    let base = if table == PROVIDERS {
        PROVIDER_FIELDS
    } else {
        MODEL_FIELDS
    };
    let mut fields: Vec<String> = base
        .iter()
        .map(|field| {
            match (*field, upgraded, table) {
                ("icon_key", true, _) => "logo_path",
                ("name", true, MODELS) => "upstream_model_id",
                _ => field,
            }
            .to_string()
        })
        .collect();
    if upgraded {
        fields.extend(
            if table == PROVIDERS {
                PROVIDER_ADDITIONS
            } else {
                MODEL_ADDITIONS
            }
            .iter()
            .map(ToString::to_string),
        );
    }
    fields
}

async fn read(manager: &SchemaManager<'_>, upgraded: bool) -> Result<Value, DbErr> {
    let db = manager.get_connection();
    let mut result = json!({});
    for table in [PROVIDERS, MODELS] {
        let fields = fields(table, upgraded);
        let mut query = Query::select();
        for field in &fields {
            if enum_type(field).is_some()
                || matches!(
                    field.as_str(),
                    "input_price" | "output_price" | "cached_price"
                )
            {
                query.expr_as(
                    Expr::col(Alias::new(field)).cast_as(Alias::new("text")),
                    Alias::new(field),
                );
            } else {
                query.column(Alias::new(field));
            }
        }
        query.from(Alias::new(table));
        if table == MODELS {
            query.order_by(Alias::new("provider_template_id"), Order::Asc);
        }
        query.order_by(Alias::new("id"), Order::Asc);
        let rows = db.query_all(&query).await?;
        let mut values = Vec::with_capacity(rows.len());
        for row in rows {
            let mut value = json!({});
            for field in &fields {
                let cell = match field.as_str() {
                    "headers" | "protocols" | "paths" | "capabilities" | "effort_mapping"
                    | "source_refs" | "auth_methods" | "auth_config" | "endpoint_config"
                    | "pricing_config" | "resolution_plan" | "unresolved_fields"
                    | "field_metadata" | "model_info_sources" => row.try_get::<Value>("", field)?,
                    "max_context_tokens" | "max_input_tokens" | "max_output_tokens" => {
                        json!(row.try_get::<Option<i64>>("", field)?)
                    }
                    "input_price" | "output_price" | "cached_price" => {
                        json!(row.try_get::<Option<String>>("", field)?)
                    }
                    _ => json!(row.try_get::<Option<String>>("", field)?),
                };
                value[field] = cell;
            }
            values.push(value);
        }
        result[table] = json!(values);
    }
    Ok(result)
}

async fn check_grants(manager: &SchemaManager<'_>, expected: bool) -> Result<(), DbErr> {
    for table in [PROVIDERS, MODELS] {
        let query = Query::select()
            .expr_as(
                Func::cust(Alias::new("has_table_privilege")).args([
                    Expr::val("compallm_restricted"),
                    Expr::val(table),
                    Expr::val("SELECT"),
                ]),
                Alias::new("granted"),
            )
            .to_owned();
        let row = manager
            .get_connection()
            .query_one(&query)
            .await?
            .ok_or_else(|| DbErr::Custom("catalog grant inspection returned no row".into()))?;
        if row.try_get::<bool>("", "granted")? != expected {
            return Err(DbErr::Custom("catalog SELECT grants differ from this migration's expected state; preserve external privilege changes".into()));
        }
    }
    Ok(())
}

fn normalize(value: &mut Value) {
    for model in value[MODELS].as_array_mut().into_iter().flatten() {
        if let Some(tokens) = model["max_context_tokens"]
            .as_str()
            .and_then(|value| value.parse::<i64>().ok())
        {
            model["max_context_tokens"] = json!(tokens);
        }
    }
    for model in value[MODELS].as_array_mut().into_iter().flatten() {
        for field in ["input_price", "output_price", "cached_price"] {
            if let Some(price) = model[field].as_str() {
                model[field] = json!(Decimal::from_str(price).unwrap().normalize().to_string());
            }
        }
    }
    for table in [PROVIDERS, MODELS] {
        value[table].as_array_mut().unwrap().sort_by_key(|row| {
            (
                row["provider_template_id"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
                row["id"].as_str().unwrap().to_string(),
            )
        });
    }
}

fn cell(field: &str, value: &Value) -> Result<sea_orm::Value, DbErr> {
    Ok(match field {
        "headers" | "protocols" | "paths" | "capabilities" | "effort_mapping" | "source_refs"
        | "auth_methods" | "auth_config" | "endpoint_config" | "pricing_config"
        | "resolution_plan" | "unresolved_fields" | "field_metadata" | "model_info_sources" => {
            value.clone().into()
        }
        "max_context_tokens" | "max_input_tokens" | "max_output_tokens" => value.as_i64().into(),
        "input_price" | "output_price" | "cached_price" => value
            .as_str()
            .map(Decimal::from_str)
            .transpose()
            .map_err(|e| DbErr::Custom(e.to_string()))?
            .into(),
        _ => value.as_str().map(ToString::to_string).into(),
    })
}

fn enum_type(field: &str) -> Option<&'static str> {
    match field {
        "endpoint_mode" => Some("catalog_endpoint_mode"),
        "service_kind" => Some("catalog_service_kind"),
        "billing_model" => Some("catalog_billing_model"),
        "billing_unit" => Some("catalog_billing_unit"),
        "pricing_mode" => Some("catalog_pricing_mode"),
        "cache_billing" => Some("catalog_cache_billing"),
        "specification_mode" => Some("catalog_specification_mode"),
        _ => None,
    }
}

fn cell_expr(field: &str, value: &Value) -> Result<SimpleExpr, DbErr> {
    let expression = Expr::val(cell(field, value)?);
    Ok(if let Some(type_name) = enum_type(field) {
        expression.cast_as(Alias::new(type_name))
    } else {
        expression
    })
}

fn identity(row: &Value) -> (String, String) {
    (
        row["provider_template_id"]
            .as_str()
            .unwrap_or("")
            .to_string(),
        row["id"].as_str().unwrap().to_string(),
    )
}

async fn write(manager: &SchemaManager<'_>, previous: &Value, target: &Value) -> Result<(), DbErr> {
    let db = manager.get_connection();
    // Catalog identity sets are checked before effects. Only explicitly removed template rows are deleted.
    for table in [MODELS, PROVIDERS] {
        for row in previous[table].as_array().unwrap() {
            if !target[table]
                .as_array()
                .unwrap()
                .iter()
                .any(|other| identity(other) == identity(row))
            {
                let mut delete = Query::delete();
                delete
                    .from_table(Alias::new(table))
                    .and_where(Expr::col(Alias::new("id")).eq(row["id"].as_str().unwrap()));
                if table == MODELS {
                    delete.and_where(
                        Expr::col(Alias::new("provider_template_id"))
                            .eq(row["provider_template_id"].as_str().unwrap()),
                    );
                }
                db.execute(&delete).await?;
            }
        }
    }
    for table in [PROVIDERS, MODELS] {
        for row in target[table].as_array().unwrap() {
            let object = row.as_object().unwrap();
            if previous[table]
                .as_array()
                .unwrap()
                .iter()
                .any(|other| identity(other) == identity(row))
            {
                let mut update = Query::update();
                update
                    .table(Alias::new(table))
                    .and_where(Expr::col(Alias::new("id")).eq(row["id"].as_str().unwrap()));
                if table == MODELS {
                    update.and_where(
                        Expr::col(Alias::new("provider_template_id"))
                            .eq(row["provider_template_id"].as_str().unwrap()),
                    );
                }
                for (field, value) in object {
                    update.value(Alias::new(field), cell_expr(field, value)?);
                }
                db.execute(&update).await?;
            } else {
                let mut insert = Query::insert();
                insert
                    .into_table(Alias::new(table))
                    .columns(object.keys().map(Alias::new));
                insert.values_panic(
                    object
                        .iter()
                        .map(|(field, value)| cell_expr(field, value))
                        .collect::<Result<Vec<_>, _>>()?,
                );
                db.execute(&insert).await?;
            }
        }
    }
    Ok(())
}

fn assert_matches(mut actual: Value, mut expected: Value) -> Result<(), DbErr> {
    normalize(&mut actual);
    normalize(&mut expected);
    if actual != expected {
        for table in [PROVIDERS, MODELS] {
            let actual_rows = actual[table].as_array().unwrap();
            let expected_rows = expected[table].as_array().unwrap();
            if actual_rows.len() != expected_rows.len() {
                return Err(DbErr::Custom(format!(
                    "catalog {table} row count differs from reviewed snapshot"
                )));
            }
            for (actual, expected) in actual_rows.iter().zip(expected_rows) {
                if actual != expected {
                    let fields: Vec<_> = expected
                        .as_object()
                        .unwrap()
                        .keys()
                        .filter(|key| actual[*key] != expected[*key])
                        .collect();
                    return Err(DbErr::Custom(format!(
                        "catalog {table} {} differs in fields {fields:?}; preserve concurrent edits before applying or reversing",
                        expected["id"]
                    )));
                }
            }
        }
        return Err(DbErr::Custom("catalog snapshot mismatch".into()));
    }
    Ok(())
}

fn check_model_info_sources(snapshot: &Value) -> Result<(), DbErr> {
    for table in [PROVIDERS, MODELS] {
        for row in snapshot[table].as_array().unwrap() {
            crate::catalog::model_info::ModelInfoSources::try_from(
                row["model_info_sources"].clone(),
            )
            .map_err(|error| {
                DbErr::Custom(format!("{table} {} model-info sources: {error}", row["id"]))
            })?;
        }
    }
    use crate::catalog::semantics;
    for row in snapshot[PROVIDERS].as_array().unwrap() {
        serde_json::from_value::<std::collections::BTreeMap<String, semantics::AuthConfig>>(
            row["auth_config"].clone(),
        )
        .map_err(|e| DbErr::Custom(e.to_string()))?;
        serde_json::from_value::<semantics::EndpointConfig>(row["endpoint_config"].clone())
            .map_err(|e| DbErr::Custom(e.to_string()))?;
    }
    for row in snapshot[MODELS].as_array().unwrap() {
        serde_json::from_value::<semantics::PricingConfig>(row["pricing_config"].clone())
            .map_err(|e| DbErr::Custom(format!("{} pricing: {e}", row["id"])))?;
        serde_json::from_value::<std::collections::BTreeMap<String, semantics::FieldResolution>>(
            row["resolution_plan"].clone(),
        )
        .map_err(|e| DbErr::Custom(e.to_string()))?;
        serde_json::from_value::<std::collections::BTreeMap<String, semantics::UnresolvedField>>(
            row["unresolved_fields"].clone(),
        )
        .map_err(|e| DbErr::Custom(e.to_string()))?;
    }
    Ok(())
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // The migrator transaction holds these catalog-only locks through verification and writes.
        manager
            .get_connection()
            .execute_unprepared(
                "LOCK TABLE provider_templates, provider_model_templates IN ACCESS EXCLUSIVE MODE",
            )
            .await?;
        check_grants(manager, false).await?;
        let mut previous = snapshot(before::DATA)?;
        assert_matches(read(manager, false).await?, previous.clone())?;
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new(PROVIDERS))
                    .rename_column(Alias::new("icon_key"), Alias::new("logo_path"))
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new(MODELS))
                    .rename_column(Alias::new("name"), Alias::new("upstream_model_id"))
                    .to_owned(),
            )
            .await?;
        manager
            .get_connection()
            .execute_unprepared(include_str!("catalog_semantics.sql"))
            .await?;
        manager.get_connection().execute_unprepared("ALTER TABLE provider_model_templates DROP CONSTRAINT provider_model_templates_capabilities_check; ALTER TABLE provider_model_templates ADD CONSTRAINT catalog_capabilities_object CHECK (jsonb_typeof(capabilities) = 'object') NOT VALID; ALTER TABLE provider_model_templates ALTER COLUMN capabilities SET DEFAULT '{}'::jsonb;").await?;
        for field in ["input_price", "output_price", "cached_price"] {
            manager
                .alter_table(
                    Table::alter()
                        .table(Alias::new(MODELS))
                        .modify_column(ColumnDef::new(Alias::new(field)).decimal().null())
                        .to_owned(),
                )
                .await?;
        }
        for (table, additions) in [(PROVIDERS, PROVIDER_ADDITIONS), (MODELS, MODEL_ADDITIONS)] {
            for column in additions {
                let mut definition = ColumnDef::new(Alias::new(*column));
                if let Some(type_name) = enum_type(column) {
                    definition.custom(Alias::new(type_name));
                    if *column == "billing_unit" {
                        definition.null();
                    } else {
                        definition
                            .not_null()
                            .default(Expr::val("unknown").cast_as(Alias::new(type_name)));
                    }
                } else if *column == "display_name" {
                    definition.text().null();
                } else if matches!(*column, "max_input_tokens" | "max_output_tokens") {
                    definition
                        .big_integer()
                        .null()
                        .check(Expr::col(Alias::new(*column)).gt(0))
                        .check(Expr::col(Alias::new(*column)).lte(4_294_967_295_i64));
                } else {
                    definition.json_binary().not_null().default(
                        if matches!(
                            *column,
                            "source_refs" | "model_info_sources" | "auth_methods"
                        ) {
                            json!([])
                        } else {
                            json!({})
                        },
                    );
                    definition.check(
                        Func::cust(Alias::new("jsonb_typeof"))
                            .arg(Expr::col(Alias::new(*column)))
                            .eq(
                                if matches!(
                                    *column,
                                    "source_refs" | "model_info_sources" | "auth_methods"
                                ) {
                                    "array"
                                } else {
                                    "object"
                                },
                            ),
                    );
                }
                manager
                    .alter_table(
                        Table::alter()
                            .table(Alias::new(table))
                            .add_column(&mut definition)
                            .to_owned(),
                    )
                    .await?;
            }
        }
        for row in previous[PROVIDERS].as_array_mut().unwrap() {
            let old = row.as_object_mut().unwrap().remove("icon_key").unwrap();
            row["logo_path"] = old;
        }
        for row in previous[MODELS].as_array_mut().unwrap() {
            let old = row.as_object_mut().unwrap().remove("name").unwrap();
            row["upstream_model_id"] = old;
        }
        let target = snapshot(after::DATA)?;
        check_model_info_sources(&target)?;
        write(manager, &previous, &target).await?;
        manager
            .get_connection()
            .execute_unprepared(include_str!("model_info_constraints.sql"))
            .await?;
        manager
            .get_connection()
            .execute_unprepared(include_str!("catalog_semantics_constraints.sql"))
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new(PROVIDERS))
                    .modify_column(ColumnDef::new(Alias::new("logo_path")).text().not_null())
                    .to_owned(),
            )
            .await?;
        // SQL role grants have no SeaQuery DDL equivalent. This grants catalog SELECT only.
        manager.get_connection().execute_unprepared("GRANT SELECT ON provider_templates, provider_model_templates TO compallm_restricted").await?;
        assert_matches(read(manager, true).await?, target)
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "LOCK TABLE provider_templates, provider_model_templates IN ACCESS EXCLUSIVE MODE",
            )
            .await?;
        check_grants(manager, true).await?;
        let previous = snapshot(after::DATA)?;
        assert_matches(read(manager, true).await?, previous.clone())?;
        manager
            .get_connection()
            .execute_unprepared(include_str!("catalog_semantics_down.sql"))
            .await?;
        manager.get_connection().execute_unprepared(
            "ALTER TABLE provider_model_templates DROP CONSTRAINT provider_model_templates_model_info_sources_valid; ALTER TABLE provider_templates DROP CONSTRAINT provider_templates_model_info_sources_valid; DROP FUNCTION provider_catalog_info_sources_valid(jsonb); DROP FUNCTION provider_catalog_info_pointer_valid(text);"
        ).await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new(PROVIDERS))
                    .modify_column(ColumnDef::new(Alias::new("logo_path")).text().null())
                    .to_owned(),
            )
            .await?;
        let mut target = snapshot(before::DATA)?;
        normalize(&mut target);
        for row in target[PROVIDERS].as_array_mut().unwrap() {
            let old = row.as_object_mut().unwrap().remove("icon_key").unwrap();
            row["logo_path"] = old;
        }
        for row in target[MODELS].as_array_mut().unwrap() {
            let old = row.as_object_mut().unwrap().remove("name").unwrap();
            row["upstream_model_id"] = old;
        }
        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE provider_model_templates DROP CONSTRAINT catalog_capabilities_object",
            )
            .await?;
        write(manager, &previous, &target).await?;
        manager.get_connection().execute_unprepared("ALTER TABLE provider_model_templates ADD CONSTRAINT provider_model_templates_capabilities_check CHECK (jsonb_typeof(capabilities) = 'array'); ALTER TABLE provider_model_templates ALTER COLUMN capabilities SET DEFAULT '[]'::jsonb;").await?;
        for field in ["input_price", "output_price", "cached_price"] {
            manager
                .alter_table(
                    Table::alter()
                        .table(Alias::new(MODELS))
                        .modify_column(ColumnDef::new(Alias::new(field)).decimal_len(20, 8).null())
                        .to_owned(),
                )
                .await?;
        }
        for (table, additions) in [(MODELS, MODEL_ADDITIONS), (PROVIDERS, PROVIDER_ADDITIONS)] {
            for column in additions {
                manager
                    .alter_table(
                        Table::alter()
                            .table(Alias::new(table))
                            .drop_column(Alias::new(*column))
                            .to_owned(),
                    )
                    .await?;
            }
        }
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new(MODELS))
                    .rename_column(Alias::new("upstream_model_id"), Alias::new("name"))
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new(PROVIDERS))
                    .rename_column(Alias::new("logo_path"), Alias::new("icon_key"))
                    .to_owned(),
            )
            .await?;
        for type_name in [
            "catalog_readiness",
            "catalog_endpoint_mode",
            "catalog_service_kind",
            "catalog_billing_model",
            "catalog_billing_unit",
            "catalog_pricing_mode",
            "catalog_cache_billing",
            "catalog_specification_mode",
        ] {
            manager
                .get_connection()
                .execute_unprepared(&format!("DROP TYPE {type_name}"))
                .await?;
        }
        manager.get_connection().execute_unprepared("REVOKE SELECT ON provider_templates, provider_model_templates FROM compallm_restricted").await?;
        assert_matches(read(manager, false).await?, snapshot(before::DATA)?)
    }
}
