---
title: Interactive UI
available: false
---

# Interactive UI

## Status

Implementation: Under development.

The following solution is planned; unresolved requirements are listed below.

## Solution

Let models explain ideas through interactive content alongside ordinary text.

- Render charts, tables, diagrams, forms, and small interactive applications inside a conversation.
- Let members explore alternatives, adjust inputs, and inspect results.
- Keep interactive content associated with the response that created it.
- Allow content to be revised through follow-up messages.
- Make data access, external requests, and actions requiring permission visible to the member.
- Run generated content in an isolated environment with a text fallback when rendering is unavailable.
- Support saving or exporting useful outputs where the output format allows it.

## Consequences

Model-generated UI requires an isolated execution boundary and a text fallback. Rendering content does not implicitly authorize data access or external actions.

## Behavior examples

### Interactive response content
The system should display supported model-generated interactive content within an isolated environment associated with its originating response.

#### Example: Explore visualization
- **WHEN** a response contains a supported interactive visualization
- **THEN** the member can interact with it while it remains associated with the response

## Open questions

Which interactive content types ship first, and which data access and external actions may they request?

## Related pages

- [Data and privacy](../data-privacy/adr.md)
- [Product overview](../overview.md)
