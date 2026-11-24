<script lang="ts">
  import { LoadingRegion } from '../../components/loading'

  import { PermissionRequest } from '../../components/permission'

  import { SearchField } from '../../components/fields'
  import { Button } from '../../components/buttons'
  import { Table } from '../../components/table'

  import { SettingsContent, Heading } from './shared'
  import { untrack } from 'svelte'
  import { readStore } from '../page-state.svelte'
  import { getSettingsController } from '../../context'
  import { Invitations, MemberRows, InvitationDialog, MemberFilters } from './members'
  import { memberColumns } from './members'

  const __settings = getSettingsController()
  const c = untrack(() => getSettingsController())
  let members = $derived(readStore(__settings.store, (s) => s.members))
  let query = $derived(readStore(__settings.store, (s) => s.query))
  let order = $derived(readStore(__settings.store, (s) => s.memberOrder))
  let ready = $derived(readStore(__settings.store, (s) => s.membersLoaded ?? !s.membersLoading))
  let loading = $derived(readStore(__settings.store, (s) => s.membersLoading))
  let next = $derived(readStore(__settings.store, (s) => s.membersHasNextPage))
  let failed = $derived(readStore(__settings.store, (s) => s.membersLoadError ?? false))
  let activeOpen = $state(true)
  let inactiveOpen = $state(true)
  let invitations = $derived(readStore(__settings.store, (s) => s.invitations.length))
  let confirmedComplete = $derived(readStore(__settings.store, (s) => s.membersComplete !== false))
  let complete = $derived(confirmedComplete && !loading && !next)
  let active = $derived((() => members.filter((member) => member.available !== false))())
  let inactive = $derived((() => members.filter((member) => member.available === false))())
  let descending = $derived(order === 'DESC_NULLS_LAST')
</script>

<SettingsContent wide class="flex min-h-full flex-col pt-16 pb-6">
  <Heading class="shrink-0">Members</Heading>
  <div class="-mt-7 mb-5 flex min-w-0 shrink-0 flex-wrap items-center gap-2">
    <SearchField
      aria-label="Search members"
      placeholder="Search by name or email"
      value={query}
      oninput={(e) => c.pages.search.setQuery(e.currentTarget.value)}
      onkeydown={(event) => {
        if (event.key === 'Enter' && !event.isComposing) {
          event.preventDefault()
          c.pages.members.search?.(event.currentTarget.value)
        }
      }}
      class="w-75 max-w-full"
    ></SearchField><MemberFilters></MemberFilters>{#if c.pages.members.invite || invitations}<PermissionRequest
        actions={['member.invite']}
      >
        {#snippet children(allowed)}{#if allowed}<InvitationDialog></InvitationDialog>{:else}{/if}{/snippet}
      </PermissionRequest>{:else}{/if}
  </div>
  <LoadingRegion {ready} pending={loading} label="Loading members" class="contents">
    <Table.Root
      forms={c.pages.members.forms}
      class="overflow-visible
[&_[data-slot=table-category-header]>td]:rounded-none [&_[data-slot=table-category-header]>td]:bg-transparent
[&_[data-slot=table-category-header]>td]:before:pointer-events-none
[&_[data-slot=table-category-header]>td]:before:absolute [&_[data-slot=table-category-header]>td]:before:inset-y-0
[&_[data-slot=table-category-header]>td]:before:-inset-x-4 lg:[&_[data-slot=table-category-header]>td]:before:-inset-x-12
[&_[data-slot=table-category-header]>td]:before:rounded-lg
[&_[data-slot=table-category-header]>td]:before:bg-inset [&_[data-slot=table-category-header]>td]:before:content-['']
[&_[data-slot=table-category-header]_button]:relative"
    >
      <Table.Content
        class="border-separate border-spacing-0"
        aria-label="Organization members"
        aria-busy={loading}
        columns={memberColumns}
      >
        <Table.Header class="sticky top-0 z-20 bg-surface">
          <Table.Row class="border-0">
            <Table.Head column="name" class="pl-0" aria-sort={descending ? 'descending' : 'ascending'}>
              <Button
                variant="ghost"
                class="h-8 gap-0.5 rounded-none p-0 text-xs font-normal text-muted-foreground"
                onclick={() => c.pages.members.setOrder?.(descending ? 'ASC_NULLS_LAST' : 'DESC_NULLS_LAST')}
              >
                Name <span aria-hidden="true">
                  {#if descending}↑{:else}↓{/if}
                </span>
              </Button>
            </Table.Head><Table.Head column="email">Email</Table.Head><Table.Head column="role" class="px-2">
              Role
            </Table.Head><Table.Head column="groups">Groups</Table.Head><Table.Head column="joined">Time</Table.Head><Table.Head
              column="actions"
            >
              <span class="sr-only">Actions</span>
            </Table.Head>
          </Table.Row>
        </Table.Header><Invitations></Invitations>{#if active.length || !inactive.length}<MemberRows
            label="Active"
            members={active}
            {complete}
            open={activeOpen}
            onOpenChange={(value) => (activeOpen = value)}
            paging={{
              hasMore: next && !inactive.length,
              loading,
              error: failed,
              onLoadMore: () => c.pages.members.loadMore?.(),
            }}
          ></MemberRows>{:else}{/if}{#if inactive.length}<MemberRows
            label="Inactive"
            members={inactive}
            {complete}
            open={inactiveOpen}
            onOpenChange={(value) => (inactiveOpen = value)}
            paging={{ hasMore: next, loading, error: failed, onLoadMore: () => c.pages.members.loadMore?.() }}
          ></MemberRows>{:else}{/if}
      </Table.Content>
    </Table.Root>
  </LoadingRegion>{#if ready && !failed && !members.length && !loading}<Table.Empty>
      No members match your search.
    </Table.Empty>{:else}{/if}
</SettingsContent>
