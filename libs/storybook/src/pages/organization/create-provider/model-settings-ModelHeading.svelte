<script lang="ts">
  import { modelSettingsIntent, textModelOptions } from './model-options'

  import { readStore } from '../../page-state.svelte'
  import { getCreateProviderController } from '../../../context'

  const __provider = getCreateProviderController()
  let draft = $derived(
    readStore(__provider.store, (s) => (s.modelDraft ? { name: s.modelDraft.name, protocol: s.modelDraft.protocol } : null), [
      'modelDraft.name',
      'modelDraft.protocol',
    ]),
  )
  let provider = $derived(readStore(__provider.store, (s) => s.name || 'New provider', ['name']))
  let editing = $derived(
    readStore(__provider.store, (s) => modelSettingsIntent(s) === 'edit', ['modelDraft.id', 'modelDraft.catalogRef', 'models']),
  )
</script>

<div class="grid gap-1 px-4">
  <h1 class="text-2xl font-medium">
    {#if editing}{draft?.name || 'Edit model'}{:else}New model{/if}
  </h1>
  <p class="text-xsm text-muted-foreground">
    {provider}{#if draft?.protocol}{` · ${textModelOptions.find((o) => o.value === draft.protocol)?.label}`}{:else}{/if}
  </p>
</div>
