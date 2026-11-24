<script lang="ts">
  import Preload from '../../../components/Preload.svelte'
  import { getLoadingContext } from '../../../context'
  import { optionalContext } from '../../../components/form-context.svelte'
  import { Skeleton } from '../../../components/loading'
  import { PermissionRequest, usePermissionChecks } from '../../../components/permission'
  import { Table } from '../../../components/table'
  import { SectionSurface } from '../../../components/surfaces'
  import { Searchbox } from '../../../components/fields'
  import { Switch } from '../../../components/selection'
  import { Popup } from '../../../components/popup'
  import { ProviderIcon } from '../../../components/provider-icons'
  import { CapabilityIndicator } from '../../../components/capability-icon'
  import { hasValidModelPrices } from './validation'
  import { capabilityOptions, modelTypeLabels, textModelOptions, effectiveModelPrice } from './model-options'
  import { ModelFilters } from './filters'
  import { untrack } from 'svelte'
  import { readStore } from '../../page-state.svelte'
  import { getCreateProviderController } from '../../../context'
  import { price } from './model-table'
  import ContentValue from '../../../components/Content.svelte'

  const __provider = getCreateProviderController()
  const region = optionalContext(getLoadingContext)
  let initial = $derived(region?.initial ?? false)
  let readOnly = $derived(readStore(__provider.store, (state) => state.readOnly))
  const controller = untrack(() => getCreateProviderController())
  let templatePreview = $derived(readStore(__provider.store, (state) => state.templatePreview))
  let creationEnabled = $derived(readStore(__provider.store, (state) => state.creationEnabled))
  let models = $derived(readStore(__provider.store, (state) => state.models, ['models']))
  let query = $derived(readStore(__provider.store, (state) => state.modelQuery))
  let status = $derived(readStore(__provider.store, (state) => state.modelStatusFilter))
  let protocol = $derived(readStore(__provider.store, (state) => state.modelProtocolFilter))
  let capabilities = $derived(readStore(__provider.store, (state) => state.modelCapabilityFilter))
  let intent = $derived(readStore(__provider.store, (state) => state.intent))
  let savedProvider = $derived(readStore(__provider.store, (state) => state.savedProvider))
  let providerId = $derived(savedProvider?.id)
  const permissionStatesState = usePermissionChecks(
    () =>
      templatePreview
        ? []
        : intent === 'edit'
          ? models.map((model) => ({
              action: 'provider.model.configuration.update',
              resourceType: 'ProviderModel',
              resourceId: model.id,
            }))
          : [{ action: 'provider.create', resourceType: 'Organization' }],
    () =>
      !templatePreview && savedProvider?.id
        ? savedProvider.models.map((model) => ({
            resourceType: 'ProviderModel',
            id: model.id,
            enabled: model.enabled,
            providerId: savedProvider.id,
          }))
        : [],
  )
  let permissionStates = $derived(permissionStatesState.current)
  const editable = (id: string) =>
    templatePreview || permissionStates[intent === 'edit' ? models.findIndex((model) => model.id === id) : 0] === 'allowed'
  let busy = $derived(readStore(__provider.store, (state) => state.submitting) || !!readOnly)
  const supported = (model: (typeof models)[number]) =>
    !!model.protocol || model.modes.some((mode) => ['chat_completions', 'responses', 'messages'].includes(mode))
  let visible = $derived(
    models.filter(
      (model) =>
        `${model.id} ${model.name} ${model.description ?? ''}`.toLowerCase().includes(query.toLowerCase()) &&
        (templatePreview || status === 'all' || (status === 'enabled') === model.enabled) &&
        (protocol === 'all' ||
          (supported(model) &&
            (model.protocol
              ? model.protocol === protocol
              : model.modes.includes(
                  protocol === 'anthropic_messages'
                    ? 'messages'
                    : protocol === 'openai_responses'
                      ? 'responses'
                      : 'chat_completions',
                )))) &&
        capabilities.every((capability) => model.capabilities?.includes(capability)),
    ),
  )
  let enabled = $derived(models.filter((model) => model.enabled).length)
</script>

