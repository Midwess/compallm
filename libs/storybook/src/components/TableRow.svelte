<script lang="ts">
  import { getCollectionRow } from '../context'
  import { optionalContext } from './form-context.svelte'
  import { cx } from './shared'
  import type { HTMLAttributes } from 'svelte/elements'
  let {
    children,
    class: className,
    ...props
  }: HTMLAttributes<HTMLElement> & {
    colspan?: number
    scope?: 'col' | 'colgroup' | 'row' | 'rowgroup'
  } = $props()
  const row = optionalContext(getCollectionRow)
</script>

<tr
  data-slot="table-row"
  data-index={row?.index}
  data-collection-key={row?.key}
  {...props}
  class={cx(`h-12 border-b border-divider last:border-b-0`, className)}
>
  {@render children?.()}
</tr>
