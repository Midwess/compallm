---
title: Organization settings design
available: false
---

# Organization settings design

Decision status: Organization settings requested for Penpot using `linear_app_devlogs_settings_workspace.h2d` as the visual reference. Implementation status: Canvas composition only; settings persistence and deletion are not implemented.

The main application sidebar keeps Members under Workspace and has no usage-limit settings entry. Usage & limits is reached through Settings.

Settings → Workspace administrator → Organization uses the shared Settings sidebar and layout on Preferences. Follow the capture's 640-pixel column, rounded groups, thin dividers, inline labels, right-aligned controls, and outside section headings. Reuse foundation fields, buttons, selectors, icons, and section surfaces.

Identity includes logo, organization name, and URL identifier. Time & region shows the first fiscal month and deployment region. Member onboarding includes welcome message, announcements, and default home view. These controls are design proposals; they do not establish available onboarding features, fiscal reporting, deployment locations, URL routing, or subscription tiers. Do not copy Linear's Enterprise gating or immutable-region rule into Compallm policy.

Danger zone exposes Delete organization with a confirmation patterned after Account. The prototype requests `I want to delete <organization name>` before enabling deletion. No canvas action deletes data. Ownership authorization, last-owner handling, retention, recovery, shared-resource effects, and the actual deletion process remain unresolved. See [Account settings](../portal/account.md).
