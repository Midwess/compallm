import { MediaQuery } from 'svelte/reactivity'
import type { StateStore } from '../state.svelte'
import type { FormScope } from '../components/form-model.svelte'
import { fieldModel } from '../components/form-context.svelte'
const light = new MediaQuery('(prefers-color-scheme: light)', false)
export function systemLight() {
  return light.current
}
export function readStore<T, R = T>(
  store: Pick<StateStore<T>, 'getState'>,
  selector?: (state: T) => R,
  _names?: readonly string[],
): R {
  return selector ? selector(store.getState()) : (store.getState() as unknown as R)
}
export function readForm<T>(form: FormScope, name: string): T {
  return form.get(name) as T
}
export function fieldState<T = unknown>(name?: string | (() => string)) {
  const form = fieldModel(name)
  const path = () => (typeof name === 'function' ? name() : name!)
  return form && name
    ? {
        get value() {
          return form.get(path()) as T
        },
        get invalid() {
          return form.invalid(path())
        },
        change(value: T) {
          form.change(path(), value)
        },
        blur() {
          form.blur(path())
        },
        enter() {
          form.enter(path())
        },
        escape() {
          form.escape(path())
        },
      }
    : null
}
export function registerElement(node: HTMLElement, register: (node: HTMLElement | null) => void) {
  register(node)
  return {
    destroy() {
      register(null)
    },
  }
}
export function toStyle(value?: string | object): string {
  if (!value) return ''
  if (typeof value === 'string') return value
  const unitless = ['opacity', 'zIndex', 'fontWeight', 'lineHeight', 'flexGrow', 'flexShrink', 'order', 'scale', 'aspectRatio']
  return Object.entries(value)
    .filter(([, value]) => value !== undefined && value !== null)
    .map(
      ([key, value]) =>
        `${key.startsWith('--') ? key : key.replace(/[A-Z]/g, (match) => '-' + match.toLowerCase())}:${typeof value === 'number' && value !== 0 && !unitless.includes(key) && !key.startsWith('--') ? value + 'px' : value}`,
    )
    .join(';')
}
