import { type FormStore } from '../../../../components/form'
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

import { getUserPreferenceController } from '../../../../context'
import { readStore, systemLight } from '../../../page-state.svelte'
import { resolveCodeAppearance } from '../../../../components/code-appearance'
export const useUserPreferenceController = getUserPreferenceController
export function useUserPreference<T>(selector: (state: UserPreferenceState) => T, names: readonly string[] = []) {
  return readStore(getUserPreferenceController().store, selector, names)
}
export function useCodeAppearance() {
  const draft = getUserPreferenceController().store.getState().draft
  return resolveCodeAppearance(draft.codeTheme, draft.theme, draft.codeFont, draft.codeFontSize, systemLight())
}
