---
title: Interactive UI
tags:
  - features
---

# Interactive UI

## Status

Planned scope for discussion. The examples describe intended behavior, not completed implementation. Release priorities and unresolved rules are tracked in [the roadmap](../roadmap.md) and [business decisions](../decisions.md).

## Scope

Let models explain ideas through interactive content alongside ordinary text.

- Render charts, tables, diagrams, forms, and small interactive applications inside a conversation.
- Let members explore alternatives, adjust inputs, and inspect results.
- Keep interactive content associated with the response that created it.
- Allow content to be revised through follow-up messages.
- Make data access, external requests, and actions requiring permission visible to the member.
- Run generated content in an isolated environment with a text fallback when rendering is unavailable.
- Support saving or exporting useful outputs where the output format allows it.

## Behavior examples

### Interactive response content
The system should display supported model-generated interactive content within an isolated environment associated with its originating response.

#### Example: Explore visualization
- **WHEN** a response contains a supported interactive visualization
- **THEN** the member can interact with it while it remains associated with the response

## Related pages

- [Data and privacy](data-privacy.md)
- [Product overview](../product.md)
- [Business decisions](../decisions.md)
