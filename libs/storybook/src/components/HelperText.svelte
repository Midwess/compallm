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

<p {...props} bind:this={element} class={cx(`text-ui-12 leading-4.5 font-book text-muted-foreground`, className)}>
  {@render children?.()}
</p>
