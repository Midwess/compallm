import type { OrganizationController, SettingsController } from '../../../../src/pages/settings'

export class DemoOrganizationController implements OrganizationController {
  constructor(
    private readonly store: SettingsController['store'],
    private readonly avatarUrl: (file: File | null) => string | null,
  ) {}
  saveProfile = (
    value: Partial<Pick<import('../../../../src/pages/settings/types').SettingsOrganization, 'name'>>,
    file: File | null,
  ) => {
    const organization = { ...this.store.getState().organization, ...value }
    if (file) organization.logo = this.avatarUrl(file)
    this.store.form.set('organization.logo', organization.logo)
    this.store.form.accept({ organization: { ...this.store.form.baseline.organization!, logo: organization.logo } })
    return organization
  }
  updateOrganization = (value: Parameters<OrganizationController['updateOrganization']>[0]) =>
    this.store.setState((state) => ({ organization: { ...state.organization, ...value } }))
  setAvatar = (file: File | null) => this.updateOrganization({ logo: this.avatarUrl(file) })
}
