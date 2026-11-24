<script lang="ts">
  import { floatingMotion } from './motion'
  let ref = $state<HTMLElement | null>(null)
  $effect(() => {
    if (!ref) return
    const motion = floatingMotion(ref)
    return motion.destroy
  })

  import { Tooltip as UI } from 'bits-ui'
  import type { ComponentProps } from 'svelte'
  import { cx } from './shared'
  import { getFloatingContainer } from '../context'
  import { optionalContext } from './form-context.svelte'
  const container = optionalContext(getFloatingContainer)
  let { children, class: className, ...props }: ComponentProps<typeof UI.Content> = $props()
</script>

<UI.Portal to={container?.() ?? undefined}>
  <UI.Content
    bind:ref
    sideOffset={6}
    {...props}
    class={cx(
      'z-50 max-w-xs rounded-full border border-border bg-elevated px-3 py-1.5 text-center text-xs leading-4 font-sans text-foreground shadow-floating',
      className,
    )}
  >
    {@render children?.()}
  </UI.Content>
</UI.Portal>
