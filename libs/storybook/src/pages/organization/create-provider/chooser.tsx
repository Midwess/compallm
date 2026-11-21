import { providerPageLayout } from './layout'
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
  const templatePreview = useProvider((state) => state.templatePreview)
  const loading = useProvider((state) => state.providersLoading)
  const error = useProvider((state) => state.providersLoadError)
  const hasMore = useProvider((state) => state.providersHasMore)
  const query = useProvider((state) => state.providerQuery)
  const protocolFilter = useProvider((state) => state.providerProtocolFilter)
  const filtered = templatePreview
    ? providers
    : providers.filter(
        (item) =>
          (protocolFilter === 'all' ||
            (protocolFilter === 'anthropic_messages'
              ? item.protocol === 'anthropic'
              : item.protocol === 'openai_compatible')) &&
          `${item.name} ${item.id} ${item.protocol} ${item.baseUrl ?? ''}`.toLowerCase().includes(query.toLowerCase()),
      )
  return (
    <div className={`${providerPageLayout} gap-6`}>
      <div className="grid gap-1 px-4">
        <h2 className="text-2xl font-medium">Choose provider</h2>
        <p className="text-xsm text-muted-foreground">Select a provider to configure its connection and models.</p>
      </div>
      <div className="flex items-center gap-3 pl-4">
        <Searchbox.Root className="flex-1">
          <Searchbox.Input
            aria-label="Search providers…"
            placeholder="Search providers…"
            value={query}
            onChange={(event) => controller.onProviderQueryChanged(event.target.value)}
            onKeyDown={(event) => {
              if (event.key !== 'Enter' || event.nativeEvent.isComposing) return
              event.preventDefault()
              controller.onProviderSearch?.()
            }}
          />
        </Searchbox.Root>
        {!templatePreview ? <ProviderFilters /> : null}
      </div>
      <Table.Root>
        <Table.Content
          className="w-full table-fixed text-left text-xs"
          aria-label="Providers"
          columns={[
            { id: 'name' },
            { id: 'endpoint', className: 'w-1/2', visibilityClassName: 'hidden md:table-cell' },
            { id: 'choose', className: 'w-16' },
          ]}
        >
          <Table.Header className="h-9 text-muted-foreground [&>tr]:h-9">
            <Table.Row className="border-b border-divider">
              <Table.Head className="px-4 font-normal">Provider</Table.Head>
              <Table.Head column="endpoint" className="px-4 font-normal">
                Base URL
              </Table.Head>
              <Table.Head column="choose" className="pr-0 text-right">
                Choose
              </Table.Head>
            </Table.Row>
          </Table.Header>
          <Table.Body
            items={filtered}
            getKey={(provider) => provider.id}
            paging={
              templatePreview
                ? {
                    hasMore: !!hasMore,
                    loading: !!loading,
                    error: !!error,
                    onLoadMore: () => controller.onLoadMoreProviders?.(),
                  }
                : undefined
            }
          >
            {(provider) => (
              <Table.Row
                key={provider.id}
                className="h-14 cursor-pointer border-b border-divider
[&>td]:relative [&>td>div]:relative [&>td>div]:z-1
[&>td]:before:pointer-events-none [&>td]:before:absolute [&>td]:before:inset-x-0 [&>td]:before:inset-y-1
[&>td]:before:bg-hover [&>td]:before:opacity-0 hover:[&>td]:before:opacity-100
focus-within:[&>td]:before:opacity-100
[&>td:first-child]:before:left-0.5 [&>td:first-child]:before:rounded-l-lg
[&>td:last-child]:before:right-0.5 [&>td:last-child]:before:rounded-r-lg"
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
                      <ProviderIcon provider={provider.id} logoPath={provider.logoPath} className="size-5" />
                      <span>
                        <span className="block truncate text-sm font-medium">{provider.name}</span>
                        <span className="mt-1 block text-xs text-muted-foreground">{provider.id}</span>
                      </span>
                    </span>
                  </button>
                </Table.Cell>
                <Table.Cell column="endpoint" className="truncate px-4 text-muted-foreground">
                  {provider.baseUrl ?? 'Configure endpoint'}
                </Table.Cell>
                <Table.Cell column="choose" className="pr-0 text-right">
                  <ToolbarAction
                    label={`Choose ${provider.name}`}
                    className="ml-auto size-6"
                    onClick={(event) => {
                      event.stopPropagation()
                      controller.onSelectProvider(provider.id)
                    }}
                  >
                    <ArrowRightIcon aria-hidden="true" className="size-4" />
                  </ToolbarAction>
                </Table.Cell>
              </Table.Row>
            )}
          </Table.Body>
        </Table.Content>
        {loading && !filtered.length ? (
          <p role="status" className="py-12 text-center text-sm text-muted-foreground">
            Loading providers…
          </p>
        ) : null}
        {error ? (
          <div role="alert" className="flex items-center justify-center gap-3 py-4 text-sm">
            <span>{error}</span>
            <Button onClick={controller.onRetryCatalog}>Retry</Button>
          </div>
        ) : null}
        {!loading && !error && !filtered.length ? (
          <p className="py-12 text-center text-sm text-muted-foreground">No providers found</p>
        ) : null}
      </Table.Root>
      {!templatePreview ? (
        <div className="flex items-center justify-between gap-4">
          <Button className="h-8 px-4 text-xs" onClick={controller.onUseCustomProvider}>
            Use custom provider
          </Button>
        </div>
      ) : null}
    </div>
  )
}
