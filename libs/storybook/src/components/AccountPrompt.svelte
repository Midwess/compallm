<script lang="ts">
  import Content from './Content.svelte'
  import { cx, type Content as Value, type ElementProps } from './shared'
  let { children, action, class: className, ...props }: ElementProps & { action: Value } = $props()
</script>

<p {...props} class={cx('flex flex-wrap items-center gap-1 text-xsm text-muted-foreground', className)}>
  <span>{@render children?.()}</span><Content value={action} />
</p>
