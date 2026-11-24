import type { PageServerLoad } from './$types'
export const load: PageServerLoad = async ({ locals, url }) => ({
  invitationRead: url.searchParams.has('unavailable')
    ? null
    : await locals.portal.query('AuthInvitationControllerInvitationContext').catch(() => null),
})
