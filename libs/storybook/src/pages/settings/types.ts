import type { FormStore } from '../../components/form'
import type { RoleController } from './role/controller'
import type { GroupController } from './group/controller'
import type { ProfileController } from './profile/controller'
import type { OrganizationController } from './organization/controller'

export type SettingsDestination =
  | 'blank'
  | 'general'
  | 'appearance'
  | 'messaging'
  | 'code'
  | 'profile'
  | 'security'
  | 'api-keys'
  | 'danger'
  | 'organization'
  | 'members'
  | 'role'
  | 'new-role'
  | 'group'
  | 'new-group'
  | 'usage-limits'
  | 'member-limits'
  | 'usage-explorer'
  | 'audit'

export interface SettingsLocation {
  page: SettingsDestination
  id?: string
}
export interface SettingsRole {
  id: string
  name: string
  description: string
  source: string
  builtin?: boolean
  color: string
}
export interface SettingsGroup {
  id: string
  name: string
  description: string
  memberIds: string[]
  roleIds: string[]
  color: string
}
export interface SettingsMember {
  id: string
  name: string
  email: string
  roleIds: string[]
  limitUsd: string | null
  currentUsageUsd: string | null
  previousUsageUsd: string | null
}
export interface SettingsCredential {
  id: string
  name: string
}
export interface SettingsApplication extends SettingsCredential {
  scope: string
}
export interface SettingsApiKey extends SettingsCredential {
  prefix: string
}
export interface PolicyProjection {
  action: string
  effect?: 'permit' | 'forbid'
  source: string
  advanced: boolean
  editable: boolean
  reason?: string
  resourceRestricted?: boolean
}
export interface SettingsPolicyAction {
  id: string
  label: string
  scope: string
  category: string
  description: string
  access?: 'Read' | 'Use' | 'Write' | 'Admin' | 'Sensitive'
}
export interface RoleDraft {
  id: string | null
  name: string
  description: string
  source: string
  mode: 'visual' | 'cedar'
  projection: PolicyProjection[]
  diagnostics: string[]
  parsing: boolean
  builtin: boolean
}
export interface GroupDraft {
  id: string | null
  name: string
  description: string
  memberIds: string[]
  roleIds: string[]
}
export interface SettingsProfile {
  name: string
  username: string
  email: string
  avatar: string | null
}
export interface SettingsOrganization {
  name: string
  slug: string
  logo: string | null
  fiscalMonth: string
  region: string
  welcome: string
  announcements: string
  home: string
}
export interface SettingsAuditEvent {
  id: string
  actor: string
  action: string
  resource: string
  result: string
}
export interface SettingsUsageRow {
  id: string
  model: string
  requests: string
  inputTokens: string
  outputTokens: string
  providerCost: string
}
export type SettingsDialogKind =
  | 'delete-account'
  | 'delete-organization'
  | 'add-passkey'
  | 'verify-passkey'
  | 'remove-passkey'
  | 'revoke-sessions'
  | 'application'
  | 'create-key'
  | 'reveal-key'
  | 'revoke-key'
  | 'remove-member'
  | 'delete-role'
  | 'delete-group'
  | 'remove-group-member'
  | 'organization-limit'
  | 'member-limit'
  | 'reset-schedule'
  | 'welcome'
  | 'announcements'
  | 'customize-sidebar'
export interface SettingsDialog {
  kind: SettingsDialogKind
  targetId?: string
  name: string
  value: string
  phrase: string
  revealedKey?: string
}
export interface SettingsState {
  profileUpload: File | null
  organizationUpload: File | null
  codeExamples: Record<string, { code: string; lineChanges?: Record<number, 'added' | 'removed'> }>
  location: SettingsLocation
  navigationQuery: string
  query: string
  filters: Record<string, string>
  preferences: Record<string, string | boolean>
  profile: SettingsProfile
  organization: SettingsOrganization
  roles: SettingsRole[]
  policyActions: SettingsPolicyAction[]
  models: string[]
  groups: SettingsGroup[]
  members: SettingsMember[]
  passkeys: SettingsCredential[]
  applications: SettingsApplication[]
  apiKeys: SettingsApiKey[]
  hasOtherSessions: boolean
  organizationLimitUsd: string | null
  memberDefaultLimitUsd: string | null
  resetFrequency: string
  timezone: string
  usage: SettingsUsageRow[]
  audit: SettingsAuditEvent[]
  dialog: SettingsDialog | null
  busy: boolean
  error: string | null
  errorToast: { message: string; retry?: () => void } | null
  notice: string | null
  readonly: boolean
  status: 'ready' | 'loading' | 'denied'
}
export interface SettingsController {
  store: FormStore<SettingsState>
  role: RoleController
  group: GroupController
  pages: SettingsPageControllers
  navigate(page: SettingsDestination, id?: string): void
  setNavigationQuery(value: string): void
  signOut(): void
  leaveSettings(): void
  apiError(error: unknown, retry?: () => void): void
  dismissErrorToast(): void
  dismissNotice(): void
}

export interface SettingsPageControllers {
  role: RoleController
  search: { setQuery(value: string): void; setFilter(key: string, value: string): void }
  preferences: {
    updatePreferences(patch: SettingsState['preferences'], signal: AbortSignal): Promise<SettingsState['preferences'] | false>
  }
  profile: ProfileController
  organization: OrganizationController
  members: { setMemberRoles(memberId: string, roleIds: string[]): void | false | Promise<void | false> }
  dialogs: {
    openDialog(kind: SettingsDialogKind, targetId?: string): void
    closeDialog(): void
    confirmDialog(dialog: SettingsDialog): void
    copy(value: string): void
  }
}
