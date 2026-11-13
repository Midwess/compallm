import { Table } from '../../../components/table'
import { SectionSurface } from '../../../components/surfaces'
import { Searchbox } from '../../../components/fields'
import { Checkbox } from '../../../components/selection'
import { Popup } from '../../../components/popup'
import { ProviderIcon } from '../../../components/provider-icons'
import { useController, useProvider } from './controller'
import { hasValidModelPrices } from './validation'
import { capabilityOptions, modelTypeLabels, textModelOptions } from './model-options'
import { ModelFilters } from './filters'

function price(value: string) {
  return value === '' ? 'Review rates' : `$${value}`
}
export function ProviderModels() {
  const controller = useController()
  const models = useProvider((state) => state.models)
  const query = useProvider((state) => state.modelQuery)
  const status = useProvider((state) => state.modelStatusFilter)
  const protocol = useProvider((state) => state.modelProtocolFilter)
  const capabilities = useProvider((state) => state.modelCapabilityFilter)
  const busy = useProvider((state) => state.submitting)
  const supported = (model: (typeof models)[number]) =>
    !!model.protocol || model.modes.some((mode) => ['chat_completions', 'responses', 'messages'].includes(mode))
  const visible = models.filter(
    (model) =>
      model.name.toLowerCase().includes(query.toLowerCase()) &&
      (status === 'all' || (status === 'enabled') === model.enabled) &&
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
  const selectable = visible.filter((model) => supported(model) && hasValidModelPrices(model))
  const selected = selectable.filter((model) => model.enabled).length
  const all = selectable.length > 0 && selected === selectable.length
  const enabled = models.filter((model) => model.enabled).length
  function toggleAll(value: boolean) {
    const ids = new Set(selectable.map((model) => model.id))
    controller.store.form.change(
      'models',
      models.map((model) => (ids.has(model.id) ? { ...model, enabled: value } : model)),
    )
  }
  return (
    <SectionSurface className="grid min-w-0 gap-4">
      <div className="flex items-center gap-3">
        <Searchbox className="flex-1">
          <Searchbox.Input
            aria-label="Search models"
            placeholder="Search models…"
            value={query}
            onChange={(event) => controller.onModelQueryChanged(event.target.value)}
          />
        </Searchbox>
        <ModelFilters />
      </div>
      <Table tabIndex={0} role="region" aria-label="Scrollable provider model table" className="min-w-0">
        <Table.Content className="w-full min-w-3xl table-fixed text-xs" aria-label="Provider models">
          <colgroup>
            <col className="w-6" />
            <col className="w-44" />
            <col className="w-52" />
            <col className="w-16" />
            <col className="w-20" />
            <col className="w-20" />
            <col className="w-20" />
            <col className="w-8" />
          </colgroup>
          <Table.Header className="h-8 text-muted">
            <Table.Row className="border-b border-divider">
              <Table.Head className="p-0 text-left">
                <Checkbox
                  size="compact"
                  aria-label="Enable all matching models"
                  checked={all}
                  indeterminate={selected > 0 && !all}
                  disabled={busy || !selectable.length}
                  onChange={(event) => toggleAll(event.target.checked)}
                />
              </Table.Head>
              {['Model name', 'Capabilities', 'Context', '1M input', '1M output', '1M cached'].map((label) => (
                <Table.Head key={label} className="px-1 text-left font-normal">
                  {label}
                </Table.Head>
              ))}
              <Table.Head className="p-0">
                <span className="sr-only">Actions</span>
              </Table.Head>
            </Table.Row>
          </Table.Header>
          <Table.Body>
            {visible.map((model) => (
              <Table.Row key={model.id} className="h-12 border-b border-divider last:border-b-0">
                <Table.Cell className="p-0 text-left">
                  <Checkbox
                    size="compact"
                    aria-label={`Enable ${model.name}`}
                    name={`models[${models.indexOf(model)}].enabled`}
                    disabled={busy || !supported(model) || !hasValidModelPrices(model)}
                  />
                </Table.Cell>
                <Table.Cell className="min-w-0 px-1 text-left">
                  <span title={model.name} className="block max-w-44 truncate font-medium">
                    {Array.from(model.name).length > 50 ? `${Array.from(model.name).slice(0, 50).join('')}…` : model.name}
                  </span>
                  <span className="mt-1 block text-xs text-muted">
                    {modelTypeLabels(model).join(' · ') || model.description || 'Not declared'}
                  </span>
                </Table.Cell>
                <Table.Cell className="px-1 py-2 text-left">
                  <div className="flex flex-wrap gap-1">
                    {model.capabilities?.length ? (
                      capabilityOptions
                        .filter((option) => model.capabilities?.includes(option.value))
                        .map((option) => (
                          <span key={option.value} className="rounded bg-elevated px-1 py-0.5 text-muted">
                            {option.label}
                          </span>
                        ))
                    ) : (
                      <span className="text-muted">Not declared</span>
                    )}
                  </div>
                </Table.Cell>
                <Table.Cell className="whitespace-nowrap px-1 text-left text-muted">
                  {model.maxContextTokens ? Number(model.maxContextTokens).toLocaleString('en-US') : 'Not set'}
                </Table.Cell>
                {[model.inputPrice, model.outputPrice, model.cachedPrice].map((value, index) => (
                  <Table.Cell key={index} className="whitespace-nowrap px-1 text-left text-muted">
                    {price(value)}
                  </Table.Cell>
                ))}
                <Table.Cell className="p-0 text-left">
                  {supported(model) ? (
                    <button
                      type="button"
                      aria-label={`Edit ${model.name}`}
                      disabled={busy}
                      className="cursor-pointer text-muted hover:text-text focus-visible:outline-muted disabled:opacity-50"
                      onClick={() => controller.onEditModel(model.id)}
                    >
                      Edit
                    </button>
                  ) : null}
                </Table.Cell>
              </Table.Row>
            ))}
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
      </Table>
      <div className="flex flex-wrap items-center justify-between gap-3">
        <p className="text-xs text-muted">
          {enabled} {enabled === 1 ? 'model' : 'models'} enabled
        </p>
        <Popup>
          <Popup.Trigger
            disabled={busy}
            className="h-8 w-auto rounded-full bg-elevated px-4 text-xs text-text hover:brightness-110"
          >
            Add generate text model
          </Popup.Trigger>
          <Popup.Content aria-label="Add generate text model" className="w-3xs">
            {textModelOptions.map((option) => (
              <Popup.Item key={option.value} className="gap-2" onClick={() => controller.onAddModel(option.value)}>
                <ProviderIcon provider={option.brand} />
                {option.label}
              </Popup.Item>
            ))}
          </Popup.Content>
        </Popup>
      </div>
    </SectionSurface>
  )
}
