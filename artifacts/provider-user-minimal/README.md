# Provider flow · user minimal style

The user-edited **Add provider · 1 Model selected** frame is the visual reference. Its shapes, text, geometry, and styles are preserved. Only the remaining four provider screens were restyled.

| State | Preview |
| --- | --- |
| User reference | [Selected model](reference.png) |
| Filtered catalog | [Filtered](filtered.png) |
| No results | [Empty search](empty.png) |
| Preset configuration | [Preset](preset.png) |
| Custom configuration | [Custom](custom.png) |

Catalog states omit the repeated introduction, result count, selection summary, pagination range, and Cancel action. They use the reference's wider search, centered pagination, and compact **Use custom provider** / **Continue** footer. The no-results state omits inactive pagination. Configuration uses the same progress placement, left alignment, and compact footer, retaining all required fields and the connected numbered sections. Repeated helper text is hidden while credential, endpoint, pricing-unit, and unknown-metadata guidance remains available.

[verification.json](verification.json) records reference preservation, required fields/defaults, shared component links, updated-content spacing/bounds, 13 added prototype links, and file validation. The inherited sidebar follows the user's reference; its existing internal sizing and zero-spacing binding observations are recorded separately from updated content. The [interactive preview checks](../provider-catalog/user-minimal-preview-verification.json) cover the current minimal layout and complete catalog/setup behavior.

Saved Penpot version: **Add provider · minimal user style verified**. Design foundations and other feature pages were not restyled.
