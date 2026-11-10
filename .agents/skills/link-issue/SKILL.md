---
name: link-issue
description: Resolve a supplied issue name, number, or URL and link it to the current Compallm PR, or retain it for a forthcoming PR. Use when the user asks to link an existing issue; ask only for missing or ambiguous issue identity.
---

# Link Issue

Accept one or more issue names, numbers, or URLs. Default bare numbers and names to `Midwess/compallm`; honor explicit repositories. If no issue identity was supplied, use [Related Issues](../related-issues/SKILL.md) to inspect the current PR. If it already has relevant links, reuse them and finish without asking. Otherwise ask for an issue name, number, or URL. Do not create an issue as a substitute for a missing identity.

Resolve numbers or URLs with `gh issue view`. Search names with `gh issue list --search` and read the matches. Prefer an exact unique title; ask the user to choose when several matches are plausible. Confirm that the issue's scope fits the change. Do not silently choose a similarly named issue or reopen a closed one.

Inspect the current or user-specified PR and preserve its existing description. Add missing issue references without duplicating links or replacing the author's content. Use `Closes OWNER/REPO#NUMBER` only when the final PR completes the issue; use `Related to OWNER/REPO#NUMBER` for partial work or a broader feature. Multiple issues can be referenced independently. Closing keywords belong in the PR description and establish automatic closure only when the PR targets the repository's default branch; ordinary references do not imply closure. Do not change the base branch merely to enable closure.

Use `rtk proxy gh pr edit` with `--body-file` pointing to a temporary file, then read the PR back. For closing links targeting the default branch, verify `closingIssuesReferences`; for ordinary references, verify the saved description. If GitHub cannot establish a closing relationship, report that limit instead of claiming success.

If there is no PR yet, retain the resolved issue URLs and intended relationship in the current work context and use them in the eventual PR description. Do not create a PR merely because `$link-issue` was invoked. When PR creation is already authorized, include the issue references in its initial body and verify them afterward.

If the current work has a pending decision summary, apply [Writing](../writing/SKILL.md) to record it in the resolved issue with its published Wiki links before PR creation. Reuse an equivalent existing note. Pure issue linking does not require inventing a decision note.

Run `gh` through `rtk` with an explicit repository. No sub-issues, unrelated comments, assignment changes, issue closure, or board status changes are implied by linking. Report the PR and linked issue URLs, or state that the issue mapping is ready for a forthcoming PR.
