<script lang="ts">
  import { cx, type ElementProps } from './shared'
  let { children, class: className, ...props }: ElementProps = $props()
</script>

<div {...props} class={cx('grid gap-4 px-4 py-6', className)}>{@render children?.()}</div>
