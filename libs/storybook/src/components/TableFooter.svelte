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

<tfoot {...props} class={cx('border-t border-border bg-elevated/40 font-medium', className)}>
  {@render children?.()}
</tfoot>
