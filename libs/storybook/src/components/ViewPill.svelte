<script lang="ts">
  import Button from './Button.svelte'
  import type { ButtonProps } from './Button.svelte'
  import { cx } from './shared'
  let { class: className, children, ...props }: ButtonProps = $props()
</script>

<Button {...props} class={cx('h-7 bg-menu text-ui-12 shadow-none', className)}>{@render children?.()}</Button>
