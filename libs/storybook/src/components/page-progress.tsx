import { BProgress } from '@bprogress/core'
import { useLayoutEffect, useRef } from 'react'
import { reducedMotion, progressWave } from './motion'

const minimumVisible = 500
const completionSpeed = 100
const template = `<div class="bar absolute inset-y-0 left-0 bg-primary"
style="box-shadow:0 0 6px color-mix(in srgb,var(--primary) 65%,transparent)">
<span class="absolute inset-x-0 top-0 h-8 overflow-hidden">
<svg data-progress-wave viewBox="0 0 640 32"
class="absolute top-0 right-0 h-8 w-160 max-w-full origin-top-right overflow-visible"
aria-hidden="true" style="color:var(--primary)">
<defs>
<linearGradient id="progress-energy" x1="0" x2="1">
<stop offset="0" stop-color="currentColor" stop-opacity="0"/>
<stop offset="0.35" stop-color="currentColor" stop-opacity="0.25"/>
<stop offset="0.75" stop-color="currentColor" stop-opacity="0.9"/>
<stop offset="1" stop-color="var(--primary-foreground)"/>
</linearGradient>
</defs>
<path d="M0 1 C150 1 270 3 360 12 S520 18 640 1 C530 7 480 5 390 4 S140 0 0 1Z"
fill="url(#progress-energy)" style="filter:blur(4px)"/>
<path d="M0 1 C180 1 290 2 380 8 S535 11 640 1 C525 5 480 3 390 2 S150 0 0 1Z"
fill="url(#progress-energy)"/>
</svg>
</span>
</div>`

export function PageProgress({ active }: { active: boolean }) {
  const host = useRef<HTMLDivElement>(null)
  const startedAt = useRef<number | null>(null)
  const completion = useRef<ReturnType<typeof setTimeout> | undefined>(undefined)

  useLayoutEffect(() => {
    const element = host.current
    if (!element) return
    const promote = (event?: Event) => {
      if (event && (event.target === element || (event as ToggleEvent).newState !== 'open')) return
      if (element.matches(':popover-open')) element.hidePopover()
      element.showPopover()
    }
    promote()
    document.addEventListener('toggle', promote, true)
    return () => {
      document.removeEventListener('toggle', promote, true)
      if (element.matches(':popover-open')) element.hidePopover()
      clearTimeout(completion.current)
      BProgress.configure({ trickle: false })
      BProgress.set(1)
      BProgress.remove()
      startedAt.current = null
    }
  }, [])

  useLayoutEffect(() => {
    if (!host.current) return
    clearTimeout(completion.current)
    const reduced = reducedMotion()
    BProgress.configure({
      parent: host.current,
      template,
      showSpinner: false,
      trickle: false,
      easing: 'linear',
      positionUsing: 'width',
      minimum: 0.08,
      maximum: 1,
      indeterminate: false,
      speed: reduced ? 0 : minimumVisible,
    })
    let wave: ReturnType<typeof progressWave> | undefined
    if (active) {
      if (startedAt.current === null || !BProgress.isStarted()) {
        BProgress.remove()
        startedAt.current = performance.now()
        BProgress.start()
        BProgress.set(reduced ? 0.35 : 0.9)
      }
      wave = progressWave(host.current)
    } else if (startedAt.current !== null) {
      const remaining = Math.max(0, minimumVisible - (performance.now() - startedAt.current))
      completion.current = setTimeout(() => {
        BProgress.configure({ speed: reduced ? 0 : completionSpeed })
        BProgress.done()
        startedAt.current = null
      }, remaining)
    }
    return () => {
      clearTimeout(completion.current)
      wave?.stop()
    }
  }, [active])

  return (
    <div
      ref={host}
      popover="manual"
      data-ui="page-progress"
      className="pointer-events-none fixed inset-x-0 top-0 bottom-auto z-50 m-0 h-0.5 w-screen max-w-none
origin-top scale-y-[0.9775] overflow-visible border-0 bg-transparent p-0"
      role="progressbar"
      aria-label="Loading requests"
      aria-hidden={!active}
    />
  )
}
