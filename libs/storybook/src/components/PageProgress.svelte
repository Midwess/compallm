<script lang="ts">
  import { onMount } from 'svelte'
  import { ProgressMotion, progressTiming, type ProgressPhase } from './progress-motion'
  import type { ProgressMessage } from './progress-shader'
  let { active }: { active: boolean } = $props()
  let canvasElement: HTMLCanvasElement
  let host: HTMLDivElement
  let surface: HTMLDivElement
  let fallback: HTMLDivElement
  let phase = $state<ProgressPhase>('idle')
  let ready = $state(false)
  let reduced = $state(false)
  let mounted = $state(false)
  let worker: Worker | null = null
  let started: number | null = null
  let finish: ReturnType<typeof setTimeout>
  let hide: ReturnType<typeof setTimeout>
  const timeline = new ProgressMotion()
  $effect(() => {
    if (!mounted) return
    clearTimeout(finish)
    clearTimeout(hide)
    if (active) {
      started ??= performance.now()
      phase = 'loading'
    } else if (started !== null) {
      finish = setTimeout(
        () => {
          phase = 'finishing'
          hide = setTimeout(
            () => {
              phase = 'idle'
              started = null
            },
            reduced ? progressTiming.settle : progressTiming.finish + progressTiming.settle + progressTiming.fade,
          )
        },
        Math.max(progressTiming.grace, progressTiming.minimumVisible - (performance.now() - started)),
      )
    }
    return () => {
      clearTimeout(finish)
      clearTimeout(hide)
    }
  })
  $effect(() => {
    if (!mounted) return
    timeline.setPhase(phase, performance.now())
    worker?.postMessage({
      type: 'state',
      phase,
      reduced,
      motion: timeline.snapshot(performance.timeOrigin),
      hidden: document.hidden,
    } satisfies ProgressMessage)
    if (phase !== 'idle') {
      if (!host.matches(':popover-open')) host.showPopover()
    } else if (host.matches(':popover-open')) host.hidePopover()
  })
  $effect(() => {
    if (!mounted || ready) return
    let frame = 0
    const draw = (now: number) => {
      fallback.style.transform = `scaleX(${reduced ? (phase === 'loading' ? progressTiming.knee : phase === 'finishing' ? 1 : 0) : timeline.value(now)})`
      fallback.style.opacity = String(reduced ? (phase === 'idle' ? 0 : 1) : timeline.opacity(now))
      if (phase !== 'idle') frame = requestAnimationFrame(draw)
    }
    draw(performance.now())
    return () => cancelAnimationFrame(frame)
  })
  onMount(() => {
    mounted = true
    const promote = (event: Event) => {
      if (event.target === host || (event as ToggleEvent).newState !== 'open' || phase === 'idle') return
      if (host.matches(':popover-open')) host.hidePopover()
      host.showPopover()
    }
    document.addEventListener('toggle', promote, true)
    const media = matchMedia('(prefers-reduced-motion: reduce)')
    const update = () => (reduced = media.matches)
    update()
    media.addEventListener('change', update)
    let canvas: HTMLCanvasElement | undefined
    let resize: ResizeObserver | undefined
    let observer: MutationObserver | undefined
    if ('transferControlToOffscreen' in HTMLCanvasElement.prototype) {
      try {
        canvas = canvasElement
        worker = new Worker(new URL('./progress-shader.js', import.meta.url), { type: 'module' })
        const offscreen = canvas.transferControlToOffscreen()
        worker.postMessage({ type: 'init', canvas: offscreen } satisfies ProgressMessage, [offscreen])
        worker.onmessage = (e) => {
          if (e.data.type === 'ready') {
            ready = true
            worker?.postMessage({
              type: 'state',
              phase,
              reduced,
              motion: timeline.snapshot(performance.timeOrigin),
              hidden: document.hidden,
            } satisfies ProgressMessage)
          }
        }
        worker.onerror = () => {
          ready = false
          worker?.terminate()
          worker = null
        }
        const sample = document.createElement('canvas').getContext('2d', { willReadFrequently: true })
        const theme = () => {
          if (!sample) return
          sample.clearRect(0, 0, 1, 1)
          sample.fillStyle = getComputedStyle(surface).getPropertyValue('--primary').trim()
          sample.fillRect(0, 0, 1, 1)
          worker?.postMessage({
            type: 'theme',
            color: Array.from(sample.getImageData(0, 0, 1, 1).data)
              .slice(0, 3)
              .map((v) => v / 255),
          } satisfies ProgressMessage)
        }
        resize = new ResizeObserver(([entry]) =>
          worker?.postMessage({
            type: 'resize',
            width: Math.max(1, Math.round(entry.contentRect.width)),
          } satisfies ProgressMessage),
        )
        resize.observe(surface)
        observer = new MutationObserver(theme)
        observer.observe(document.documentElement, {
          attributes: true,
          attributeFilter: ['class', 'data-theme', 'data-appearance'],
        })
        theme()
      } catch {
        worker?.terminate()
        worker = null
      }
    }
    return () => {
      document.removeEventListener('toggle', promote, true)
      mounted = false
      clearTimeout(finish)
      clearTimeout(hide)
      media.removeEventListener('change', update)
      worker?.terminate()
      resize?.disconnect()
      observer?.disconnect()
    }
  })
</script>

<div
  bind:this={host}
  popover="manual"
  data-ui="page-progress"
  class="pointer-events-none fixed inset-x-0 top-0 bottom-auto z-50 m-0 h-10 w-screen max-w-none overflow-hidden border-0 bg-transparent p-0"
  role="progressbar"
  aria-label="Loading requests"
  aria-hidden={phase === 'idle'}
>
  <div bind:this={surface} class="absolute inset-0" hidden={phase === 'idle' || !ready}>
    <canvas bind:this={canvasElement} class="absolute inset-x-0 top-0 h-10 w-full"></canvas>
  </div>
  <div
    bind:this={fallback}
    aria-hidden="true"
    class="absolute inset-x-0 top-0 h-0.5 origin-left bg-primary will-change-transform"
    hidden={ready}
    style="transform:scaleX(0);opacity:0"
  ></div>
</div>
