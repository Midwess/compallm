<script lang="ts">
  import { DropdownMenu as UI } from 'bits-ui'
  import type { ComponentProps } from 'svelte'
  import { cx } from './shared'
  let { children, class: className, ...props }: ComponentProps<typeof UI.RadioGroup> = $props()
</script>

<UI.RadioGroup {...props} class={cx('', className)}>{@render children?.()}</UI.RadioGroup>
