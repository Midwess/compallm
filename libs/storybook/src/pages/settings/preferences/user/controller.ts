import { useContext } from 'react'
import { resolveCodeAppearance, useSystemLight } from '../../../../components/code-appearance'
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
export function useCodeAppearance() {
  const selected = useUserPreference((s) => s.draft.codeTheme, ['draft.codeTheme']),
    appTheme = useUserPreference((s) => s.draft.theme, ['draft.theme']),
    font = useUserPreference((s) => s.draft.codeFont, ['draft.codeFont']),
    size = useUserPreference((s) => s.draft.codeFontSize, ['draft.codeFontSize']),
    light = useSystemLight()
  return resolveCodeAppearance(selected, appTheme, font, size, light)
}
