import { useEffect, useState } from 'react'
import { redirect, type LoaderFunctionArgs } from 'react-router'
import { useLoaderData } from 'react-router'
import { SettingsControllerContext } from '@compallm/storybook/context'
import { SettingsPage, type SettingsDestination } from '@compallm/storybook/pages/settings'
import { useUrlController } from '../url/use-url-controller'
import { useAuthController } from '../auth/use-auth-controller'
import { useOrganizationController } from '../organization/use-organization-controller'
import { SettingsController, settingsPages } from '../settings/controller'

export function loader({ params }: LoaderFunctionArgs) {
  const page = settingsPages.find((page) => page === params.page)
  if (!page) return redirect(`/${encodeURIComponent(params.organizationName!)}/settings/general`)
  return page
}

export function Component() {
  const page = useLoaderData() as SettingsDestination
  const url = useUrlController()
  const auth = useAuthController()
  const organization = useOrganizationController().getCurrentOrganization()!
  const [controller] = useState(() => new SettingsController(organization.name, url, auth, page))
  useEffect(() => controller.setPage(page), [controller, page])
  return (
    <SettingsControllerContext value={controller}>
      <SettingsPage />
    </SettingsControllerContext>
  )
}
