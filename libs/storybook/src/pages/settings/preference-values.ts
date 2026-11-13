import type { FieldsSelection, UserPreferences, UserPreferencesGenqlSelection } from '@compallm/contract/graphql'
import { UserPreferencesPatchInput, type UserPreferencesPatchInputFormOutput } from '@compallm/contract/validation'
import get from 'lodash-es/get.js'
import set from 'lodash-es/set.js'

export const preferenceSelection = {
  navigation: { home: true },
  memberDisplay: { nameFormat: true },
  calendar: { firstDay: true },
  theme: { mode: true, palettes: { light: true, dark: true } },
  typography: { size: true },
  accessibility: { underlineLinks: true, reduceMotion: true, pointerCursors: true },
  messageComposition: { convertEmoticons: true, sendShortcut: true },
  codeAppearance: { theme: true, font: { family: true, size: true } },
  sidebar: { badgeStyle: true, entries: { item: true, visibility: true } },
} satisfies UserPreferencesGenqlSelection

type ConfirmedPreferences = FieldsSelection<UserPreferences, typeof preferenceSelection>
export type PreferenceDraft = Record<string, string | boolean>
const paths: Record<string, string> = {
  home: 'navigation.home',
  displayNames: 'memberDisplay.nameFormat',
  firstDay: 'calendar.firstDay',
  theme: 'theme.mode',
  lightTheme: 'theme.palettes.light',
  darkTheme: 'theme.palettes.dark',
  fontSize: 'typography.size',
  underlineLinks: 'accessibility.underlineLinks',
  reduceMotion: 'accessibility.reduceMotion',
  pointerCursors: 'accessibility.pointerCursors',
  emoticons: 'messageComposition.convertEmoticons',
  sendShortcut: 'messageComposition.sendShortcut',
  codeTheme: 'codeAppearance.theme',
  codeFont: 'codeAppearance.font.family',
  codeFontSize: 'codeAppearance.font.size',
  sidebarBadgeStyle: 'sidebar.badgeStyle',
  sidebarOrder: 'sidebar.order',
}
const aliases: Record<string, string> = {
  'Full name': 'FULL_NAME',
  'System preference': 'SYSTEM',
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
  'Always show': 'ALWAYS',
  'Show when badged': 'WHEN_BADGED',
  "Don't show": 'HIDDEN',
}
const wire = (value: string | boolean) => (typeof value === 'string' ? (aliases[value] ?? value.toUpperCase()) : value)
const sidebarItems = ['Providers', 'Models', 'Notebooks', 'Plugins']

export function parsePreferencePatch(draft: PreferenceDraft): {
  input?: UserPreferencesPatchInputFormOutput
  issues: { path: PropertyKey[]; message: string }[]
} {
  const patch: Record<string, unknown> = {}
  const fields = new Map<string, string>()
  for (const [key, value] of Object.entries(draft)) {
    if (key === 'codeLanguage') continue
    const item = sidebarItems.find((item) => key === `sidebar${item}`)
    const path = item
      ? `sidebar.visibility.${(get(patch, 'sidebar.visibility') as unknown[] | undefined)?.length ?? 0}`
      : paths[key]
    if (!path) throw new Error(`Unknown preference: ${key}`)
    fields.set(path, key)
    set(
      patch,
      path,
      item
        ? { item: item.toUpperCase(), visibility: wire(value) }
        : key === 'sidebarOrder'
          ? String(value)
              .split(',')
              .map((item) => item.toUpperCase())
          : wire(value),
    )
  }
  const result = UserPreferencesPatchInput.safeParse(patch)
  if (result.success) return { input: result.data, issues: [] }
  return {
    issues: result.error.issues.map((issue) => ({
      ...issue,
      path: ['preferences', [...fields].find(([path]) => issue.path.join('.').startsWith(path))?.[1] ?? Object.keys(draft)[0]],
    })),
  }
}

export function confirmedPreferenceDraft(node: ConfirmedPreferences, submitted: PreferenceDraft): PreferenceDraft {
  return Object.fromEntries(
    Object.entries(submitted).map(([key, draft]) => {
      if (key === 'codeLanguage') return [key, draft]
      if (key === 'sidebarOrder')
        return [
          key,
          node.sidebar.entries
            .map(({ item }) => sidebarItems.find((label) => label.toUpperCase() === item))
            .filter(Boolean)
            .join(','),
        ]
      const item = sidebarItems.find((item) => key === `sidebar${item}`)
      const value = item
        ? node.sidebar.entries.find((entry) => entry.item === item.toUpperCase())?.visibility
        : get(node, paths[key])
      if (value === undefined) throw new Error(`Preference save did not return ${key}`)
      if (wire(draft) === value) return [key, draft]
      const label = Object.entries(aliases).find(([, wire]) => wire === value)?.[0]
      return [
        key,
        typeof value === 'boolean' ? value : (label ?? String(value).slice(0, 1) + String(value).slice(1).toLowerCase()),
      ]
    }),
  )
}
