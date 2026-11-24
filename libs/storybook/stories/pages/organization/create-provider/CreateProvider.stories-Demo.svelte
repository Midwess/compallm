<script lang="ts">
  import { CreateProviderPage } from '../../../../src/pages/organization/create-provider'
  import { OrganizationDemo } from '../organization-demo-helpers'
  import { DemoCreateProviderController, type Scenario } from './demo-controller'
  import { untrack } from 'svelte'

  import { setCreateProviderController } from '../../../../src/context'

  let {
    scenario = 'choose',
  }: {
    scenario?: Scenario
  } = $props()

  let intent = $state('')
  let controller = $state(untrack(() => new DemoCreateProviderController(scenario, (value) => (intent = value))))
  $effect(() => {
    void [controller]
    return untrack(() => controller.stop)
  })
  setCreateProviderController(controller)
</script>

<OrganizationDemo>
  <CreateProviderPage></CreateProviderPage><output class="sr-only" aria-label="Provider controller intent">{intent}</output>
</OrganizationDemo>
