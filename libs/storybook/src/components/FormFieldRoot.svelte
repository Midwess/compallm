<script lang="ts">
  import { setFieldContext } from '../context'
  import { cx } from './shared'
  import type { ElementProps } from './shared'
  let { children, class: className, ...props }: ElementProps = $props()
  const id = $props.id()
  setFieldContext(id)
</script>

<div {...props} class={cx('grid min-w-0 content-start gap-1', className)}>{@render children?.()}</div>
