<script lang="ts">
  import Skeleton from './Skeleton.svelte'
  import { getLoadingContext } from '../context'
  import { optionalContext } from './form-context.svelte'
  import { cx } from './shared'
  import type { HTMLAttributes } from 'svelte/elements'
  import { getVisibleColumns } from './table-context'
  import { columnVisibility } from './table-context'
  const visibility = columnVisibility()
  let {
    children,
    class: className,
    column,
    colspan,
    ...props
  }: HTMLAttributes<HTMLElement> & {
    column?: string
    colspan?: number
    scope?: 'col' | 'colgroup' | 'row' | 'rowgroup'
  } = $props()
  const loading = optionalContext(getLoadingContext)
  const visibleColumns = getVisibleColumns()
</script>

<td
  data-slot="table-cell"
  data-column={column}
  colspan={colspan && visibleColumns() ? Math.min(colspan, visibleColumns()!) : colspan}
  {...props}
  class={cx(`min-w-0 overflow-hidden text-left`, visibility(column), className)}
>
  <Skeleton loading={!!loading?.refreshing && loading.visible}>
    <div class="min-w-0 truncate px-2 **:min-w-0 *:max-w-full">{@render children?.()}</div>
  </Skeleton>
</td>
