import type { ProfileController, SettingsController } from '../../../../src/pages/settings'

export class DemoProfileController implements ProfileController {
  constructor(
    private readonly store: SettingsController['store'],
    private readonly avatarUrl: (file: File | null) => string | null,
  ) {}
  updateProfile = (value: Parameters<ProfileController['updateProfile']>[0]) =>
    this.store.setState((state) => ({ profile: { ...state.profile, ...value } }))
  setAvatar = (file: File | null) => this.updateProfile({ avatar: this.avatarUrl(file) })
}
