<script lang="ts">
  import Content from './Content.svelte'
  import type { Content as Value } from './shared'
  import type { ElementProps } from './shared'
  import { cx } from './shared'
  let {
    children,
    title,
    action,
    tone = 'neutral',
    class: className,
    ...props
  }: ElementProps & { title?: string; action?: Value; tone?: 'neutral' | 'info' | 'warning' | 'error' | 'success' } = $props()
</script>

<aside
  {...props}
  class={cx(
    'rounded-lg border border-border bg-elevated p-3 text-xsm',
    tone === 'error' && 'border-destructive text-destructive',
    tone === 'warning' && 'text-warning',
    className,
  )}
>
  {#if title}<strong class="block font-medium">{title}</strong>{/if}{@render children?.()}<Content value={action} />
</aside>
