// Generated from Rust. Do not edit; run cargo run -p compallm-api -- generate.
import type { PreferenceDestination, UserPreferences } from './graphql.ts'
type PreferenceValues<T> = T extends readonly (infer U)[]
  ? readonly PreferenceValues<U>[]
  : T extends object
    ? { [K in keyof T as K extends '__typename' ? never : K]: PreferenceValues<T[K]> }
    : T
export const userPreferenceDefaults = {
  navigation: {
    home: 'CHAT',
  },
  sidebar: {
    badgeStyle: 'DOT',
    entries: [
      {
        item: 'PROVIDERS',
        visibility: 'ALWAYS',
      },
      {
        item: 'MODELS',
        visibility: 'ALWAYS',
      },
      {
        item: 'NOTEBOOKS',
        visibility: 'ALWAYS',
      },
      {
        item: 'PLUGINS',
        visibility: 'ALWAYS',
      },
    ],
  },
  memberDisplay: {
    nameFormat: 'FULL_NAME',
  },
  calendar: {
    firstDay: 'MONDAY',
  },
  theme: {
    mode: 'SYSTEM',
    palettes: {
      light: 'LIGHT',
      dark: 'DARK',
    },
  },
  typography: {
    family: 'DEFAULT',
    size: 'DEFAULT',
  },
  accessibility: {
    underlineLinks: false,
    reduceMotion: true,
    pointerCursors: false,
  },
  messageComposition: {
    convertEmoticons: true,
    sendShortcut: 'MODIFIER_ENTER',
  },
  codeAppearance: {
    theme: 'FOLLOW_INTERFACE',
    font: {
      family: 'DEFAULT',
      size: 'PX12',
    },
  },
} as const satisfies PreferenceValues<UserPreferences>
export const userPreferenceHomeDestinations = [
  'CHAT',
  'MODELS',
  'PROVIDERS',
] as const satisfies readonly PreferenceDestination[]
export const userPreferenceSidebarDestinations = [
  'PROVIDERS',
  'MODELS',
  'NOTEBOOKS',
  'PLUGINS',
] as const satisfies readonly PreferenceDestination[]
