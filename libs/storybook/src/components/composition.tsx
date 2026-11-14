import {
  useState,
  Children,
  Fragment,
  cloneElement,
  isValidElement,
  type HTMLAttributes,
  type ReactElement,
  type Ref,
} from 'react'
import { composeRefs, cx } from './shared'

export interface CompositionProps {
  asChild?: boolean
}

type ChildProps = HTMLAttributes<HTMLElement> & {
  ref?: Ref<HTMLElement>
  disabled?: boolean
  type?: string
  href?: string
}

export function Slot({ children, ref, ...props }: ChildProps) {
  if (Children.count(children) !== 1 || !isValidElement<ChildProps>(children) || children.type === Fragment) {
    throw new Error('asChild requires one element that forwards its props and ref.')
  }
  const child = children
  const merged = { ...props, ...child.props }
  for (const key of Object.keys(props)) {
    if (!/^on[A-Z]/.test(key)) continue
    const wrapper = (props as ChildProps)[key as keyof ChildProps]
    const handler = child.props[key as keyof ChildProps]
    if (typeof wrapper === 'function' && typeof handler === 'function') {
      Object.assign(merged, {
        [key]: (event: { defaultPrevented: boolean }, ...args: unknown[]) => {
          handler(event, ...args)
          if (!event.defaultPrevented) wrapper(event, ...args)
        },
      })
    }
  }
  merged.className = cx(props.className, child.props.className)
  merged.style = { ...props.style, ...child.props.style }
  merged.ref = composeRefs(ref, child.props.ref)
  if (
    props.disabled ||
    child.props.disabled ||
    String(props['aria-disabled']) === 'true' ||
    String(child.props['aria-disabled']) === 'true'
  ) {
    merged.disabled = true
    merged['aria-disabled'] = true
    Object.assign(merged, { 'data-disabled': '' })
    if ('href' in merged) {
      merged.href = undefined
      merged.tabIndex = -1
    }
    merged.onClick = (event) => {
      event.preventDefault()
      event.stopPropagation()
    }
    merged.onKeyDown = (event) => {
      if (event.key === 'Enter' || event.key === ' ') {
        event.preventDefault()
        event.stopPropagation()
      }
    }
  }
  return cloneElement(child as ReactElement<ChildProps>, merged)
}

export interface OpenStateProps {
  open?: boolean
  defaultOpen?: boolean
  onOpenChange?(open: boolean): void
}

export function useControllableState<T>(value: T | undefined, defaultValue: T, onChange?: (value: T) => void) {
  const [local, setLocal] = useState(defaultValue)
  const current = value === undefined ? local : value
  return [
    current,
    (next: T) => {
      if (Object.is(next, current)) return
      if (value === undefined) setLocal(next)
      onChange?.(next)
    },
  ] as const
}
