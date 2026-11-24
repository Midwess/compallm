<script lang="ts">
  import { getDialog } from './dialog-context'
  import { cx, type ElementProps } from './shared'
  let { children, class: className, ...props }: ElementProps = $props()
  const scope = getDialog()
</script>

<p {...props} id={scope.descriptionId} class={cx('text-sm text-muted-foreground', className)}>{@render children?.()}</p>
