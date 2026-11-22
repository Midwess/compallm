import { LoadingRegion } from '../../../components/loading'
import { providerPageLayout } from './layout'
import { CatalogStatus } from './catalog-status'
import { ImageUploadInput } from '@compallm/contract/validation'
import { DisabledTooltip } from '../../../components/feedback'
import { PermissionRequest, usePermission } from '../../../components/permission'
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
export const modelDraftRules = {
  'modelDraft.path': { accepts: (value: unknown) => ProviderModelFieldsInput.path('/' + String(value).replace(/^\/+/, '')) },
  'modelDraft.name': { accepts: (value: unknown) => ProviderModelFieldsInput.name(String(value)) },
  'modelDraft.maxContextTokens': { accepts: (value: unknown) => ProviderModelFieldsInput.maxContextTokens(String(value)) },
  'modelDraft.maxInputTokens': { accepts: (value: unknown) => ProviderModelFieldsInput.maxContextTokens(String(value)) },
  'modelDraft.maxOutputTokens': { accepts: (value: unknown) => ProviderModelFieldsInput.maxContextTokens(String(value)) },
  'modelDraft.inputPrice': { accepts: (value: unknown) => ProviderModelFieldsInput.inputPrice(String(value)) },
  'modelDraft.outputPrice': { accepts: (value: unknown) => ProviderModelFieldsInput.inputPrice(String(value)) },
  'modelDraft.cachedPrice': { accepts: (value: unknown) => ProviderModelFieldsInput.inputPrice(String(value)) },
}
function ConnectionFields() {
  const [keyVisible, setKeyVisible] = useState(false)
  const controller = useController()
  const editing = useProvider((state) => state.intent === 'edit')
  const preview = useProvider((state) => state.templatePreview)
  const submitting = useProvider((state) => state.submitting)
  const providerId = useProvider((state) => state.savedProvider?.id)
  const edit = usePermission({
    actions: ['provider.configuration.update'],
    resource: { type: 'Provider', id: providerId },
    enabled: editing && !!providerId,
  })
  const loading = useProvider((s) => s.providerLoading && !s.providerLoaded)
  const readOnly = useProvider((state) => state.readOnly)
  const busy = !!readOnly || submitting || !!loading || (editing && !edit.allowed)
  const selected = useProvider((state) => state.selectedProviderId)
  const avatarUrl = useProvider((state) => state.avatarUrl, ['avatarUrl'])
  return (
    <SectionSurface className="grid min-w-0 gap-4">
      <fieldset disabled={busy} className="grid min-w-0 gap-4">
        <div className="flex min-w-0 items-end gap-3">
          <FormField.Root className="flex-1">
            <FormField.Label>Connection name</FormField.Label>
            <FormField.Input name="name" required placeholder="Enter connection name" />
            <FormError name="name" />
          </FormField.Root>
          <ProviderAvatar name="avatarFile" provider={selected} src={avatarUrl} disabled={busy} />
          <FormError name="avatarFile" />
        </div>
        <FormField.Root>
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
        </FormField.Root>
        {!readOnly ? (
          <FormField.Root>
            <div className="flex items-center gap-1">
              <FormField.Label>API key</FormField.Label>
              <FieldHelp label="About API keys">
                An API key is a secret credential from your provider that lets Compallm send requests to its models on your
                behalf. Create one in your provider’s dashboard and keep it private. Up to{' '}
                {ProviderFieldsConstraints.apiKey.maxLength} characters, without whitespace. When editing, leave this field
                empty to keep the current key.
              </FieldHelp>
            </div>
            <div className="relative min-w-0">
              <PermissionRequest
                actions={[editing ? 'provider.credential.replace' : 'provider.create']}
                resource={editing ? { type: 'Provider', id: providerId } : undefined}
                enabled={!editing || !!providerId}
              >
                {(allowed) => (
                  <DisabledTooltip disabled={!allowed && !preview} reason="You do not have permission to perform this action.">
                    <FormField.Input
                      disabled={!allowed && !preview}
                      type={keyVisible ? 'text' : 'password'}
                      className="pr-9"
                      autoComplete="off"
                      required={!editing}
                      autoCapitalize="none"
                      spellCheck={false}
                      name="apiKey"
                      placeholder={editing ? 'Leave empty to keep current key' : 'Enter your API key'}
                    />
                  </DisabledTooltip>
                )}
              </PermissionRequest>
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
          </FormField.Root>
        ) : null}
      </fieldset>
      <KeyValueEditor.Root
        name="headers"
        acceptEntry={acceptsHeaderInput}
        keyMaxLength={ProviderHeaderConstraints.key.maxLength}
        disabled={busy || !controller.onHeadersChanged}
      >
        <KeyValueEditor.Disclosure>Headers</KeyValueEditor.Disclosure>
      </KeyValueEditor.Root>
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
            <Tooltip.Control content={result.message}>
              <button
                type="button"
                className="flex cursor-help items-center gap-1.5 text-xs text-destructive focus-visible:outline-ring"
              >
                <XCircleIcon aria-hidden="true" className="size-3.5" />
                Connection failed
              </button>
            </Tooltip.Control>
          ) : null}
        </span>
      </div>
    </SectionSurface>
  )
}
export function ProviderConfiguration() {
  const controller = useController()
  const intent = useProvider((state) => state.intent)
  const readOnly = useProvider((state) => state.readOnly)
  const create = usePermission({
    actions: ['provider.create', 'provider.read'],
    enabled: !!controller.store.getState().creationEnabled,
  })
  const submitting = useProvider((state) => state.submitting)
  const providerId = useProvider((state) => state.savedProvider?.id)
  const edit = usePermission({
    actions: ['provider.configuration.update'],
    resource: { type: 'Provider', id: providerId },
    enabled: intent === 'edit' && !!providerId,
  })
  const loading = useProvider((s) => s.providerLoading && !s.providerLoaded)
  const busy = !!readOnly || submitting || !!loading || (intent === 'edit' && !edit.allowed)
  const error = useProvider((state) => state.error)
  const form = controller.store.form
  const templatePreview = useProvider((state) => state.templatePreview)
  const creationEnabled = useProvider((state) => state.creationEnabled)
  const ready = useProvider((state) => !state.templatePreview || !!state.providerLoaded)
  return (
    <Form.Root model={form} className={`${providerPageLayout} gap-6`}>
      <div className="grid gap-1 px-4">
        <h2 className="text-2xl font-medium">Configure</h2>
        {creationEnabled ? <p className="text-xsm text-muted-foreground">Create uses saved model configurations.</p> : null}
      </div>
      {templatePreview ? <CatalogStatus /> : null}
      {intent === 'edit' && !edit.allowed && !readOnly ? (
        <p className="text-xs text-muted-foreground">You do not have permission to edit this provider configuration.</p>
      ) : null}
      <LoadingRegion
        className="contents"
        ready={ready}
        pending={loading}
        controller={controller.loading?.provider}
        label="Loading provider configuration"
      >
        <fieldset disabled={busy} className="contents">
          <FormSection.Root>
            <FormSection.Marker>1</FormSection.Marker>
            <FormSection.Heading>Connection</FormSection.Heading>
            <ConnectionFields />
          </FormSection.Root>
          <FormSection.Root>
            <FormSection.Marker>2</FormSection.Marker>
            <FormSection.Heading>{templatePreview ? 'Available models' : 'Enabled models'}</FormSection.Heading>
            <ProviderModels />
          </FormSection.Root>
          {!templatePreview && !readOnly ? (
            <FormSection.Root>
              <FormSection.Marker>3</FormSection.Marker>
              <FormSection.Heading>Test connection</FormSection.Heading>
              <TestConnection />
            </FormSection.Root>
          ) : null}
          {error ? (
            <div className="fixed right-6 bottom-6 z-50">
              <Toast tone="error" message={error} onDismiss={() => controller.store.setState({ error: null })} />
            </div>
          ) : null}
          {!readOnly && (!templatePreview || creationEnabled) ? (
            <div className="flex justify-end gap-2">
              {intent !== 'create' || creationEnabled ? (
                <Button className="h-7 px-4 text-xs" disabled={busy} onClick={controller.onCancelProvider}>
                  Cancel
                </Button>
              ) : null}
              <DisabledTooltip
                disabled={!!creationEnabled && !create.allowed}
                reason="You need permission to create and view providers."
              >
                <Button
                  type="submit"
                  variant="primary"
                  className="h-7 px-4 text-xs"
                  disabled={busy || (creationEnabled && !create.allowed)}
                  loading={busy}
                >
                  {intent === 'edit' ? 'Save changes' : intent === 'clone' ? 'Create provider' : 'Create'}
                </Button>
              </DisabledTooltip>
            </div>
          ) : null}
        </fieldset>
      </LoadingRegion>
    </Form.Root>
  )
}

function avatarInput(file: File) {
  return ImageUploadInput.safeParse({
    size: file.size,
    mediaType: ({ 'image/jpeg': 'JPEG', 'image/png': 'PNG', 'image/webp': 'WEBP' } as Record<string, string>)[file.type],
  })
}

export function useProviderForm() {
  const controller = useController()
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
          ...modelDraftRules,
        },
        changed: (field, value) => {
          if (field === 'name' || field === 'baseUrl' || field === 'apiKey')
            controller.onConnectionChanged(field, String(value))
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
}
