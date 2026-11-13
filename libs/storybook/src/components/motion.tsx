import {
  createElement,
  useCallback,
  useLayoutEffect,
  useRef,
  type RefObject,
  type ComponentPropsWithRef,
  type Ref,
} from 'react'
import { animate, type Transition } from 'motion'
import { motion, useReducedMotion, type HTMLMotionProps } from 'motion/react'

export const animation = {
  interaction: { duration: 0.12, ease: 'easeOut' },
  enter: { duration: 0.15, ease: 'easeOut' },
  exit: { duration: 0.1, ease: 'easeIn' },
  disclosure: { duration: 0.18, ease: 'easeOut' },
  reorder: { duration: 0.18, ease: 'easeOut' },
  panel: { duration: 0.2, ease: 'easeOut' },
  loading: { duration: 1, ease: 'linear', repeat: Infinity },
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

export function transition(
  preset: keyof Pick<typeof animation, 'interaction' | 'enter' | 'exit' | 'disclosure' | 'reorder' | 'panel' | 'loading'>,
): Transition {
  return reducedMotion() ? { duration: 0 } : animation[preset]
}

export function useTransition(preset: Parameters<typeof transition>[0]) {
  const reduced = useReducedMotion()
  return reduced ? { duration: 0 } : animation[preset]
}

export function LoadingMotion({ children }: { children: React.ReactNode }) {
  const reduced = useReducedMotion()
  return (
    <motion.span
      className="inline-flex"
      animate={{ rotate: reduced ? 0 : 360 }}
      transition={reduced ? { duration: 0 } : animation.loading}
    >
      {children}
    </motion.span>
  )
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

function setRef<T>(ref: Ref<T> | undefined, value: T | null) {
  if (typeof ref === 'function') return ref(value)
  if (ref) ref.current = value
}

function animatedElement<Tag extends 'button' | 'a' | 'input' | 'textarea' | 'label' | 'div'>(tag: Tag) {
  return function AnimatedElement({ ref, ...props }: ComponentPropsWithRef<Tag>) {
    const animatedRef = useCallback(
      (node: HTMLElement | null) => {
        const cleanupRef = setRef(ref as Ref<HTMLElement> | undefined, node)
        if (!node) return
        const cleanup = attachInteraction(node)
        return () => {
          cleanup()
          if (typeof cleanupRef === 'function') cleanupRef()
          else setRef(ref as Ref<HTMLElement> | undefined, null)
        }
      },
      [ref],
    )
    return createElement(tag, { ...props, ref: animatedRef })
  }
}

export const animated = {
  button: animatedElement('button'),
  a: animatedElement('a'),
  input: animatedElement('input'),
  textarea: animatedElement('textarea'),
  label: animatedElement('label'),
  div: animatedElement('div'),
}

function overlayElement<Tag extends 'div' | 'span' | 'dialog'>(tag: Tag) {
  return function Overlay({ ref, ...props }: ComponentPropsWithRef<Tag>) {
    const animatedRef = useCallback(
      (node: HTMLElement | null) => {
        const cleanupRef = setRef(ref as Ref<HTMLElement> | undefined, node)
        if (!node) return
        let running: ReturnType<typeof animate> | undefined
        let ghost: HTMLElement | undefined
        let exiting = false
        const clear = () => {
          running?.stop()
          ghost?.remove()
          ghost = undefined
        }
        const exit = () => {
          if (exiting || reducedMotion() || !node.getClientRects().length) return
          exiting = true
          const bounds = node.getBoundingClientRect()
          const image = node.cloneNode(true) as HTMLElement
          image.removeAttribute('id')
          image.removeAttribute('open')
          image.setAttribute('popover', 'manual')
          image.setAttribute('aria-hidden', 'true')
          image.inert = true
          image.querySelectorAll('[id]').forEach((child) => child.removeAttribute('id'))
          Object.assign(image.style, {
            position: 'fixed',
            inset: 'auto',
            left: `${bounds.left}px`,
            top: `${bounds.top}px`,
            width: `${bounds.width}px`,
            height: `${bounds.height}px`,
            margin: '0',
            pointerEvents: 'none',
            display: getComputedStyle(node).display,
            transform: 'none',
            opacity: '1',
          })
          ;(node.closest('dialog')?.parentElement ?? document.body).appendChild(image)
          image.showPopover()
          ghost = image
          running?.stop()
          running = animate(image, { opacity: 0, scale: animation.overlayScale }, animation.exit)
          void running.then(() => image.remove())
        }
        const enter = () => {
          clear()
          exiting = false
          if (reducedMotion()) return
          running = animate(node, { opacity: [0, 1], scale: [animation.overlayScale, 1] }, animation.enter)
        }
        const toggle = (event: Event) => {
          if (event.target !== node) return
          if ((event as ToggleEvent).newState === 'open') enter()
          else exit()
        }
        node.addEventListener('beforetoggle', toggle)
        if (!node.hasAttribute('popover') && tag !== 'dialog') enter()
        return () => {
          exit()
          node.removeEventListener('beforetoggle', toggle)
          if (typeof cleanupRef === 'function') cleanupRef()
          else setRef(ref as Ref<HTMLElement> | undefined, null)
        }
      },
      [ref],
    )
    return createElement(tag, { ...props, ref: animatedRef })
  }
}

export const overlay = { div: overlayElement('div'), span: overlayElement('span'), dialog: overlayElement('dialog') }

export function FloatingMotion({ ref, ...props }: HTMLMotionProps<'div'>) {
  const timing = useTransition('enter')
  const reduced = useReducedMotion()
  return (
    <motion.div
      {...props}
      ref={ref}
      initial={reduced ? false : { opacity: 0, scale: animation.overlayScale }}
      animate={{ opacity: 1, scale: 1 }}
      transition={timing}
    />
  )
}

export function useSelectionMotion(
  inputRef: RefObject<HTMLInputElement | null>,
  kind: 'switch' | 'checkbox' | 'radio',
  indeterminate = false,
) {
  const initialized = useRef(false)
  const previous = useRef('')
  const controls = useRef<ReturnType<typeof animate>[]>([])
  const update = useCallback(() => {
    const node = inputRef.current
    if (!node) return
    const selected = node.checked || indeterminate
    const state = `${selected}:${indeterminate}`
    if (initialized.current && previous.current === state) return
    previous.current = state
    for (const control of controls.current) control.stop()
    const timing = initialized.current ? transition('interaction') : { duration: 0 }
    initialized.current = true
    const track = node.parentElement?.querySelector<HTMLElement>('[data-selection-track]')
    const indicator = node.parentElement?.querySelector<HTMLElement>('[data-selection-indicator]')
    const colors = getComputedStyle(node)
    const primary = colors.getPropertyValue('--color-primary').trim()
    const disabled = colors.getPropertyValue('--color-disabled').trim()
    const border = colors.getPropertyValue('--color-input-border').trim()
    controls.current = []
    if (track)
      controls.current.push(
        animate(
          track,
          {
            backgroundColor: selected ? primary : kind === 'switch' ? disabled : 'rgba(0,0,0,0)',
            borderColor: selected ? primary : border,
          },
          timing,
        ),
      )
    if (indicator)
      controls.current.push(
        animate(
          indicator,
          kind === 'switch'
            ? { x: selected ? animation.switchTravel : 0 }
            : { opacity: selected ? 1 : 0, scale: selected ? 1 : animation.indicatorScale },
          timing,
        ),
      )
  }, [inputRef, kind, indeterminate])
  useLayoutEffect(update)
  useLayoutEffect(() => {
    const node = inputRef.current
    if (!node) return
    let frame = 0
    const reset = () => {
      frame = requestAnimationFrame(update)
    }
    const change = (event: Event) => {
      if (event.target === node || kind === 'radio') update()
    }
    const changeTarget = kind === 'radio' ? document : node
    changeTarget.addEventListener('change', change)
    node.form?.addEventListener('reset', reset)
    return () => {
      cancelAnimationFrame(frame)
      changeTarget.removeEventListener('change', change)
      node.form?.removeEventListener('reset', reset)
      for (const control of controls.current) control.stop()
      controls.current = []
      initialized.current = false
      previous.current = ''
    }
  }, [inputRef, kind, update])
}

export function useDisclosure(defaultOpen: boolean) {
  const details = useRef<HTMLDetailsElement>(null)
  const content = useRef<HTMLDivElement>(null)
  const caret = useRef<SVGSVGElement>(null)
  const desired = useRef(defaultOpen)
  const running = useRef<ReturnType<typeof animate>[]>([])
  const change = useCallback((open: boolean) => {
    const root = details.current,
      body = content.current,
      icon = caret.current
    if (!root || !body || !icon) return
    desired.current = open
    root.querySelector('summary')?.setAttribute('aria-expanded', String(open))
    const height = root.open ? body.getBoundingClientRect().height : 0
    for (const control of running.current) control.stop()
    root.open = true
    body.inert = !open
    body.setAttribute('aria-hidden', String(!open))
    body.style.overflow = 'hidden'
    const controls = animate(
      body,
      { height: [height, open ? body.scrollHeight : 0], opacity: open ? 1 : 0 },
      transition('disclosure'),
    )
    const rotation = animate(icon, { rotate: open ? 0 : animation.collapsedRotation }, transition('disclosure'))
    running.current = [controls, rotation]
    void controls.then(() => {
      if (running.current[0] !== controls) return
      root.open = open
      body.style.height = open ? 'auto' : '0px'
      body.style.overflow = open ? '' : 'hidden'
    })
  }, [])
  useLayoutEffect(() => {
    const root = details.current,
      body = content.current,
      icon = caret.current
    if (!root || !body || !icon) return
    root.open = desired.current
    body.inert = !desired.current
    body.setAttribute('aria-hidden', String(!desired.current))
    icon.style.transform = `rotate(${desired.current ? 0 : animation.collapsedRotation}deg)`
    return () => {
      for (const control of running.current) control.stop()
    }
  }, [])
  return {
    detailsRef: details,
    contentRef: content,
    caretRef: caret,
    toggle: () => change(!desired.current),
    expand: () => change(true),
  }
}

export function FieldErrorMotion({ children, ...props }: HTMLMotionProps<'div'> & { 'data-side'?: string }) {
  const timing = useTransition('disclosure')
  return (
    <motion.div
      {...props}
      initial={{ opacity: 0, clipPath: props['data-side'] === 'left' ? 'inset(0 0 0 100%)' : 'inset(0 100% 0 0)' }}
      animate={{ opacity: 1, clipPath: 'inset(0 0 0 0)' }}
      transition={timing}
    >
      {children}
    </motion.div>
  )
}
