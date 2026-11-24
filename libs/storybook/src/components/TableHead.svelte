<script lang="ts">
  import { cx } from './shared'
  import type { HTMLAttributes } from 'svelte/elements'
  import { columnVisibility } from './table-context'
  const visibility = columnVisibility()
  let {
    children,
    class: className,
    column,
    ...props
  }: HTMLAttributes<HTMLElement> & {
    column?: string
    colspan?: number
    scope?: 'col' | 'colgroup' | 'row' | 'rowgroup'
  } = $props()
</script>

<th
  data-slot="table-head"
  data-column={column}
  scope="col"
  {...props}
  class={cx(`min-w-0 overflow-hidden text-left font-normal`, visibility(column), className)}
>
  <div class="min-w-0 truncate px-2 **:min-w-0 *:max-w-full">{@render children?.()}</div>
</th>
