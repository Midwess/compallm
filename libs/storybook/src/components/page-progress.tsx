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
    return () => {
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
      data-ui="page-progress"
      className="pointer-events-none fixed inset-x-0 top-0 z-50 h-0.5 origin-top scale-y-[1.15] overflow-hidden"
      role="progressbar"
      aria-label="Loading requests"
      aria-hidden={!active}
    />
  )
}
