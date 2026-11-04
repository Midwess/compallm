---
name: related-issues
description: Find issues related to the current Compallm PR, branch, or requested change. Use for current issue context and before PR creation. Read-only discovery; reuse existing links without asking and never gate implementation on task selection.
---

# Related Issues

The application repository is `Midwess/compallm`; its board is [Compallm Project 7](https://github.com/orgs/Midwess/projects/7). Prefer existing issue context over asking the user to repeat it. This skill reads GitHub state; it does not create issues, comments, sub-issues, assignments, or Project updates.

1. Identify the current branch and any PR explicitly named by the user. Otherwise use `rtk proxy gh pr view --repo Midwess/compallm --json number,url,title,body,baseRefName,closingIssuesReferences` to inspect the current branch's PR. On versions where explicit `--repo` cannot infer the branch, pass the branch as the positional argument. A branch without a PR is normal. Authentication, network, and permission errors are not evidence that no PR or issue exists.
2. Read `closingIssuesReferences` and the PR description's issue references. Distinguish issues GitHub will close from ordinary related links. Inspect each relevant issue's title, body, state, and repository; ignore unrelated references, PR numbers, and examples. Existing cross-repository links can be valid; do not migrate them into Compallm automatically.
3. If no PR or relevant links exist, check issue links already supplied in the conversation, the branch name, and commit messages. Read issues named by those references. Search repository issues by concrete feature or bug keywords when needed, for example with `rtk proxy gh issue list --repo Midwess/compallm --search 'provider in:title' --state open --json number,title,url,body`. Read likely matches before presenting them. Names and search matches alone are candidates, not confirmed links.
4. Return established related issues first, with their URLs and why they apply; list uncertain candidates separately. If established issues cover the current change, retain that context and do not ask again. More than one issue may apply. A closed issue is context, not permission to reopen it or claim the current change has already shipped.

Before creating a PR, if no related issue is established, ask once: “Which issue should this PR link to? Send a name, number, or URL, or describe a feature, bug, or task and I can create it on the Compallm board.” Explain that `AGENTS.md` requires this check before PR creation. Continue authorized preparation and checks while waiting; do not block starting work. If the user already requested issue creation or supplied an issue, proceed without redundant confirmation.

Use [Link Issue](../link-issue/SKILL.md) for an existing issue or [Create Issue](../create-issue/SKILL.md) for an authorized new one. Do not open a PR without the issue mapping unless the user explicitly overrides the repository rule. Discovery failures must be reported rather than treated as an empty result.
