<script lang="ts">
  import { onDestroy } from 'svelte'
  import { Table } from '../../src/components/table'
  import { List } from '../../src/components/list'
  import { CollectionForms } from '../../src/components/collection-forms'
  import Button from '../../src/components/Button.svelte'
  import RowEditor from './RowEditor.svelte'
  import PageProgress from '../../src/components/PageProgress.svelte'
  let { scenario = 'Table' }: { scenario?: string } = $props()
  const all = Array.from({ length: 1000 }, (_, i) => ({ id: `row-${i}`, name: `Record ${i + 1}` }))
  const forms = new CollectionForms()
  onDestroy(forms.dispose)
  let items = $state(all.slice(0, 20))
  let loading = $state(false)
  let error = $state(false)
  let active = $state(false)
  let timer: ReturnType<typeof setTimeout>
  onDestroy(() => clearTimeout(timer))
  function more() {
    if (loading || error || items.length >= all.length) return
    loading = true
    active = true
    timer = setTimeout(() => {
      items = [...items, ...all.slice(items.length, items.length + 20)]
      loading = false
      active = false
    }, 300)
  }
  let paging = $derived({
    hasMore: items.length < all.length,
    loading,
    error,
    onLoadMore: () => {
      error = false
      more()
    },
  })
</script>

<div
  class={scenario === 'WindowList'
    ? 'min-h-dvh bg-surface p-6 text-foreground'
    : 'h-dvh overflow-auto bg-surface p-6 text-foreground'}
>
  <PageProgress {active} />
  <div class="mb-4 flex flex-wrap gap-2">
    <Button onclick={() => (items = all.slice(0, 20))}>Show 20</Button><Button onclick={() => (items = all.slice(0, 100))}>
      Show 100
    </Button><Button onclick={() => (items = all)}>Show 1,000</Button><Button onclick={() => (items = items.slice().reverse())}>
      Reverse order
    </Button><Button onclick={() => (error = !error)}>Toggle load failure</Button>
  </div>
  {#if scenario.toLowerCase().includes('list')}<List.Root>
      <List.Items {items} getKey={(row) => row.id} {paging}>
        {#snippet children(row)}<div class="border-b border-divider py-2">{row.name}</div>{/snippet}
      </List.Items>
    </List.Root>{:else}<Table.Root {forms} class="overflow-visible">
      <Table.Content
        aria-label="Editable records"
        columns={[{ id: 'name' }, { id: 'edit', visibilityClassName: 'hidden sm:table-cell' }]}
      >
        <Table.Header class="sticky top-0 z-20 bg-surface">
          <Table.Row><Table.Head>Name</Table.Head><Table.Head column="edit">Edit</Table.Head></Table.Row>
        </Table.Header>{#if scenario.toLowerCase().includes('categor')}<Table.Category
            label="Members"
            {items}
            getKey={(row) => row.id}
            columns={2}
            {paging}
          >
            {#snippet children(row, index)}<Table.Row
                class={scenario.toLowerCase().includes('variable') && index % 4 === 0 ? 'h-24' : undefined}
              >
                <Table.Cell>{row.name}</Table.Cell><Table.Cell column="edit"><RowEditor record={row} /></Table.Cell>
              </Table.Row>{/snippet}
          </Table.Category>{:else}<Table.Body {items} getKey={(row) => row.id} columns={2} {paging}>
            {#snippet children(row, index)}<Table.Row
                class={scenario.toLowerCase().includes('variable') && index % 4 === 0 ? 'h-24' : undefined}
              >
                <Table.Cell>{row.name}</Table.Cell><Table.Cell column="edit"><RowEditor record={row} /></Table.Cell>
              </Table.Row>{/snippet}
          </Table.Body>{/if}
      </Table.Content>
    </Table.Root>{/if}
</div>
