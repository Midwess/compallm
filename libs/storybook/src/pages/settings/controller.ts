import type { SettingsState } from './types'

import { getSettingsController } from '../../context'
import { readStore } from '../page-state.svelte'
export function useSettingsController() {
  return getSettingsController()
}
export function useSettings<T>(selector: (state: SettingsState) => T, names: readonly string[] = []) {
  return readStore(useSettingsController().store, selector, names)
}
