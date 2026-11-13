import {
  userPreferenceDefaults,
  userPreferenceHomeDestinations,
  userPreferenceSidebarDestinations,
} from '@compallm/contract/preference'
import type {
  FieldsSelection,
  UserPreferences,
  UserPreferencesGenqlSelection,
  PreferenceDestination,
} from '@compallm/contract/graphql'
import {
  UserPreferencesPatchInput,
  type UserPreferencesPatchInputFormOutput,
  PreferenceNamingFormat,
  PreferenceWeekday,
  PreferenceRelativeFontSize,
  PreferenceAppearanceMode,
  PreferenceLightPalette,
  PreferenceDarkPalette,
  PreferenceKeyCombination,
  PreferenceCodeTheme,
  PreferenceFontFamily,
  PreferencePixelFontSize,
  PreferenceBadgeStyle,
  PreferenceSidebarVisibility,
} from '@compallm/contract/validation'
import get from 'lodash-es/get.js'
import set from 'lodash-es/set.js'

export const preferenceSelection = {
  navigation: { home: true },
  memberDisplay: { nameFormat: true },
  calendar: { firstDay: true },
  theme: { mode: true, palettes: { light: true, dark: true } },
  typography: { family: true, size: true },
  accessibility: { underlineLinks: true, reduceMotion: true, pointerCursors: true },
  messageComposition: { convertEmoticons: true, sendShortcut: true },
  codeAppearance: { theme: true, font: { family: true, size: true } },
  sidebar: { badgeStyle: true, entries: { item: true, visibility: true } },
} satisfies UserPreferencesGenqlSelection

export type UserPreferenceValues = FieldsSelection<UserPreferences, typeof preferenceSelection>
const paths = {
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
} as const

type ValueAt<T, P extends string> = P extends `${infer K}.${infer Rest}`
  ? K extends keyof T
    ? ValueAt<T[K], Rest>
    : never
  : P extends keyof T
    ? T[P]
    : never
export type PreferenceDraft = { [K in keyof typeof paths]: ValueAt<UserPreferences, (typeof paths)[K]> } & {
  sidebarOrder: PreferenceDestination[]
  codeLanguage: string
} & {
  [
    K in (typeof userPreferenceSidebarDestinations)[number] as `sidebar${K}`
  ]: UserPreferences['sidebar']['entries'][number]['visibility']
}

export const preferenceLabel = (value: string): string => {
  const labels: Record<string, string> = {
    SYSTEM: 'System preference',
    MODIFIER_ENTER: '⌘ + Enter',
    ALWAYS: 'Always show',
    WHEN_BADGED: 'Show when badged',
    HIDDEN: "Don't show",
    FULL_NAME: 'Full name',
  }
  if (labels[value]) return labels[value]
  if (/^PX[0-9]+$/.test(value)) return `${value.slice(2)}px`
  return value
    .toLowerCase()
    .split('_')
    .map((part) => part[0].toUpperCase() + part.slice(1))
    .join(' ')
}
const options = <T extends string>(values: readonly T[]) => values.map((value) => ({ value, label: preferenceLabel(value) }))
export const preferenceOptions = {
  home: options(userPreferenceHomeDestinations),
  displayNames: options(PreferenceNamingFormat.options),
  firstDay: options(PreferenceWeekday.options),
  fontSize: options(PreferenceRelativeFontSize.options),
  theme: options(PreferenceAppearanceMode.options),
  lightTheme: options(PreferenceLightPalette.options),
  darkTheme: options(PreferenceDarkPalette.options),
  sendShortcut: options(PreferenceKeyCombination.options),
  codeTheme: options(PreferenceCodeTheme.options),
  codeFont: options(PreferenceFontFamily.options),
  codeFontSize: options(PreferencePixelFontSize.options),
  sidebarBadgeStyle: options(PreferenceBadgeStyle.options),
  sidebarVisibility: options(PreferenceSidebarVisibility.options),
}
export function initialPreferenceDraft(): PreferenceDraft {
  return {
    ...Object.fromEntries(Object.entries(paths).map(([key, path]) => [key, get(userPreferenceDefaults, path)])),
    sidebarOrder: [...userPreferenceSidebarDestinations],
    codeLanguage: 'typescript',
    ...Object.fromEntries(userPreferenceDefaults.sidebar.entries.map(({ item, visibility }) => [`sidebar${item}`, visibility])),
  } as PreferenceDraft
}

export function parsePreferencePatch(draft: Partial<PreferenceDraft>): {
  input?: UserPreferencesPatchInputFormOutput
  issues: { path: PropertyKey[]; message: string }[]
} {
  const patch: Record<string, unknown> = {}
  const fields = new Map<string, string>()
  for (const [key, value] of Object.entries(draft)) {
    if (key === 'codeLanguage') continue
    const item = userPreferenceSidebarDestinations.find((item) => key === `sidebar${item}`)
    const path = item
      ? `sidebar.visibility.${(get(patch, 'sidebar.visibility') as unknown[] | undefined)?.length ?? 0}`
      : key === 'sidebarOrder'
        ? 'sidebar.order'
        : paths[key as keyof typeof paths]
    if (!path) throw new Error(`Unknown preference: ${key}`)
    fields.set(path, key)
    set(patch, path, item ? { item, visibility: value } : value)
  }
  const result = UserPreferencesPatchInput.safeParse(patch)
  if (result.success) return { input: result.data, issues: [] }
  return {
    issues: result.error.issues.map((issue) => ({
      ...issue,
      path: ['draft', [...fields].find(([path]) => issue.path.join('.').startsWith(path))?.[1] ?? Object.keys(draft)[0]],
    })),
  }
}

export function confirmedPreferenceDraft(
  node: UserPreferenceValues,
  submitted: Partial<PreferenceDraft>,
): Partial<PreferenceDraft> {
  return Object.fromEntries(
    Object.entries(submitted).map(([key, draft]) => {
      if (key === 'codeLanguage') return [key, draft]
      if (key === 'sidebarOrder') return [key, node.sidebar.entries.map(({ item }) => item)]
      const item = userPreferenceSidebarDestinations.find((item) => key === `sidebar${item}`)
      const value = item
        ? node.sidebar.entries.find((entry) => entry.item === item)?.visibility
        : get(node, paths[key as keyof typeof paths])
      if (value === undefined) throw new Error(`Preference save did not return ${key}`)
      return [key, value]
    }),
  )
}

export function loadedPreferenceDraft(node: UserPreferenceValues, language = 'typescript'): PreferenceDraft {
  return { ...initialPreferenceDraft(), ...confirmedPreferenceDraft(node, initialPreferenceDraft()), codeLanguage: language }
}
