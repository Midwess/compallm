<script lang="ts">
  import { floatingMotion } from './motion'
  let ref = $state<HTMLElement | null>(null)
  $effect(() => {
    if (!ref) return
    const motion = floatingMotion(ref)
    return motion.destroy
  })

  import { Popover as UI } from 'bits-ui'
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
    collisionPadding={8}
    {...props}
    class={cx(
      'z-50 w-12 max-w-[calc(100vw-32px)] rounded-xl border-(length:--border-hairline) border-input bg-menu p-2 font-sans text-xsm text-foreground outline-none [&>*+*]:mt-1.5',
      className,
    )}
  >
    {@render children?.()}
  </UI.Content>
</UI.Portal>
