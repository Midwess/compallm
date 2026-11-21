import { PermissionRequest, usePermissionChecks } from '../../../components/permission'
import { Table } from '../../../components/table'
import { SectionSurface } from '../../../components/surfaces'
import { Searchbox } from '../../../components/fields'
import { Switch } from '../../../components/selection'
import { Popup } from '../../../components/popup'
import { ProviderIcon } from '../../../components/provider-icons'
import { CapabilityIndicator } from '../../../components/capability-icon'
import { useController, useProvider } from './controller'
import { hasValidModelPrices } from './validation'
import { capabilityOptions, modelTypeLabels, textModelOptions, effectiveModelPrice } from './model-options'
import { ModelFilters } from './filters'

function price(value: string) {
  return value === '' ? 'Review rates' : `$${value}`
}
export function ProviderModels() {
  const controller = useController()
  const templatePreview = useProvider((state) => state.templatePreview)
  const models = useProvider((state) => state.models, ['models'])
  const query = useProvider((state) => state.modelQuery)
  const status = useProvider((state) => state.modelStatusFilter)
  const protocol = useProvider((state) => state.modelProtocolFilter)
  const capabilities = useProvider((state) => state.modelCapabilityFilter)
  const intent = useProvider((state) => state.intent)
  const savedProvider = useProvider((state) => state.savedProvider)
  const providerId = savedProvider?.id
  const permissionStates = usePermissionChecks(
    templatePreview
      ? []
      : intent === 'edit'
        ? models.map((model) => ({
            action: 'provider.model.configuration.update',
            resourceType: 'ProviderModel',
            resourceId: model.id,
          }))
        : [{ action: 'provider.create', resourceType: 'Organization' }],
    !templatePreview && savedProvider?.id
      ? savedProvider.models.map((model) => ({
          resourceType: 'ProviderModel',
          id: model.id,
          enabled: model.enabled,
          providerId: savedProvider.id,
        }))
      : [],
  )
  const editable = (id: string) =>
    templatePreview || permissionStates[intent === 'edit' ? models.findIndex((model) => model.id === id) : 0] === 'allowed'
  const busy = useProvider((state) => state.submitting)
  const supported = (model: (typeof models)[number]) =>
    !!model.protocol || model.modes.some((mode) => ['chat_completions', 'responses', 'messages'].includes(mode))
  const visible = models.filter(
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
  )
  const enabled = models.filter((model) => model.enabled).length
  return (
    <SectionSurface className="grid min-w-0 gap-4">
      <div className="flex items-center gap-3">
        <Searchbox.Root className="flex-1">
          <Searchbox.Input
            aria-label="Search models"
            placeholder="Search models…"
            value={query}
            onChange={(event) => controller.onModelQueryChanged(event.target.value)}
          />
        </Searchbox.Root>
        <ModelFilters />
      </div>
      <Table.Root tabIndex={0} role="region" aria-label="Scrollable provider model table" className="min-w-0">
        <Table.Content
          className="w-full table-auto text-xs"
          aria-label="Provider models"
          columns={[
            { id: 'name', className: 'w-0' },
            { id: 'capabilities', visibilityClassName: 'hidden xl:table-cell' },
            { id: 'context', className: 'w-20', visibilityClassName: 'hidden lg:table-cell' },
            { id: 'input', className: 'w-20', visibilityClassName: 'hidden md:table-cell' },
            { id: 'output', className: 'w-20', visibilityClassName: 'hidden md:table-cell' },
            { id: 'cached', className: 'w-20', visibilityClassName: 'hidden xl:table-cell' },
            { id: 'actions', className: 'w-12' },
            { id: 'enabled', className: 'w-12' },
          ]}
        >
          <Table.Header className="h-8 text-muted-foreground">
            <Table.Row className="border-b border-divider">
              {[
                ['name', 'Model name'],
                ['capabilities', 'Capabilities'],
                ['context', 'Context'],
                ['input', '1M input'],
                ['output', '1M output'],
                ['cached', '1M cached'],
              ].map(([column, label]) => (
                <Table.Head column={column} key={label} className="px-1 text-left font-normal">
                  {label}
                </Table.Head>
              ))}
              <Table.Head className="p-0">
                <span className="sr-only">Actions</span>
              </Table.Head>
              <Table.Head column="enabled" className="p-0 text-right">
                <span className="sr-only">Enabled</span>
              </Table.Head>
            </Table.Row>
          </Table.Header>
          <Table.Body items={visible} getKey={(model) => model.id}>
            {(model) => (
              <Table.Row key={model.id} className="h-12 border-b border-divider last:border-b-0">
                <Table.Cell column="name" className="min-w-0 px-1 text-left">
                  <div className="w-max max-w-40 sm:max-w-44">
                    <span title={model.name} className="block truncate font-medium">
                      {model.name}
                    </span>
                    <span className="mt-1 block truncate text-xs text-muted-foreground">
                      {modelTypeLabels(model).join(' · ') || model.description || 'Not declared'}
                    </span>
                  </div>
                </Table.Cell>
                <Table.Cell column="capabilities" className="px-1 py-1 text-left">
                  <div className="flex min-w-0 flex-wrap items-center gap-0.5">
                    {model.capabilities?.length ? (
                      capabilityOptions
                        .filter((option) => model.capabilities?.includes(option.value))
                        .map((option) => <CapabilityIndicator key={option.value} capability={option.value} />)
                    ) : (
                      <span className="text-muted-foreground">Not declared</span>
                    )}
                  </div>
                </Table.Cell>
                <Table.Cell column="context" className="whitespace-nowrap px-1 text-left text-muted-foreground">
                  {model.maxContextTokens ? Number(model.maxContextTokens).toLocaleString('en-US') : 'Not set'}
                </Table.Cell>
                {(['inputPrice', 'outputPrice', 'cachedPrice'] as const)
                  .map((key) => effectiveModelPrice(model, key))
                  .map((value, index) => (
                    <Table.Cell
                      column={['input', 'output', 'cached'][index]}
                      key={index}
                      className="whitespace-nowrap px-1 text-left text-muted-foreground"
                    >
                      {templatePreview ? (model.catalogPriceLabels?.[index] ?? price(value)) : price(value)}
                    </Table.Cell>
                  ))}
                <Table.Cell className="p-0 text-left">
                  {supported(model) || templatePreview ? (
                    <button
                      type="button"
                      aria-label={`Edit ${model.name}`}
                      disabled={busy || (!templatePreview && !editable(model.id))}
                      className="cursor-pointer text-muted-foreground hover:text-foreground
focus-visible:outline-muted-foreground
disabled:opacity-50"
                      onClick={() => controller.onEditModel(model.id)}
                    >
                      Edit
                    </button>
                  ) : null}
                </Table.Cell>
                <Table.Cell column="enabled" className="p-0 text-right">
                  <Switch
                    aria-label={`Enable ${model.name}`}
                    name={`models[${models.indexOf(model)}].enabled`}
                    disabled={
                      busy || (!templatePreview && (!editable(model.id) || !supported(model) || !hasValidModelPrices(model)))
                    }
                  />
                </Table.Cell>
              </Table.Row>
            )}
          </Table.Body>
        </Table.Content>
        {!visible.length ? (
          <Table.Empty>
            <p>
              {query || status !== 'all' || protocol !== 'all' || capabilities.length > 0
                ? 'No matching models.'
                : 'No known models for a custom provider.'}
            </p>
          </Table.Empty>
        ) : null}
      </Table.Root>
      <div className="flex flex-wrap items-center justify-between gap-3">
        <p className="text-xs text-muted-foreground">
          {templatePreview ? `${models.length} models available` : `${enabled} ${enabled === 1 ? 'model' : 'models'} enabled`}
        </p>
        {!templatePreview ? (
          <PermissionRequest
            actions={[intent === 'edit' ? 'provider.model.create' : 'provider.create']}
            resource={intent === 'edit' ? { type: 'Provider', id: providerId } : undefined}
            enabled={intent !== 'edit' || !!providerId}
          >
            {(allowed) =>
              allowed ? (
                <Popup.Root>
                  <Popup.Trigger
                    disabled={busy}
                    className="h-8 w-auto rounded-full bg-elevated px-4 text-xs text-foreground hover:brightness-110"
                  >
                    Add generate text model
                  </Popup.Trigger>
                  <Popup.Content aria-label="Add generate text model" className="w-3xs">
                    {textModelOptions.map((option) => (
                      <Popup.Item key={option.value} className="gap-2" onSelect={() => controller.onAddModel(option.value)}>
                        <ProviderIcon provider={option.brand} />
                        {option.label}
                      </Popup.Item>
                    ))}
                  </Popup.Content>
                </Popup.Root>
              ) : null
            }
          </PermissionRequest>
        ) : null}
      </div>
    </SectionSurface>
  )
}
