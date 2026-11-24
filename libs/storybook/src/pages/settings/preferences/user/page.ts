import { preferenceOptions } from './values'

export type Preference = {
  key: string
  label: string
  description: string
  options?: {
    value: string
    label: string
  }[]
}
export const general: Preference[] = [
  {
    key: 'home',
    label: 'Default home view',
    description: 'The first page you see when you open Compallm.',
    options: preferenceOptions.home,
  },
  {
    key: 'displayNames',
    label: 'Display names',
    description: 'Choose how member names appear. Applying this preference is not available yet.',
    options: preferenceOptions.displayNames,
  },
  {
    key: 'firstDay',
    label: 'First day of the week',
    description: 'Used in calendars and reporting periods. Applying this preference is not available yet.',
    options: preferenceOptions.firstDay,
  },
]
export const appearance: Preference[] = [
  {
    key: 'fontSize',
    label: 'Font size',
    description: 'Adjust the size of interface text.',
    options: preferenceOptions.fontSize,
  },
  { key: 'pointerCursors', label: 'Use pointer cursors', description: 'Use a pointer cursor on interactive elements.' },
  { key: 'underlineLinks', label: 'Underline links', description: 'Always underline links in message content.' },
  { key: 'reduceMotion', label: 'Reduce motion', description: 'Keep animated images and emoji still.' },
]
export const themes: Preference[] = [
  {
    key: 'theme',
    label: 'Appearance',
    description: 'Choose your preferred color scheme.',
    options: preferenceOptions.theme,
  },
  {
    key: 'lightTheme',
    label: 'Light theme',
    description: 'Used when your system has a light appearance.',
    options: preferenceOptions.lightTheme,
  },
  {
    key: 'darkTheme',
    label: 'Dark theme',
    description: 'Used when your system has a dark appearance.',
    options: preferenceOptions.darkTheme,
  },
]
export { default as PreferenceControl } from './page-PreferenceControl.svelte'
export { default as PreferenceRows } from './page-PreferenceRows.svelte'
export { default as GeneralSettings } from './page-GeneralSettings.svelte'
export { default as MessagingSettings } from './page-MessagingSettings.svelte'
export { default as CodeAppearanceSection } from './page-CodeAppearanceSection.svelte'
export { default as CodeSettings } from './page-CodeSettings.svelte'
