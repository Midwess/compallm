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

<div {...props} bind:this={element} class={cx(`h-fit py-1 overflow-y-auto`, className)}>{@render children?.()}</div>
