<script lang="ts">
  import { PermissionRequest } from '../../../components/permission'

  import { useProviderForm } from './configuration'
  import { ModelSettingsPage } from './model-settings'

  import { readStore } from '../../page-state.svelte'
  import { getCreateProviderController } from '../../../context'
  import { ProviderHeader, ProviderContent } from './index'
  import ContentValue from '../../../components/Content.svelte'

  const __provider = getCreateProviderController()
  useProviderForm()
  let intent = $derived(readStore(__provider.store, (state) => state.intent))
  let modelId = $derived(
    readStore(__provider.store, (state) => (state.templatePreview ? state.editingModelId : state.modelDraft?.id), [
      'modelDraft.id',
    ]),
  )
  let templatePreview = $derived(readStore(__provider.store, (state) => state.templatePreview))
</script>

{#snippet content()}<div class={modelId ? 'hidden' : 'contents'} inert={!!modelId || undefined}>
    {#if !modelId}<ProviderHeader></ProviderHeader>{:else}{/if}<ProviderContent></ProviderContent>
  </div>
  {#if modelId}<ModelSettingsPage></ModelSettingsPage>{:else}{/if}{/snippet}
{#if templatePreview}<ContentValue value={content} />{:else}<PermissionRequest
    actions={intent === 'edit' ? ['provider.read'] : ['provider.create']}
  >
    {#snippet children(allowed)}{#if allowed}<ContentValue value={content} />{:else}<p role="status">
          You do not have access to this page.
        </p>{/if}{/snippet}
  </PermissionRequest>{/if}
