import { animate } from 'motion'
import type { AnimationPlaybackControls } from 'motion'
import { progressTiming } from './progress-motion'
export const animation = {
  skeleton: { delayMs: 180, minimumVisibleMs: 180 },
  skeletonShimmer: { duration: 3.2, ease: 'linear', repeat: Infinity },
  progress: progressTiming,
  interaction: { duration: 0.12, ease: 'easeOut' },
  enter: { duration: 0.15, ease: 'easeOut' },
  exit: { duration: 0.1, ease: 'easeIn' },
  disclosure: { duration: 0.18, ease: 'easeOut' },
  reorder: { duration: 0.18, ease: 'easeOut' },
  panel: { duration: 0.2, ease: 'easeOut' },
  loading: { duration: 1, ease: 'linear', repeat: Infinity },
  textEntrance: { duration: 1.1, ease: [0.12, 1, 0.18, 1], stagger: 0.1, delay: 0.3, distance: 20 },
  celebration: { count: 100, spread: 90, startVelocity: 22, ticks: 180, durationMs: 3800 },
  cascade: { duration: 1.5, ease: [0.12, 1, 0.18, 1], stagger: 0.14, delay: 0.3, distance: 128 },
  distance: 6,
  panelDistance: 12,
  overlayScale: 0.96,
  collapsedRotation: -90,
  pressScale: 0.98,
  switchTravel: 10,
  indicatorScale: 0.8,
} as const
export function reducedMotion() {
  return typeof window !== 'undefined' && window.matchMedia('(prefers-reduced-motion: reduce)').matches
}
export function transition(name: 'interaction' | 'enter' | 'exit' | 'disclosure' | 'reorder' | 'panel') {
  return reducedMotion() ? { duration: 0 } : animation[name]
}
const properties = ['backgroundColor', 'borderColor', 'color', 'opacity', 'filter'] as const
type StyleSnapshot = Record<(typeof properties)[number], string>
function snapshot(node: HTMLElement): StyleSnapshot {
  const style = getComputedStyle(node)
  return Object.fromEntries(properties.map((key) => [key, style[key]])) as StyleSnapshot
}

function attachInteraction(node: HTMLElement) {
  let previous = snapshot(node)
  let running: ReturnType<typeof animate> | undefined
  let frame = 0
  let press: ReturnType<typeof animate> | undefined
  const original = Object.fromEntries(properties.map((key) => [key, node.style[key]])) as StyleSnapshot
  const update = () => {
    const current = running ? snapshot(node) : previous
    running?.stop()
    for (const key of properties) node.style[key] = original[key]
    const next = snapshot(node)
    previous = next
    if (node.matches(':disabled')) {
      press?.stop()
      animate(node, { scale: 1 }, { duration: 0 })
    }
    if (reducedMotion()) return
    const values = Object.fromEntries(
      properties.filter((key) => current[key] !== next[key]).map((key) => [key, [current[key], next[key]]]),
    )
    if (!Object.keys(values).length) return
    const controls = animate(node, values, animation.interaction)
    running = controls
    void controls.then(() => {
      if (running !== controls) return
      running = undefined
      for (const key of properties) node.style[key] = original[key]
    })
  }
  const schedule = () => {
    cancelAnimationFrame(frame)
    frame = requestAnimationFrame(update)
  }
  const pressed = (event: Event) => {
    if (node.tagName !== 'BUTTON' || node.matches(':disabled') || reducedMotion()) return
    if (event instanceof KeyboardEvent && ![' ', 'Enter'].includes(event.key)) return
    press?.stop()
    press = animate(
      node,
      { scale: event.type === 'pointerdown' || event.type === 'keydown' ? animation.pressScale : 1 },
      transition('interaction'),
    )
  }
  const pressEvents = ['pointerdown', 'pointerup', 'pointerleave', 'pointercancel', 'keydown', 'keyup', 'blur']
  for (const event of pressEvents) node.addEventListener(event, pressed)
  const events = ['pointerenter', 'pointerleave', 'pointerdown', 'pointerup', 'pointercancel', 'focus', 'blur', 'change']
  for (const event of events) node.addEventListener(event, schedule)
  const observer = new MutationObserver(schedule)
  observer.observe(node, {
    attributes: true,
    attributeFilter: [
      'class',
      'disabled',
      'aria-pressed',
      'aria-checked',
      'aria-selected',
      'aria-current',
      'aria-invalid',
      'data-active',
    ],
  })
  return () => {
    cancelAnimationFrame(frame)
    running?.stop()
    press?.stop()
    observer.disconnect()
    for (const event of events) node.removeEventListener(event, schedule)
    for (const event of pressEvents) node.removeEventListener(event, pressed)
  }
}

