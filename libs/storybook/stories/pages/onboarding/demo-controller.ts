import { createStore } from 'zustand/vanilla'
import { UserProfileUpdateInput, OrganizationProfileUpdateInput } from '@compallm/contract/validation'
import type { OnboardingController, OnboardingState, OnboardingStep } from '../../../src/pages/onboarding'

export class DemoOnboardingController implements OnboardingController {
  readonly store
  private photo: string | null = null
  constructor(
    initial: Partial<OnboardingState> = {},
    private rejectNext = false,
  ) {
    this.store = createStore<OnboardingState>(() => ({
      step: initial.existingUser ? 'organization' : 'welcome',
      invited: false,
      existingUser: false,
      profile: { displayName: 'Alex', username: 'alex', email: 'alex@example.com', avatarUrl: null },
      workType: '',
      appearance: 'SYSTEM',
      organization: { name: 'Alex-workspace', welcome: '', links: [], destination: 'Chat' },
      completed: false,
      error: null,
      ...initial,
    }))
  }
  private steps(): OnboardingStep[] {
    const { invited, existingUser } = this.store.getState()
    return [
      ...(existingUser ? [] : (['welcome', ...(invited ? [] : ['workspace']), 'work', 'preferences'] as OnboardingStep[])),
      ...(invited ? (['organization', 'introduction'] as OnboardingStep[]) : []),
      'ready',
    ]
  }
  next = () => {
    const steps = this.steps(),
      step = this.store.getState().step
    this.store.setState({ step: steps[Math.min(steps.indexOf(step) + 1, steps.length - 1)], error: null })
  }
  back = () => {
    const steps = this.steps()
    this.store.setState({ step: steps[Math.max(0, steps.indexOf(this.store.getState().step) - 1)], error: null })
  }
  skip = () => this.next()
  finish = () => this.store.setState({ completed: true })
  dismissError = () => this.store.setState({ error: null })
  private accepts(signal: AbortSignal) {
    if (signal.aborted) return false
    if (this.rejectNext) {
      this.rejectNext = false
      this.store.setState({ error: 'Couldn’t save your changes. Please try Continue again.' })
      return false
    }
    return true
  }
  saveProfile: OnboardingController['saveProfile'] = async (values, file, signal) => {
    if (!this.accepts(signal)) return false
    const parsed = UserProfileUpdateInput.parse(values)
    if (file) {
      if (this.photo) URL.revokeObjectURL(this.photo)
      this.photo = URL.createObjectURL(file)
    }
    const profile = {
      ...this.store.getState().profile,
      displayName: parsed.displayName ?? this.store.getState().profile.displayName,
      username: parsed.username ?? this.store.getState().profile.username,
      avatarUrl: this.photo ?? this.store.getState().profile.avatarUrl,
    }
    this.store.setState({ profile })
    return profile
  }
  saveWorkspaceName: OnboardingController['saveWorkspaceName'] = async (name, signal) => {
    if (!this.accepts(signal)) return false
    const parsed = OrganizationProfileUpdateInput.parse({ name })
    this.store.setState({ organization: { ...this.store.getState().organization, name: parsed.name! } })
    return true
  }
  saveWorkType: OnboardingController['saveWorkType'] = async (workType, signal) => {
    if (!this.accepts(signal)) return false
    this.store.setState({ workType })
    return true
  }
  saveAppearance: OnboardingController['saveAppearance'] = async (appearance, signal) => {
    if (!this.accepts(signal)) return false
    this.store.setState({ appearance })
    return true
  }
  stop = () => {
    if (this.photo) URL.revokeObjectURL(this.photo)
  }
}
