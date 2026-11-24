<script lang="ts">
  import Radio from './Radio.svelte'
  import Content from './Content.svelte'
  import type { ComponentProps, Snippet } from 'svelte'
  import { cx, type Content as ContentValue } from './shared'
  let {
    children,
    description,
    class: className,
    ...props
  }: ComponentProps<typeof Radio> & { children?: Snippet; description?: ContentValue } = $props()
</script>

<label
  class={cx(
    'flex min-h-15.5 w-75 max-w-full cursor-pointer items-center gap-1.5 rounded-lg border border-border bg-inset px-1.5 py-1.5 has-checked:border-control has-checked:bg-[color-mix(in_srgb,var(--control)_14%,var(--inset))] has-disabled:cursor-not-allowed has-disabled:opacity-50 [&_strong]:text-xsm [&_strong]:font-medium [&>span>span]:block',
    className,
  )}
>
  <Radio {...props} /><span>
    <strong>{@render children?.()}</strong>{#if description}<span class="text-ui-12 leading-4.5 text-muted-foreground">
        <Content value={description} />
      </span>{/if}
  </span>
</label>
