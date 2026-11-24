<script lang="ts">
  import CaretDownIcon from 'phosphor-svelte/lib/CaretDownIcon'
  import { Select as UI } from 'bits-ui'
  import type { ComponentProps } from 'svelte'
  import { getSelect } from './select-context'
  const scope = getSelect()
  import { cx } from './shared'
  let { children, class: className, ...props }: ComponentProps<typeof UI.Trigger> = $props()
</script>

<UI.Trigger
  data-form-field={scope.name}
  aria-invalid={scope.invalid || undefined}
  aria-describedby={scope.name ? `${scope.name}-error` : undefined}
  {...props}
  class={cx(
    `flex min-h-7.5 w-full min-w-0 cursor-pointer items-center justify-between gap-1 rounded-xl
border-(length:--border-hairline)
        border-transparent bg-elevated px-1.5 py-1 text-left text-xsm leading-5 text-foreground shadow-control
        focus-visible:outline-1 focus-visible:outline-offset-1 focus-visible:outline-ring
        disabled:cursor-not-allowed disabled:opacity-50 disabled:shadow-none`,
    className,
  )}
>
  {@render children?.()}<CaretDownIcon size={10} aria-hidden="true" />
</UI.Trigger>
