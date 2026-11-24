import { redirect } from '@sveltejs/kit'
import type { LayoutServerLoad } from './$types'

export const load: LayoutServerLoad = async ({ locals, route, depends, cookies, request }) => {
  depends('portal:session')
  const special = route.id?.includes('confirmation') || route.id === '/invitation'
  const guest = route.id?.startsWith('/(guest)')
  const user = special ? null : await locals.portal.session()
  if (guest && user) redirect(303, '/' + encodeURIComponent(user.organizationName))
  if (route.id?.startsWith('/(workspace)')) await locals.portal.requireUser()
  const [authority, preferences, providers] = user
    ? await Promise.all([
        locals.portal.query('PortalAuthority'),
        locals.portal.query('PreferenceUserUserPreferences'),
        locals.portal.query('OrganizationProviders').catch(() => undefined),
      ])
    : []
  if (preferences) {
    const saved = preferences.userPreferences.node
    const dark =
      saved.theme.mode === 'DARK' ||
      (saved.theme.mode === 'SYSTEM' &&
        (cookies.get('compallm-appearance') === 'dark' || request.headers.get('sec-ch-prefers-color-scheme') === 'dark'))
    locals.appearance = {
      dark,
      theme: saved.theme.palettes[dark ? 'dark' : 'light'].toLowerCase().replaceAll('_', '-'),
      size: saved.typography.size.toLowerCase(),
      underline: saved.accessibility.underlineLinks,
    }
  }
  return { user, authority, preferences, providers }
}