export function interaction(node: HTMLElement) {
  return { destroy: attachInteraction(node) }
}
export function floatingMotion(node: HTMLElement) {
  const controls = animate(
    node,
    reducedMotion() ? { opacity: 1, scale: 1 } : { opacity: [0, 1], scale: [animation.overlayScale, 1] },
    transition('enter'),
  )
  return {
    destroy() {
      controls.stop()
    },
  }
}
export function loadingMotion(node: HTMLElement) {
  const media = window.matchMedia('(prefers-reduced-motion: reduce)')
  let controls: AnimationPlaybackControls | undefined
  const update = () => {
    controls?.stop()
    node.style.transform = ''
    if (!media.matches) controls = animate(node, { rotate: [0, 360] }, animation.loading)
  }
  update()
  media.addEventListener('change', update)
  return {
    destroy() {
      controls?.stop()
      media.removeEventListener('change', update)
    },
  }
}
export function skeletonMotion(node: HTMLElement) {
  const media = window.matchMedia('(prefers-reduced-motion: reduce)')
  let controls: AnimationPlaybackControls | undefined
  const update = () => {
    controls?.stop()
    node.style.backgroundPosition = '50% 50%'
    if (!media.matches) controls = animate(node, { backgroundPosition: ['-200% 50%', '300% 50%'] }, animation.skeletonShimmer)
  }
  update()
  media.addEventListener('change', update)
  return {
    destroy() {
      controls?.stop()
      media.removeEventListener('change', update)
    },
  }
}
export function disclosure(node: HTMLDetailsElement) {
  let controls: AnimationPlaybackControls | undefined
  let previousOpen = node.open
  const toggle = (event: Event) => {
    if (event.target !== node || node.open === previousOpen) return
    previousOpen = node.open
    const body = node.querySelector<HTMLElement>('[data-disclosure-body]')
    if (!body) return
    controls?.stop()
    body.style.height = ''
    body.style.opacity = ''
    if (node.open) {
      const running = animate(body, { opacity: [0, 1], height: [0, body.scrollHeight] }, transition('disclosure'))
      controls = running
      void running.then(() => {
        if (controls !== running) return
        body.style.height = ''
        body.style.opacity = ''
        controls = undefined
      })
    }
  }
  node.addEventListener('toggle', toggle)
  return {
    destroy() {
      controls?.stop()
      node.removeEventListener('toggle', toggle)
    },
  }
}

export function sidebarDisclosure(node: HTMLDetailsElement, config: { open: boolean; onOpenChange(open: boolean): void }) {
  const body = node.querySelector<HTMLElement>('[data-disclosure-body]')!
  const summary = node.querySelector('summary')!
  const caret = node.querySelector<SVGSVGElement>('[data-disclosure-caret]')!
  let desired = config.open
  let running: AnimationPlaybackControls | undefined
  let rotation: AnimationPlaybackControls | undefined
  const present = () => {
    summary.setAttribute('aria-expanded', String(desired))
    node.dataset.state = desired ? 'open' : 'closed'
    body.inert = !desired
    body.setAttribute('aria-hidden', String(!desired))
  }
  present()
  node.open = desired
  caret.style.transform = `rotate(${desired ? 0 : animation.collapsedRotation}deg)`
  const update = (next: typeof config) => {
    config = next
    if (next.open === desired) return
    const height = node.open ? body.getBoundingClientRect().height : 0
    running?.stop()
    rotation?.stop()
    desired = next.open
    present()
    node.open = true
    body.style.overflow = 'hidden'
    const controls = animate(
      body,
      { height: [height, desired ? body.scrollHeight : 0], opacity: desired ? 1 : 0 },
      transition('disclosure'),
    )
    running = controls
    rotation = animate(caret, { rotate: desired ? 0 : animation.collapsedRotation }, transition('disclosure'))
    void controls.then(() => {
      if (running !== controls) return
      node.open = desired
      body.style.height = ''
      body.style.opacity = ''
      body.style.overflow = ''
      running = undefined
    })
  }
  const click = (event: MouseEvent) => {
    if (event.defaultPrevented) return
    event.preventDefault()
    config.onOpenChange(!desired)
  }
  summary.addEventListener('click', click)
  return {
    update,
    destroy() {
      running?.stop()
      rotation?.stop()
      summary.removeEventListener('click', click)
    },
  }
}

