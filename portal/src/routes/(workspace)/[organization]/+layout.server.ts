import type { LayoutServerLoad } from './$types'
export const load: LayoutServerLoad = async ({ locals, params }) => {
  const user = await locals.portal.requireUser(params.organization)
  return {
    onboardingRead: await locals.portal.query('OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile', {
      userProfileId: user.userId,
    }),
  }
}
