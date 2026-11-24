<script lang="ts">
  import { List } from '../../components/list'

  import { PermissionRequest, usePermission } from '../../components/permission'
  import HardDrivesIcon from 'phosphor-svelte/lib/HardDrivesIcon'
  import ChartBarIcon from 'phosphor-svelte/lib/ChartBarIcon'
  import KeyIcon from 'phosphor-svelte/lib/KeyIcon'
  import CubeIcon from 'phosphor-svelte/lib/CubeIcon'
  import UsersIcon from 'phosphor-svelte/lib/UsersIcon'
  import NotebookIcon from 'phosphor-svelte/lib/NotebookIcon'
  import PlugsIcon from 'phosphor-svelte/lib/PlugsIcon'

  import { SidebarSection, SidebarFolder, SidebarLoading } from '../../components/sidebar'

  import { untrack } from 'svelte'

  import { readStore } from '../page-state.svelte'
  import { getOrganizationController } from '../../context'
  import { Destination, Conversations } from './index'

  const __organization = getOrganizationController()
  const controller = untrack(() => getOrganizationController())
  const providerCreate = usePermission(() => ({ actions: ['provider.create'] }))
  const modelCreate = usePermission(() => ({ actions: ['model.create'] }))
  let providers = $derived(readStore(__organization.store, (state) => state.providers))
  let providersLoading = $derived(readStore(__organization.store, (state) => state.providersLoading))
  let providersHasMore = $derived(readStore(__organization.store, (state) => state.providersHasMore))
  let providersLoadError = $derived(readStore(__organization.store, (state) => state.providersLoadError))
  let models = $derived(readStore(__organization.store, (state) => state.models))
  let links = $derived(readStore(__organization.store, (state) => state.links))
  let layout = $derived(readStore(__organization.store, (state) => state.sidebarLayout))
</script>

{#each layout.filter((section) => section.items.length > 0) as section (section.category)}<SidebarSection
    label={section.category[0] + section.category.slice(1).toLowerCase()}
    onAdd={section.category === 'ROUTERS' ? controller.onCreateRouter : undefined}
    addLabel={section.category === 'ROUTERS' ? 'Add router' : undefined}
  >
    {#each section.items as item (item)}{#if item === 'PROVIDERS'}<PermissionRequest actions={['provider.read']}>
          {#snippet pending()}<SidebarLoading rows={1}></SidebarLoading>{/snippet}{#snippet children(
            allowed,
          )}{#if allowed}<SidebarFolder
                label="Providers"
                onAdd={providerCreate.allowed ? controller.onCreateProvider : undefined}
                addLabel="Add provider"
              >
                {#snippet icon()}<HardDrivesIcon
                  ></HardDrivesIcon>{/snippet}{#if providersLoading && !providers.length}<SidebarLoading rows={2}
                  ></SidebarLoading>{:else}<List.Root>
                    <List.Items
                      items={providers}
                      getKey={(entry) => entry.id}
                      paging={{
                        hasMore: !!providersHasMore,
                        loading: !!providersLoading,
                        error: !!providersLoadError,
                        onLoadMore: () =>
                          providersLoadError ? controller.onRetryProviders?.() : controller.onLoadMoreProviders?.(),
                      }}
                    >
                      {#snippet children(entry)}<Destination item={entry}>
                          {#snippet icon()}<HardDrivesIcon></HardDrivesIcon>{/snippet}
                        </Destination>{/snippet}
                    </List.Items>
                  </List.Root>{/if}
              </SidebarFolder>{:else}{/if}{/snippet}
        </PermissionRequest>{:else if item === 'MODELS'}<PermissionRequest actions={['model.read']}>
          {#snippet pending()}<SidebarLoading rows={1}></SidebarLoading>{/snippet}{#snippet children(
            allowed,
          )}{#if allowed}<SidebarFolder
                label="Models"
                onAdd={modelCreate.allowed ? controller.onCreateModel : undefined}
                addLabel="Add model"
              >
                {#snippet icon()}<CubeIcon></CubeIcon>{/snippet}{#each models as entry (entry.id)}<Destination item={entry}>
                    {#snippet icon()}<CubeIcon></CubeIcon>{/snippet}
                  </Destination>{/each}
              </SidebarFolder>{:else}{/if}{/snippet}
        </PermissionRequest>{:else if item === 'NOTEBOOKS'}<Destination
          item={{ id: 'notebooks', name: 'Notebooks', href: links.notebooks }}
        >
          {#snippet icon()}<NotebookIcon></NotebookIcon>{/snippet}
        </Destination>{:else if item === 'PLUGINS'}<Destination item={{ id: 'plugins', name: 'Plugins', href: links.plugins }}>
          {#snippet icon()}<PlugsIcon></PlugsIcon>{/snippet}
        </Destination>{:else if item === 'STATS'}<Destination item={{ id: 'stats', name: 'Stats', href: links.stats }}>
          {#snippet icon()}<ChartBarIcon></ChartBarIcon>{/snippet}
        </Destination>{:else if item === 'API_KEYS'}<Destination item={{ id: 'api-key', name: 'API key', href: links.keys }}>
          {#snippet icon()}<KeyIcon></KeyIcon>{/snippet}
        </Destination>{:else if item === 'MEMBERS'}<PermissionRequest actions={['member.read']}>
          {#snippet pending()}<SidebarLoading rows={1}></SidebarLoading>{/snippet}{#snippet children(
            allowed,
          )}{#if allowed}<Destination item={{ id: 'members', name: 'Members', href: links.members }}>
                {#snippet icon()}<UsersIcon></UsersIcon>{/snippet}
              </Destination>{:else}{/if}{/snippet}
        </PermissionRequest>{:else if item === 'CONVERSATIONS'}<Conversations></Conversations>{/if}{/each}
  </SidebarSection>{/each}
