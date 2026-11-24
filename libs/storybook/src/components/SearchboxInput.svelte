<script lang="ts">
  import type { ComponentProps } from 'svelte'
  import TextField from './TextField.svelte'
  import { cx } from './shared'
  let { value = $bindable(''), class: className, ...props }: ComponentProps<typeof TextField> = $props()
</script>

<TextField type="search" {...props} bind:value class={cx('h-8 py-1 pr-1.5 pl-6 text-xsm leading-5', className)} />
