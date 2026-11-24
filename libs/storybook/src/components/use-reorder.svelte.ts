import { animate } from 'motion'
import { transition } from './motion'
import { onDestroy, tick } from 'svelte'
import { SvelteMap } from 'svelte/reactivity'
const useRef = <T>(value: T) => ({ current: value })

interface DragState {
  pointerId: number
  key: string
  startY: number
  offsetY: number
  left: number
  width: number
  moved: boolean
  initial: string[]
  order: string[]
  slots: Array<{ key: string; top: number; height: number; offset: number }>
}
export function useReorder(
  readKeys: () => string[],
  onChange: (order: string[]) => void,
  readDisabled: () => boolean,
  readLabel: () => string,
) {
  const rowNodes = useRef(new SvelteMap<string, HTMLElement>())
  const beforeMove = useRef(new SvelteMap<string, number>())
  const drag = useRef<DragState | null>(null)
  const ghost = useRef<HTMLElement | null>(null)
  let announcement = $state('')
  const announce = (value: string) => {
    announcement = value
  }
  let committedDrop = $state(false)
  const setCommittedDrop = (value: boolean) => {
    committedDrop = value
  }
  let preview = $state<{ key: string; offsets: Record<string, number> } | null>(null)
  const setPreview = (value: typeof preview) => {
    preview = value
  }
  const id = 'reorder-' + crypto.randomUUID()
  const keys = $derived(readKeys())
  const disabled = $derived(readDisabled())
  const label = $derived(readLabel())
  onDestroy(() => ghost.current?.remove())
  $effect(() => {
    void keys
    void tick().then(() => {
      const previous = beforeMove.current
      if (!previous.size) return
      const reduced = window.matchMedia('(prefers-reduced-motion: reduce)').matches
      for (const [key, node] of rowNodes.current) {
        const top = previous.get(key)
        if (top === undefined || reduced) continue
        const offset = top - node.offsetTop
        if (Math.abs(offset) > 1) {
          animate(node, { y: [offset, 0] }, transition('reorder'))
        }
      }
      beforeMove.current.clear()
    })
  })
  function keyboardMove(from: number, to: number) {
    if (disabled || from === to || to < 0 || to >= keys.length) return
    setCommittedDrop(false)
    beforeMove.current = new SvelteMap(Array.from(rowNodes.current, ([key, node]) => [key, node.offsetTop]))
    const next = [...keys]
    const [entry] = next.splice(from, 1)
    next.splice(to, 0, entry)
    onChange(next)
    announce(`${label} moved to position ${to + 1} of ${next.length}`)
  }
  function endDrag(commit: boolean) {
    const current = drag.current
    setCommittedDrop(Boolean(current?.moved && commit))
    if (current?.moved && commit) {
      onChange(current.order)
      announce(`${label} placed at position ${current.order.indexOf(current.key) + 1} of ${keys.length}`)
    }
    drag.current = null
    ghost.current?.remove()
    ghost.current = null
    setPreview(null)
  }
  function startDrag(key: string, event: PointerEvent & { currentTarget: EventTarget & HTMLButtonElement }) {
    if (disabled || event.button !== 0) return
    const row = rowNodes.current.get(key)
    if (!row) return
    setCommittedDrop(false)
    const rect = row.getBoundingClientRect()
    drag.current = {
      pointerId: event.pointerId,
      key,
      startY: event.clientY,
      offsetY: event.clientY - rect.top,
      left: rect.left,
      width: rect.width,
      moved: false,
      initial: keys,
      order: keys,
      slots: keys.flatMap((entry) => {
        const node = rowNodes.current.get(entry)
        if (!node) return []
        const bounds = node.getBoundingClientRect()
        return [{ key: entry, top: bounds.top, height: bounds.height, offset: node.offsetTop }]
      }),
    }
    event.currentTarget.setPointerCapture(event.pointerId)
  }
  function pointerMove(event: PointerEvent & { currentTarget: EventTarget & HTMLButtonElement }) {
    const current = drag.current
    if (!current || current.pointerId !== event.pointerId) return
    if (!current.moved && Math.abs(event.clientY - current.startY) < 4) return
    event.preventDefault()
    if (!current.moved) {
      const row = rowNodes.current.get(current.key)
      if (!row) return
      const image = row.cloneNode(true) as HTMLElement
      image.removeAttribute('style')
      image.className =
        'pointer-events-none fixed top-0 left-0 z-50 flex items-center gap-2 rounded-lg bg-inset shadow-menu ring-1 ring-border'
      image.style.width = `${current.width}px`
      image.setAttribute('aria-hidden', 'true')
      image.inert = true
      image.querySelectorAll('[id], [aria-label], [aria-labelledby]').forEach((node) => {
        node.removeAttribute('id')
        node.removeAttribute('aria-label')
        node.removeAttribute('aria-labelledby')
      })
      row.querySelectorAll('input').forEach((input, index) => {
        const clone = image.querySelectorAll('input')[index]
        if (clone) clone.value = input.value
      })
      ;(row.closest('dialog') ?? document.body).appendChild(image)
      ghost.current = image
      current.moved = true
    }
    if (ghost.current) ghost.current.style.transform = `translate3d(${current.left}px,${event.clientY - current.offsetY}px,0)`
    let destination = 0
    for (let index = 1; index < current.slots.length; index++) {
      if (
        Math.abs(event.clientY - (current.slots[index].top + current.slots[index].height / 2)) <
        Math.abs(event.clientY - (current.slots[destination].top + current.slots[destination].height / 2))
      )
        destination = index
    }
    const order = current.initial.filter((key) => key !== current.key)
    order.splice(destination, 0, current.key)
    current.order = order
    const positions = Object.fromEntries(current.slots.map((slot) => [slot.key, slot.offset]))
    const offsets = Object.fromEntries(order.map((key, index) => [key, current.slots[index].offset - positions[key]]))
    setPreview({ key: current.key, offsets })
  }
  return {
    rowNodes,
    beforeMove,
    drag,
    get preview() {
      return preview
    },
    get committedDrop() {
      return committedDrop
    },
    get announcement() {
      return announcement
    },
    id,
    keyboardMove,
    startDrag,
    pointerMove,
    endDrag,
  }
}
