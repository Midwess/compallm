<script lang="ts">
  import { DropdownMenu as UI } from 'bits-ui'
  import CheckIcon from 'phosphor-svelte/lib/CheckIcon'
  import type { ComponentProps, Snippet } from 'svelte'
  import { cx } from './shared'
  let {
    children: content,
    class: className,
    ...props
  }: Omit<ComponentProps<typeof UI.RadioItem>, 'children'> & { children?: Snippet } = $props()
</script>

<UI.RadioItem
  {...props}
  closeOnSelect
  class={cx(
    'flex h-8 w-full cursor-pointer items-center justify-between gap-4 rounded-md px-2 text-xs outline-none data-[highlighted]:bg-menu-hover focus-visible:outline-ring data-[disabled]:cursor-not-allowed data-[disabled]:opacity-50',
    className,
  )}
>
  {#snippet children({ checked })}{@render content?.()}{#if checked}<CheckIcon
        aria-hidden="true"
        class="size-3"
      />{/if}{/snippet}
</UI.RadioItem>
