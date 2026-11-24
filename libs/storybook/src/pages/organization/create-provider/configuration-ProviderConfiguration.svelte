<script lang="ts">
  import { LoadingRegion } from '../../../components/loading'
  import { providerPageLayout } from './layout'
  import { CatalogStatus } from './catalog-status'

  import { DisabledTooltip } from '../../../components/feedback'
  import { usePermission } from '../../../components/permission'

  import { Form } from '../../../components/form'

  import { FormSection } from '../../../components/form-section'

  import { Button } from '../../../components/buttons'

  import { Toast } from '../../../components/feedback'
  import { ProviderModels } from './model-table'

  import { untrack } from 'svelte'

  import { readStore } from '../../page-state.svelte'
  import { getCreateProviderController } from '../../../context'
  import { ConnectionFields, TestConnection } from './configuration'

  const __provider = getCreateProviderController()
  const controller = untrack(() => getCreateProviderController())
  let intent = $derived(readStore(__provider.store, (state) => state.intent))
  let readOnly = $derived(readStore(__provider.store, (state) => state.readOnly))
  const create = usePermission(() => ({
    actions: ['provider.create', 'provider.read'],
    enabled: !!controller.store.getState().creationEnabled,
  }))
  let submitting = $derived(readStore(__provider.store, (state) => state.submitting))
  let providerId = $derived(readStore(__provider.store, (state) => state.savedProvider?.id))
  const edit = usePermission(() => ({
    actions: ['provider.configuration.update'],
    resource: { type: 'Provider', id: providerId },
    enabled: intent === 'edit' && !!providerId,
  }))
  let loading = $derived(readStore(__provider.store, (s) => s.providerLoading && !s.providerLoaded))
  let busy = $derived(!!readOnly || submitting || !!loading || (intent === 'edit' && !edit.allowed))
  let error = $derived(readStore(__provider.store, (state) => state.error))
  let form = $derived(controller.store.form)
  let templatePreview = $derived(readStore(__provider.store, (state) => state.templatePreview))
  let creationEnabled = $derived(readStore(__provider.store, (state) => state.creationEnabled))
  let ready = $derived(readStore(__provider.store, (state) => !state.templatePreview || !!state.providerLoaded))
</script>

<Form.Root model={form} class={`${providerPageLayout} gap-6`}>
  <div class="grid gap-1 px-4">
    <h2 class="text-2xl font-medium">Configure</h2>
    {#if creationEnabled}<p class="text-xsm text-muted-foreground">Create uses saved model configurations.</p>{:else}{/if}
  </div>
  {#if templatePreview}<CatalogStatus></CatalogStatus>{:else}{/if}{#if intent === 'edit' && !edit.allowed && !readOnly}<p
      class="text-xs text-muted-foreground"
    >
      You do not have permission to edit this provider configuration.
    </p>{:else}{/if}<LoadingRegion
    class="contents"
    {ready}
    pending={loading}
    controller={controller.loading?.provider}
    label="Loading provider configuration"
  >
    <fieldset disabled={busy} class="contents">
      <FormSection.Root>
        <FormSection.Marker>1</FormSection.Marker><FormSection.Heading>Connection</FormSection.Heading><ConnectionFields
        ></ConnectionFields>
      </FormSection.Root><FormSection.Root>
        <FormSection.Marker>2</FormSection.Marker><FormSection.Heading>
          {#if templatePreview}Available models{:else}Enabled models{/if}
        </FormSection.Heading><ProviderModels></ProviderModels>
      </FormSection.Root>{#if !templatePreview && !readOnly}<FormSection.Root>
          <FormSection.Marker>3</FormSection.Marker><FormSection.Heading>Test connection</FormSection.Heading><TestConnection
          ></TestConnection>
        </FormSection.Root>{:else}{/if}{#if error}<div class="fixed right-6 bottom-6 z-50">
          <Toast tone="error" message={error} onDismiss={() => controller.store.setState({ error: null })}></Toast>
        </div>{:else}{/if}{#if !readOnly && (!templatePreview || creationEnabled)}<div class="flex justify-end gap-2">
          {#if intent !== 'create' || creationEnabled}<Button
              class="h-7 px-4 text-xs"
              disabled={busy}
              onclick={controller.onCancelProvider}
            >
              Cancel
            </Button>{:else}{/if}<DisabledTooltip
            disabled={!!creationEnabled && !create.allowed}
            reason="You need permission to create and view providers."
          >
            <Button
              type="submit"
              variant="primary"
              class="h-7 px-4 text-xs"
              disabled={busy || (creationEnabled && !create.allowed)}
              loading={busy}
            >
              {#if intent === 'edit'}Save changes{:else}{#if intent === 'clone'}Create provider{:else}Create{/if}{/if}
            </Button>
          </DisabledTooltip>
        </div>{:else}{/if}
    </fieldset>
  </LoadingRegion>
</Form.Root>
