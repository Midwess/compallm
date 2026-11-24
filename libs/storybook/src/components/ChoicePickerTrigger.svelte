<script lang="ts">
  import { Combobox as UI } from 'bits-ui'
  import type { ComponentProps } from 'svelte'
  import { cx } from './shared'
  import { getChoice } from './choice-picker-context'
  const scope = getChoice()
  let ref = $state<HTMLElement | null>(null)
  $effect(() => scope.setTrigger(ref))
  let { children, class: className, ...props }: ComponentProps<typeof UI.Trigger> = $props()
</script>

<UI.Trigger
  bind:ref
  data-form-field={scope.name}
  aria-invalid={scope.invalid || undefined}
  aria-describedby={scope.name ? `${scope.name}-error` : undefined}
  {...props}
  class={cx(
    'inline-flex max-w-full shrink-0 cursor-pointer items-center gap-1 rounded-full border-(length:--border-hairline) border-transparent bg-elevated py-0.5 px-2 text-left text-xs font-medium text-foreground shadow-control min-h-6 h-auto justify-start enabled:hover:bg-elevated-hover focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring disabled:cursor-not-allowed disabled:opacity-50',
    className,
  )}
>
  <span class="line-clamp-2 min-w-0 whitespace-normal text-left">{@render children?.()}</span>
</UI.Trigger>
