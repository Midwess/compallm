<script lang="ts">
  import Content from './Content.svelte'
  import { cx, type ElementProps, type Content as Value } from './shared'
  let {
    description,
    action,
    children,
    class: className,
    ...props
  }: ElementProps & { description?: Value; action?: Value } = $props()
</script>

<div {...props} class={cx('flex min-w-0 items-center justify-between gap-1.5', className)}>
  <div class="min-w-0">
    <h2 class="text-ui-15 leading-5.75 font-medium">{@render children?.()}</h2>
    {#if description}<p class="mt-0.5 text-xsm leading-5 text-muted-foreground"><Content value={description} /></p>{/if}
  </div>
  <Content value={action} />
</div>
