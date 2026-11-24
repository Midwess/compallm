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

<p {...props} bind:this={element} class={cx('m-0 text-xsm leading-6 font-book text-muted-foreground', className)}>
  {@render children?.()}
</p>
