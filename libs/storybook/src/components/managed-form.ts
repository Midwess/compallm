import { onMount } from 'svelte'
import { getCollectionRow } from '../context'
import { optionalContext } from './form-context.svelte'
import { createForm, type FormOptions } from './form-model.svelte'
export function managedForm<T extends object>(values: T, options: FormOptions<T> = {}, formKey = 'form') {
  const row = optionalContext(getCollectionRow)
  if (!row) return createForm(values, options)
  const key = `${row.key}:${formKey}`
  const model = row.forms.get(key, values, options)
  onMount(() => row.forms.attach(key, model))
  return model
}
