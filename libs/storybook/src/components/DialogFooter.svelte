<script lang="ts">
  import { cx, type ElementProps } from './shared'
  let { children, class: className, ...props }: ElementProps = $props()
</script>

<div {...props} class={cx('flex justify-end gap-2 border-t border-divider px-4 py-3.5', className)}>{@render children?.()}</div>
