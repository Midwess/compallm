<script lang="ts">
  import type { ComponentProps } from 'svelte'
  import TextField from './TextField.svelte'
  import { cx } from './shared'
  import { getFieldContext } from '../context'
  let { value = $bindable(''), class: className, ...props }: ComponentProps<typeof TextField> = $props()
  const id = getFieldContext()
</script>

<TextField
  {id}
  {...props}
  bind:value
  class={cx('h-8 rounded-xl px-1.5 py-1 text-sm leading-5 focus:outline-input', className)}
/>
