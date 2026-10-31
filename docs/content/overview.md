---
title: Overview
order: 0
available: true
---

# Product overview

Compallm helps individuals and teams work, learn, and build with LLMs in an organization workspace. It combines model access and usage governance with chat, interactive content, plugins, and knowledge notebooks.

To deploy the application, follow the [setup guideline](setup-guideline.md).

## Feature availability

The filter beside Compallm shows **Available** by default. Select **All** to explore work in progress. A feature appears in the Available selection only when its page explicitly sets the YAML boolean `available: true`; `available: false` or an omitted flag identifies work in progress.

## Planning status

Development features describe intended scope. Each feature's `adr.md` describes its current architecture, solution, and unresolved requirements. No release commitment is implied by listing a development feature.

## Feature plans

Each feature has its own folder directly under `docs/content/`, with shared decisions in `adr.md` and focused flows in sibling pages.

- [Login](login/adr.md): [Email](login/email.md), [GitHub](login/github.md), and [GitLab](login/gitlab.md).
- [Mailing](mailing/adr.md): bounded asynchronous SMTP delivery and recipient throttling.
- [API reference](api/index.md): generated endpoint guide with request and response examples.
- [Organization and membership](organization/adr.md).
- [LLM Router](llm-router/adr.md).
- [LLM Gateway](llm-gateway/adr.md).
- [LLM Chat](llm-chat/adr.md).
- [Interactive UI](interactive-ui/adr.md).
- [Organization plugins](plugins/adr.md).
- [Notebook](notebook/adr.md).
- [Usage and administration](usage-administration/adr.md).
- [Data and privacy](data-privacy/adr.md).
