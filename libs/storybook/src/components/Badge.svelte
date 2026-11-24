<script lang="ts">
  import { cx, type ElementProps } from './shared'
  let {
    tone = 'neutral',
    children,
    class: className,
    ...props
  }: ElementProps & { tone?: 'neutral' | 'success' | 'warning' | 'error' } = $props()
</script>

<span
  {...props}
  data-tone={tone}
  class={cx(
    'inline-flex min-h-6 items-center justify-center gap-1 rounded-md bg-elevated px-1 py-0 text-ui-12 font-medium data-[tone=success]:text-success data-[tone=warning]:text-warning data-[tone=error]:text-destructive',
    className,
  )}
>
  {@render children?.()}
</span>
