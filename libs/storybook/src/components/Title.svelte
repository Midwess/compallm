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

<h1
  {...props}
  bind:this={element}
  class={cx('m-0 min-w-0 flex-1 truncate text-xs leading-5 font-normal text-foreground', className)}
>
  {@render children?.()}
</h1>
