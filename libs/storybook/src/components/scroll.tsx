import { useEffect } from 'react'
import { animate } from 'motion'
import { animation, reducedMotion } from './motion'

function scrollOwner(event: WheelEvent): HTMLElement | undefined {
  for (const node of event.composedPath()) {
    if (!(node instanceof HTMLElement)) continue
    const style = getComputedStyle(node)
    const scrollable = node === document.scrollingElement || /^(auto|scroll)$/.test(style.overflowY)
    if (!scrollable) continue
    const limit = node.scrollHeight - node.clientHeight
    if (limit > 1 && (event.deltaY < 0 ? node.scrollTop > 0 : node.scrollTop < limit - 1)) return node
    if (style.overscrollBehaviorY !== 'auto') return
  }
}

function attachScrollMotion() {
  let active: { node: HTMLElement; target: number; controls: ReturnType<typeof animate<number>>; position: number } | undefined
  const stop = () => {
    active?.controls.stop()
    active = undefined
  }
  const wheel = (event: WheelEvent) => {
    if (event.defaultPrevented || !event.cancelable || event.ctrlKey || event.metaKey) return stop()
    if (reducedMotion() || event.shiftKey || Math.abs(event.deltaX) >= Math.abs(event.deltaY)) {
      stop()
      return
    }
    const node = scrollOwner(event)
    if (!node) return stop()
    const delta =
      event.deltaY *
      (event.deltaMode === WheelEvent.DOM_DELTA_LINE
        ? 16
        : event.deltaMode === WheelEvent.DOM_DELTA_PAGE
          ? node.clientHeight
          : 1)
    const current = node.scrollTop
    const previous = active?.node === node ? active : undefined
    const origin = previous && Math.sign(previous.target - current) === Math.sign(delta) ? previous.target : current
    const target = Math.max(0, Math.min(node.scrollHeight - node.clientHeight, origin + delta))
    stop()
    if (target === current) return
    event.preventDefault()
    const immediate = current + (target - current) * animation.scrollResponse
    node.scrollTo({ top: immediate, behavior: 'instant' })
    const controls = animate(immediate, target, {
      ...animation.scroll,
      onUpdate: (position) => {
        if (!node.isConnected) return stop()
        if (active) active.position = position
        node.scrollTo({ top: position, behavior: 'instant' })
      },
    })
    const scroll = { node, target, controls, position: immediate }
    active = scroll
    void controls.then(() => {
      if (active === scroll) active = undefined
    })
  }
  const keydown = (event: KeyboardEvent) => {
    if (['ArrowUp', 'ArrowDown', 'PageUp', 'PageDown', 'Home', 'End', ' ', 'Tab'].includes(event.key)) stop()
  }
  const scroll = (event: Event) => {
    if (active && event.target === active.node && Math.abs(active.node.scrollTop - active.position) > 2) stop()
  }
  const media = window.matchMedia('(prefers-reduced-motion: reduce)')
  document.addEventListener('wheel', wheel, { passive: false })
  document.addEventListener('pointerdown', stop, { passive: true, capture: true })
  document.addEventListener('touchstart', stop, { passive: true, capture: true })
  document.addEventListener('keydown', keydown, { capture: true })
  document.addEventListener('scroll', scroll, { passive: true, capture: true })
  media.addEventListener('change', stop)
  return () => {
    stop()
    document.removeEventListener('wheel', wheel)
    document.removeEventListener('pointerdown', stop, true)
    document.removeEventListener('touchstart', stop, true)
    document.removeEventListener('keydown', keydown, true)
    document.removeEventListener('scroll', scroll, true)
    media.removeEventListener('change', stop)
  }
}

let owners = 0
let detach: (() => void) | undefined

export function ScrollMotion() {
  useEffect(() => {
    if (owners++ === 0) detach = attachScrollMotion()
    return () => {
      if (--owners === 0) {
        detach?.()
        detach = undefined
      }
    }
  }, [])
  return null
}
