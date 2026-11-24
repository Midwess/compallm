<script lang="ts">
  import { DropdownMenu as UI } from 'bits-ui'
  import type { ComponentProps } from 'svelte'
  import { cx } from './shared'
  let { children, class: className, ...props }: ComponentProps<typeof UI.GroupHeading> = $props()
</script>

<UI.GroupHeading {...props} class={cx('px-2 py-1 text-xs text-muted-foreground', className)}>
  {@render children?.()}
</UI.GroupHeading>
