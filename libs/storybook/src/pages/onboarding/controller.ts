import { useContext } from 'react'
import { useStore } from 'zustand'
import { OnboardingControllerContext } from '../../context'
import type { OnboardingState } from './types'

export function useOnboardingController() {
  const controller = useContext(OnboardingControllerContext)
  if (!controller) throw new Error('Onboarding requires OnboardingControllerContext.')
  return controller
}
export function useOnboarding<T>(selector: (state: OnboardingState) => T) {
  return useStore(useOnboardingController().store, selector)
}
