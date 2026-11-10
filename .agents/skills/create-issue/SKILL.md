---
name: create-issue
description: Resolve open requirements with the user, write a concise Compallm issue using the Writing skill, and add it to the Project board. Use when issue creation is requested or accepted before a PR.
---

# Create Issue

Create issues in `Midwess/compallm` and add them to [Compallm Project 7](https://github.com/orgs/Midwess/projects/7). A creation request authorizes both actions once the requirements are settled. Read and apply [Writing](../writing/SKILL.md) to the issue description: establish its reader and outcome, use concrete language, and remove repetition. Issue descriptions belong in GitHub Issues; feature specifications remain in the Wiki.

## Settle the requirements

Use the description and existing conversation to identify the intended outcome, scope, business rules, and acceptance criteria. Infer routine title wording or a clear category; never silently choose unresolved product behavior.

If questions, conflicting answers, or alternatives remain about what the issue should require, ask the user and wait before creating it. Bundle related questions, recommend an option when useful, and use answers already given. Explain that the clarification comes from this skill's requirement to resolve open decisions before creation. Continue read-only research or drafting while waiting; silence is not a decision.

Create the issue as the final agreed specification, not as a placeholder containing unanswered requirements or a transcript of the discussion. If the user chooses an investigation task, its agreed outcome can be answering a specific unknown; do not invent the answer or present a hypothesis as fact. Once the relevant decisions are settled, proceed without a separate confirmation ceremony.

Check existing issues for the same outcome before creating. Reuse an exact match and ensure board membership. Ask when several plausible matches have different scope. Similar keywords alone do not establish a duplicate.

## Write the final issue

Apply Technical Writing's reader-first and editing checks. Include only the context needed to understand and verify the agreed outcome:

- **Feature:** user problem, agreed behavior and scope, observable acceptance criteria, and relevant Wiki links.
- **Bug:** observed and expected behavior, available reproduction steps and environment, impact, and completion criteria. Clearly label missing reproduction evidence; never invent logs or confirmation.
- **Task:** concrete deliverable, scope, completion criteria, and relevant dependencies or links.
- **Other requested category:** adapt the description to its agreed purpose.

State the latest decision directly. Omit abandoned options, conversational history, invented evidence, and repeated requirements. Read the draft without relying on the conversation: can another member understand the work and determine when it is complete? Resolve remaining requirement gaps with the user before publishing.

If the work has already produced a final decision, include its summary and published Wiki links in the initial issue description. This is the issue's decision record; do not add a second comment with the same content. If reusing an existing issue, follow Writing's decision-recording workflow to add any missing agreed decision without overwriting others' content. Do not require implementation documentation merely to capture a new issue whose specification work has not started.

## Create and verify

Use existing repository types or labels when available; otherwise retain the category in the description. Do not change repository classification settings. Assign owners, priority, dates, or status only when established by the user or context. Do not infer ownership from the CLI account or create unrequested sub-issues.

Write the body to a temporary file and create with `rtk proxy gh issue create --repo Midwess/compallm --title 'Concrete outcome' --body-file /private/tmp/compallm-issue-body.md`, supplying any supported existing classification. Treat shell arguments as code: use proper quoting or structured subprocess arguments for user-provided text.

Add the returned issue URL with `rtk proxy gh project item-add 7 --owner Midwess --url ISSUE_URL --format json`. Verify the issue's saved description and membership in the intended Project. Do not assume that templates or auto-add workflows did this. If a parent was requested, verify the parent and attach the issue as an actual sub-issue using supported CLI flags or GraphQL; otherwise leave it independent.

If issue creation succeeds but board addition fails, retain and report the created issue URL and the specific missing board update. Check existing state before retrying any ambiguous creation or addition; never create a second issue to recover from a Project error. Report authentication and network failures without claiming success or exposing secrets.

Return the issue URL and board-addition result. Retain the issue for the current work so later PR creation does not ask for it again. Use [Link Issue](../link-issue/SKILL.md) to link it to an existing PR or include it in an already-authorized forthcoming PR. Creating an issue does not authorize implementation, PR creation, merging, or deployment by itself.
