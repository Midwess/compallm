# Compallm product plan

## Why

Compallm's product direction currently lives in a single README and conversation history. Separate, reviewable Markdown plans are needed before selecting release scope and implementing business rules.

## What Changes

- Define six core and four supporting capabilities as proposed specifications.
- Capture the agreed passwordless login direction and client-selected format translation.
- Keep membership, budgets, supported protocols, and release priorities explicitly unresolved.
- Use the root README as an index and this open change to track discussion and progress.

## Capabilities

### New Capabilities

- `llm-router`: LLM Router.
- `llm-gateway`: LLM Gateway.
- `llm-chat`: LLM Chat.
- `interactive-ui`: Chat with interactive UI.
- `organization-plugins`: Organization plugins.
- `notebook`: Notebook.
- `organization-membership`: Organization and membership.
- `passwordless-login`: Passwordless email login.
- `usage-administration`: Usage and administration.
- `data-privacy`: Data and privacy.

### Modified Capabilities

None. There are no existing baseline OpenSpec specifications.

## Impact

This is an initial product proposal, not implementation approval. Future work affects the Rust backend, PostgreSQL schema, generated API contracts, React frontend, and external integrations. The planning CLI adds no application runtime dependency. Candidate features from the 9Router comparison are not automatically accepted into scope.
