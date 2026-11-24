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

<header {...props} bind:this={element} class={cx('grid min-w-0 gap-1', className)}>{@render children?.()}</header>
