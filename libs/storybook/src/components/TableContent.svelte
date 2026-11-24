<script lang="ts">
  import { onMount, tick } from 'svelte'
  import { setVisibleColumns } from './table-context'
  import { setColumns, columnDisplayClass, type TableColumn } from './table-context'
  import { cx } from './shared'
  import type { HTMLTableAttributes } from 'svelte/elements'
  let { columns = [], children, class: className, ...props }: HTMLTableAttributes & { columns?: TableColumn[] } = $props()
  setColumns(() => columns)
  let table: HTMLTableElement
  let visible = $state<number>()
  setVisibleColumns(() => visible)
  onMount(() => {
    const update = () => {
      const row = table.tHead?.rows[0]
      visible = row
        ? Array.from(row.cells)
            .filter((cell) => getComputedStyle(cell).display !== 'none')
            .reduce((sum, cell) => sum + cell.colSpan, 0)
        : undefined
    }
    const observer = new ResizeObserver(update)
    observer.observe(table)
    const mutations = new MutationObserver(() => {
      void tick().then(update)
    })
    mutations.observe(table, { childList: true, subtree: true })
    window.addEventListener('resize', update)
    update()
    return () => {
      observer.disconnect()
      mutations.disconnect()
      window.removeEventListener('resize', update)
    }
  })
</script>

<table bind:this={table} {...props} data-slot="table" class={cx(`w-full max-w-full table-fixed text-left text-xs`, className)}>
  {#if columns.length}<colgroup>
      {#each columns as column (column.id)}<col
          class={cx(column.class, columnDisplayClass(column.visibilityClassName))}
        />{/each}
    </colgroup>{/if}{@render children?.()}
</table>
