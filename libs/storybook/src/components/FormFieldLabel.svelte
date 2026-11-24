<script lang="ts">
  import type { HTMLLabelAttributes } from 'svelte/elements'
  import { cx } from './shared'
  let {
    children,
    class: className,
    element = $bindable(),
    ...props
  }: HTMLLabelAttributes & { element?: HTMLElement } = $props()
</script>

<label {...props} bind:this={element} class={cx('block text-ui-14 leading-5.25 font-medium', className)}>
  {@render children?.()}
</label>
