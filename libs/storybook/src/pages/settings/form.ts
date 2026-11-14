import { parseSettings } from './validation'
import type { SettingsController } from './types'

export function configureSettingsForm(c: SettingsController) {
  const form = c.store.form
  let active = true
  if (c.store.getState().location.page === 'profile') {
    form.configure({
      mode: 'blur',
      names: ['profile.name', 'profile.username', 'profileUpload'],
      fields: { profileUpload: { mode: 'change' } },
      parse: parseSettings,
      error: (error, retry) =>
        c.apiError(error, () => {
          if (active) retry()
        }),
      submit: async (values, _patch, signal, names) => {
        const confirmed = await c.pages.profile.saveProfile(
          {
            ...(names.includes('profile.name') ? { name: values.profile!.name } : {}),
            ...(names.includes('profile.username') ? { username: values.profile!.username } : {}),
          },
          names.includes('profileUpload') ? (values.profileUpload ?? null) : null,
          signal,
        )
        if (confirmed === false) return false
        return { profile: { ...values.profile!, ...confirmed }, profileUpload: null }
      },
    })
    return () => {
      active = false
      form.stop()
    }
  }
  if (c.store.getState().location.page === 'organization') {
    form.configure({
      mode: 'blur',
      names: ['organization.name', 'organizationUpload'],
      fields: { organizationUpload: { mode: 'change' } },
      parse: parseSettings,
      error: (error, retry) =>
        c.apiError(error, () => {
          if (active) retry()
        }),
      submit: async (values, _patch, signal, names) => {
        const confirmed = await c.pages.organization.saveProfile(
          names.includes('organization.name') ? { name: values.organization!.name } : {},
          names.includes('organizationUpload') ? (values.organizationUpload ?? null) : null,
          signal,
        )
        if (confirmed === false) return false
        return { organization: { ...values.organization!, ...confirmed }, organizationUpload: null }
      },
    })
    return () => {
      active = false
      form.stop()
    }
  }
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
        if (root === 'organization' && key) {
          if (
            (await c.pages.organization.updateOrganization({
              [key]: values.organization![key as keyof typeof values.organization],
            })) === false
          )
            return false
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
