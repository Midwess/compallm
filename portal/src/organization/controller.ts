import { createOrganizationStore, type OrganizationController as Controller } from '@compallm/storybook/pages/organization'
import type { AuthController } from '@compallm/storybook'
import find from 'lodash-es/find'
import type { CurrentUserResponse } from '../contract/api'
import { notify } from '../notifications/toast'
import { organizationHome } from './paths'

export class OrganizationController implements Controller {
  readonly store = createOrganizationStore()
  private readonly navigate: (path: string) => void
  private chatPath = ''
  private readonly auth: Pick<AuthController, 'signOut'>

  constructor(user: CurrentUserResponse, navigate: (path: string) => void, auth: Pick<AuthController, 'signOut'>) {
    this.navigate = navigate
    this.auth = auth
    this.setCurrentUser(user)
  }

  getCurrentOrganization = () => {
    const state = this.store.getState()
    return find(state.organizations, { id: state.organizationId }) ?? null
  }

  setCurrentUser = (user: CurrentUserResponse) => {
    const id = String(user.organizationId)
    this.chatPath = organizationHome(user)
    const state = this.store.getState()
    if (state.organizationId === id && state.organizations[0]?.name === user.organizationName) return
    this.store.setState({
      organizationId: id,
      organizations: [{ id, name: user.organizationName }],
      ...(state.organizationId !== id ? { providers: [], models: [], conversations: [], selectedDestination: null } : {}),
    })
  }

  onOpenChat = () => this.navigate(this.chatPath)
  onSelectOrganization = (id: string) => {
    if (id !== this.store.getState().organizationId) notify.info('Organization switching is not available yet.')
  }
  onSignOut = async () => {
    await this.auth.signOut()
  }
  onOpenSettings = () => {
    notify.info('Settings are not available yet.')
  }
  onSearch = () => {
    notify.info('Workspace search is not available yet.')
  }
  onOpenDestination = () => {
    notify.info('This feature is not available yet.')
  }
  onCreateProvider = () => {
    notify.info('Provider setup is not available yet.')
  }
  onCreateRouter = () => {
    notify.info('Router setup is not available yet.')
  }
  onCreateModel = () => {
    notify.info('Model setup is not available yet.')
  }
}
