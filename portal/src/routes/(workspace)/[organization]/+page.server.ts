import { redirect } from '@sveltejs/kit'
import type { PageServerLoad } from './$types'
export const load: PageServerLoad = async ({ locals, params }) => {
  const user = await locals.portal.requireUser(params.organization)
  const preferences = await locals.portal.query('PreferenceUserUserPreferences')
  const home = preferences.userPreferences.node.navigation.home
  const destination = ['CHAT', 'MODELS', 'PROVIDERS'].includes(home) ? home.toLowerCase() : 'chat'
  redirect(303, '/' + encodeURIComponent(user.organizationName) + '/' + destination)
}
