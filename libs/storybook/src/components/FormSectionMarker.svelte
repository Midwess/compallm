<script lang="ts">
  import { cx, type ElementProps } from './shared'
  let { children, class: className, ...props }: ElementProps = $props()
</script>

<span
  {...props}
  aria-hidden="true"
  class={cx(
    'absolute top-0.5 -left-3 flex size-5 items-center justify-center rounded-full border border-border bg-elevated text-xs text-muted-foreground',
    className,
  )}
>
  {@render children?.()}
</span>
