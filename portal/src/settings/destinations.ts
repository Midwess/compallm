import type { SettingsDestination } from '@compallm/storybook/pages/settings'
export const settingsPages = [
  'blank',
  'general',
  'messaging',
  'code',
  'profile',
  'access',
  'security',
  'api-keys',
  'organization',
  'organization-onboarding',
  'members',
  'usage-limits',
  'member-limits',
  'usage-explorer',
  'audit',
] as const satisfies readonly SettingsDestination[]
