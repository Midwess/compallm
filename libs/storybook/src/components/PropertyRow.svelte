<script lang="ts">
  import Content from './Content.svelte'
  import { cx, type Content as Value, type ElementProps } from './shared'
  let { label, children, class: className, ...props }: ElementProps & { label: Value } = $props()
</script>

<div {...props} class={cx('grid min-h-7 grid-cols-[minmax(80px,1fr)_2fr] items-center gap-1.5', className)}>
  <span class="text-ui-12 leading-4.5 text-muted-foreground"><Content value={label} /></span>
  <div>{@render children?.()}</div>
</div>
