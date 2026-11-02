import { useEffect, useState } from 'react'
import { CreateProviderControllerContext } from '@compallm/storybook/context'
import { CreateProviderPage } from '@compallm/storybook/pages/organization/create-provider'
import { useUrlController } from '../url/use-url-controller'
import { CreateProviderController } from '../provider/controller'
import { useOrganizationController } from '../organization/use-organization-controller'

export function Component() {
  const organization = useOrganizationController()
  const url = useUrlController()
  const [controller] = useState(() => new CreateProviderController(url, organization))
  useEffect(() => controller.start(), [controller])
  return (
    <CreateProviderControllerContext key={organization.getCurrentOrganization()?.id} value={controller}>
      <CreateProviderPage />
    </CreateProviderControllerContext>
  )
}
