<script lang="ts">
  import Button from './Button.svelte'
  import type { ButtonProps } from './Button.svelte'
  import { cx } from './shared'
  let { class: className, children, ...props }: ButtonProps = $props()
</script>

<Button
  {...props}
  class={cx(
    'size-7 rounded-full border border-border/60 bg-inset text-muted-foreground hover:border-input hover:text-foreground [&_svg]:size-3.5',
    className,
  )}
>
  {@render children?.()}
</Button>
