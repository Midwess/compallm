import { getFormContext } from '../context'
import type { FormModel } from './form-model.svelte'
export function currentForm(): FormModel<object> | null {
  try {
    return getFormContext()
  } catch {
    return null
  }
}
export function fieldModel(name?: string | null | (() => string | null | undefined)) {
  const form = currentForm()
  const key = typeof name === 'function' ? name() : name
  return form && key && form.has(key) ? form : null
}
export function optionalContext<T>(read: () => T): T | undefined {
  try {
    return read()
  } catch {
    return undefined
  }
}
