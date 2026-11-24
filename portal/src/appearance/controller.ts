import type { PreferenceRelativeFontSize$options as PreferenceRelativeFontSize } from '@compallm/contract/houdini'
type ThemePreferences =
  import('@compallm/contract/houdini').PreferenceUserUserPreferences$result['userPreferences']['node']['theme']
import { userPreferenceDefaults } from '@compallm/contract/preference'

export class AppearanceController {
  private theme: ThemePreferences = userPreferenceDefaults.theme

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
    document.cookie = `compallm-appearance=${dark ? 'dark' : 'light'}; Path=/; SameSite=Lax; Max-Age=31536000${window.location.protocol === 'https:' ? '; Secure' : ''}`
  }

  setTextSize = (size: PreferenceRelativeFontSize) => {
    if (typeof document !== 'undefined') document.documentElement.dataset.textSize = size.toLowerCase()
  }

  setUnderlineLinks = (enabled: boolean) => {
    if (typeof document !== 'undefined') document.documentElement.dataset.underlineLinks = String(enabled)
  }
}

export const appearanceController = new AppearanceController()
