<script lang="ts">
  import { LoadingFallback } from '../../components/loading'
  import { usePermission } from '../../components/permission'
  import ArrowLeftIcon from 'phosphor-svelte/lib/ArrowLeftIcon'

  import UsersThreeIcon from 'phosphor-svelte/lib/UsersThreeIcon'

  import ShieldCheckIcon from 'phosphor-svelte/lib/ShieldCheckIcon'
  import { Sidebar, SidebarSection, SidebarFolder, SidebarItem, SidebarLoading } from '../../components/sidebar'
  import { SearchField } from '../../components/fields'
  import { Indicator } from './shared'
  import type { SettingsDestination } from './types'
  import { untrack } from 'svelte'

  import { readStore } from '../page-state.svelte'
  import { getSettingsController } from '../../context'
  import { NavigationPermission } from './sidebar'
  import { personal, account, organization, category, folder, item } from './sidebar'

  let {
    onNavigate,
  }: {
    onNavigate?: () => void
  } = $props()
  const __settings = getSettingsController()
  const roleRead = usePermission(() => ({ actions: ['role.read'] }))
  const groupRead = usePermission(() => ({ actions: ['group.read'] }))
  const roleCreate = usePermission(() => ({ actions: ['role.read', 'role.create'] }))
  const groupCreate = usePermission(() => ({ actions: ['group.read', 'group.create'] }))
  const c = untrack(() => getSettingsController())
  let location = $derived(readStore(__settings.store, (s) => s.location))
  let query = $derived(readStore(__settings.store, (s) => s.navigationQuery))
  let navigationReady = $derived(readStore(__settings.store, (s) => s.navigationLoaded !== false))
  let navigationError = $derived(readStore(__settings.store, (s) => s.navigationLoadError))
  let actorIsOwner = $derived(readStore(__settings.store, (s) => s.actorIsOwner))
  let roles = $derived(readStore(__settings.store, (s) => s.roles))
  let groups = $derived(readStore(__settings.store, (s) => s.groups))
  const match = (label: string) => label.toLowerCase().includes(query.toLowerCase())
  const navigate = (page: SettingsDestination, id?: string) => {
    c.navigate(page, id)
    onNavigate?.()
  }
</script>

{#snippet links(
  entries: readonly ((typeof personal)[number] | (typeof account)[number] | (typeof organization)[number])[],
)}{#each entries.filter((e) => match(e.label)) as { page, label, Icon } (page)}<NavigationPermission {page}>
      <SidebarItem
        class={item}
        {label}
        href={c.href?.(page) ?? `#/settings/${page}`}
        preload={!!c.href && ['profile', 'organization', 'organization-onboarding', 'members'].includes(page)}
        selected={location.page === page}
        onNavigate={() => navigate(page)}
      >
        {#snippet icon()}<Icon></Icon>{/snippet}
      </SidebarItem>
    </NavigationPermission>{/each}{/snippet}
<Sidebar navigationLabel="Settings navigation" class="h-full pt-4 pb-3">
  <button
    type="button"
    onclick={() => c.leaveSettings()}
    class="flex h-7 items-center gap-2 px-2 text-left text-xsm text-muted-foreground hover:text-foreground"
  >
    <ArrowLeftIcon class="size-3" aria-hidden="true"></ArrowLeftIcon>Back to app
  </button><SearchField
    aria-label="Search settings"
    placeholder="Search settings…"
    value={query}
    oninput={(e) => c.setNavigationQuery(e.currentTarget.value)}
  ></SearchField><SidebarSection label="Personal settings" class={`${category} mt-2`}>
    {@render links(personal)}
  </SidebarSection><SidebarSection label="Profile" class={category}>{@render links(account)}</SidebarSection><SidebarSection
    label="Organization"
    class={category}
  >
    {@render links(organization)}
  </SidebarSection>{#if navigationError}<button
      type="button"
      onclick={() => c.reloadNavigation?.()}
      class="px-2 text-left text-xs text-muted-foreground"
    >
      Retry navigation
    </button>{:else}{/if}<LoadingFallback
    pending={roleRead.pending || groupRead.pending || (!navigationReady && !navigationError)}
  >
    {#snippet fallback()}<SidebarLoading rows={6}
      ></SidebarLoading>{/snippet}{#if roleRead.allowed || groupRead.allowed}<SidebarSection
        label="Permissions"
        class={category}
      >
        {#if groupRead.allowed}<SidebarFolder
            label="Groups"
            class={folder}
            onAdd={groupCreate.allowed ? () => navigate('new-group') : undefined}
            addLabel="Create group"
          >
            {#snippet icon()}<UsersThreeIcon
              ></UsersThreeIcon>{/snippet}{#each groups.filter((g) => match(g.name)) as g (g.id)}<SidebarItem
                label={g.name}
                href={c.href?.('group', g.id) ?? `#/settings/groups/${g.id}`}
                class="gap-2 text-xsm [&>span:first-child]:size-3"
                selected={location.page === 'group' && location.id === g.id}
                onNavigate={() => navigate('group', g.id)}
              >
                {#snippet icon()}<UsersThreeIcon></UsersThreeIcon>{/snippet}
              </SidebarItem>{/each}
          </SidebarFolder>{:else}{/if}{#if roleRead.allowed}<SidebarFolder
            label="Roles"
            class={folder}
            onAdd={roleCreate.allowed ? () => navigate('new-role') : undefined}
            addLabel="Create role"
          >
            {#snippet icon()}<ShieldCheckIcon
              ></ShieldCheckIcon>{/snippet}{#each roles.filter((r) => (!r.owner || actorIsOwner) && match(r.name)) as r (r.id)}<SidebarItem
                label={r.name}
                href={c.href?.('role', r.id) ?? `#/settings/roles/${r.id}`}
                class="gap-2 text-xsm [&>span:first-child]:size-1.5"
                selected={location.page === 'role' && location.id === r.id}
                onNavigate={() => navigate('role', r.id)}
              >
                {#snippet icon()}<Indicator color={r.color}></Indicator>{/snippet}
              </SidebarItem>{/each}
          </SidebarFolder>{:else}{/if}
      </SidebarSection>{:else}{/if}
  </LoadingFallback>
</Sidebar>
