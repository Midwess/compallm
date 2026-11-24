<script lang="ts">
  import type { HTMLAttributes } from 'svelte/elements'
  import { cx } from './shared'
  let {
    children: _children,
    class: className,
    element = $bindable(),
    ...props
  }: HTMLAttributes<HTMLElement> & { element?: HTMLElement } = $props()
</script>

<hr {...props} bind:this={element} class={cx('m-0 h-(--border-hairline) shrink-0 border-0 bg-divider', className)} />
