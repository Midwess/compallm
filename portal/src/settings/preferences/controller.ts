import type { SettingsPageControllers, SettingsState } from '@compallm/storybook/pages/settings'
import type { StoreApi } from 'zustand/vanilla'
import { notify } from '../../notifications/toast'

type Contract = SettingsPageControllers['preferences']
export class PreferencesController implements Contract {
  private readonly store: StoreApi<SettingsState>
  constructor(store: StoreApi<SettingsState>) {
    this.store = store
  }
  setPreference = (key: string, value: string | boolean) => {
    if (!['codeTheme', 'codeFont', 'codeFontSize', 'codeLanguage'].includes(key)) {
      notify.info('Settings changes are not available yet.')
      return
    }
    this.store.setState((state) => ({ preferences: { ...state.preferences, [key]: value } }))
  }
}
