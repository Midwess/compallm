import { SvelteMap } from 'svelte/reactivity'
import get from 'lodash-es/get.js'
import set from 'lodash-es/set.js'
import cloneDeepWith from 'lodash-es/cloneDeepWith.js'

type FieldError = { message?: string; type?: string; [key: string]: unknown }
export type Resolver<T extends object> = (
  values: T,
  context: undefined,
  options: { fields: object; shouldUseNativeValidation: boolean },
) => Promise<{ values?: T; errors: Record<string, FieldError> }> | { values?: T; errors: Record<string, FieldError> }
export class NativeFormControl {
  private values = $state<object>({})
  private errors = new SvelteMap<string, FieldError>()
  private nodes = new Map<string, Set<HTMLInputElement>>()
  constructor(values: object) {
    this.values = this.clone(values)
  }
  private clone<T>(value: T): T {
    return cloneDeepWith(value, (item) => (typeof Blob !== 'undefined' && item instanceof Blob ? item : undefined))
  }
  private emit(path: string) {
    for (const [key, nodes] of this.nodes)
      if (key === path || key.startsWith(path + '.') || !path) for (const node of nodes) this.writeNode(node, key)
  }
  private writeNode(node: HTMLInputElement, name: string) {
    const value = get(this.values, name)
    if (node.type === 'checkbox' || node.type === 'radio')
      node.checked = Array.isArray(value)
        ? value.includes(node.value)
        : node.type === 'radio'
          ? value === node.value
          : Boolean(value)
    else if (node.type !== 'file') node.value = String(value ?? '')
  }
  getValues(name?: string): unknown {
    return name ? get(this.values, name) : this.values
  }
  setValue(name: string, value: unknown, _options?: object) {
    set(this.values, name, value)
    this.emit(name)
  }
  reset(values: object, _options?: object) {
    this.values = this.clone(values)
    this.errors.clear()
    this.emit('')
  }
  getFieldState(name: string) {
    return { error: this.errors.get(name) }
  }
  setError(name: string, error: FieldError) {
    this.errors.set(name, error)
    this.emit(name)
  }
  clearErrors(name?: string) {
    for (const key of this.errors.keys()) if (!name || key === name || key.startsWith(name + '.')) this.errors.delete(key)
    this.emit(name ?? '')
  }
  register(name: string) {
    let current: HTMLInputElement | null = null
    return {
      name,
      ref: (node: HTMLInputElement | null) => {
        if (current) this.nodes.get(name)?.delete(current)
        current = node
        if (node) {
          const nodes = this.nodes.get(name) ?? new Set<HTMLInputElement>()
          this.nodes.set(name, nodes)
          nodes.add(node)
          this.writeNode(node, name)
        }
      },
    }
  }
}
export const createFormControl = (options: { defaultValues: object; mode?: string; reValidateMode?: string }) =>
  new NativeFormControl(options.defaultValues)
