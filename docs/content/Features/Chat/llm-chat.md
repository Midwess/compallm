---
title: LLM Chat
available: false
---

# LLM Chat

## Status

Planned scope for discussion. The examples describe intended behavior, not completed implementation. Unresolved rules are tracked in [business decisions](../../business-decisions.md).

This is an initial design pass. It records the current persistence starting point and the decisions needed before migrations or chat implementation begin.

## Current database schema

The PostgreSQL schema currently contains only these tables:

| Table | Purpose |
| --- | --- |
| `users` | Verified user identity and account status |
| `organizations` | Organization workspace and creator |
| `organization_members` | User membership and role within an organization |
| `providers` | An organization's LLM provider connection and encrypted credential |

The provider migration and its SeaORM model now exist, but provider management has no HTTP endpoints yet. There is still no persisted conversation, message, or generation. The schema is defined by the migrations in `libs/postgres/src/migration/`; generated SeaORM models reflect migrations already applied to the generation database.

## Provider prerequisite

Chat needs to identify the model and provider used for each generated response, but "provider" can represent different concepts:

- **Provider type**: a supported integration such as Anthropic, Gemini, OpenAI, or an OpenAI-compatible service.
- **Provider connection**: an organization's configured endpoint and credentials for a provider type.
- **Available model**: a model identifier and the capabilities available through a connection.

The initial `providers` table represents an organization-owned provider connection. It stores a display name, URL, encrypted API key, and API protocol. The API protocol is a PostgreSQL enum with `anthropic`, `openai`, and `openai_compatible` values. Available models are not yet represented. Before chat tables reference a provider or model, the Router design still needs to decide:

1. Whether members may also supply personal credentials.
2. Whether available models are persisted, discovered dynamically, configured manually, or use a combination of these approaches.
3. How custom and self-hosted OpenAI-compatible endpoints are identified beyond their protocol and URL.
4. Which provider and model details are retained as an immutable snapshot on a generation so historical conversations remain understandable after configuration changes.

Credentials must belong to the provider connection boundary, not to conversations or messages. Secrets must be encrypted or stored through an external secret facility and must never appear in browser responses, logs, or conversation exports.

## Scope

Provide a familiar chat experience across multiple providers, with access to richer model capabilities.

- Create, rename, organize, search, and delete conversations.
- Stream responses and allow members to stop generation, retry a response, and edit a previous prompt.
- Select models and show which model produced each response.
- Attach files and images when the selected model supports them.
- Use reusable prompts and conversation instructions.
- Call approved plugins and use notebook sources as conversation context.
- Show usage and estimated cost for a conversation.
- Share conversations within the organization according to access rules.

## Initial domain model

The first persistence design should distinguish user-authored content from provider generation attempts.

| Concept | Responsibility |
| --- | --- |
| Conversation | Organization-scoped chat metadata, ownership, title, and lifecycle |
| Message | Durable user, assistant, system, or tool content shown in the conversation |
| Generation | One attempt to produce an assistant response, including selected provider/model, status, timing, usage, and failure details safe to retain |

A separate generation record is important because retrying a response, falling back to another provider, or cancelling a stream can produce multiple attempts for the same conversation position. Provider credentials must never be copied into a generation record.

The exact table design remains open until these behavior rules are resolved:

- Does editing an earlier prompt create a new branch, hide later messages, or replace history?
- Does retrying retain all responses and attempts or expose only the selected response?
- When is a streamed assistant message persisted, and what content remains after cancellation or failure?
- Are conversations private to their creator by default, and how is organization sharing represented?
- What are the retention and hard-deletion rules for messages, attachments, generation metadata, and provider request identifiers?

## First implementation slice

The smallest useful chat workflow is:

1. An authenticated organization member creates a conversation.
2. The member selects an allowed available model.
3. The member submits a text message.
4. The Gateway authorizes and attributes the request.
5. The Router selects the provider connection and invokes its concrete adapter.
6. The response streams to the member when supported.
7. The conversation stores the message, completed response, provider/model identity, status, and available usage data.

Attachments, tools, notebooks, branching edits, fallback, sharing, and cost calculation should be added only after their rules are defined. This document does not authorize migrations or implementation yet.

## Behavior examples

### Conversation interaction
The system should let members create conversations, select supported models, send messages, and receive streamed responses where supported.

#### Example: Chat response
- **WHEN** an authorized member submits a message to a supported model
- **THEN** the conversation displays the response and identifies the model that produced it

#### Example: Provider configuration is unavailable
- **WHEN** the selected model has no enabled provider connection for the organization
- **THEN** the system rejects the request before creating a billable provider request
- **AND** reports a safe configuration error without exposing credentials or upstream secrets

#### Example: Cancelled streamed response
- **WHEN** a member stops an in-progress streamed response
- **THEN** the system requests cancellation from the provider when supported
- **AND** records the attempt as cancelled rather than completed
- **AND** applies the usage-accounting rule selected for partial responses

## Persistence sequence

1. Define provider types, organization provider connections, credential handling, available models, and capability representation with the Router and Gateway features.
2. Add conversations, messages, and generation attempts after their ownership, editing, retry, cancellation, and retention rules are resolved.
3. Add attachments, tool calls, notebook citations, sharing, and search as separate increments.

## Related pages

- [LLM Router](../llm-router.md)
- [LLM Gateway](../llm-gateway.md)
- [Data and privacy](../data-privacy.md)
- [Product overview](../../overview.md)
- [Business decisions](../../business-decisions.md)
