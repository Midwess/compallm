<script lang="ts">
  import { onMount } from 'svelte'
  import { animate } from 'motion'
  import { scrollOwner } from './collection'
  import { reducedMotion } from './motion'
  onMount(() => {
    let controls: ReturnType<typeof animate> | undefined
    const wheel = (e: WheelEvent) => {
      if (
        reducedMotion() ||
        e.ctrlKey ||
        Math.abs(e.deltaX) > Math.abs(e.deltaY) ||
        !e.deltaY ||
        !(e.target instanceof HTMLElement)
      )
        return
      const owner = scrollOwner(e.target)
      if (!owner || owner.scrollHeight <= owner.clientHeight) return
      const next = Math.max(0, Math.min(owner.scrollHeight - owner.clientHeight, owner.scrollTop + e.deltaY))
      if (next === owner.scrollTop) return
      e.preventDefault()
      controls?.stop()
      const start = owner.scrollTop
      owner.scrollTop = start + (next - start) * 0.8
      controls = animate(owner.scrollTop, next, { duration: 0.12, ease: 'easeOut', onUpdate: (v) => (owner.scrollTop = v) })
    }
    document.addEventListener('wheel', wheel, { passive: false })
    const stop = () => controls?.stop()
    document.addEventListener('keydown', stop)
    return () => {
      controls?.stop()
      document.removeEventListener('wheel', wheel)
      document.removeEventListener('keydown', stop)
    }
  })
</script>
