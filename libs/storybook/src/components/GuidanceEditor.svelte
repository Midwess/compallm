<script lang="ts">
  import type { ComponentProps } from 'svelte'
  import MultilineField from './MultilineField.svelte'
  import { cx } from './shared'
  let { value = $bindable(''), class: className, ...props }: ComponentProps<typeof MultilineField> = $props()
</script>

<MultilineField {...props} bind:value class={cx('min-h-45 text-xsm', className)} />
