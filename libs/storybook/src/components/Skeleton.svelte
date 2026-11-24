<script lang="ts">
  import { getLoadingContext } from '../context'
  import { optionalContext } from './form-context.svelte'
  import { cx, type ElementProps } from './shared'
  import { skeletonMotion } from './motion'
  import { loadingPresentation } from './loading-presentation.svelte'
  let { loading, class: className, children, ...props }: ElementProps & { loading?: boolean } = $props()
  const region = optionalContext(getLoadingContext)
  let active = $derived(loading ?? region?.initial ?? false)
  const presentation = loadingPresentation(() => !region && active)
  let show = $derived(active || (!region && presentation.visible))
  let visible = $derived(region?.visible ?? presentation.visible)
  const skin =
    'pointer-events-none select-none rounded-lg animate-none! border-transparent! bg-foreground/6! text-transparent! bg-[radial-gradient(ellipse_at_center,color-mix(in_oklab,var(--foreground)_5%,transparent),transparent_70%)]! bg-size-[70%_250%]! bg-no-repeat! caret-transparent shadow-none! placeholder:text-transparent! *:invisible!'
  function decorate(node: HTMLElement, state: { show: boolean; visible: boolean; className?: ElementProps['class'] }) {
    let cleanup = () => {}
    function update(next: typeof state) {
      cleanup()
      if (!next.show) return
      const targets = Array.from(node.children).filter((child): child is HTMLElement => child instanceof HTMLElement)
      if (!targets.length) targets.push(node)
      const classes = cx(skin, !next.visible && 'opacity-0!', next.className)
        .split(/\s+/)
        .filter(Boolean)
      const cleanups = targets.map((target) => {
        const added = classes.filter((name) => !target.classList.contains(name))
        const original = target.style.backgroundPosition
        target.classList.add(...added)
        const motion = next.visible ? skeletonMotion(target) : undefined
        return () => {
          motion?.destroy()
          target.style.backgroundPosition = original
          target.classList.remove(...added)
        }
      })
      cleanup = () => cleanups.forEach((stop) => stop())
    }
    update(state)
    return { update, destroy: () => cleanup() }
  }
</script>

<span
  {...props}
  class="contents"
  aria-hidden={show || undefined}
  inert={show}
  data-skeleton={show || undefined}
  use:decorate={{ show, visible, className }}
>
  {@render children?.()}
</span>
