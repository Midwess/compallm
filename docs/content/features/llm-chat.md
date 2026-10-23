---
title: LLM Chat
tags:
  - features
---

# LLM Chat

## Status

Planned scope for discussion. The examples describe intended behavior, not completed implementation. Release priorities and unresolved rules are tracked in [the roadmap](../roadmap.md) and [business decisions](../decisions.md).

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

## Behavior examples

### Conversation interaction
The system should let members create conversations, select supported models, send messages, and receive streamed responses where supported.

#### Example: Chat response
- **WHEN** an authorized member submits a message to a supported model
- **THEN** the conversation displays the response and identifies the model that produced it

## Related pages

- [Data and privacy](data-privacy.md)
- [Product overview](../product.md)
- [Business decisions](../decisions.md)
