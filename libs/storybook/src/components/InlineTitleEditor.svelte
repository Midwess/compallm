<script lang="ts">
  import type { ComponentProps } from 'svelte'
  import TextField from './TextField.svelte'
  import { cx } from './shared'
  let { value = $bindable(''), class: className, ...props }: ComponentProps<typeof TextField> = $props()
</script>

<TextField
  {...props}
  bind:value
  class={cx('border-transparent bg-transparent p-0 h-8 text-ui-24 leading-8 font-medium', className)}
/>
