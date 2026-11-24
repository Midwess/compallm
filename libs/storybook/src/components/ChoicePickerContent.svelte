<script lang="ts">
  import { floatingMotion } from './motion'
  let ref = $state<HTMLElement | null>(null)
  $effect(() => {
    if (!ref) return
    const motion = floatingMotion(ref)
    return motion.destroy
  })

  import { Combobox as UI } from 'bits-ui'
  import type { ComponentProps } from 'svelte'
  import { cx } from './shared'
  import { getChoice } from './choice-picker-context'
  const scope = getChoice()
  import { getFloatingContainer } from '../context'
  import { optionalContext } from './form-context.svelte'
  const container = optionalContext(getFloatingContainer)
  let { children, class: className, ...props }: ComponentProps<typeof UI.Content> = $props()
</script>

<UI.Portal to={container?.() ?? undefined}>
  <UI.Content
    bind:ref
    customAnchor={scope.trigger}
    sideOffset={6}
    align="end"
    strategy="fixed"
    collisionPadding={8}
    preventScroll={false}
    {...props}
    class={cx(
      'z-50 flex max-h-(--bits-combobox-content-available-height) w-2xs max-w-[calc(100vw-1rem)] flex-col overflow-hidden rounded-xl border border-border bg-menu font-sans text-xsm text-foreground shadow-menu outline-none origin-(--bits-combobox-content-transform-origin)',
      className,
    )}
  >
    {@render children?.()}
  </UI.Content>
</UI.Portal>
