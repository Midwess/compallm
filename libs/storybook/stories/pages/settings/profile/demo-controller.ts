import type { ProfileController, SettingsController } from '../../../../src/pages/settings'

export class DemoProfileController implements ProfileController {
  constructor(
    private readonly store: SettingsController['store'],
    private readonly avatarUrl: (file: File | null) => string | null,
  ) {}
  saveProfile = (value: Parameters<ProfileController['saveProfile']>[0], file: File | null) => {
    const profile = { ...this.store.getState().profile, ...value }
    if (file) profile.avatar = this.avatarUrl(file)
    this.store.form.set('profile.avatar', profile.avatar)
    this.store.form.accept({ profile: { ...this.store.form.baseline.profile!, avatar: profile.avatar } })
    return profile
  }
}
