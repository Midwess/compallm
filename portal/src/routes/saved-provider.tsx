import { appController } from '../app/controller'
import { useLayoutEffect, useState } from 'react'
import { useParams } from 'react-router'
import { CreateProviderControllerContext } from '@compallm/storybook/context'
import { CreateProviderPage } from '@compallm/storybook/pages/organization/create-provider'
import { useUrlController } from '../url/use-url-controller'
import { CreateProviderController } from '../provider/controller'
import { useOrganizationController } from '../organization/use-organization-controller'
export function Component() {
  const { id } = useParams()
  const organization = useOrganizationController(),
    url = useUrlController()
  const [controller] = useState(() => new CreateProviderController(url, organization, appController))
  useLayoutEffect(() => (id ? controller.startSaved(id) : undefined), [controller, id])
  return (
    <CreateProviderControllerContext value={controller}>
      <CreateProviderPage />
    </CreateProviderControllerContext>
  )
}
