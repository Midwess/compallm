import type { ProfileController as Contract } from '@compallm/storybook/pages/settings'

export class ProfileController implements Contract {
  updateProfile = () => false as const
  setAvatar = () => false as const
}
