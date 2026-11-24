import { redirect } from '@sveltejs/kit'
import type { PageServerLoad } from './$types'
export const load: PageServerLoad = async ({ locals, params }) => {
  const user = await locals.portal.requireUser(params.organization)
  redirect(303, '/' + encodeURIComponent(user.organizationName) + '/settings/general')
}
