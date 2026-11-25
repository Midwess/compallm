<script lang="ts">
  import { SvelteMap } from 'svelte/reactivity'
  import { PermissionRequest } from '../../../components/permission'

  import { Button } from '../../../components/buttons'
  import { SearchField } from '../../../components/fields'

  import { Switch } from '../../../components/selection'
  import { SectionSurface } from '../../../components/surfaces'
  import { Table } from '../../../components/table'
  import { DisabledTooltip } from '../../../components/feedback'
  import type { PolicyProjection } from '../types'
  import { untrack } from 'svelte'
  import { readStore } from '../../page-state.svelte'
  import { getSettingsController } from '../../../context'
  import { PolicyFilters, PermissionRow } from './policy-table'
  import { ownerExplanation, defaultExpanded } from './policy-table'

  const __settings = getSettingsController()
  const c = untrack(() => getSettingsController().pages.role)
  let actions = $derived(readStore(__settings.store, (s) => s.policyActions))
  let draft = $derived(
    readStore(
      __settings.pages.role.store,
      (s) =>
        s.draft
          ? {
              builtin: s.draft.builtin,
              parsing: s.draft.parsing,
              diagnostics: s.draft.diagnostics,
              projection: s.draft.projection,
            }
          : null,
      ['draft.builtin', 'draft.parsing', 'draft.diagnostics', 'draft.projection'],
    ),
  )
  let query = $derived(readStore(__settings.pages.role.store, (s) => s.query))
  let filters = $derived(readStore(__settings.pages.role.store, (s) => s.filters))
  let busy = $derived(readStore(__settings.pages.role.store, (s) => s.busy))
  let disclosure: Record<string, boolean> = $state({})
  const id = $props.id()
  let projection = $derived.by(() => {
    const result = new SvelteMap<string, PolicyProjection[]>()
    for (const statement of draft?.projection ?? []) {
      const entries = result.get(statement.action) ?? []
      entries.push(statement)
      result.set(statement.action, entries)
    }
    return result
  })
</script>

