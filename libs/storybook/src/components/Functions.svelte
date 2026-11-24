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

<div {...props} bind:this={element} class={cx('ml-auto flex min-w-0 shrink-0 items-center gap-2', className)}>
  {@render children?.()}
</div>
