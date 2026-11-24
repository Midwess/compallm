<script lang="ts">
  import { DropdownMenu as UI } from 'bits-ui'
  import type { ComponentProps } from 'svelte'
  import { cx } from './shared'
  let { children, class: className, ...props }: ComponentProps<typeof UI.Item> = $props()
</script>

<UI.Item
  {...props}
  class={cx(
    `flex h-8 w-full cursor-pointer items-center rounded-md px-2 text-xs outline-none
data-highlighted:bg-menu-hover focus-visible:outline-ring data-disabled:cursor-not-allowed data-disabled:opacity-50`,
    className,
  )}
>
  {@render children?.()}
</UI.Item>
