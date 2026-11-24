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

<p {...props} bind:this={element} class={cx(`text-xsm leading-5 text-destructive`, className)}>{@render children?.()}</p>
