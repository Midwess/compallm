<script lang="ts">
  import { cx, type ElementProps } from './shared'
  let { children, class: className, ...props }: ElementProps = $props()
</script>

<div {...props} class={cx('flex min-h-12 items-center justify-between gap-4 border-b border-divider px-4', className)}>
  {@render children?.()}
</div>
