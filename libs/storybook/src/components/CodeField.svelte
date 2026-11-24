<script lang="ts">
  import TextField from './TextField.svelte'
  import { otpPattern } from '@compallm/contract/auth'
  import type { ComponentProps } from 'svelte'
  import { cx } from './shared'
  let { value = $bindable(''), class: className, ...props }: ComponentProps<typeof TextField> = $props()
</script>

<TextField
  inputmode="numeric"
  pattern={otpPattern}
  maxlength={1}
  {...props}
  bind:value
  class={cx('aspect-square h-auto w-13 p-0 text-center text-ui-20', className)}
/>
