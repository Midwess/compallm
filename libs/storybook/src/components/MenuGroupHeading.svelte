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

<div {...props} bind:this={element} class={cx('p-1.5 text-ui-12 font-medium text-muted-foreground', className)}>
  {@render children?.()}
</div>
