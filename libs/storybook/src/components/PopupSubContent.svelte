<script lang="ts">
  import { floatingMotion } from './motion'
  let ref = $state<HTMLElement | null>(null)
  $effect(() => {
    if (!ref) return
    const motion = floatingMotion(ref)
    return motion.destroy
  })

  import { DropdownMenu as UI } from 'bits-ui'
  import type { ComponentProps } from 'svelte'
  import { cx } from './shared'
  import { getFloatingContainer } from '../context'
  import { optionalContext } from './form-context.svelte'
  const container = optionalContext(getFloatingContainer)
  let { children, class: className, ...props }: ComponentProps<typeof UI.SubContent> = $props()
</script>

<UI.Portal to={container?.() ?? undefined}>
  <UI.SubContent
    bind:ref
    sideOffset={4}
    alignOffset={-4}
    collisionPadding={8}
    {...props}
    class={cx(
      `z-50 max-h-(--available-height) w-48 max-w-[calc(100vw-1rem)] overflow-y-auto overscroll-y-none
rounded-xl border border-border bg-menu p-1 font-sans text-xsm text-foreground shadow-menu outline-none origin-(--transform-origin)`,
      className,
    )}
  >
    {@render children?.()}
  </UI.SubContent>
</UI.Portal>
