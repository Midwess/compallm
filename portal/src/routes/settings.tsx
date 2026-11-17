import { passkeyController } from '../passkey/instance'
import { useContext, useEffect, useState } from 'react'
import { redirect, type LoaderFunctionArgs } from 'react-router'
import { useLoaderData } from 'react-router'
import { OnboardingControllerContext, SettingsControllerContext } from '@compallm/storybook/context'
import { SettingsPage, type SettingsDestination } from '@compallm/storybook/pages/settings'
import { useUrlController } from '../url/use-url-controller'
import { useAuthController } from '../auth/use-auth-controller'
import { useOrganizationController } from '../organization/use-organization-controller'
import { useUserPreferenceController } from '../preference/user/use-user-preference-controller'
import { usePermissionController } from '@compallm/storybook'
import { SettingsController, settingsPages } from '../settings/controller'

export function loader({ params }: LoaderFunctionArgs) {
  if ((params.page === 'role' || params.page === 'group') && params.id) return { page: params.page, id: params.id }
  const page = settingsPages.find((page) => page === params.page)
  if (!page || params.id) return redirect(`/${encodeURIComponent(params.organizationName!)}/settings/general`)
  return { page }
}

export function Component() {
  const { page, id } = useLoaderData() as { page: SettingsDestination; id?: string }
  const url = useUrlController()
  const auth = useAuthController()
  const permissions = usePermissionController()
  const onboarding = useContext(OnboardingControllerContext)
  const preferences = useUserPreferenceController()
  const organization = useOrganizationController().getCurrentOrganization()!
  const [controller] = useState(
    () =>
      new SettingsController(organization.name, url, auth, page, preferences, permissions, () => {
        void onboarding?.reopen?.()
      }),
  )
  useEffect(() => {
    let disposed = false
    let stop: (() => void) | undefined
    queueMicrotask(() => {
      if (!disposed) {
        stop = controller.start()
      }
    })
    return () => {
      disposed = true
      stop?.()
    }
  }, [controller])
  useEffect(() => controller.setPage(page, id), [controller, page, id])
  useEffect(() => (page === 'security' ? passkeyController.startView() : undefined), [page])
  return (
    <SettingsControllerContext value={controller}>
      <SettingsPage />
    </SettingsControllerContext>
  )
}
