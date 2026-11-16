import type { FormStore } from '../../../components/form'
import type { OrganizationOnboardingSettingsInputFormOutput } from '@compallm/contract/validation'
export type OnboardingSettingsValues = OrganizationOnboardingSettingsInputFormOutput
export interface OrganizationOnboardingState {
  draft: OnboardingSettingsValues
  saved: OnboardingSettingsValues | null
  status: 'idle' | 'loading' | 'ready' | 'error' | 'denied'
}
export interface OrganizationOnboardingController {
  store: FormStore<OrganizationOnboardingState>
  load(): Promise<void>
  cancel(): void
  update(patch: Partial<OnboardingSettingsValues>, signal: AbortSignal): Promise<OnboardingSettingsValues>
}
