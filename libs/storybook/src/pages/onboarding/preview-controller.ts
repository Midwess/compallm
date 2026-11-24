import { createStore } from '../../state.svelte'
import type { OnboardingController, OnboardingState, OnboardingStep } from './types'
export class OnboardingPreviewController implements OnboardingController {
  readonly store
  constructor(initial: Partial<OnboardingState> = {}) {
    this.store = createStore<OnboardingState>(() => ({
      step: 'profile',
      invited: true,
      existingUser: false,
      profile: { displayName: 'Alex', username: 'alex', email: 'alex@example.com', avatarUrl: null },
      workType: '',
      appearance: 'SYSTEM',
      organization: { name: 'Workspace', welcome: '', links: [], destination: 'CHAT' },
      completed: false,
      error: null,
      embedded: true,
      ...initial,
    }))
  }
  private steps = (): OnboardingStep[] =>
    this.store.getState().existingUser ? ['introduction', 'ready'] : ['profile', 'work', 'preferences', 'introduction', 'ready']
  next = () => {
    const steps = this.steps()
    this.store.setState({ step: steps[Math.min(steps.indexOf(this.store.getState().step) + 1, steps.length - 1)] })
  }
  back = () => {
    const steps = this.steps()
    this.store.setState({ step: steps[Math.max(steps.indexOf(this.store.getState().step) - 1, 0)] })
  }
  skip = this.next
  finish = () => this.store.setState({ step: 'profile' })
  dismissError = () => this.store.setState({ error: null })
  saveProfile: OnboardingController['saveProfile'] = async () => this.store.getState().profile
  saveWorkspaceName: OnboardingController['saveWorkspaceName'] = async () => true
  saveWorkType: OnboardingController['saveWorkType'] = async () => true
  saveAppearance: OnboardingController['saveAppearance'] = async (appearance) => {
    this.store.setState({ appearance })
    return true
  }
}
