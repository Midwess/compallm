import { BProgress } from '@bprogress/core'
import { useLayoutEffect, useRef } from 'react'
import { reducedMotion } from './motion'

const minimumVisible = 500
const completionSpeed = 100
const template = '<div class="bar absolute inset-y-0 left-0 bg-primary"></div>'

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
    if (active) {
      if (startedAt.current === null || !BProgress.isStarted()) {
        BProgress.remove()
        startedAt.current = performance.now()
        BProgress.start()
        BProgress.set(reduced ? 0.35 : 0.9)
      }
    } else if (startedAt.current !== null) {
      const remaining = Math.max(0, minimumVisible - (performance.now() - startedAt.current))
      completion.current = setTimeout(() => {
        BProgress.configure({ speed: reduced ? 0 : completionSpeed })
        BProgress.done()
        startedAt.current = null
      }, remaining)
    }
    return () => clearTimeout(completion.current)
  }, [active])

  return (
    <div
      ref={host}
      popover="manual"
      data-ui="page-progress"
      className="pointer-events-none fixed inset-x-0 top-0 bottom-auto z-50 m-0 h-0.5 w-screen max-w-none
origin-top scale-y-[1.15] overflow-hidden border-0 bg-transparent p-0"
      role="progressbar"
      aria-label="Loading requests"
      aria-hidden={!active}
    />
  )
}
