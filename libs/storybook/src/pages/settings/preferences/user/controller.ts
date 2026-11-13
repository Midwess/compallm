import { useContext, useSyncExternalStore } from 'react'
import { useFormStore, type FormStore } from '../../../../components/form'
import { UserPreferenceControllerContext } from '../../../../context'
import type { PreferenceDraft, UserPreferenceValues } from './values'

export interface UserPreferenceState {
  draft: PreferenceDraft
  saved: UserPreferenceValues | null
  status: 'idle' | 'loading' | 'ready' | 'error' | 'denied'
}
export interface UserPreferenceController {
  store: FormStore<UserPreferenceState>
  load(): Promise<void>
  cancel(): void
  update(patch: Partial<PreferenceDraft>, signal: AbortSignal): Promise<Partial<PreferenceDraft>>
}
export function useUserPreferenceController() {
  const controller = useContext(UserPreferenceControllerContext)
  if (!controller) throw new Error('User preferences require UserPreferenceControllerContext.')
  return controller
}
export function useUserPreference<T>(selector: (state: UserPreferenceState) => T, names: readonly string[] = []) {
  return useFormStore(useUserPreferenceController().store, selector, names)
}
const systemTheme = '(prefers-color-scheme: light)'
const subscribeTheme = (notify: () => void) => {
  const query = window.matchMedia(systemTheme)
  query.addEventListener('change', notify)
  return () => query.removeEventListener('change', notify)
}
export function useCodeAppearance() {
  const selected = useUserPreference((s) => s.draft.codeTheme, ['draft.codeTheme']),
    appTheme = useUserPreference((s) => s.draft.theme, ['draft.theme']),
    font = useUserPreference((s) => s.draft.codeFont, ['draft.codeFont']),
    size = useUserPreference((s) => s.draft.codeFontSize, ['draft.codeFontSize']),
    light = useSyncExternalStore(
      subscribeTheme,
      () => window.matchMedia(systemTheme).matches,
      () => false,
    )
  const effective = selected === 'FOLLOW_INTERFACE' ? appTheme : selected
  return {
    theme: (effective.includes('LIGHT') || (effective === 'SYSTEM' && light) ? 'light' : 'dark') as 'light' | 'dark',
    fontSize: Number(size.slice(2)),
    fontFamily: font === 'MONOSPACE' ? 'monospace' : undefined,
  }
}
