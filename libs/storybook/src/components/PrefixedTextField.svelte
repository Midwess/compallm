<script lang="ts">
  import TextField from './TextField.svelte'
  import Content from './Content.svelte'
  import type { ComponentProps } from 'svelte'
  import { cx, type Content as Value } from './shared'
  import { fieldModel } from './form-context.svelte'
  let {
    prefix,
    class: className,
    inputClassName,
    value = $bindable(''),
    ...props
  }: Omit<ComponentProps<typeof TextField>, 'prefix'> & { prefix: Value; inputClassName?: string } = $props()
  const form = fieldModel(() => props.name)
</script>

<div
  data-form-field={props.name}
  aria-invalid={props.name ? form?.invalid(props.name) || undefined : undefined}
  class={cx(
    'flex h-8 w-full min-w-0 overflow-hidden rounded-lg border-(length:--border-hairline) border-input bg-inset focus-within:border-muted-foreground/60 aria-invalid:border-destructive has-disabled:opacity-50',
    className,
  )}
>
  <span class="flex shrink-0 items-center border-r border-divider bg-elevated/40 text-xs text-muted-foreground">
    <Content value={prefix} />
  </span><TextField
    {...props}
    bind:value
    class={cx('h-full min-w-0 flex-1 rounded-none border-0 bg-transparent focus:border-transparent', inputClassName)}
  />
</div>
