<script lang="ts">
  import { ChatPage } from '../../../../src/pages/organization/chat'
  import { OrganizationDemo } from '../organization-demo-helpers'
  import { untrack } from 'svelte'

  import { setChatController } from '../../../../src/context'

  import { DemoChatController } from './Chat.stories-helpers'
  import ContentValue from '../../../../src/components/Content.svelte'

  let {
    creating = false,
    noModels = false,
  }: {
    creating?: boolean
    noModels?: boolean
  } = $props()

  let action = $state('')
  let controller = $state(
    untrack(() => {
      const result = new DemoChatController((value) => (action = value))
      if (creating) result.store.setState({ creating: true, draft: 'Help me explore an idea.' })
      if (noModels) result.store.setState({ models: [], modelId: '' })
      return result
    }),
  )
  setChatController(controller)
</script>

<OrganizationDemo><ChatPage></ChatPage></OrganizationDemo><output aria-label="Chat intent" class="sr-only">
  <ContentValue value={action} />
</output>
