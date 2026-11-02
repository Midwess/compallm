---
title: LLM Chat
available: false
---

# LLM Chat

## Status

Implementation: Under development.

The following solution is planned; unresolved requirements are listed below.

## Solution

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

Which chat capabilities ship first, and what access and sharing rules apply to conversations?

## Related pages

- [Product overview](../overview.md)

## UI controller boundary

The library exports the create-chat content from `pages/organization/chat`. Consumers wrap it in `ChatControllerContext` and compose it inside the empty `OrganizationPage` shell. The controller owns draft text, selected model, available models, and creation-pending state through a scoped store. Input, model selection, attachment, and create-chat actions notify the controller. Enter inserts a new line; chat creation requires the explicit submit action. The input grows with multiline content up to a bounded height, then scrolls internally. The component does not generate responses or render a conversation transcript. The portal owns the `/:organizationName/chat` route, soft routing from the organization entry, and API integration. Storybook uses local demo controllers.
