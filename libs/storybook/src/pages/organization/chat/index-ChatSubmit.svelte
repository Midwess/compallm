<script lang="ts">
  import { getChatController } from '../../../context'
  import { readStore } from '../../page-state.svelte'
  const chatController = getChatController()

  import { DisabledTooltip } from '../../../components/feedback'
  import { PermissionRequest, usePermissionChecks } from '../../../components/permission'
  import { ChatCreateInput } from '@compallm/contract/validation'

  import ArrowUpIcon from 'phosphor-svelte/lib/ArrowUpIcon'

  import { Button } from '../../../components/buttons'

  let draft = $derived(readStore(chatController.store, (state) => state.draft, ['draft']))
  let modelId = $derived(readStore(chatController.store, (state) => state.modelId, ['modelId']))
  let models = $derived(readStore(chatController.store, (state) => state.models))
  let creating = $derived(readStore(chatController.store, (state) => state.creating))
  usePermissionChecks(
    () => [],
    () =>
      models
        .filter((model) => model.enabled !== undefined)
        .map((model) => ({ resourceType: 'Model', id: model.value, enabled: model.enabled })),
  )
  let canSend = $derived(
    ChatCreateInput.safeParse({ message: draft, modelId }).success &&
      models.some((model) => model.value === modelId) &&
      !creating,
  )
</script>

<PermissionRequest actions={['model.use']} resource={{ type: 'Model', id: modelId || undefined }} enabled={!!modelId}>
  {#snippet children(allowed)}<DisabledTooltip disabled={!allowed} reason="You do not have permission to perform this action.">
      <Button
        type="submit"
        variant="primary"
        aria-label="Create chat"
        disabled={!allowed || !canSend}
        loading={creating}
        class="size-6 p-0"
      >
        {#if !creating}<ArrowUpIcon size={14} aria-hidden="true"></ArrowUpIcon>{:else}{/if}
      </Button>
    </DisabledTooltip>{/snippet}
</PermissionRequest>
