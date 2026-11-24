<script lang="ts">
  import Trigger from './PopupTrigger.svelte'
  import type { ComponentProps } from 'svelte'
  import { cx } from './shared'
  let { class: className, ...props }: ComponentProps<typeof Trigger> = $props()
</script>

<Trigger
  {...props}
  class={cx(
    'inline-flex size-7 items-center justify-center rounded-full border border-border/60 bg-inset text-muted-foreground [&_svg]:size-3.5',
    className,
  )}
/>
