import { useContext, useSyncExternalStore } from 'react'
import { useFormStore } from '../../components/form'
import { SettingsControllerContext } from '../../context'
import type { SettingsState } from './types'

export function useSettingsController() {
  const controller = useContext(SettingsControllerContext)
  if (!controller) throw new Error('SettingsPage requires SettingsControllerContext.')
  return controller
}
export function useSettings<T>(selector: (state: SettingsState) => T, names: readonly string[] = []) {
  return useFormStore(useSettingsController().store, selector, names)
}

const systemTheme = '(prefers-color-scheme: light)'
const subscribeTheme = (notify: () => void) => {
  const query = window.matchMedia(systemTheme)
  query.addEventListener('change', notify)
  return () => query.removeEventListener('change', notify)
}
export function useCodeAppearance() {
  const selected = useSettings((s) => s.preferences.codeTheme, ['preferences.codeTheme']),
    appTheme = useSettings((s) => s.preferences.theme, ['preferences.theme']),
    font = useSettings((s) => s.preferences.codeFont, ['preferences.codeFont']),
    size = useSettings((s) => s.preferences.codeFontSize, ['preferences.codeFontSize']),
    light = useSyncExternalStore(
      subscribeTheme,
      () => window.matchMedia(systemTheme).matches,
      () => false,
    )
  const effective = selected === 'FOLLOW_INTERFACE' ? appTheme : selected
  return {
    theme: (String(effective).includes('LIGHT') || (effective === 'SYSTEM' && light) ? 'light' : 'dark') as 'light' | 'dark',
    fontSize: Number(size.slice(2)),
    fontFamily: font === 'MONOSPACE' ? 'monospace' : undefined,
  }
}
