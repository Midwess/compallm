# Planning with OpenSpec

Product plans and requirements are ordinary Markdown tracked in Git. OpenSpec provides validation and coding-agent workflows around them.

## Install

Requires Node.js 20.19 or newer; this project requires Node.js 22 or newer.

```bash
npm install -g @fission-ai/openspec@1.14.3
openspec --version
```

The CLI is a developer tool, not a runtime dependency. Codex integration lives in `.agents/skills/`.

## Current plan

Start with [plan-compallm](changes/plan-compallm/proposal.md). Each capability has its own scope and scenarios under that change's `specs/` directory. Business questions live in [design.md](changes/plan-compallm/design.md); progress lives in [tasks.md](changes/plan-compallm/tasks.md).

This plan remains a draft. Completing artifacts or passing validation does not approve rules or implement features. No product capabilities have been promoted into baseline specs yet.

## Layout

- `specs/<capability>/spec.md`: baseline requirements consolidated from completed changes.
- `changes/<change>/proposal.md`: motivation and scope.
- `changes/<change>/specs/<capability>/spec.md`: proposed requirements and scenarios.
- `changes/<change>/design.md`: decisions, constraints, and open questions.
- `changes/<change>/tasks.md`: pending and completed work.
- `changes/archive/`: completed changes.

## Commands

```bash
openspec list
openspec status --change plan-compallm
openspec validate plan-compallm --strict
openspec new change <change-name>
openspec update
```

In Codex, use the generated `openspec-explore`, `openspec-propose`, `openspec-apply-change`, and `openspec-archive-change` skills. Restart the session if newly installed skills do not appear. Apply an agreed, implementation-ready change; archive only when its work is complete and verified.

For this broad initial plan, resolve scope first and create focused implementation changes. Keep baseline requirements and completion status separate from future ideas.
