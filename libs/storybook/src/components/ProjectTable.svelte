<script lang="ts" generics="T">
  import { Table } from './table'
  import ProjectTableHeader from './ProjectTableHeader.svelte'
  import type { Snippet } from 'svelte'
  let {
    children: renderRow,
    label,
    items,
    getKey,
  }: { label: string; items?: readonly T[]; getKey?: (item: T) => string; children: Snippet<[T, number]> | Snippet } = $props()
</script>

<Table.Root tabindex={0} role="region" aria-label={label}>
  <Table.Content
    columns={[
      { id: 'name' },
      { id: 'health', visibilityClassName: 'hidden md:table-cell' },
      { id: 'priority', visibilityClassName: 'hidden lg:table-cell' },
      { id: 'lead', visibilityClassName: 'hidden md:table-cell' },
      { id: 'progress', visibilityClassName: 'hidden lg:table-cell' },
    ]}
    class="w-full min-w-0 border-collapse text-left text-xsm [&_th]:h-8 [&_th]:text-ui-12 [&_th]:font-normal [&_th]:text-muted-foreground [&_td]:h-12 [&_th]:border-b-[0.5px] [&_th]:border-divider [&_th]:py-1 [&_td]:border-b-[0.5px] [&_td]:border-divider [&_td]:py-1"
  >
    {#if items && getKey}<ProjectTableHeader /><Table.Body {items} {getKey} columns={5}>
        {#snippet children(item, index)}{@render (renderRow as Snippet<[T, number]>)(item, index)}{/snippet}
      </Table.Body>{:else}{@render (renderRow as Snippet)()}{/if}
  </Table.Content>
</Table.Root>
