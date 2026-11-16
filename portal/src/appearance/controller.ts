import type { FieldsSelection, PreferenceRelativeFontSize, ThemePreferences } from '@compallm/contract/graphql'
import { userPreferenceDefaults } from '@compallm/contract/preference'

export class AppearanceController {
  private theme: FieldsSelection<ThemePreferences, { mode: true; palettes: { light: true; dark: true } }> =
    userPreferenceDefaults.theme

  start = () => {
    const media = window.matchMedia('(prefers-color-scheme: dark)')
    const apply = () => this.applyTheme(media.matches)
    media.addEventListener('change', apply)
    apply()
    return () => media.removeEventListener('change', apply)
  }

  setTheme = (theme: typeof this.theme) => {
    this.theme = theme
    if (typeof window !== 'undefined') this.applyTheme(window.matchMedia('(prefers-color-scheme: dark)').matches)
  }

  private applyTheme = (systemDark: boolean) => {
    const dark = this.theme.mode === 'DARK' || (this.theme.mode === 'SYSTEM' && systemDark)
    const root = document.documentElement
    root.dataset.theme = this.theme.palettes[dark ? 'dark' : 'light'].toLowerCase().replaceAll('_', '-')
    root.dataset.appearance = dark ? 'dark' : 'light'
    root.classList.toggle('dark', dark)
  }

  setTextSize = (size: PreferenceRelativeFontSize) => {
    if (typeof document !== 'undefined') document.documentElement.dataset.textSize = size.toLowerCase()
  }

  setUnderlineLinks = (enabled: boolean) => {
    if (typeof document !== 'undefined') document.documentElement.dataset.underlineLinks = String(enabled)
  }
}

export const appearanceController = new AppearanceController()
