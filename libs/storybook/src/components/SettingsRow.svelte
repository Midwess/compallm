<script lang="ts">
  import Content from './Content.svelte'
  import { cx, type ElementProps, type Content as Value } from './shared'
  let {
    label,
    description,
    children,
    class: className,
    ...props
  }: ElementProps & { label: Value; description?: Value } = $props()
</script>

<div
  {...props}
  class={cx(
    'flex min-h-16.5 items-center gap-1.5 bg-inset p-2 max-[480px]:flex-wrap [&>div:first-child]:min-w-0 [&>div:first-child]:flex-1 [&_strong]:text-xsm [&_strong]:font-medium',
    className,
  )}
>
  <div>
    <strong class="block leading-4"><Content value={label} /></strong>{#if description}<p
        class="mt-0.5 text-ui-12 leading-4 font-book text-muted-foreground"
      >
        <Content value={description} />
      </p>{/if}
  </div>
  {@render children?.()}
</div>