{#if !draft}{:else}{@const categories = [...new Set(actions.map((a) => a.category))]}{@const normalized = query
    .trim()
    .toLocaleLowerCase()}{@const filtered = actions.filter((a) => {
    const selected = (projection.get(a.id) ?? []).some((entry) => entry.effect !== 'forbid')
    return (
      (!normalized || `${a.label} ${a.id} ${a.description} ${a.category}`.toLocaleLowerCase().includes(normalized)) &&
      (!filters['policy-resource'] || filters['policy-resource'] === 'all' || filters['policy-resource'] === a.category) &&
      (!filters['policy-access'] || filters['policy-access'] === 'all' || filters['policy-access'] === a.access) &&
      (filters['policy-selection'] === 'enabled' ? selected : filters['policy-selection'] === 'disabled' ? !selected : true)
    )
  })}{@const filtering = Boolean(normalized) || Object.values(filters).some((v) => v && v !== 'all')}{@const disabled =
    draft.builtin || draft.parsing || draft.diagnostics.length > 0}{@const selectedCount = actions.filter((a) =>
    (projection.get(a.id) ?? []).some((entry) => entry.effect !== 'forbid'),
  ).length}<SectionSurface class="flex min-w-0 flex-col overflow-hidden p-0">
    <div class="flex shrink-0 flex-wrap items-center gap-2 p-4">
      <SearchField
        aria-label="Search policies"
        placeholder="Search policies…"
        class="min-w-0 flex-1 [&>input]:border-border"
        value={query}
        oninput={(event) => c.setQuery(event.currentTarget.value)}
      ></SearchField><PolicyFilters {categories}
      ></PolicyFilters>{#each ['policy-resource', 'policy-selection', 'policy-access'] as const as key (key)}{#if filters[key] && filters[key] !== 'all'}<Button
            variant="ghost"
            class="h-7 px-2 text-xs"
            onclick={() => c.setFilter(key, 'all')}
          >
            {filters[key]}<span aria-hidden="true">×</span><span class="sr-only">
              Clear {key.replace('policy-', '')} filter
            </span>
          </Button>{:else}{/if}{/each}
    </div>
    {#if draft.projection.some((entry) => !entry.action)}<div class="px-4 pb-4 text-xs text-muted-foreground">
        {#each [...new SvelteMap(draft.projection
              .filter((entry) => !entry.action)
              .map((entry) => [entry.source, entry])).values()] as entry (entry.source)}<details class="mt-2">
            <summary class="cursor-pointer">{entry.reason ?? 'Advanced Cedar statement'}</summary>
            <pre class="mt-1 whitespace-pre-wrap wrap-break-word">{entry.source}</pre>
            <Button variant="ghost" class="h-6 px-0 text-xs" onclick={() => c.setPolicyMode('cedar')}>Edit in Cedar</Button>
          </details>{/each}
      </div>{:else}{/if}
    <div
      tabindex={0}
      role="grid"
      onkeydown={(event) => {
        if (event.key === 'ArrowLeft' || event.key === 'ArrowRight') {
          event.currentTarget.scrollLeft += event.key === 'ArrowRight' ? 40 : -40
          event.preventDefault()
        }
      }}
      aria-label="Permission catalog"
      class="mx-2 min-w-0 overflow-x-auto outline-offset-2 focus-visible:outline-2 focus-visible:outline-ring"
    >
      <Table.Root class="overflow-visible">
        <Table.Content aria-label="Permission policies" class="min-w-0 border-separate border-spacing-0">
          <colgroup><col class="w-[60%]" /><col /><col class="w-16" /></colgroup><Table.Header
            class="sticky top-0 z-20 bg-inset"
          >
            <Table.Row>
              <Table.Head class="px-2 py-0">Policy</Table.Head><Table.Head class="px-1 py-0">Description</Table.Head><Table.Head
                class="px-0 py-0 text-center"
              >
                Allow
              </Table.Head>
            </Table.Row>
          </Table.Header>{#each categories as category, index (index)}{@const matched = filtered.filter(
              (a) => a.category === category,
            )}{#if !matched.length}{:else}{@const all = actions.filter((a) => a.category === category)}{@const selected =
                all.filter((a) => (projection.get(a.id) ?? []).some((p) => p.effect !== 'forbid')).length}{@const editable =
                matched.filter(
                  (a) => !(projection.get(a.id) ?? []).some((p) => p.effect !== 'forbid' && !p.editable),
                )}{@const allEditableSelected =
                editable.length > 0 &&
                editable.every((a) => (projection.get(a.id) ?? []).some((p) => p.effect !== 'forbid'))}{@const open =
                disclosure[category] ?? (filtering || defaultExpanded.includes(category))}<Table.Category
                items={matched}
                getKey={(action) => action.id}
                label={category}
                count={selected + ' / ' + all.length}
                {open}
                onOpenChange={(next) => (disclosure = ((current) => ({ ...current, [category]: next }))(disclosure))}
              >
                {#snippet action()}<div class="flex justify-center">
                    <DisabledTooltip disabled={draft.builtin} reason={ownerExplanation}>
                      <PermissionRequest
                        actions={['role.policy.update']}
                        resource={{ type: 'Role', id: c.store.getState().draft?.id }}
                      >
                        {#snippet children(allowed)}<DisabledTooltip
                            disabled={!allowed}
                            reason="You do not have permission to perform this action."
                          >
                            <Switch
                              aria-label={'Allow ' + category + ' policies'}
                              checked={allEditableSelected}
                              disabled={!allowed || disabled || editable.length === 0}
                              aria-disabled={!allowed || disabled || editable.length === 0 || busy}
                              onchange={() =>
                                !busy &&
                                c.store.form.perform(
                                  () =>
                                    c.setPolicyActions(
                                      editable.map((a) => a.id),
                                      !allEditableSelected,
                                    ),
                                  ['draft.source'],
                                )}
                            ></Switch>
                          </DisabledTooltip>{/snippet}
                      </PermissionRequest>
                    </DisabledTooltip>
                  </div>{/snippet}{#snippet children(action)}<PermissionRow
                    id={id + '-category-' + index + '-' + action.id}
                    {action}
                    statements={projection.get(action.id) ?? []}
                    {disabled}
                    owner={draft.builtin}
                    {busy}
                  ></PermissionRow>{/snippet}
              </Table.Category>{/if}{/each}{#if !filtered.length}<Table.Body>
              <Table.Row>
                <Table.Cell colspan={3} class="py-8 text-center text-muted-foreground">
                  No policies match your search or filters.
                </Table.Cell>
              </Table.Row>
            </Table.Body>{:else}{/if}
        </Table.Content>
      </Table.Root>
    </div>
    <p class="shrink-0 px-4 py-4 text-xs leading-4.5 text-muted-foreground" aria-live="polite">
      {selectedCount} selected · {filtered.length} policies
    </p>
  </SectionSurface>{/if}
