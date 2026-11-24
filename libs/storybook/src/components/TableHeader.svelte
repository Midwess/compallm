<script lang="ts">
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
</script>

<thead data-slot="table-header" {...props} class={cx(`h-8 text-muted-foreground [&>tr]:h-8`, className)}>
  {@render children?.()}
</thead>
