# Shared spacing policy

The saved Penpot policy uses `layout.spacing.scale = 0.5` and `layout.spacing.minimum = 2`. Every nonzero nominal spacing resolves through `max(minimum, nominal × scale)`; zero remains zero. The sixty legacy spacing names remain compatible aliases. Control dimensions, font metrics, icon artwork, borders, and radii are separate.

[policy.json](policy.json) records the policy and the latest verified checkpoint. The foundation audit covered 555 Auto Layouts, with zero unbound gap/padding properties, no positive values below 2 px, and no broken component links. Final composition reflow, the remaining screen audits, and fresh visual exports are pending because Penpot MCP began timing out even on read-only requests.

The [agent workflow](../../docs/content/portal/spacing.md) explains editing and refreshing the policy. Agents must check an existing binding before applying a token, since reapplying it can toggle that binding off. Do not halve the stored nominal number again: change the scale, retain the 2 px floor, refresh resolution, and reflow intrinsic compositions. Gallery positions, table columns, code line geometry, and icon construction must not be mistaken for component padding.

Save a named version before editing. Verify Main, login, signup, and Design foundations, restore any propagation probe, and inspect fresh exports before marking a density migration complete. Existing previews under `artifacts/add-provider` and `artifacts/linear-h2d-reference` predate this spacing migration.
