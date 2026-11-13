import { ProviderPatchInput } from '@compallm/contract/validation'
import { contractResult } from '../../../components/form'
import { FormError } from '../../../components/form'
import { Form } from '../../../components/form'
import {
  ProviderFieldsInput,
  ProviderFieldsConstraints,
  ProviderHeaderConstraints,
  ProviderModelFieldsInput,
} from '@compallm/contract/provider'
import { useEffect, useState } from 'react'
import { EyeIcon } from '@phosphor-icons/react/dist/csr/Eye'
import { EyeSlashIcon } from '@phosphor-icons/react/dist/csr/EyeSlash'
import { FormSection } from '../../../components/form-section'
import { SectionSurface } from '../../../components/surfaces'
import { FormField } from '../../../components/form-field'
import { CheckIcon } from '@phosphor-icons/react/dist/csr/Check'
import { XCircleIcon } from '@phosphor-icons/react/dist/csr/XCircle'
import { LinkSimpleIcon } from '@phosphor-icons/react/dist/csr/LinkSimple'
import { Button } from '../../../components/buttons'
import { KeyValueEditor } from '../../../components/key-value-editor'
import { ProviderAvatar } from '../../../components/provider-avatar'
import { FieldHelp, Tooltip, Toast } from '../../../components/feedback'

