<script lang="ts">
  import { LoadingRegion } from '../../../components/loading'
  import { PermissionRequest } from '../../../components/permission'

  import { FormError } from '../../../components/form'
  import { DisabledTooltip } from '../../../components/feedback'

  import { SearchField } from '../../../components/fields'
  import { Switch } from '../../../components/selection'
  import { SectionSurface } from '../../../components/surfaces'
  import { SettingsSectionTitle } from '../../../components/settings'
  import { Table } from '../../../components/table'
  import { untrack } from 'svelte'
  import { readStore } from '../../page-state.svelte'
  import { getSettingsController } from '../../../context'

  import ContentValue from '../../../components/Content.svelte'

  const __settings = getSettingsController()
  const group = untrack(() => getSettingsController().pages.group)
  let draft = $derived(
    readStore(
      __settings.pages.group.store,
      (s) => (s.draft ? { id: s.draft.id, memberIds: s.draft.memberIds, memberSelection: s.draft.memberSelection } : null),
      ['draft.id', 'draft.memberIds', 'draft.memberSelection'],
    ),
  )
  let fallbackMembers = $derived(readStore(__settings.store, (s) => s.members))
  let pageMembers = $derived(readStore(__settings.pages.group.store, (s) => s.members))
  let count = $derived(readStore(__settings.pages.group.store, (s) => s.memberCount))
  let membersReady = $derived(readStore(__settings.pages.group.store, (s) => s.membersLoaded !== false))
  let membersLoadError = $derived(readStore(__settings.pages.group.store, (s) => s.membersLoadError))
  let hasNextPage = $derived(readStore(__settings.pages.group.store, (s) => s.hasNextPage))
  let membersLoading = $derived(readStore(__settings.pages.group.store, (s) => s.membersLoading))
  let query = $derived(readStore(__settings.pages.group.store, (s) => s.query))
  let readonly = $derived(readStore(__settings.store, (s) => s.readonly))
  let busy = $derived(readStore(__settings.pages.group.store, (s) => s.busy))
</script>

{#if !draft}{:else}{@const members = pageMembers ?? fallbackMembers}{@const entries = members.map((member) => ({
    id: member.id,
    name: member.name,
    detail: member.email,
    available: !('available' in member) || member.available,
  }))}{@const selected = draft.memberSelection
    ? Object.keys(draft.memberSelection).filter((id) => draft.memberSelection?.[id])
    : draft.memberIds}{@const normalized = query.trim().toLocaleLowerCase()}{@const visible = entries.filter((e) =>
    `${e.name} ${e.detail}`.toLocaleLowerCase().includes(normalized),
  )}{@const title = 'Members'}
  <section class="flex min-w-0 flex-col gap-4" aria-label={title}>
    <div class="flex shrink-0 items-center gap-2 px-4">
      <SettingsSectionTitle><ContentValue value={title} /></SettingsSectionTitle><span class="text-xs text-muted-foreground">
        {count ?? selected.length} selected
      </span>
    </div>
    <LoadingRegion ready={membersReady} pending={!!membersLoading} label="Loading group members">
      <SectionSurface class="flex min-w-0 flex-col p-0">
        <div class="shrink-0 p-4">
          <SearchField
            aria-label="Search group members"
            placeholder="Search members…"
            class="[&>input]:border-border"
            value={query}
            oninput={(event) => group.setQuery(event.currentTarget.value)}
          ></SearchField>
        </div>
        <div
          role="grid"
          aria-label={`${title} selection`}
          tabindex={0}
          class="mx-4 min-w-0 overflow-x-auto outline-offset-2 focus-visible:outline-2 focus-visible:outline-ring"
        >
          <Table.Root class="overflow-visible">
            <Table.Content aria-label="Select group members" class="min-w-0">
              <colgroup><col class="w-[42%]" /><col /><col class="w-12" /></colgroup><Table.Header
                class="sticky top-0 z-10 bg-inset"
              >
                <Table.Row>
                  <Table.Head class="px-0">Name</Table.Head><Table.Head class="px-0">Email</Table.Head><Table.Head class="px-0">
                    Include
                  </Table.Head>
                </Table.Row>
              </Table.Header><Table.Body
                items={visible}
                getKey={(entry) => entry.id}
                paging={{
                  hasMore: Boolean(hasNextPage),
                  loading: Boolean(membersLoading),
                  error: membersLoadError,
                  onLoadMore: () => group.loadMore?.(),
                }}
              >
                {#snippet children(entry)}<Table.Row>
                    <Table.Cell class="px-0 pr-3">
                      <span class="flex items-center gap-2 truncate text-xsm leading-4.5">{entry.name}</span>
                    </Table.Cell><Table.Cell class="truncate px-0 pr-3 text-xs text-muted-foreground" title={entry.detail}>
                      {entry.detail}
                    </Table.Cell><Table.Cell class="px-0">
                      <DisabledTooltip
                        disabled={readonly || busy || (!entry.available && !selected.includes(entry.id))}
                        reason={readonly
                          ? 'This group is read-only.'
                          : busy
                            ? 'Wait for the current update to finish.'
                            : 'This member is inactive and cannot be added.'}
                      >
                        <PermissionRequest
                          actions={[selected.includes(entry.id) ? 'group.member.remove' : 'group.member.add']}
                          resource={{ type: 'Group', id: draft.id }}
                          context={{ memberId: entry.id }}
                        >
                          {#snippet children(allowed)}<DisabledTooltip
                              disabled={!allowed}
                              reason="You do not have permission to perform this action."
                            >
                              <Switch
                                aria-label={`Include ${entry.name}`}
                                name={`draft.memberSelection.${entry.id}`}
                                disabled={!allowed || readonly || busy || (!entry.available && !selected.includes(entry.id))}
                              ></Switch>
                            </DisabledTooltip>{/snippet}
                        </PermissionRequest>
                      </DisabledTooltip>
                    </Table.Cell>
                  </Table.Row>{/snippet}
              </Table.Body>{#if !visible.length}<Table.Body>
                  <Table.Row>
                    <Table.Cell colspan={3} class="py-6 text-center text-muted-foreground">
                      No members match your search.
                    </Table.Cell>
                  </Table.Row>
                </Table.Body>{:else}{/if}
            </Table.Content>
          </Table.Root>
        </div>
        <FormError name="draft.memberSelection"></FormError>
      </SectionSurface>
    </LoadingRegion>
  </section>{/if}
