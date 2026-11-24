<script lang="ts">
  import { PermissionRequest } from '../../../components/permission'
  import DotsThreeIcon from 'phosphor-svelte/lib/DotsThreeIcon'
  import { Popup } from '../../../components/popup'

  import { untrack } from 'svelte'

  import { getCreateProviderController } from '../../../context'

  const controller = untrack(() => getCreateProviderController())
</script>

<Popup.Root>
  <Popup.Trigger aria-label="Provider actions">
    <DotsThreeIcon aria-hidden="true" class="size-4"></DotsThreeIcon>
  </Popup.Trigger><Popup.Content aria-label="Provider actions">
    <PermissionRequest actions={['provider.create']}>
      {#snippet children(allowed)}{#if allowed}<Popup.Item onSelect={controller.onCloneProvider}>
            Clone provider
          </Popup.Item>{:else}{/if}{/snippet}
    </PermissionRequest>
  </Popup.Content>
</Popup.Root>
