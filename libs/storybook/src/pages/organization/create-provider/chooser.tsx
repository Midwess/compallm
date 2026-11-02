import { Table } from '../../../components/table'
import { Searchbox } from '../../../components/fields'
import { ArrowRightIcon } from '@phosphor-icons/react/dist/csr/ArrowRight'
import { Button } from '../../../components/buttons'

import { useController, useProvider } from './controller'
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
  const filtered = providers.filter((item) =>
    `${item.name} ${item.id} ${item.protocol} ${item.baseUrl}`.toLowerCase().includes(query.toLowerCase()),
  )
  return (
    <div className="grid gap-4 px-3 py-6">
      <Searchbox>
        <Searchbox.Input
          aria-label="Search providers…"
          placeholder="Search providers…"
          value={query}
          onChange={(event) => controller.onProviderQueryChanged(event.target.value)}
        />
      </Searchbox>
      <Table>
        <Table.Content className="w-full min-w-3xl table-fixed text-left text-xs" aria-label="Providers">
          <colgroup>
            <col className="w-1/4" />
            <col className="w-1/5" />
            <col />
            <col className="w-20" />
          </colgroup>
          <Table.Header className="h-9 text-muted [&>tr]:h-9">
            <Table.Row className="border-b border-divider">
              <Table.Head className="px-4 font-normal">Provider</Table.Head>
              <Table.Head className="px-4 font-normal">API protocol</Table.Head>
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
                    className="block w-full cursor-pointer text-left focus-visible:outline-focus"
                    onClick={(event) => {
                      event.stopPropagation()
                      controller.onSelectProvider(provider.id)
                    }}
                  >
                    <span className="block text-sm font-medium">{provider.name}</span>
                    <span className="mt-1 block text-xs text-muted">{provider.id}</span>
                  </button>
                </Table.Cell>
                <Table.Cell className="px-4 text-muted">{protocolLabels[provider.protocol]}</Table.Cell>
                <Table.Cell className="truncate px-4 text-muted">{provider.baseUrl}</Table.Cell>
                <Table.Cell className="text-center">
                  <button
                    type="button"
                    aria-label={`Choose ${provider.name}`}
                    className={`inline-flex size-6 cursor-pointer items-center justify-center rounded text-muted
hover:bg-elevated hover:text-text focus-visible:outline-focus`}
                    onClick={(event) => {
                      event.stopPropagation()
                      controller.onSelectProvider(provider.id)
                    }}
                  >
                    <ArrowRightIcon aria-hidden="true" className="size-4" />
                  </button>
                </Table.Cell>
              </Table.Row>
            ))}
          </Table.Body>
        </Table.Content>
        {!filtered.length ? <p className="py-12 text-center text-sm text-muted">No providers found</p> : null}
      </Table>
      <div className="flex items-center justify-between gap-4">
        <Button className="h-8 px-4 text-xs" onClick={controller.onUseCustomProvider}>
          Use custom provider
        </Button>
      </div>
    </div>
  )
}
