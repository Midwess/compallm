<script lang="ts">
  import { joinModelEndpoint } from './model-options'

  import { readStore } from '../../page-state.svelte'
  import { getCreateProviderController } from '../../../context'

  const __provider = getCreateProviderController()
  let baseUrl = $derived(readStore(__provider.store, (s) => s.baseUrl, ['baseUrl']))
  let path = $derived(readStore(__provider.store, (s) => s.modelDraft?.path ?? '', ['modelDraft.path']))
</script>

<div class="grid min-w-0 gap-2">
  <div>
    <p class="text-ui-11 text-muted-foreground">Provider base URL</p>
    <p class="break-all text-xs">{baseUrl || 'Not configured'}</p>
  </div>
  {#if baseUrl}<div>
      <p class="text-ui-11 text-muted-foreground">Complete endpoint</p>
      <p class="break-all text-xs">{joinModelEndpoint(baseUrl, path)}</p>
    </div>{:else}{/if}
</div>
