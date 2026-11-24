<script lang="ts">
  import { Tooltip as UI } from 'bits-ui'
  import type { ComponentProps } from 'svelte'
  import { cx } from './shared'
  let { children, class: className, ...props }: ComponentProps<typeof UI.Trigger> = $props()
</script>

<UI.Trigger {...props} class={cx('inline-flex', className)}>{@render children?.()}</UI.Trigger>
