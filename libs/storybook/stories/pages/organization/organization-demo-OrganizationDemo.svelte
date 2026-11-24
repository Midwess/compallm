<script lang="ts">
  import { DemoUserPreferenceController } from '../settings/preferences/user/demo-controller'

  import { OrganizationPage, type OrganizationState } from '../../../src/pages/organization'
  import { untrack } from 'svelte'
  import type { Content } from '../../../src/components/shared'

  import { setOrganizationController, setUserPreferenceController } from '../../../src/context'
  import { DemoOrganizationController } from './organization-demo-helpers'
  import ContentValue from '../../../src/components/Content.svelte'

  let {
    empty = false,
    sidebarLoading = false,
    children,
    sidebarOrder,
    sidebarLayout,
  }: {
    empty?: boolean
    sidebarLoading?: boolean
    children?: Content
    sidebarOrder?: OrganizationState['sidebarOrder']
    sidebarLayout?: OrganizationState['sidebarLayout']
  } = $props()

  let intent = $state('')
  let preferences = $state(
    untrack(() => {
      const value = new DemoUserPreferenceController()
      if (sidebarLoading) value.store.setState({ status: 'loading', saved: null })
      return value
    }),
  )
  let controller = $state(
    untrack(() => {
      const result = new DemoOrganizationController((value) => (intent = value))
      if (sidebarOrder)
        result.store.setState({
          sidebarOrder,
          sidebarLayout: result.store.getState().sidebarLayout.map((section) => ({
            ...section,
            items: [...section.items].sort((a, b) => sidebarOrder.indexOf(a) - sidebarOrder.indexOf(b)),
          })),
        })
      if (sidebarLayout) result.store.setState({ sidebarLayout })
      if (empty) result.store.setState({ providers: [], models: [], conversations: [] })
      return result
    }),
  )
  setOrganizationController(controller)
  setUserPreferenceController(preferences)
</script>

<OrganizationPage><ContentValue value={children} /></OrganizationPage><output
  aria-label="Controller intent"
  class="fixed right-8 bottom-12 text-xs text-muted-foreground"
>
  {intent}
</output>
