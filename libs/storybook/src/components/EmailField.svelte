<script lang="ts">
  import type { ComponentProps } from 'svelte'
  import TextField from './TextField.svelte'
  import { cx } from './shared'
  let { value = $bindable(''), class: className, ...props }: ComponentProps<typeof TextField> = $props()
</script>

<TextField type="email" autocomplete="email" {...props} bind:value class={cx('h-12 p-1.5 text-ui-15', className)} />
