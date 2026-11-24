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
  class={cx('m-0 text-2xl leading-8 font-medium tracking-[-0.16px] text-foreground', className)}
>
  {@render children?.()}
</h1>
