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

<caption {...props} class={cx('mt-4 text-sm text-muted-foreground', className)}>
  {@render children?.()}
</caption>
