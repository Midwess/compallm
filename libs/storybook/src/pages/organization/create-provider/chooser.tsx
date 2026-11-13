import { Table } from '../../../components/table'
import { Searchbox } from '../../../components/fields'
import { ArrowRightIcon } from '@phosphor-icons/react/dist/csr/ArrowRight'
import { Button } from '../../../components/buttons'
import { ToolbarAction } from '../../../components/navigation'

import { useController, useProvider } from './controller'
import { ProviderIcon } from '../../../components/provider-icons'
import { ProviderFilters } from './filters'
import type { ApiProtocol } from './types'

export const protocolLabels: Record<ApiProtocol, string> = {
  openai_compatible: 'OpenAI compatible',
  anthropic: 'Anthropic',
  provider_specific: 'Provider-specific',
}
export function ProviderChooser() {
  const controller = useController()
  const providers = useProvider((state) => state.providers)
  const query = useProvider((state) => state.providerQuery)
  const protocolFilter = useProvider((state) => state.providerProtocolFilter)
  const filtered = providers.filter(
    (item) =>
      (protocolFilter === 'all' ||
        (protocolFilter === 'anthropic_messages' ? item.protocol === 'anthropic' : item.protocol === 'openai_compatible')) &&
      `${item.name} ${item.id} ${item.protocol} ${item.baseUrl ?? ''}`.toLowerCase().includes(query.toLowerCase()),
  )
  return (
    <div className="mx-auto grid w-full max-w-4xl gap-4 px-4 py-6">
      <div className="flex items-center gap-3">
        <Searchbox className="flex-1">
          <Searchbox.Input
            aria-label="Search providers…"
            placeholder="Search providers…"
            value={query}
            onChange={(event) => controller.onProviderQueryChanged(event.target.value)}
          />
        </Searchbox>
        <ProviderFilters />
      </div>
      <Table>
        <Table.Content className="w-full min-w-3xl table-fixed text-left text-xs" aria-label="Providers">
          <colgroup>
            <col className="w-1/4" />
            <col />
            <col className="w-20" />
          </colgroup>
          <Table.Header className="h-9 text-muted-foreground [&>tr]:h-9">
            <Table.Row className="border-b border-divider">
              <Table.Head className="px-4 font-normal">Provider</Table.Head>
              <Table.Head className="px-4 font-normal">Base URL</Table.Head>
              <Table.Head className="text-center">Choose</Table.Head>
            </Table.Row>
          </Table.Header>
          <Table.Body>
            {filtered.map((provider) => (
              <Table.Row
                key={provider.id}
                className="h-14 cursor-pointer border-b border-divider hover:bg-inset"
                onClick={() => controller.onSelectProvider(provider.id)}
              >
                <Table.Cell className="px-4">
                  <button
                    type="button"
                    className="block w-full cursor-pointer text-left focus-visible:outline-ring"
                    onClick={(event) => {
                      event.stopPropagation()
                      controller.onSelectProvider(provider.id)
                    }}
                  >
                    <span className="flex items-center gap-3">
                      <ProviderIcon provider={provider.id} className="size-5" />
                      <span>
                        <span className="block text-sm font-medium">{provider.name}</span>
                        <span className="mt-1 block text-xs text-muted-foreground">{provider.id}</span>
                      </span>
                    </span>
                  </button>
                </Table.Cell>
                <Table.Cell className="truncate px-4 text-muted-foreground">
                  {provider.baseUrl ?? 'Configure endpoint'}
                </Table.Cell>
                <Table.Cell className="text-center">
                  <ToolbarAction
                    label={`Choose ${provider.name}`}
                    className="size-6"
                    onClick={(event) => {
                      event.stopPropagation()
                      controller.onSelectProvider(provider.id)
                    }}
                  >
                    <ArrowRightIcon aria-hidden="true" className="size-4" />
                  </ToolbarAction>
                </Table.Cell>
              </Table.Row>
            ))}
          </Table.Body>
        </Table.Content>
        {!filtered.length ? <p className="py-12 text-center text-sm text-muted-foreground">No providers found</p> : null}
      </Table>
      <div className="flex items-center justify-between gap-4">
        <Button className="h-8 px-4 text-xs" onClick={controller.onUseCustomProvider}>
          Use custom provider
        </Button>
      </div>
    </div>
  )
}
