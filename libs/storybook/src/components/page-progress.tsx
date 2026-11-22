import { useEffect, useLayoutEffect, useRef, useState } from 'react'
import { useReducedMotion } from 'motion/react'
import { animation, useRequestProgressMotion } from './motion'
import type { ProgressMessage } from './progress-shader'
import { ProgressMotion, type ProgressPhase } from './progress-motion'

export function PageProgress({ active }: { active: boolean }) {
  const host = useRef<HTMLDivElement>(null)
  const surface = useRef<HTMLDivElement>(null)
  const fallback = useRef<HTMLDivElement>(null)
  const worker = useRef<Worker | null>(null)
  const startedAt = useRef<number | null>(null)
  const [phase, setPhase] = useState<ProgressPhase>('idle')
  const [ready, setReady] = useState(false)
  const reduced = useReducedMotion()
  const [timeline] = useState(() => new ProgressMotion())
  useRequestProgressMotion(fallback, phase, !ready, timeline)
  const current = useRef({ phase, reduced: !!reduced, motion: timeline.snapshot() })
  useLayoutEffect(() => {
    timeline.setPhase(phase, performance.now())
    current.current = { phase, reduced: !!reduced, motion: timeline.snapshot(performance.timeOrigin) }
  }, [phase, reduced, timeline])

  useEffect(() => {
    const element = surface.current
    if (!element || !HTMLCanvasElement.prototype.transferControlToOffscreen || typeof Worker === 'undefined') return
    const canvas = document.createElement('canvas')
    canvas.className = 'absolute inset-x-0 top-0 h-10 w-full'
    element.append(canvas)
    let instance: Worker
    try {
      instance = new Worker(new URL('./progress-shader.ts', import.meta.url), { type: 'module' })
      worker.current = instance
      const offscreen = canvas.transferControlToOffscreen()
      instance.postMessage({ type: 'init', canvas: offscreen } satisfies ProgressMessage, [offscreen])
    } catch {
      worker.current?.terminate()
      worker.current = null
      canvas.remove()
      return
    }
    const send = (message: ProgressMessage) => instance.postMessage(message)
    const state = () => send({ type: 'state', ...current.current, hidden: document.hidden })
    const fail = () => {
      setReady(false)
      instance.terminate()
      if (worker.current === instance) worker.current = null
      canvas.remove()
    }
    instance.onerror = fail
    instance.onmessage = (event: MessageEvent<{ type: string }>) => {
      if (event.data.type === 'ready') {
        setReady(true)
        state()
      } else fail()
    }
    const sample = document.createElement('canvas').getContext('2d', { willReadFrequently: true })
    const theme = () => {
      if (!sample) return
      sample.clearRect(0, 0, 1, 1)
      sample.fillStyle = getComputedStyle(element).getPropertyValue('--primary').trim()
      sample.fillRect(0, 0, 1, 1)
      send({
        type: 'theme',
        color: Array.from(sample.getImageData(0, 0, 1, 1).data)
          .slice(0, 3)
          .map((v) => v / 255),
      })
    }
    const resize = new ResizeObserver(([entry]) =>
      send({ type: 'resize', width: Math.max(1, Math.round(entry.contentRect.width)) }),
    )
    resize.observe(element)
    const observer = new MutationObserver(theme)
    observer.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ['class', 'data-theme', 'data-appearance'],
    })
    document.addEventListener('visibilitychange', state)
    theme()
    state()
    return () => {
      instance.terminate()
      worker.current = null
      resize.disconnect()
      observer.disconnect()
      document.removeEventListener('visibilitychange', state)
      canvas.remove()
    }
  }, [])

  useEffect(() => {
    worker.current?.postMessage({ type: 'state', ...current.current, hidden: document.hidden } satisfies ProgressMessage)
  }, [phase, reduced])

  useLayoutEffect(() => {
    let finish: ReturnType<typeof setTimeout> | undefined
    let hide: ReturnType<typeof setTimeout> | undefined
    if (active) {
      startedAt.current ??= performance.now()
      const frame = requestAnimationFrame(() => setPhase('loading'))
      return () => cancelAnimationFrame(frame)
    } else if (startedAt.current !== null) {
      finish = setTimeout(
        () => {
          setPhase('finishing')
          hide = setTimeout(
            () => {
              setPhase('idle')
              startedAt.current = null
            },
            reduced
              ? animation.progress.settle
              : animation.progress.finish + animation.progress.settle + animation.progress.fade,
          )
        },
        Math.max(animation.progress.grace, animation.progress.minimumVisible - (performance.now() - startedAt.current)),
      )
    }
    return () => {
      clearTimeout(finish)
      clearTimeout(hide)
    }
  }, [active, reduced])

  useLayoutEffect(() => {
    const element = host.current
    if (!element || phase === 'idle') return
    const promote = (event?: Event) => {
      if (event && (event.target === element || (event as ToggleEvent).newState !== 'open')) return
      if (event && element.matches(':popover-open')) element.hidePopover()
      if (!element.matches(':popover-open')) element.showPopover()
    }
    promote()
    document.addEventListener('toggle', promote, true)
    return () => {
      document.removeEventListener('toggle', promote, true)
      if (element.matches(':popover-open')) element.hidePopover()
    }
  }, [phase])

  return (
    <div
      ref={host}
      popover="manual"
      data-ui="page-progress"
      className="pointer-events-none fixed inset-x-0 top-0 bottom-auto z-50 m-0 h-10 w-screen max-w-none
      overflow-hidden border-0 bg-transparent p-0"
      role="progressbar"
      aria-label="Loading requests"
      aria-hidden={phase === 'idle'}
    >
      <div ref={surface} className="absolute inset-0" hidden={phase === 'idle' || !ready} />
      <div
        ref={fallback}
        aria-hidden="true"
        className="absolute inset-x-0 top-0 h-0.5 origin-left bg-primary will-change-transform"
        hidden={ready}
        style={{ transform: 'scaleX(0)', opacity: 0 }}
      />
    </div>
  )
}
