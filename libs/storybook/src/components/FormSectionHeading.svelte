<script lang="ts">
  import { getSectionContext } from '../context'
  import { cx, type ElementProps } from './shared'
  let { children, class: className, ...props }: ElementProps = $props()
  const id = getSectionContext()
</script>

<h2 {...props} {id} class={cx('flex h-6 items-center px-4 text-sm font-medium', className)}>{@render children?.()}</h2>
