<script lang="ts">
  import { onMount, untrack } from 'svelte'
  import { page } from '$app/state'
  import { CreateProviderPage } from '@compallm/storybook/pages/organization/create-provider'
  import { setCreateProviderController } from '@compallm/storybook/context'
  import { CreateProviderController } from './provider/controller'
  import { getPortalControllers, getWorkspaceControllers } from './context'
  import { appController } from './app/controller'
  import { graphqlClient } from './graphql-client'
  let { id }: { id?: string } = $props()
  const { url } = getPortalControllers()
  const { organization } = getWorkspaceControllers()
  const controller = new CreateProviderController(url, organization, appController, () => page.data.providerRead)
  untrack(() => {
    if (page.data.providerRead && page.data.providerRead.version === graphqlClient.readVersion)
      controller.seed(page.data.providerRead)
  })
  setCreateProviderController(controller)
  onMount(() => {
    const stop = id ? controller.startSaved(id) : controller.start()
    const current = url.store.getState()
    if (!id && current.pathname.endsWith('/providers/new'))
      void url.navigate(current.pathname + '/choose-provider' + current.search, { replace: true })
    return stop
  })
</script>

<CreateProviderPage />
