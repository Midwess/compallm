<script lang="ts">
  import Preload from '../../../components/Preload.svelte'
  import { LoadingRegion } from '../../../components/loading'
  import { providerPageLayout } from './layout'
  import { Table } from '../../../components/table'
  import { Searchbox } from '../../../components/fields'
  import ArrowRightIcon from 'phosphor-svelte/lib/ArrowRightIcon'
  import { Button } from '../../../components/buttons'
  import { ProviderIcon } from '../../../components/provider-icons'
  import { ProviderFilters } from './filters'

  import { untrack } from 'svelte'
  import { readStore } from '../../page-state.svelte'
  import { getCreateProviderController } from '../../../context'

  const __provider = getCreateProviderController()
  const controller = untrack(() => getCreateProviderController())
  let providers = $derived(readStore(__provider.store, (state) => state.providers))
  let templatePreview = $derived(readStore(__provider.store, (state) => state.templatePreview))
  let ready = $derived(readStore(__provider.store, (state) => !state.templatePreview || !!state.providersLoaded))
  let loading = $derived(readStore(__provider.store, (state) => state.providersLoading))
  let error = $derived(readStore(__provider.store, (state) => state.providersLoadError))
  let hasMore = $derived(readStore(__provider.store, (state) => state.providersHasMore))
  let query = $derived(readStore(__provider.store, (state) => state.providerQuery))
  let protocolFilter = $derived(readStore(__provider.store, (state) => state.providerProtocolFilter))
  let filtered = $derived(
    templatePreview
      ? providers
      : providers.filter(
          (item) =>
            (protocolFilter === 'all' ||
              (protocolFilter === 'anthropic_messages'
                ? item.protocol === 'anthropic'
                : item.protocol === 'openai_compatible')) &&
            `${item.name} ${item.id} ${item.protocol} ${item.baseUrl ?? ''}`.toLowerCase().includes(query.toLowerCase()),
        ),
  )
</script>

<div class={`${providerPageLayout} gap-6`}>
  <div class="grid gap-1 px-4">
    <h2 class="text-2xl font-medium">Choose provider</h2>
    <p class="text-xsm text-muted-foreground">Select a provider to configure its connection and models.</p>
  </div>
  <div class="flex items-center gap-3 pl-4">
    <Searchbox.Root class="flex-1">
      <Searchbox.Input
        aria-label="Search providers…"
        placeholder="Search providers…"
        value={query}
        oninput={(event) => controller.onProviderQueryChanged(event.currentTarget.value)}
        onkeydown={(event) => {
          if (event.key !== 'Enter' || event.isComposing) return
          event.preventDefault()
          controller.onProviderSearch?.()
        }}
      ></Searchbox.Input>
    </Searchbox.Root>{#if !templatePreview}<ProviderFilters></ProviderFilters>{:else}{/if}
  </div>
  <LoadingRegion
    {ready}
    refreshing={!!loading && ready}
    pending={loading}
    controller={controller.loading?.providers}
    label="Loading providers"
  >
    <Table.Root>
      <Table.Content
        class="w-full table-fixed text-left text-xs"
        aria-label="Providers"
        columns={[
          { id: 'name' },
          { id: 'endpoint', class: 'w-1/2', visibilityClassName: 'hidden md:table-cell' },
          { id: 'choose', class: 'w-16' },
        ]}
      >
        <Table.Header class="h-9 text-muted-foreground [&>tr]:h-9">
          <Table.Row class="border-b border-divider">
            <Table.Head class="px-4 font-normal">Provider</Table.Head><Table.Head column="endpoint" class="px-4 font-normal">
              Base URL
            </Table.Head><Table.Head column="choose" class="pr-0 text-right">Choose</Table.Head>
          </Table.Row>
        </Table.Header><Table.Body
          loadingRowClass="h-14"
          items={filtered}
          getKey={(provider) => provider.id}
          paging={templatePreview
            ? {
                hasMore: !!hasMore,
                loading: !!loading,
                error: !!error,
                onLoadMore: () => controller.onLoadMoreProviders?.(),
              }
            : undefined}
        >
          {#snippet children(provider)}<Table.Row
              class="h-14 cursor-pointer border-b border-divider
[&>td]:relative [&>td]:isolate [&>td]:before:-z-1
[&>td]:before:pointer-events-none [&>td]:before:absolute [&>td]:before:inset-x-0 [&>td]:before:inset-y-1
[&>td]:before:bg-hover [&>td]:before:opacity-0 hover:[&>td]:before:opacity-100
focus-within:[&>td]:before:opacity-100
[&>td:first-child]:before:left-0.5 [&>td:first-child]:before:rounded-l-lg
[&>td:last-child]:before:right-0.5 [&>td:last-child]:before:rounded-r-lg"
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
                controller.onSelectProvider(provider.id)
              }}
            >
              <Table.Cell class="px-4">
                <Preload
                  href={controller.providerHref?.(provider.id) ?? '#'}
                  enabled={!!controller.providerHref}
                  class="block w-full cursor-pointer text-left focus-visible:outline-ring"
                  onclick={(event) => {
                    event.stopPropagation()
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
                    controller.onSelectProvider(provider.id)
                  }}
                >
                  <span class="flex items-center gap-3">
                    <ProviderIcon provider={provider.id} logoPath={provider.logoPath} class="size-5"></ProviderIcon><span>
                      <span class="block truncate text-sm font-medium">{provider.name}</span><span
                        class="mt-1 block text-xs text-muted-foreground"
                      >
                        {provider.id}
                      </span>
                    </span>
                  </span>
                </Preload>
              </Table.Cell><Table.Cell column="endpoint" class="truncate px-4 text-muted-foreground">
                {provider.baseUrl ?? 'Configure endpoint'}
              </Table.Cell><Table.Cell column="choose" class="pr-0 text-right">
                <Preload
                  href={controller.providerHref?.(provider.id) ?? '#'}
                  enabled={!!controller.providerHref}
                  aria-label={`Choose ${provider.name}`}
                  class="ml-auto flex size-6 items-center justify-center rounded-full text-muted-foreground hover:bg-hover hover:text-foreground focus-visible:outline-ring"
                  onclick={(event) => {
                    event.stopPropagation()
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
                    controller.onSelectProvider(provider.id)
                  }}
                >
                  <ArrowRightIcon aria-hidden="true" class="size-4"></ArrowRightIcon>
                </Preload>
              </Table.Cell>
            </Table.Row>{/snippet}
        </Table.Body>
      </Table.Content>{#if ready && !error && !filtered.length}<p class="py-12 text-center text-sm text-muted-foreground">
          No providers found
        </p>{:else}{/if}
    </Table.Root>
  </LoadingRegion>{#if error}<div role="alert" class="flex items-center justify-center gap-3 py-4 text-sm">
      <span>{error}</span><Button onclick={controller.onRetryCatalog}>Retry</Button>
    </div>{:else}{/if}{#if !templatePreview}<div class="flex items-center justify-between gap-4">
      <Button class="h-8 px-4 text-xs" onclick={controller.onUseCustomProvider}>Use custom provider</Button>
    </div>{:else}{/if}
</div>