export function selectionMotion(
  node: HTMLInputElement,
  config: { kind: 'switch' | 'checkbox' | 'radio'; indeterminate: boolean; selected: boolean },
) {
  let initialized = false,
    previous = ''
  let controls: ReturnType<typeof animate>[] = []
  const update = () => {
    const selected = node.checked || config.indeterminate
    const state = `${selected}:${config.indeterminate}`
    if (initialized && previous === state) return
    previous = state
    controls.forEach((control) => control.stop())
    controls = []
    const timing = initialized ? transition('interaction') : { duration: 0 }
    initialized = true
    const track = node.parentElement?.querySelector<HTMLElement>('[data-selection-track]')
    const indicator = node.parentElement?.querySelector<HTMLElement>('[data-selection-indicator]')
    if (track) {
      const backgroundColor = selected ? 'var(--control)' : config.kind === 'switch' ? 'var(--disabled)' : 'transparent'
      const borderColor = selected ? 'var(--control)' : 'var(--input)'
      controls.push(animate(track, { backgroundColor, borderColor }, timing))
    }
    if (indicator)
      controls.push(
        animate(
          indicator,
          config.kind === 'switch'
            ? { x: selected ? animation.switchTravel : 0 }
            : { opacity: selected ? 1 : 0, scale: selected ? 1 : animation.indicatorScale },
          timing,
        ),
      )
  }
  let frame = 0
  const schedule = () => {
    cancelAnimationFrame(frame)
    frame = requestAnimationFrame(update)
  }
  const target = config.kind === 'radio' ? document : node
  target.addEventListener('change', schedule)
  node.form?.addEventListener('reset', schedule)
  update()
  return {
    update(next: typeof config) {
      config = next
      schedule()
    },
    destroy() {
      controls.forEach((control) => control.stop())
      cancelAnimationFrame(frame)
      target.removeEventListener('change', schedule)
      node.form?.removeEventListener('reset', schedule)
    },
  }
}

export function fieldErrorMotion(node: HTMLElement) {
  const controls = animate(
    node,
    reducedMotion()
      ? { opacity: 1 }
      : {
          opacity: [0, 1],
          clipPath: [node.dataset.side === 'left' ? 'inset(0 0 0 100%)' : 'inset(0 100% 0 0)', 'inset(0 0 0 0)'],
        },
    transition('disclosure'),
  )
  return {
    destroy() {
      controls.stop()
    },
  }
}

export function entranceMotion(node: HTMLElement, { order, subtle }: { order: number; subtle: boolean }) {
  const preset = subtle ? animation.textEntrance : animation.cascade
  const media = window.matchMedia('(prefers-reduced-motion: reduce)')
  const controls = animate(
    node,
    media.matches ? { opacity: 1, y: 0 } : { opacity: [0, 1], y: [preset.distance, 0] },
    media.matches
      ? { duration: 0 }
      : { duration: preset.duration, ease: [...preset.ease], delay: preset.delay + order * preset.stagger },
  )
  const finish = () => {
    if (media.matches) {
      controls.stop()
      animate(node, { opacity: 1, y: 0 }, { duration: 0 })
    }
  }
  media.addEventListener('change', finish)
  return {
    destroy() {
      controls.stop()
      media.removeEventListener('change', finish)
    },
  }
}

export function panelMotion(node: HTMLElement) {
  const controls = animate(
    node,
    reducedMotion() ? { opacity: 1, y: 0 } : { opacity: [0, 1], y: [animation.panelDistance, 0] },
    transition('panel'),
  )
  return {
    destroy() {
      controls.stop()
    },
  }
}
export const useTransition = transition
