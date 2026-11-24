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

<span
  {...props}
  bind:this={element}
  class={cx(
    'inline-flex size-5 items-center justify-center rounded-full border-(length:--border-hairline) border-input bg-elevated text-ui-12 font-normal',
    className,
  )}
>
  {@render children?.()}
</span>
