<script lang="ts">
  import SlidersHorizontalIcon from 'phosphor-svelte/lib/SlidersHorizontalIcon'
  import { Button } from '../../components/buttons'

  import { SearchField } from '../../components/fields'
  import { Table } from '../../components/table'
  import { Popup } from '../../components/popup'
  import { SettingsContent, Heading } from './shared'
  import { untrack } from 'svelte'
  import { readStore } from '../page-state.svelte'
  import { getSettingsController } from '../../context'

  const __settings = getSettingsController()
  const c = untrack(() => getSettingsController())
  let members = $derived(readStore(__settings.store, (s) => s.members))
  let query = $derived(readStore(__settings.store, (s) => s.query))
  let defaultLimit = $derived(readStore(__settings.store, (s) => s.memberDefaultLimitUsd))
  let columns = $state(['previous', 'current', 'used'])
  let visible = $derived(members.filter((m) => `${m.name} ${m.email}`.toLowerCase().includes(query.toLowerCase())))
  const money = (value: string | null) => (value === null ? '—' : `$${value}`)
</script>

<SettingsContent wide>
  <Heading description="Review individual monthly limits and usage.">Member limit overrides</Heading>
  <div class="mb-6 flex items-center justify-between gap-4 px-4">
    <SearchField
      aria-label="Search member limits"
      placeholder="Search by name…"
      value={query}
      oninput={(e) => c.pages.search.setQuery(e.currentTarget.value)}
      class="w-75 max-w-full"
    ></SearchField><Popup.Root>
      <Popup.Trigger aria-label="Display properties">
        <SlidersHorizontalIcon class="size-4"></SlidersHorizontalIcon>
      </Popup.Trigger><Popup.Content aria-label="Display properties" class="w-60">
        {#each ['previous', 'current', 'used'] as key (key)}<Popup.CheckboxItem
            checked={columns.includes(key)}
            onCheckedChange={() =>
              (columns = ((items) => (items.includes(key) ? items.filter((item) => item !== key) : [...items, key]))(columns))}
          >
            {{ previous: 'Last month', current: 'This month', used: 'Limit used' }[key]}
          </Popup.CheckboxItem>{/each}
      </Popup.Content>
    </Popup.Root>
  </div>
  <Table.Root>
    <Table.Content aria-label="Member spending limits" class="min-w-0">
      <Table.Header>
        <Table.Row>
          <Table.Head>Name</Table.Head>{#if columns.includes('previous')}<Table.Head>
              Last month
            </Table.Head>{:else}{/if}{#if columns.includes('current')}<Table.Head>
              This month
            </Table.Head>{:else}{/if}<Table.Head>Limit (monthly)</Table.Head>{#if columns.includes('used')}<Table.Head>
              Limit used
            </Table.Head>{:else}{/if}
        </Table.Row>
      </Table.Header><Table.Body items={visible} getKey={(m) => m.id}>
        {#snippet children(m)}<Table.Row>
            <Table.Cell>
              <span class="block text-xsm font-medium">{m.name}</span><span class="text-muted-foreground">{m.email}</span>
            </Table.Cell>{#if columns.includes('previous')}<Table.Cell>
                {money(m.previousUsageUsd)}
              </Table.Cell>{:else}{/if}{#if columns.includes('current')}<Table.Cell>
                {money(m.currentUsageUsd)}
              </Table.Cell>{:else}{/if}<Table.Cell>
              <Button variant="ghost" onclick={() => c.pages.dialogs.openDialog('member-limit', m.id)}>
                {#if m.limitUsd === null}{#if defaultLimit === null}Organization default{:else}{`$${defaultLimit} · default`}{/if}{:else}{money(
                    m.limitUsd,
                  )}{/if}
              </Button>
            </Table.Cell>{#if columns.includes('used')}<Table.Cell>
                {#if m.currentUsageUsd !== null && Number(m.limitUsd ?? defaultLimit) > 0}{`${Math.round((Number(m.currentUsageUsd) / Number(m.limitUsd ?? defaultLimit)) * 100)}%`}{:else}—{/if}
              </Table.Cell>{:else}{/if}
          </Table.Row>{/snippet}
      </Table.Body>
    </Table.Content>
  </Table.Root>
</SettingsContent>
