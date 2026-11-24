<script lang="ts">
  import Button from './Button.svelte'
  import type { ButtonProps } from './Button.svelte'
  import { cx } from './shared'
  let { class: className, children, ...props }: ButtonProps = $props()
</script>

<Button size="content" {...props} class={cx('min-w-41.5', className)}>{@render children?.()}</Button>
