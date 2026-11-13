import type { UserPreferenceController } from './controller'
import { parsePreferencePatch, type PreferenceDraft } from './values'

export function configureUserPreferenceForm(
  controller: UserPreferenceController,
  error: (error: unknown, retry: () => void) => void,
) {
  const form = controller.store.form
  let active = true
  form.configure({
    mode: 'change',
    fields: { 'draft.codeLanguage': { group: 'preview' } },
    parse: (values, names) => {
      const keys = names.includes('draft') ? Object.keys(values.draft ?? {}) : names.map((name) => name.slice('draft.'.length))
      const patch = Object.fromEntries(keys.map((key) => [key, values.draft![key as keyof PreferenceDraft]]))
      const result = parsePreferencePatch(patch)
      return result.issues.length ? { issues: result.issues } : { values }
    },
    error: (failure, retry) =>
      error(failure, () => {
        if (active) retry()
      }),
    submit: async (values, patch, signal) => {
      const confirmed = await controller.update(patch.draft ?? {}, signal)
      return { draft: { ...values.draft!, ...confirmed } }
    },
  })
  return () => {
    active = false
    form.stop()
  }
}
