<script lang="ts">
  import { Combobox } from 'bits-ui'
  import type { ComponentProps } from 'svelte'
  import { cx } from './shared'
  import { getChoice } from './choice-picker-context'
  let { class: className, oninput, ...props }: ComponentProps<typeof Combobox.Input> = $props()
  const scope = getChoice()
</script>

<Combobox.Input
  {...props}
  oninput={(event) => {
    scope.setQuery(event.currentTarget.value)
    oninput?.(event)
  }}
  class={cx(
    'block h-8 w-full min-w-0 rounded-xl border-(length:--border-hairline) border-input bg-inset py-1 pr-1.5 pl-6 text-left text-xsm leading-5 text-foreground placeholder:text-muted-foreground focus:border-muted-foreground/60 focus:outline-none',
    className,
  )}
/>
