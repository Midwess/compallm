import type { CreateProviderState } from './types'

import { getCreateProviderController } from '../../../context'
import { readStore } from '../../page-state.svelte'
export function useController() {
  return getCreateProviderController()
}
export function useProvider<T>(selector: (state: CreateProviderState) => T, names: readonly string[] = []) {
  return readStore(useController().store, selector, names)
}
