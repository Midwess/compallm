<script lang="ts">
  import { modelSettingsIntent } from './model-options'

  import { untrack } from 'svelte'

  import { readStore } from '../../page-state.svelte'
  import { getCreateProviderController } from '../../../context'

  let {
    onBack,
  }: {
    onBack(): void
  } = $props()
  const __provider = getCreateProviderController()
  const controller = untrack(() => getCreateProviderController())
  let creating = $derived(
    readStore(__provider.store, (s) => modelSettingsIntent(s) === 'new', ['modelDraft.id', 'modelDraft.catalogRef', 'models']),
  )
</script>

<nav aria-label="Provider configuration steps" class="flex min-w-0 flex-1 items-center gap-4 text-xs">
  <button
    type="button"
    onclick={controller.onChooseProvider}
    class="truncate text-muted-foreground hover:text-foreground focus-visible:outline-ring"
  >
    Choose provider
  </button><span aria-hidden="true" class="text-muted-foreground">/ </span><button
    type="button"
    onclick={onBack}
    class="truncate text-muted-foreground hover:text-foreground focus-visible:outline-ring"
  >
    Configure
  </button><span aria-hidden="true" class="text-muted-foreground">/ </span><span
    aria-current="page"
    class="shrink-0 whitespace-nowrap"
  >
    {#if creating}New model{:else}Edit model{/if}
  </span>
</nav>
