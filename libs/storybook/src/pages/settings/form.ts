import { parseSettings } from './validation'
import type { SettingsController } from './types'

export function configureSettingsForm(c: SettingsController) {
  const form = c.store.form
  let active = true
  form.configure({
    mode: 'change',
    parse: parseSettings,
    error: (error, retry) =>
      c.apiError(error, () => {
        if (active) retry()
      }),
    fields: {
      'dialog.name': { mode: 'button' },
      'dialog.value': { mode: 'button' },
      'dialog.phrase': { mode: 'button' },
    },
    submit: async (values, _patch, _signal, names) => {
      for (const name of names) {
        const [root, key] = name.split('.')
        if (root === 'profile' && key) {
          if ((await c.pages.profile.updateProfile({ [key]: values.profile![key as keyof typeof values.profile] })) === false)
            return false
        } else if (root === 'organization' && key) {
          if (
            (await c.pages.organization.updateOrganization({
              [key]: values.organization![key as keyof typeof values.organization],
            })) === false
          )
            return false
        } else if (root === 'profileUpload') {
          if ((await c.pages.profile.setAvatar(values.profileUpload ?? null)) === false) return false
        } else if (root === 'organizationUpload') {
          if ((await c.pages.organization.setAvatar(values.organizationUpload ?? null)) === false) return false
        }
      }
      return values
    },
  })
  return () => {
    active = false
    form.stop()
  }
}
