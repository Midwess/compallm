<script lang="ts">
  import { floatingMotion } from './motion'
  let ref = $state<HTMLElement | null>(null)
  $effect(() => {
    if (!ref) return
    const motion = floatingMotion(ref)
    return motion.destroy
  })

  import { Select as UI } from 'bits-ui'
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
    sideOffset={4}
    align="start"
    strategy="fixed"
    collisionPadding={8}
    preventScroll={false}
    {...props}
    class={cx(
      'z-50 w-(--bits-select-anchor-width) max-w-[calc(100vw-1rem)] origin-(--bits-select-content-transform-origin) rounded-xl border-(length:--border-hairline) border-border bg-elevated font-sans text-xsm text-foreground shadow-menu outline-none',
      className,
    )}
  >
    <div class="max-h-[min(--spacing(80),var(--bits-select-content-available-height))] overflow-y-auto overscroll-y-none p-1">
      {@render children?.()}
    </div>
  </UI.Content>
</UI.Portal>
