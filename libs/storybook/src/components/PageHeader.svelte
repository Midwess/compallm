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

<header
  {...props}
  bind:this={element}
  data-ui="page-header"
  class={cx(
    'sticky top-0 z-10 flex h-10 min-w-0 shrink-0 items-center gap-4 border-b-[0.5px] border-divider bg-toolbar px-4',
    className,
  )}
>
  {@render children?.()}
</header>
