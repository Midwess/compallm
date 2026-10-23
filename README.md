# Compallm

Help individuals and teams work, learn, and build with LLMs through an organization workspace.

Compallm combines access to multiple model providers, governed API usage, chat, interactive responses, plugins, and knowledge notebooks.

## Product plan

The product plan lives in Markdown under [OpenSpec](openspec/README.md). Feature documents describe intended scope and proposed requirements, not completed implementation.

| Feature | Plan |
| --- | --- |
| LLM Router | [Scope and requirements](openspec/changes/plan-compallm/specs/llm-router/README.md) |
| LLM Gateway | [Scope and requirements](openspec/changes/plan-compallm/specs/llm-gateway/README.md) |
| LLM Chat | [Scope and requirements](openspec/changes/plan-compallm/specs/llm-chat/README.md) |
| Chat with interactive UI | [Scope and requirements](openspec/changes/plan-compallm/specs/interactive-ui/README.md) |
| Organization plugins | [Scope and requirements](openspec/changes/plan-compallm/specs/organization-plugins/README.md) |
| Notebook | [Scope and requirements](openspec/changes/plan-compallm/specs/notebook/README.md) |
| Organization and membership | [Scope and requirements](openspec/changes/plan-compallm/specs/organization-membership/README.md) |
| Passwordless email login | [Scope and requirements](openspec/changes/plan-compallm/specs/passwordless-login/README.md) |
| Usage and administration | [Scope and requirements](openspec/changes/plan-compallm/specs/usage-administration/README.md) |
| Data and privacy | [Scope and requirements](openspec/changes/plan-compallm/specs/data-privacy/README.md) |

- [Proposal](openspec/changes/plan-compallm/proposal.md): motivation and overall scope.
- [Decisions and open questions](openspec/changes/plan-compallm/design.md): business rules still to resolve.
- [Tasks](openspec/changes/plan-compallm/tasks.md): planning progress and implementation preparation.

## Development flow

Discuss requirements in the relevant feature document before implementing them. Resolve business decisions and select release scope, then create focused OpenSpec changes with concrete scenarios and implementation tasks. Keep incomplete work unchecked and archive changes only after implementation and verification.

See the [OpenSpec workflow](openspec/README.md) for installation and commands.

## Development setup

See [development documentation](docs/development.md) for installation, database setup, build commands, and API generation.

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) and [NOTICE](NOTICE).
