<script lang="ts">
  import { DropdownMenu as UI } from 'bits-ui'
  import type { ComponentProps } from 'svelte'
  import CaretRightIcon from 'phosphor-svelte/lib/CaretRightIcon'
  import { cx } from './shared'
  let { children, class: className, ...props }: ComponentProps<typeof UI.SubTrigger> = $props()
</script>

<UI.SubTrigger
  {...props}
  class={cx(
    `flex h-8 w-full cursor-pointer items-center rounded-md px-2 text-xs outline-none
data-highlighted:bg-menu-hover focus-visible:outline-ring data-disabled:cursor-not-allowed data-disabled:opacity-50`,
    className,
  )}
>
  {@render children?.()}<CaretRightIcon class="ml-auto size-3" aria-hidden="true" />
</UI.SubTrigger>
