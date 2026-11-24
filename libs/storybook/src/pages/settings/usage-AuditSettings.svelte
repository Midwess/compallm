<script lang="ts">
  import { SearchField } from '../../components/fields'
  import { Table } from '../../components/table'

  import { SettingsContent, Heading } from './shared'
  import { untrack } from 'svelte'
  import { readStore } from '../page-state.svelte'
  import { getSettingsController } from '../../context'
  import { ReportFilter } from './usage'
  import ContentValue from '../../components/Content.svelte'

  const __settings = getSettingsController()
  const c = untrack(() => getSettingsController())
  let data = $derived(readStore(__settings.store, (s) => s.audit))
  let query = $derived(readStore(__settings.store, (s) => s.query))
  let actors = $derived(readStore(__settings.store, (s) => s.members))
  let filters = $derived(readStore(__settings.store, (s) => s.filters))
  let visible = $derived(
    data.filter(
      (e) =>
        `${e.actor} ${e.action} ${e.resource}`.toLowerCase().includes(query.toLowerCase()) &&
        (!filters.Actors || filters.Actors === 'all' || e.actor === filters.Actors) &&
        (!filters.Actions || filters.Actions === 'all' || e.action === filters.Actions),
    ),
  )
</script>

<SettingsContent wide>
  <Heading description="Review security and permission changes in Compallm.">Audit log</Heading>
  <div class="mb-8 flex flex-wrap gap-4 px-4">
    <SearchField
      aria-label="Search events"
      placeholder="Search events…"
      value={query}
      oninput={(e) => c.pages.search.setQuery(e.currentTarget.value)}
      class="w-xs max-w-full"
    ></SearchField><ReportFilter name="Actors" options={actors.map((m) => m.name)}></ReportFilter><ReportFilter
      name="Actions"
      options={[...new Set(data.map((e) => e.action))]}
    ></ReportFilter>
  </div>
  <Table.Root>
    <Table.Content aria-label="Audit events" class="min-w-0">
      <Table.Header>
        <Table.Row>
          {#each ['Actor', 'Action', 'Resource', 'Result'] as label (label)}<Table.Head>
              <ContentValue value={label} />
            </Table.Head>{/each}
        </Table.Row>
      </Table.Header><Table.Body items={visible} getKey={(e) => e.id}>
        {#snippet children(e)}<Table.Row>
            <Table.Cell>{e.actor}</Table.Cell><Table.Cell>{e.action}</Table.Cell><Table.Cell>
              {e.resource}
            </Table.Cell><Table.Cell>{e.result}</Table.Cell>
          </Table.Row>{/snippet}
      </Table.Body>
    </Table.Content>
  </Table.Root>{#if !visible.length}<Table.Empty>
      No events to display. Security and permission events will appear here.
    </Table.Empty>{:else}{/if}
</SettingsContent>
