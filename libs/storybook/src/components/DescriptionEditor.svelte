<script lang="ts">
  import type { ComponentProps } from 'svelte'
  import MultilineField from './MultilineField.svelte'
  import { cx } from './shared'
  let { value = $bindable(''), class: className, ...props }: ComponentProps<typeof MultilineField> = $props()
</script>

<MultilineField {...props} bind:value class={cx('border-transparent bg-transparent p-0 min-h-27.5 text-xsm', className)} />
