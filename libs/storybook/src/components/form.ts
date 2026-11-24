import Reset from './FormReset.svelte'
import Root from './FormRoot.svelte'
import Scope from './FormScope.svelte'
import Error from './FormError.svelte'
import Submit from './FormSubmit.svelte'
import Status from './FormStatus.svelte'
export const Form: {
  Root: typeof Root
  Scope: typeof Scope
  Error: typeof Error
  Submit: typeof Submit
  Status: typeof Status
  Reset: typeof Reset
} = { Root, Scope, Error, Submit, Status, Reset }
export * from './form-model.svelte'
export { currentForm, fieldModel } from './form-context.svelte'
export { managedForm } from './managed-form'

export function contractResult<T>(
  result: { success: boolean; data?: unknown; error?: { issues: { path: PropertyKey[]; message: string }[] } },
  values: T,
  prefix = '',
) {
  return result.success
    ? { values }
    : {
        issues: result.error!.issues.map((issue) => ({
          ...issue,
          path: [...(prefix ? prefix.split('.') : []), ...issue.path],
        })),
      }
}

export { default as FormError } from './FormError.svelte'
export function zodResolver<T extends object>(schema: {
  safeParseAsync(
    values: unknown,
  ): Promise<{ success: boolean; data?: unknown; error?: { issues: { path: PropertyKey[]; message: string }[] } }>
}): import('./native-form.svelte').Resolver<T> {
  return async (values) => {
    const result = await schema.safeParseAsync(values)
    return result.success
      ? { values: result.data as T, errors: {} }
      : { errors: Object.fromEntries(result.error!.issues.map((issue) => [issue.path.join('.'), { message: issue.message }])) }
  }
}
