import { Table } from '../../../components/table'
import { SectionSurface } from '../../../components/surfaces'
import { Searchbox } from '../../../components/fields'
import { Checkbox } from '../../../components/selection'
import { Button } from '../../../components/buttons'

import { useController, useProvider } from './controller'
import { hasValidModelPrices } from './validation'

function price(value: string) {
  return value === '' ? 'Review rates' : `$${value}`
}
export function ProviderModels() {
  const controller = useController()
  const models = useProvider((state) => state.models)
  const query = useProvider((state) => state.modelQuery)
  const mode = useProvider((state) => state.mode)
  const busy = useProvider((state) => state.submitting)
  const visible = models.filter((model) => model.name.toLowerCase().includes(query.toLowerCase()))
  const enabled = models.filter((model) => model.enabled).length
  return (
    <SectionSurface className="grid gap-4">
      <Searchbox>
        <Searchbox.Input
          aria-label="Search models…"
          placeholder="Search models…"
          value={query}
          onChange={(event) => controller.onModelQueryChanged(event.target.value)}
        />
      </Searchbox>
      <Table>
        <Table.Content className="w-full min-w-2xl table-fixed text-xs" aria-label="Provider models">
          <colgroup>
            <col className="w-16" />
            <col className="w-48" />
            <col className="w-28" />
            <col className="w-24" />
            <col className="w-24" />
            <col className="w-24" />
            <col className="w-14" />
          </colgroup>
          <Table.Header className="h-8 text-muted">
            <Table.Row className="border-b border-divider">
              {['Enabled', 'Model', 'Context · tokens', '1M input', '1M output', '1M cached'].map((label) => (
                <Table.Head key={label} className="px-2 text-left font-normal">
                  {label}
                </Table.Head>
              ))}
              <Table.Head className="sticky right-0 bg-inset">
                <span className="sr-only">Actions</span>
              </Table.Head>
            </Table.Row>
          </Table.Header>
          <Table.Body>
            {visible.map((model) => {
              const compatible = model.modes.includes(mode)
              return (
                <Table.Row key={model.id} className="h-12 border-b border-divider last:border-b-0" data-compatible={compatible}>
                  <Table.Cell className="px-2 text-left">
                    <Checkbox
                      aria-label={`Enable ${model.name}`}
                      checked={model.enabled}
                      disabled={!compatible || busy || !hasValidModelPrices(model)}
                      onChange={(event) => controller.onModelEnabledChanged(model.id, event.target.checked)}
                    />
                  </Table.Cell>
                  <Table.Cell className="px-2 text-left">
                    <span title={model.name} className="block max-w-48 truncate font-medium">
                      {model.name}
                    </span>
                    {model.description || !compatible ? (
                      <span
                        title={model.description ?? 'Unavailable in this mode'}
                        className="mt-1 block max-w-48 truncate text-xs text-muted"
                      >
                        {model.description ?? 'Unavailable in this mode'}
                      </span>
                    ) : null}
                  </Table.Cell>
                  <Table.Cell className="px-2 text-left text-muted">
                    {model.maxContextTokens ? Number(model.maxContextTokens).toLocaleString('en-US') : 'Not set'}
                  </Table.Cell>
                  {[model.inputPrice, model.outputPrice, model.cachedPrice].map((value, index) => (
                    <Table.Cell key={index} className="px-2 text-left text-muted">
                      {price(value)}
                    </Table.Cell>
                  ))}
                  <Table.Cell className="sticky right-0 bg-inset px-2 text-left">
                    {compatible ? (
                      <button
                        type="button"
                        aria-label={`Edit ${model.name}`}
                        disabled={busy}
                        className="cursor-pointer text-muted hover:text-text focus-visible:outline-focus disabled:opacity-50"
                        onClick={() => controller.onEditModel(model.id)}
                      >
                        Edit
                      </button>
                    ) : null}
                  </Table.Cell>
                </Table.Row>
              )
            })}
          </Table.Body>
        </Table.Content>
        {!visible.length ? (
          <Table.Empty>
            <p>{query ? 'No matching models.' : 'No known models for a custom provider.'}</p>
          </Table.Empty>
        ) : null}
      </Table>
      <div className="flex items-center justify-between gap-4">
        <p className="text-xs text-muted">
          {enabled} {enabled === 1 ? 'model' : 'models'} enabled
        </p>
        <Button className="h-8 px-6 text-xs" disabled={busy} onClick={controller.onAddModel}>
          Add custom model
        </Button>
      </div>
    </SectionSurface>
  )
}
