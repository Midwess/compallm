<script lang="ts">
  import { getDialog } from './dialog-context'
  import { cx, type ElementProps } from './shared'
  let { children, class: className, ...props }: ElementProps = $props()
  const scope = getDialog()
</script>

<h2 {...props} id={scope.titleId} class={cx('text-sm font-medium', className)}>{@render children?.()}</h2>
