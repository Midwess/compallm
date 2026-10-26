---
title: LLM Chat
available: false
---

# LLM Chat

## Status

Decision status: Proposed. Implementation status: Under development.

Planned scope for discussion. The examples describe intended behavior, not completed implementation. Open questions are recorded below.

## Context and proposed scope

Provide a familiar chat experience across multiple providers, with access to richer model capabilities.

- Create, rename, organize, search, and delete conversations.
- Stream responses and allow members to stop generation, retry a response, and edit a previous prompt.
- Select models and show which model produced each response.
- Attach files and images when the selected model supports them.
- Use reusable prompts and conversation instructions.
- Call approved plugins and use notebook sources as conversation context.
- Show usage and estimated cost for a conversation.
- Share conversations within the organization according to access rules.

## Consequences

Conversation sharing, model access, plugins, and notebook context remain subject to organization permissions and privacy rules.

## Behavior examples

### Conversation interaction
The system should let members create conversations, select supported models, send messages, and receive streamed responses where supported.

#### Example: Chat response
- **WHEN** an authorized member submits a message to a supported model
- **THEN** the conversation displays the response and identifies the model that produced it

## Open questions

Which chat capabilities ship first, and how do the [privacy and sharing rules](../data-privacy/adr.md) apply to conversations?

## Related pages

- [Data and privacy](../data-privacy/adr.md)
- [Product overview](../overview.md)
