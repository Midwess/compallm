import type { SettingsRouteData } from '../load'
import {
  SettingsOnboardingOrganizationOnboardingSettingsStore,
  SettingsOnboardingOrganizationOnboardingSettingsUpdateStore,
} from '@compallm/contract/houdini'
import { createFormStore } from '@compallm/storybook'
import type {
  OrganizationOnboardingController as Contract,
  OrganizationOnboardingState,
  OnboardingSettingsValues,
} from '@compallm/storybook/pages/settings'

import { OrganizationOnboardingSettingsInput } from '@compallm/contract/validation'
import { graphqlClient, requireData, ApiError } from '../../graphql-client'

function settingsValues(
  value: import('@compallm/contract/houdini').SettingsOnboardingOrganizationOnboardingSettings$result['organizationOnboardingSettings']['node'],
) {
  return OrganizationOnboardingSettingsInput.parse({
    welcome: value.welcome,
    links: value.links.map(({ label, href }) => ({ label, href })),
    destination: value.destination,
    artworkColor: value.artworkColor,
    requireFullName: value.requireFullName,
    requireAvatar: value.requireAvatar,
  })
}
export class OrganizationOnboardingController implements Contract {
  readonly store = createFormStore<OrganizationOnboardingState>(
    {
      draft: { welcome: '', links: [], destination: 'CHAT', artworkColor: null, requireFullName: false, requireAvatar: false },
      saved: null,
      status: 'idle',
    },
    ['draft'],
  )
  private preparation?: SettingsRouteData
  routeData?: () => SettingsRouteData | undefined
  private scope = new AbortController()
  private read?: AbortController
  seed(data: SettingsRouteData) {
    if (!data.onboarding) return
    const saved = settingsValues(data.onboarding.organizationOnboardingSettings.node)
    this.store.form.reset({ draft: saved })
    this.store.setState({ saved, status: 'ready' })
    this.preparation = data
    this.seeded = true
  }
  private seeded = false
  cancel = () => {
    this.scope.abort()
    this.read?.abort()
    this.store.form.stop()
  }
  load = async () => {
    if (this.seeded) {
      this.seeded = false
      return
    }
    if (this.scope.signal.aborted) this.scope = new AbortController()
    const scope = this.scope
    this.read?.abort()
    const read = new AbortController()
    this.read = read
    const signal = AbortSignal.any([scope.signal, read.signal])
    if (this.store.getState().saved && this.store.form.dirty) return
    this.store.setState({ status: 'loading' })
    try {
      const prepared = this.routeData?.()
      const usable =
        prepared && prepared !== this.preparation && prepared.version === graphqlClient.readVersion && prepared.onboarding
      this.preparation = prepared
      const result = usable
        ? prepared.onboarding!
        : await new SettingsOnboardingOrganizationOnboardingSettingsStore()
            .fetch({ fetch: graphqlClient.fetch(signal), policy: 'NoCache', blocking: true })
            .then(requireData)
      if (signal.aborted || this.read !== read) return
      const saved = settingsValues(result.organizationOnboardingSettings.node)
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

    const result = await new SettingsOnboardingOrganizationOnboardingSettingsUpdateStore()
      .mutate(
        { organizationOnboardingSettingsUpdateInput: input },
        { fetch: graphqlClient.fetch(AbortSignal.any([scope.signal, signal])) },
      )
      .then(requireData)
    if (scope.signal.aborted || signal.aborted) throw new DOMException('Cancelled', 'AbortError')
    const confirmed = settingsValues(result.organizationOnboardingSettingsUpdate.node)
    this.store.setState({ saved: confirmed })
    return confirmed
  }
}
