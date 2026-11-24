<script lang="ts">
  import { animation, reducedMotion } from './motion'
  let { active }: { active: boolean } = $props()
  let canvas = $state<HTMLCanvasElement>()
  $effect(() => {
    const node = canvas
    if (!active || !node || reducedMotion()) return
    let disposed = false
    let stop: (() => void) | undefined
    let timer: ReturnType<typeof setTimeout> | undefined
    const preference = window.matchMedia('(prefers-reduced-motion: reduce)')
    const cancel = () => {
      disposed = true
      if (timer) clearTimeout(timer)
      stop?.()
    }
    preference.addEventListener('change', cancel)
    const launch = async () => {
      const { confetti } = await import('@tsparticles/confetti')
      if (disposed) return
      const sample = document.createElement('canvas').getContext('2d')
      if (!sample) return
      const style = getComputedStyle(node)
      const colors = ['--primary', '--ring', '--foreground', '--warning', '--success'].map((key) => {
        sample.clearRect(0, 0, 1, 1)
        sample.fillStyle = style.getPropertyValue(key).trim()
        sample.fillRect(0, 0, 1, 1)
        return (
          '#' +
          [...sample.getImageData(0, 0, 1, 1).data]
            .slice(0, 3)
            .map((channel) => channel.toString(16).padStart(2, '0'))
            .join('')
        )
      })
      const preset = animation.celebration
      const fire = await confetti.create(node, { count: 0, disableForReducedMotion: true })
      const container = await fire({
        count: disposed ? 0 : preset.count,
        colors,
        angle: 90,
        spread: preset.spread,
        startVelocity: preset.startVelocity,
        ticks: preset.ticks,
        position: { x: 50, y: 80 },
        scalar: 0.8,
        disableForReducedMotion: true,
      })
      stop = () => container?.destroy()
      if (disposed) {
        stop()
        return
      }
      timer = setTimeout(stop, preset.durationMs)
    }
    void launch().catch(cancel)
    return () => {
      cancel()
      preference.removeEventListener('change', cancel)
    }
  })
</script>

{#if active}<canvas bind:this={canvas} aria-hidden="true" class="pointer-events-none absolute inset-0 z-50 size-full"
  ></canvas>{/if}
