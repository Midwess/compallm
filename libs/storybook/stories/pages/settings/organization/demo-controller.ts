import type { OrganizationController, SettingsController } from '../../../../src/pages/settings'

export class DemoOrganizationController implements OrganizationController {
  constructor(
    private readonly store: SettingsController['store'],
    private readonly avatarUrl: (file: File | null) => string | null,
  ) {}
  updateOrganization = (value: Parameters<OrganizationController['updateOrganization']>[0]) =>
    this.store.setState((state) => ({ organization: { ...state.organization, ...value } }))
  setAvatar = (file: File | null) => this.updateOrganization({ logo: this.avatarUrl(file) })
}
