<script lang="ts">
  import Checkbox from './Checkbox.svelte'
  import type { ComponentProps, Snippet } from 'svelte'
  import { cx } from './shared'
  let {
    children,
    checked = $bindable(false),
    class: className,
    ...props
  }: ComponentProps<typeof Checkbox> & { children?: Snippet } = $props()
</script>

<label class={cx('inline-flex min-h-6 cursor-pointer items-center gap-0.5 text-xsm', className)}>
  <Checkbox {...props} bind:checked /><span>{@render children?.()}</span>
</label>
