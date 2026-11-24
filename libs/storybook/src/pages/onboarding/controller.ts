import type { OnboardingState } from './types'

import { getOnboardingController } from '../../context'
import { readStore } from '../page-state.svelte'
export function useOnboardingController() {
  return getOnboardingController()
}
export function useOnboarding<T>(selector: (state: OnboardingState) => T, names: readonly string[] = []) {
  return readStore(useOnboardingController().store, selector, names)
}
