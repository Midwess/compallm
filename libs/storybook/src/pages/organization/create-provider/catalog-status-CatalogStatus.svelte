<script lang="ts">
  import { Button } from '../../../components/buttons'
  import { untrack } from 'svelte'

  import { readStore } from '../../page-state.svelte'
  import { getCreateProviderController } from '../../../context'

  const __provider = getCreateProviderController()
  const controller = untrack(() => getCreateProviderController())
  let error = $derived(readStore(__provider.store, (s) => s.catalogError))
</script>

{#if error}<div role="alert" class="flex items-center gap-3 px-4 text-sm">
    <span>{error}</span><Button onclick={controller.onRetryCatalog}>Retry</Button>
  </div>{:else}{/if}
