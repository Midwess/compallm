import { createFormStore } from '../../../../src/components/form'
import type { OrganizationOnboardingController, OrganizationOnboardingState } from '../../../../src/pages/settings'
export class DemoOrganizationOnboardingController implements OrganizationOnboardingController {
  readonly store = createFormStore<OrganizationOnboardingState>(
    {
      draft: {
        welcome: 'A little space for our next big idea. Welcome to the team.',
        links: [{ label: 'Team handbook', href: 'https://example.com/handbook' }],
        destination: 'CHAT',
        artworkColor: null,
        requireFullName: false,
        requireAvatar: false,
      },
      saved: null,
      status: 'idle',
    },
    ['draft'],
  )
  load = async () => {
    this.store.setState({ saved: this.store.form.values.draft!, status: 'ready' })
  }
  cancel = () => this.store.form.stop()
  update: OrganizationOnboardingController['update'] = async (patch) => {
    const saved = { ...this.store.getState().saved!, ...patch }
    this.store.setState({ saved })
    return saved
  }
}
