<script lang="ts">
  import { cx, type ElementProps } from './shared'
  let { children, class: className, ...props }: ElementProps = $props()
</script>

<div
  {...props}
  class={cx('flex min-h-20 items-center justify-center py-4 text-center text-xs text-muted-foreground', className)}
>
  {@render children?.()}
</div>
