<script lang="ts">
  import { Select as UI } from 'bits-ui'
  import type { ComponentProps } from 'svelte'
  import { cx } from './shared'
  let {
    children,
    class: className,
    ...props
  }: Omit<ComponentProps<typeof UI.Item>, 'children'> & { children?: import('svelte').Snippet } = $props()
</script>

<UI.Item
  {...props}
  class={cx(
    'flex min-h-8 w-full cursor-pointer items-center gap-1 rounded-md px-1.5 py-0.5 text-xsm outline-none data-[highlighted]:bg-menu-hover data-[disabled]:cursor-not-allowed data-[disabled]:opacity-50',
    className,
  )}
>
  <span class="flex min-w-0 flex-1 items-center gap-1">{@render children?.()}</span>
</UI.Item>
