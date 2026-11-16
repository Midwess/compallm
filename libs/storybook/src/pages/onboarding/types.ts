import type { StoreApi } from 'zustand/vanilla'
import type { PreferenceAppearanceMode, User } from '@compallm/contract/graphql'
import type { UserProfileUpdateInputFormOutput } from '@compallm/contract/validation'

export type OnboardingStep =
  'welcome' | 'profile' | 'workspace' | 'work' | 'preferences' | 'explore' | 'organization' | 'introduction' | 'ready'
export type OnboardingProfile = Pick<User, 'displayName' | 'username' | 'email' | 'avatarUrl'>
export interface OnboardingOrganization {
  name: string
  welcome: string
  links: { label: string; href: string }[]
  destination: string
}
export interface OnboardingState {
  step: OnboardingStep
  invited: boolean
  existingUser: boolean
  profile: OnboardingProfile
  workType: string
  appearance: PreferenceAppearanceMode
  organization: OnboardingOrganization
  completed: boolean
  error: string | null
}
export interface OnboardingController {
  store: StoreApi<OnboardingState>
  next(): void
  back(): void
  skip(): void
  finish(): void
  dismissError(): void
  saveProfile(
    values: Pick<UserProfileUpdateInputFormOutput, 'displayName' | 'username'>,
    photo: File | null,
    signal: AbortSignal,
  ): Promise<OnboardingProfile | false>
  saveWorkspaceName(value: string, signal: AbortSignal): Promise<boolean>
  saveWorkType(value: string, signal: AbortSignal): Promise<boolean>
  saveAppearance(value: PreferenceAppearanceMode, signal: AbortSignal): Promise<boolean>
}
