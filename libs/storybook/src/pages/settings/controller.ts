import { useContext, useSyncExternalStore } from 'react'
import { useStore } from 'zustand'
import { SettingsControllerContext } from '../../context'
import type { SettingsState } from './types'

export function useSettingsController() {
  const controller = useContext(SettingsControllerContext)
  if (!controller) throw new Error('SettingsPage requires SettingsControllerContext.')
  return controller
}
export function useSettings<T>(selector: (state: SettingsState) => T) {
  return useStore(useSettingsController().store, selector)
}

const systemTheme = '(prefers-color-scheme: light)'
const subscribeTheme = (notify: () => void) => {
  const query = window.matchMedia(systemTheme)
  query.addEventListener('change', notify)
  return () => query.removeEventListener('change', notify)
}
export function useCodeAppearance() {
  const selected = useSettings((s) => s.preferences.codeTheme),
    appTheme = useSettings((s) => s.preferences.theme),
    font = useSettings((s) => s.preferences.codeFont),
    size = useSettings((s) => s.preferences.codeFontSize),
    light = useSyncExternalStore(
      subscribeTheme,
      () => window.matchMedia(systemTheme).matches,
      () => false,
    )
  const effective = selected === 'System preference' ? appTheme : selected
  return {
    theme: (String(effective).includes('Light') || (effective === 'System preference' && light) ? 'light' : 'dark') as
      'light' | 'dark',
    fontSize: Number.parseInt(String(size)) || 12,
    fontFamily: font === 'Monospace' ? 'monospace' : undefined,
  }
}
