import {
  ProfileUpdateInput,
  PreferencesUpdateInput,
  OrganizationSettingsUpdateInput,
  ImageUploadInput,
  CredentialCreateInput,
  SpendingLimitInput,
  ReportingScheduleInput,
} from '@compallm/contract/validation'
import { contractResult } from '../../components/form'
import type { SettingsState } from './types'

const aliases: Record<string, string> = {
  'Full name': 'FULL_NAME',
  'System preference': 'SYSTEM_PREFERENCE',
  'Pure Light': 'PURE_LIGHT',
  'Magic Blue': 'MAGIC_BLUE',
  'Classic Dark': 'CLASSIC_DARK',
  '⌘ + Enter': 'MODIFIER_ENTER',
  'Linear Dark': 'LINEAR_DARK',
  'Linear Light': 'LINEAR_LIGHT',
  'Follow interface': 'FOLLOW_INTERFACE',
  '12px': 'PX12',
  '13px': 'PX13',
  '14px': 'PX14',
  '16px': 'PX16',
  'Always show': 'ALWAYS_SHOW',
  'Only when badged': 'ONLY_WHEN_BADGED',
  'Show when badged': 'ONLY_WHEN_BADGED',
  "Don't show": 'HIDDEN',
}
const toWire = (value: unknown) => (typeof value === 'string' ? (aliases[value] ?? value.toUpperCase()) : value)
const fromWire = (value: unknown, draft: unknown) => (typeof draft === 'string' && toWire(draft) === value ? draft : value)

export function parseSettings(values: Partial<SettingsState>, names: string[]) {
  const next = { ...values }
  const issues: { path: PropertyKey[]; message: string }[] = []
  for (const root of ['profile', 'organization', 'preferences'] as const) {
    const keys = names.filter((name) => name.startsWith(root + '.')).map((name) => name.slice(root.length + 1))
    if (!keys.length) continue
    const draft = values[root] as Record<string, unknown>
    const patch = Object.fromEntries(
      keys.map((key) => [
        key,
        root === 'preferences' && key !== 'sidebarOrder'
          ? toWire(draft[key])
          : root === 'organization' && key === 'home'
            ? toWire(draft[key])
            : draft[key],
      ]),
    )
    // Preview syntax language is transient rather than an API preference.
    if (root === 'preferences') delete patch.codeLanguage
    const schema =
      root === 'profile'
        ? ProfileUpdateInput
        : root === 'organization'
          ? OrganizationSettingsUpdateInput
          : PreferencesUpdateInput
    const result = schema.safeParse(patch)
    if (!result.success) issues.push(...result.error.issues.map((issue) => ({ ...issue, path: [root, ...issue.path] })))
    else
      Object.assign(next, {
        [root]: {
          ...draft,
          ...Object.fromEntries(Object.entries(result.data).map(([key, value]) => [key, fromWire(value, draft[key])])),
        },
      })
  }
  for (const name of ['profileUpload', 'organizationUpload'] as const) {
    const file = values[name]
    if (!names.includes(name) || !file) continue
    const result = ImageUploadInput.safeParse({
      size: file.size,
      mediaType: ({ 'image/jpeg': 'JPEG', 'image/png': 'PNG', 'image/webp': 'WEBP' } as Record<string, string>)[file.type],
    })
    if (!result.success) issues.push(...result.error.issues.map((issue) => ({ ...issue, path: [name] })))
  }
  return issues.length ? { issues } : contractResult({ success: true }, next)
}

export function parseSettingsDialog(values: Partial<SettingsState>) {
  const dialog = values.dialog
  if (!dialog) return { values }
  if (dialog.kind === 'welcome' || dialog.kind === 'announcements') {
    const result = OrganizationSettingsUpdateInput.safeParse({ [dialog.kind]: dialog.value })
    return result.success
      ? { values }
      : { issues: result.error.issues.map((issue) => ({ ...issue, path: ['dialog', 'value'] })) }
  }
  if (dialog.kind === 'organization-limit' || dialog.kind === 'member-limit') {
    const result = SpendingLimitInput.safeParse({ amount: dialog.value === '' ? null : dialog.value })
    return result.success
      ? { values }
      : { issues: result.error.issues.map((issue) => ({ ...issue, path: ['dialog', 'value'] })) }
  }
  if (dialog.kind === 'reset-schedule') {
    const result = ReportingScheduleInput.safeParse({ timezone: dialog.value })
    return result.success
      ? { values }
      : { issues: result.error.issues.map((issue) => ({ ...issue, path: ['dialog', 'value'] })) }
  }
  if (dialog.kind === 'create-key' || dialog.kind === 'add-passkey') {
    const result = CredentialCreateInput.safeParse({ name: dialog.name })
    return contractResult(
      result,
      result.success ? { ...values, dialog: { ...dialog, name: result.data.name } } : values,
      'dialog',
    )
  }
  return { values }
}
