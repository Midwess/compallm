<script lang="ts">
  import { DropdownMenu as UI } from 'bits-ui'
  import type { ComponentProps } from 'svelte'
  import { cx } from './shared'
  let { children, class: className, ...props }: ComponentProps<typeof UI.Trigger> = $props()
</script>

<UI.Trigger
  {...props}
  class={cx(
    'flex size-7 cursor-pointer items-center justify-center rounded-xl text-muted-foreground hover:bg-hover focus-visible:outline-ring',
    className,
  )}
>
  {@render children?.()}
</UI.Trigger>