<SectionSurface class="grid min-w-0 gap-4">
  <div class="flex items-center gap-3">
    <Searchbox.Root class="flex-1">
      <Searchbox.Input
        aria-label="Search models"
        placeholder="Search models…"
        value={query}
        oninput={(event) => controller.onModelQueryChanged(event.currentTarget.value)}
      ></Searchbox.Input>
    </Searchbox.Root><ModelFilters></ModelFilters>
  </div>
  <Table.Root tabindex={0} role="region" aria-label="Scrollable provider model table" class="min-w-0">
    <Table.Content
      class="w-full table-auto text-xs"
      aria-label="Provider models"
      columns={[
        { id: 'name', class: 'w-0', loadingWidthClassName: 'w-40 sm:w-44' },
        { id: 'capabilities', visibilityClassName: 'hidden xl:table-cell' },
        { id: 'context', class: 'w-20', visibilityClassName: 'hidden lg:table-cell' },
        { id: 'input', class: 'w-20', visibilityClassName: 'hidden md:table-cell' },
        { id: 'output', class: 'w-20', visibilityClassName: 'hidden md:table-cell' },
        { id: 'cached', class: 'w-20', visibilityClassName: 'hidden xl:table-cell' },
        { id: 'actions', class: 'w-12' },
        { id: 'enabled', class: 'w-12' },
      ]}
    >
      <Table.Header class="h-8 text-muted-foreground">
        <Table.Row class="border-b border-divider">
          {#each [['name', 'Model name'], ['capabilities', 'Capabilities'], ['context', 'Context'], ['input', '1M input'], ['output', '1M output'], ['cached', '1M cached']] as [column, label] (label)}<Table.Head
              {column}
              class="px-1 text-left font-normal"
            >
              <ContentValue value={label} />
            </Table.Head>{/each}<Table.Head class="p-0"><span class="sr-only">Actions</span></Table.Head><Table.Head
            column="enabled"
            class="p-0 text-right"
          >
            <span class="sr-only">Enabled</span>
          </Table.Head>
        </Table.Row>
      </Table.Header><Table.Body items={visible} getKey={(model) => model.id}>
        {#snippet children(model)}<Table.Row class="h-12 border-b border-divider last:border-b-0">
            <Table.Cell column="name" class="min-w-0 px-1 text-left">
              <div class="w-max max-w-40 sm:max-w-44">
                <span title={model.name} class="block truncate font-medium">{model.name}</span><span
                  class="mt-1 block truncate text-xs text-muted-foreground"
                >
                  {modelTypeLabels(model).join(' · ') || model.description || 'Not declared'}
                </span>
              </div>
            </Table.Cell><Table.Cell column="capabilities" class="px-1 py-1 text-left">
              <div class="flex min-w-0 flex-wrap items-center gap-0.5">
                {#if model.capabilities?.length}{#each capabilityOptions.filter( (option) => model.capabilities?.includes(option.value) ) as option (option)}<CapabilityIndicator
                      capability={option.value}
                    ></CapabilityIndicator>{/each}{:else}<span class="text-muted-foreground">Not declared</span>{/if}
              </div>
            </Table.Cell><Table.Cell column="context" class="whitespace-nowrap px-1 text-left text-muted-foreground">
              {#if model.maxContextTokens}{Number(model.maxContextTokens).toLocaleString('en-US')}{:else}Not set{/if}
            </Table.Cell>{#each (['inputPrice', 'outputPrice', 'cachedPrice'] as const).map( (key) => effectiveModelPrice(model, key) ) as value, index (index)}<Table.Cell
                column={['input', 'output', 'cached'][index]}
                class="whitespace-nowrap px-1 text-left text-muted-foreground"
              >
                {#if templatePreview}{model.catalogPriceLabels?.[index] ?? price(value)}{:else}{price(value)}{/if}
              </Table.Cell>{/each}<Table.Cell class="p-0 text-left">
              {#if supported(model) || templatePreview}<Preload
                  href={controller.modelHref?.(model.id) ?? '#'}
                  enabled={!!controller.modelHref && !model.id.startsWith('draft:')}
                  aria-label={`Edit ${model.name}`}
                  disabled={busy || (!templatePreview && !editable(model.id))}
                  class="cursor-pointer text-muted-foreground hover:text-foreground
focus-visible:outline-muted-foreground
aria-disabled:opacity-50"
                  onclick={(event) => {
                    if (
                      event.defaultPrevented ||
                      event.button !== 0 ||
                      event.metaKey ||
                      event.ctrlKey ||
                      event.shiftKey ||
                      event.altKey
                    )
                      return
                    event.preventDefault()
                    controller.onEditModel(model.id)
                  }}
                >
                  Edit
                </Preload>{:else}{/if}
            </Table.Cell><Table.Cell column="enabled" class="p-0 text-right">
              <Switch
                aria-label={`Enable ${model.name}`}
                name={`models[${models.indexOf(model)}].enabled`}
                disabled={busy ||
                  (!templatePreview && (!editable(model.id) || !supported(model) || !hasValidModelPrices(model)))}
              ></Switch>
            </Table.Cell>
          </Table.Row>{/snippet}
      </Table.Body>
    </Table.Content>{#if !initial && !visible.length}<Table.Empty>
        <p>
          {#if query || status !== 'all' || protocol !== 'all' || capabilities.length > 0}No matching models.{:else}No known
            models for a custom provider.{/if}
        </p>
      </Table.Empty>{:else}{/if}
  </Table.Root>
  <div class="flex flex-wrap items-center justify-between gap-3">
    <Skeleton>
      <p class="text-xs text-muted-foreground">
        {#if templatePreview}{`${models.length} models available`}{:else}{`${enabled} ${enabled === 1 ? 'model' : 'models'} enabled`}{/if}
      </p>
    </Skeleton>{#if !readOnly && (!templatePreview || creationEnabled)}<PermissionRequest
        actions={[intent === 'edit' ? 'provider.model.create' : 'provider.create']}
        resource={intent === 'edit' ? { type: 'Provider', id: providerId } : undefined}
        enabled={intent !== 'edit' || !!providerId}
      >
        {#snippet children(allowed)}{#if allowed || creationEnabled}<Popup.Root>
              <Popup.Trigger
                disabled={busy}
                class="h-8 w-auto rounded-full bg-elevated px-4 text-xs text-foreground hover:brightness-110"
              >
                Add generate text model
              </Popup.Trigger><Popup.Content aria-label="Add generate text model" class="w-3xs">
                {#each textModelOptions as option (option.value)}<Popup.Item
                    class="gap-2"
                    onSelect={() => controller.onAddModel(option.value)}
                  >
                    <ProviderIcon provider={option.brand}></ProviderIcon>{option.label}
                  </Popup.Item>{/each}
              </Popup.Content>
            </Popup.Root>{:else}{/if}{/snippet}
      </PermissionRequest>{:else}{/if}
  </div>
</SectionSurface>
