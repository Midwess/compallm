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

<h2 {...props} bind:this={element} class={cx('m-0 text-ui-15 leading-5.75 font-medium text-foreground', className)}>
  {@render children?.()}
</h2>