import { useController, useProvider } from './controller'
import { ProviderModels } from './model-table'
import { parseProvider } from './validation'
import { acceptsHeaderInput } from './validation'
function ConnectionFields() {
  const [keyVisible, setKeyVisible] = useState(false)
  const controller = useController()
  const editing = useProvider((state) => state.intent === 'edit')
  const busy = useProvider((state) => state.submitting)
  const selected = useProvider((state) => state.selectedProviderId)
  const avatarUrl = useProvider((state) => state.avatarUrl, ['avatarUrl'])
  return (
    <SectionSurface className="grid min-w-0 gap-4">
      <fieldset disabled={busy} className="grid min-w-0 gap-4">
        <div className="flex min-w-0 items-end gap-3">
          <FormField className="flex-1">
            <FormField.Label>Connection name</FormField.Label>
            <FormField.Input name="name" required placeholder="Enter connection name" />
            <FormError name="name" />
          </FormField>
          <ProviderAvatar name="avatarFile" provider={selected} src={avatarUrl} disabled={busy} />
          <FormError name="avatarFile" />
        </div>
        <FormField>
          <FormField.Label>Base URL</FormField.Label>
          <FormField.Input
            name="baseUrl"
            type="url"
            required
            autoCapitalize="none"
            spellCheck={false}
            placeholder="https://api.example.com/v1"
          />
          <FormError name="baseUrl" />
        </FormField>
        <FormField>
          <div className="flex items-center gap-1">
            <FormField.Label>API key</FormField.Label>
            <FieldHelp label="About API keys">
              An API key is a secret credential from your provider that lets Compallm send requests to its models on your
              behalf. Create one in your provider’s dashboard and keep it private. Up to{' '}
              {ProviderFieldsConstraints.apiKey.maxLength} characters, without whitespace. When editing, leave this field empty
              to keep the current key.
            </FieldHelp>
          </div>
          <div className="relative min-w-0">
            <FormField.Input
              type={keyVisible ? 'text' : 'password'}
              className="pr-9"
              autoComplete="off"
              required={!editing}
              autoCapitalize="none"
              spellCheck={false}
              name="apiKey"
              placeholder={editing ? 'Leave empty to keep current key' : 'Enter your API key'}
            />
            <FormError name="apiKey" />
            <button
              type="button"
              aria-label={keyVisible ? 'Hide API key' : 'Show API key'}
              aria-pressed={keyVisible}
              className={`
absolute inset-y-0 right-0 flex w-8 items-center justify-center rounded-lg text-muted-foreground
hover:text-foreground focus-visible:outline-muted-foreground
`}
              onClick={() => setKeyVisible((visible) => !visible)}
            >
              {keyVisible ? (
                <EyeSlashIcon aria-hidden="true" className="size-4" />
              ) : (
                <EyeIcon aria-hidden="true" className="size-4" />
              )}
            </button>
          </div>
        </FormField>
      </fieldset>
      <KeyValueEditor
        name="headers"
        acceptEntry={acceptsHeaderInput}
        keyMaxLength={ProviderHeaderConstraints.key.maxLength}
        disabled={busy || !controller.onHeadersChanged}
      >
        <KeyValueEditor.Disclosure>Headers</KeyValueEditor.Disclosure>
      </KeyValueEditor>
      <FormError name="headers" />
      <FormError name="models" />
    </SectionSurface>
  )
}
function TestConnection() {
  const controller = useController()
  const result = useProvider((state) => state.test)
  const endpoint = useProvider((state) => state.endpointUrl)
  const busy = useProvider((state) => state.submitting)
  return (
    <SectionSurface className="grid gap-2">
      <p className="flex min-w-0 items-center gap-2 text-xs text-ring">
        <LinkSimpleIcon aria-hidden="true" className="size-3.5 shrink-0" />
        <span className="truncate">{endpoint || 'Configure the connection endpoint'}</span>
      </p>
      <div className="flex flex-wrap items-center gap-3">
        <Button
          className="h-7 px-3 text-xs"
          disabled={busy}
          loading={result.status === 'testing'}
          onClick={() => void controller.onTestConnection()}
        >
          Test connection
        </Button>
        <span role="status" aria-live="polite">
          {result.status === 'passed' ? (
            <span className="flex items-center gap-1.5 text-xs text-success">
              <CheckIcon aria-hidden="true" className="size-3.5" />
              Check passed
            </span>
          ) : result.status === 'failed' ? (
            <Tooltip content={result.message}>
              <button
                type="button"
                className="flex cursor-help items-center gap-1.5 text-xs text-destructive focus-visible:outline-ring"
              >
                <XCircleIcon aria-hidden="true" className="size-3.5" />
                Connection failed
              </button>
            </Tooltip>
          ) : null}
        </span>
      </div>
    </SectionSurface>
  )
}
export function ProviderConfiguration() {
  const controller = useController()
  const intent = useProvider((state) => state.intent)
  const busy = useProvider((state) => state.submitting)
  const error = useProvider((state) => state.error)
  const form = controller.store.form
  useEffect(
    () =>
      form.configure({
        mode: 'button',
        command: true,
        fields: {
          name: { accepts: (value) => ProviderFieldsInput.name(String(value)) },
          baseUrl: { accepts: (value) => ProviderFieldsInput.baseUrl(String(value)) },
          apiKey: { accepts: (value) => ProviderFieldsInput.apiKey(String(value)) },
          'modelDraft.path': { accepts: (value) => ProviderModelFieldsInput.path('/' + String(value).replace(/^\/+/, '')) },
          'modelDraft.name': { accepts: (value) => ProviderModelFieldsInput.name(String(value)) },
          'modelDraft.maxContextTokens': { accepts: (value) => ProviderModelFieldsInput.maxContextTokens(String(value)) },
          'modelDraft.inputPrice': { accepts: (value) => ProviderModelFieldsInput.inputPrice(String(value)) },
          'modelDraft.outputPrice': { accepts: (value) => ProviderModelFieldsInput.inputPrice(String(value)) },
          'modelDraft.cachedPrice': { accepts: (value) => ProviderModelFieldsInput.inputPrice(String(value)) },
        },
        changed: (field, value) => {
          if (field === 'name' || field === 'baseUrl' || field === 'apiKey')
            controller.onConnectionChanged(field, String(value))
          if (field === 'models' || field.startsWith('models[')) controller.onModelsChanged(controller.store.getState().models)
          if (field === 'avatarFile') controller.onAvatarChanged?.(value as File | null)
          if (field === 'headers' || field.startsWith('headers.'))
            controller.onHeadersChanged?.(controller.store.getState().headers)
          if (field === 'modelDraft.path') controller.onModelDraftChanged({ path: '/' + String(value).replace(/^\/+/, '') })
          else if (field.startsWith('modelDraft.'))
            controller.onModelDraftChanged({
              [field.slice(11)]: value,
              ...(field === 'modelDraft.effortMapping'
                ? { thinkingLevels: Object.keys(value as object) as import('./types').ThinkingLevel[] }
                : {}),
            })
        },
        parse: (values, names) => {
          if (names.length === 1 && ['name', 'baseUrl', 'apiKey'].includes(names[0])) {
            const name = names[0] as 'name' | 'baseUrl' | 'apiKey'
            const patch =
              name === 'apiKey' && !values.apiKey && controller.store.getState().intent === 'edit'
                ? {}
                : { [name]: values[name] }
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
    [controller, form],
  )
  return (
    <Form model={form} className="mx-auto grid min-w-0 w-full max-w-4xl gap-6 px-4 py-8">
      <FormSection>
        <FormSection.Marker>1</FormSection.Marker>
        <FormSection.Heading>Connection</FormSection.Heading>
        <ConnectionFields />
      </FormSection>
      <FormSection>
        <FormSection.Marker>2</FormSection.Marker>
        <FormSection.Heading>Enabled models</FormSection.Heading>
        <ProviderModels />
      </FormSection>
      <FormSection>
        <FormSection.Marker>3</FormSection.Marker>
        <FormSection.Heading>Test connection</FormSection.Heading>
        <TestConnection />
      </FormSection>
      {error ? (
        <div className="fixed right-6 bottom-6 z-50">
          <Toast tone="error" message={error} onDismiss={() => controller.store.setState({ error: null })} />
        </div>
      ) : null}
      <div className="flex justify-end gap-2">
        {intent !== 'create' ? (
          <Button className="h-7 px-4 text-xs" disabled={busy} onClick={controller.onCancelProvider}>
            Cancel
          </Button>
        ) : null}
        <Button type="submit" variant="primary" className="h-7 w-32 text-xs" disabled={busy} loading={busy}>
          {intent === 'edit' ? 'Save changes' : intent === 'clone' ? 'Create provider' : 'Add provider'}
        </Button>
      </div>
    </Form>
  )
}
