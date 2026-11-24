<script lang="ts">
  import { DropdownMenu as UI } from 'bits-ui'
  import type { ComponentProps } from 'svelte'
  import { cx } from './shared'
  let { children, class: className, ...props }: ComponentProps<typeof UI.Separator> = $props()
</script>

<UI.Separator {...props} class={cx('my-1 h-(--border-hairline) bg-divider', className)}>{@render children?.()}</UI.Separator>
