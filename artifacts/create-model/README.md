# Create model · ordered fallback

The dedicated Create model page uses shared fields, selects, badges, buttons, helpers, and the app shell. Only the atomic six-dot drag handle and insertion indicator were added to Design foundations; the route editor is a feature composition, not a page-pattern component.

The page has a plain title and no breadcrumb or numeric badges. The first selected provider is Primary; later rows are Fallback. The empty add row uses the same 892 px dropdown width without a drag handle. Selecting a pair creates a draggable row and retains a fresh empty add dropdown. Existing model-name and system-prompt values are preserved.

| Penpot view | Preview |
| --- | --- |
| Populated | [Create model](populated.png) |
| Dragging | [Lifted row and insertion line](dragging.png) |
| Reordered | [New primary provider](reordered.png) |
| Empty | [First provider dropdown](empty.png) |

Other canvas states demonstrate adding, removing, and replacing a provider. Penpot has fixed click transitions, not live drag handling. The [interactive local reference](preview.html) supports actual pointer and keyboard reordering, Escape cancellation, move controls, inline additions, duplicate prevention, removals, and name/route validation. It sends no requests and stores no configuration.

[penpot-verification.json](penpot-verification.json) records shared-component links, matching dropdown widths, removed numeric/breadcrumb controls, preserved values, spacing/bounds checks, propagation restoration, and prototype links. [preview-verification.json](preview-verification.json) records meaningful interaction checks. [source-reference.json](source-reference.json) identifies the supplied H2D and distinguishes source measurements from proposed drag states.

Ordered fallback is accepted for the design. Exact failure eligibility, retries, timeouts, partial-stream behavior, and charging remain unresolved in the [design notes](../../docs/content/llm-router/custom-model.md). No routing backend or application integration is implemented here.
