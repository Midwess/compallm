<script lang="ts">
  import { animate as motionAnimate } from 'motion'
  import { untrack, onMount, tick } from 'svelte'
  import type { ElementProps } from './shared'
  import { interaction, reducedMotion, transition as timing } from './motion'
  type Frame = {
    opacity?: number
    x?: number | string
    y?: number | string
    display?: string
    scale?: number
    transitionEnd?: { display?: string }
  }
  let {
    children,
    initial = false,
    animate,
    transition = timing('panel'),
    variants,
    layout,
    element = $bindable(),
    ...props
  }: ElementProps & {
    initial?: Frame | string | boolean
    animate?: Frame | string
    transition?: ReturnType<typeof timing>
    variants?: Record<string, Frame>
    layout?: boolean
  } = $props()
  let mounted = $state(false)
  onMount(() => {
    const start = typeof initial === 'string' ? variants?.[initial] : typeof initial === 'object' ? initial : undefined
    if (element && start && !reducedMotion()) motionAnimate(element, start, { duration: 0 })
    mounted = true
  })
  $effect(() => {
    const target = typeof animate === 'string' ? variants?.[animate] : animate
    const node = element
    const settings = transition
    if (!mounted || !node || !target) return
    return untrack(() => {
      const frame = { ...target },
        end = frame.transitionEnd
      delete frame.transitionEnd
      if (frame.display) node.style.display = frame.display
      delete frame.display
      const animation = motionAnimate(node, frame, reducedMotion() ? { duration: 0 } : settings)
      void animation.then(() => {
        if (end?.display) node.style.display = end.display
      })
      return () => animation.stop()
    })
  })
  let previous: { x: number; y: number } | undefined
  $effect.pre(() => {
    if (!layout || !element) return
    const node = element
    const observer = new MutationObserver(() => {
      void tick().then(() => {
        const bounds = node.getBoundingClientRect(),
          before = previous
        previous = { x: bounds.x, y: bounds.y }
        if (!before || reducedMotion()) return
        const x = before.x - bounds.x,
          y = before.y - bounds.y
        if (Math.abs(x) + Math.abs(y) > 1) motionAnimate(node, { x: [x, 0], y: [y, 0] }, timing('reorder'))
      })
    })
    const bounds = node.getBoundingClientRect()
    previous = { x: bounds.x, y: bounds.y }
    observer.observe(node.parentElement!, { childList: true })
    return () => observer.disconnect()
  })
</script>

<div {...props} bind:this={element} use:interaction>{@render children?.()}</div>
