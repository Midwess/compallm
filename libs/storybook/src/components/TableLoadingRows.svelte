<script lang="ts">
  import { getColumns } from './table-context'
  import TableRow from './TableRow.svelte'
  import TableCell from './TableCell.svelte'
  import Skeleton from './Skeleton.svelte'
  import { cx } from './shared'
  let { rowClass = 'h-12' }: { rowClass?: string } = $props()
  const read = getColumns()
</script>

{#each Array(8) as _, index (index)}<TableRow class={rowClass} aria-hidden="true">
    {#each read().length ? read() : [{ id: 'content', loadingWidthClassName: undefined }] as column (column.id)}<TableCell
        column={column.id}
      >
        <Skeleton loading><span class={cx('block h-4 w-3/4 rounded-md', column.loadingWidthClassName)}></span></Skeleton>
      </TableCell>{/each}
  </TableRow>{/each}
