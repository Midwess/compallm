import { createFormStore } from '@compallm/storybook'
import type {
  OrganizationOnboardingController as Contract,
  OrganizationOnboardingState,
  OnboardingSettingsValues,
} from '@compallm/storybook/pages/settings'
import type { QueryGenqlSelection, MutationGenqlSelection } from '@compallm/contract/graphql'
import { OrganizationOnboardingSettingsInput } from '@compallm/contract/validation'
import { graphql, ApiError } from '../../graphql-client'
import { onboardingSelection } from '../../onboarding/controller'
export class OrganizationOnboardingController implements Contract {
  readonly store = createFormStore<OrganizationOnboardingState>(
    {
      draft: { welcome: '', links: [], destination: 'CHAT', artworkColor: null, requireFullName: false, requireAvatar: false },
      saved: null,
      status: 'idle',
    },
    ['draft'],
  )
  private scope = new AbortController()
  private read?: AbortController
  cancel = () => {
    this.scope.abort()
    this.read?.abort()
    this.store.form.stop()
  }
  load = async () => {
    if (this.scope.signal.aborted) this.scope = new AbortController()
    const scope = this.scope
    this.read?.abort()
    const read = new AbortController()
    this.read = read
    const signal = AbortSignal.any([scope.signal, read.signal])
    if (this.store.getState().saved && this.store.form.dirty) return
    this.store.setState({ status: 'loading' })
    try {
      const selection = { organizationOnboardingSettings: { node: onboardingSelection.settings } } satisfies QueryGenqlSelection
      const result = await graphql(signal).query(selection)
      if (signal.aborted || this.read !== read) return
      const saved = OrganizationOnboardingSettingsInput.parse(result.organizationOnboardingSettings.node)
      this.store.form.reset({ draft: saved })
      this.store.setState({ saved, status: 'ready' })
    } catch (error) {
      if (!signal.aborted)
        this.store.setState({ status: error instanceof ApiError && [401, 403].includes(error.status) ? 'denied' : 'error' })
    }
  }
  update = async (patch: Partial<OnboardingSettingsValues>, signal: AbortSignal) => {
    const scope = this.scope
    this.read?.abort()
    const saved = this.store.getState().saved
    if (!saved) throw new Error('Onboarding settings are unavailable')
    const input = OrganizationOnboardingSettingsInput.parse({
      ...saved,
      ...patch,
      ...(patch.links ? { links: patch.links } : {}),
    })
    const selection = {
      organizationOnboardingSettingsUpdate: { __args: { input }, node: onboardingSelection.settings },
    } satisfies MutationGenqlSelection
    const result = await graphql(AbortSignal.any([scope.signal, signal])).mutation(selection)
    if (scope.signal.aborted || signal.aborted) throw new DOMException('Cancelled', 'AbortError')
    const confirmed = OrganizationOnboardingSettingsInput.parse(result.organizationOnboardingSettingsUpdate.node)
    this.store.setState({ saved: confirmed })
    return confirmed
  }
}
