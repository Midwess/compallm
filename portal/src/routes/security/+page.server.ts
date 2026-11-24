import type { PageServerLoad } from './$types'
export const load: PageServerLoad = async ({ locals }) => {
  await locals.portal.requireUser()
  return {}
}
