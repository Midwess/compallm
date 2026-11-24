<script lang="ts">
  import type { HTMLAttributes } from 'svelte/elements'
  import { cx } from './shared'
  let {
    children,
    class: className,
    element = $bindable(),
    ...props
  }: HTMLAttributes<HTMLElement> & { element?: HTMLElement } = $props()
</script>

<kbd
  {...props}
  bind:this={element}
  class={cx(
    `max-w-xs rounded-full border border-border bg-elevated px-3 py-1.5 text-center text-xs leading-4
text-foreground shadow-floating`,
    className,
  )}
>
  {@render children?.()}
</kbd>
