<script lang="ts">
  import { Table } from '../../components/table'

  import { SettingsContent, Heading, Section } from './shared'

  import { readStore } from '../page-state.svelte'
  import { getSettingsController } from '../../context'
  import { ReportFilter } from './usage'
  import ContentValue from '../../components/Content.svelte'

  const __settings = getSettingsController()
  let data = $derived(readStore(__settings.store, (s) => s.usage))
  let models = $derived(readStore(__settings.store, (s) => s.models))
  let members = $derived(readStore(__settings.store, (s) => s.members))
  let keys = $derived(readStore(__settings.store, (s) => s.apiKeys))
</script>

<SettingsContent wide>
  <Heading description="Explore model usage and provider measurements for Compallm.">Usage explorer</Heading>
  <div class="mb-8 flex flex-wrap gap-4 px-4">
    <ReportFilter name="Models" options={models}></ReportFilter><ReportFilter
      name="Members"
      options={members.map((m) => m.name)}
    ></ReportFilter><ReportFilter name="API keys" options={keys.map((k) => k.name)}></ReportFilter><ReportFilter
      name="Providers"
      options={[]}
    ></ReportFilter>
  </div>
  <div class="mb-12 grid grid-cols-2 gap-4 px-4 lg:grid-cols-4">
    {#each ['Client requests', 'Input tokens', 'Output tokens', 'Provider cost estimate'] as label (label)}<div
        class="rounded-control bg-inset p-4"
      >
        <p class="text-xs text-muted-foreground"><ContentValue value={label} /></p>
        <p class="mt-5 text-2xl">—</p>
      </div>{/each}
  </div>
  <Section title="Usage by model" description="Token measurements and estimated upstream costs.">
    <Table.Root>
      <Table.Content aria-label="Usage by model" class="min-w-0">
        <Table.Header>
          <Table.Row>
            <Table.Head>Model</Table.Head><Table.Head>Requests</Table.Head><Table.Head>Input tokens</Table.Head><Table.Head>
              Output tokens
            </Table.Head><Table.Head>Provider cost</Table.Head>
          </Table.Row>
        </Table.Header><Table.Body items={data} getKey={(row) => row.id}>
          {#snippet children(row)}<Table.Row>
              <Table.Cell>{row.model}</Table.Cell><Table.Cell>{row.requests}</Table.Cell><Table.Cell>
                {row.inputTokens}
              </Table.Cell><Table.Cell>{row.outputTokens}</Table.Cell><Table.Cell>{row.providerCost}</Table.Cell>
            </Table.Row>{/snippet}
        </Table.Body>
      </Table.Content>
    </Table.Root>{#if !data.length}<Table.Empty>
        No usage to display. Usage measurements will appear when records are available.
      </Table.Empty>{:else}{/if}
  </Section>
  <p class="mt-6 px-4 text-xs leading-5 text-muted-foreground">
    Unknown measurements are shown as —. Provider cost is an estimate from saved rates.
  </p>
</SettingsContent>
