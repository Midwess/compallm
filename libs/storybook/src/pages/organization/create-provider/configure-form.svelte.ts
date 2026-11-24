import { useController } from './controller'
import { ProviderFieldsInput } from '@compallm/contract/provider'
import { ProviderPatchInput } from '@compallm/contract/validation'
import { contractResult } from '../../../components/form'
import { parseProvider } from './validation'
import { avatarInput, modelDraftRules } from './configuration'
export function useProviderForm() {
  const controller = useController()
  const form = controller.store.form
  $effect(() =>
    form.configure({
      mode: 'button',
      command: true,
      fields: {
        name: { accepts: (value) => ProviderFieldsInput.name(String(value)) },
        baseUrl: { accepts: (value) => ProviderFieldsInput.baseUrl(String(value)) },
        apiKey: { accepts: (value) => ProviderFieldsInput.apiKey(String(value)) },
        ...modelDraftRules,
      },
      changed: (field, value) => {
        if (field === 'name' || field === 'baseUrl' || field === 'apiKey') controller.onConnectionChanged(field, String(value))
        if (field === 'models' || field.startsWith('models[')) controller.onModelsChanged(controller.store.getState().models)
        if (field === 'avatarFile') {
          const file = value as File | null
          if (!file || avatarInput(file).success) controller.onAvatarChanged?.(file)
          form.blur(field)
        }
        if (field === 'headers' || field.startsWith('headers.'))
          controller.onHeadersChanged?.(controller.store.getState().headers)
        if (field === 'modelDraft.path') controller.onModelDraftChanged({ path: '/' + String(value).replace(/^\/+/, '') })
        else if (field.startsWith('modelDraft.'))
          controller.onModelDraftChanged({
            ...(controller.store.form.get('modelDraft') as import('./types').ModelDraft),
            ...(field === 'modelDraft.effortMapping'
              ? { thinkingLevels: Object.keys(value as object) as import('./types').ThinkingLevel[] }
              : {}),
          })
      },
      parse: (values, names) => {
        if (controller.store.getState().templatePreview && !controller.store.getState().creationEnabled) return { values }
        if (controller.store.getState().creationEnabled) return parseProvider({ ...controller.store.getState(), ...values })
        if (values.avatarFile) {
          const image = avatarInput(values.avatarFile)
          if (!image.success) return { issues: image.error.issues.map((issue) => ({ ...issue, path: ['avatarFile'] })) }
        }
        if (names.length === 1 && names[0] === 'avatarFile') return { values }
        if (names.length === 1 && ['name', 'baseUrl', 'apiKey'].includes(names[0])) {
          const name = names[0] as 'name' | 'baseUrl' | 'apiKey'
          const patch =
            name === 'apiKey' && !values.apiKey && controller.store.getState().intent === 'edit' ? {} : { [name]: values[name] }
          if (!Object.keys(patch).length) return { values }
          const result = ProviderPatchInput.safeParse(patch)
          return contractResult(result, result.success ? { ...values, [name]: result.data[name] } : values)
        }
        const state = { ...controller.store.getState(), ...values }
        const result = parseProvider(state)
        return result.issues ? result : { values: { ...values, ...result.values } }
      },
      submit: async (values) => {
        const result =
          controller.store.getState().intent === 'edit'
            ? await controller.onSaveProvider(values)
            : await controller.onCreateProvider(values)
        return result && typeof result === 'object' ? result : false
      },
    }),
  )
}
