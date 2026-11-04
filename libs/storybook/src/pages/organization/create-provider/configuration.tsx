import { FormSection } from '../../../components/form-section'
import { SectionSurface } from '../../../components/surfaces'
import { FormField } from '../../../components/form-field'
import { CheckIcon } from '@phosphor-icons/react/dist/csr/Check'
import { XCircleIcon } from '@phosphor-icons/react/dist/csr/XCircle'
import { LinkSimpleIcon } from '@phosphor-icons/react/dist/csr/LinkSimple'
import { Button } from '../../../components/buttons'
import { KeyValueEditor } from '../../../components/key-value-editor'
import { ProviderAvatar } from '../../../components/provider-avatar'
import { Tooltip } from '../../../components/feedback'

import { useController, useProvider } from './controller'
import { ProviderModels } from './model-table'
import { canCreateProvider, canSaveProvider } from './validation'
import { headersAreValid } from './validation'
function ConnectionFields() {
  const controller = useController()
  const name = useProvider((state) => state.name)
  const baseUrl = useProvider((state) => state.baseUrl)
  const apiKey = useProvider((state) => state.apiKey)
  const editing = useProvider((state) => state.intent === 'edit')
  const busy = useProvider((state) => state.submitting)
  const selected = useProvider((state) => state.selectedProviderId)
  const avatarUrl = useProvider((state) => state.avatarUrl)
  const headers = useProvider((state) => state.headers)
  const validHeaders = headersAreValid(headers)
  return (
    <SectionSurface className="grid gap-4">
      <fieldset disabled={busy} className="grid min-w-0 gap-4">
        <div className="flex min-w-0 items-end gap-3">
          <FormField className="flex-1">
            <FormField.Label>Connection name</FormField.Label>
            <FormField.Input
              value={name}
              placeholder="Enter connection name"
              onChange={(event) => controller.onConnectionChanged('name', event.target.value)}
            />
          </FormField>
          <ProviderAvatar provider={selected} src={avatarUrl} disabled={busy} onFileChange={controller.onAvatarChanged} />
        </div>
        <FormField>
          <FormField.Label>Base URL</FormField.Label>
          <FormField.Input
            value={baseUrl}
            placeholder="https://api.example.com/v1"
            onChange={(event) => controller.onConnectionChanged('baseUrl', event.target.value)}
          />
        </FormField>
        <FormField>
          <FormField.Label>API key</FormField.Label>
          <FormField.Input
            type="password"
            autoComplete="off"
            value={apiKey}
            placeholder={editing ? 'Leave empty to keep current key' : 'Enter your API key'}
            onChange={(event) => controller.onConnectionChanged('apiKey', event.target.value)}
          />
        </FormField>
      </fieldset>
      <KeyValueEditor
        value={headers}
        disabled={busy || !controller.onHeadersChanged}
        onValueChange={(value) => controller.onHeadersChanged?.(value)}
      >
        <KeyValueEditor.Disclosure>Headers</KeyValueEditor.Disclosure>
      </KeyValueEditor>
      {!validHeaders ? (
        <p role="alert" className="text-xs text-error">
          Use unique valid header names and values without line breaks.
        </p>
      ) : null}
    </SectionSurface>
  )
}
function TestConnection() {
  const controller = useController()
  const result = useProvider((state) => state.test)
  const endpoint = useProvider((state) => state.endpointUrl)
  const busy = useProvider((state) => state.submitting)
  const ready = useProvider(canCreateProvider)
  return (
    <SectionSurface className="grid gap-2">
      <p className="flex min-w-0 items-center gap-2 text-xs text-focus">
        <LinkSimpleIcon aria-hidden="true" className="size-3.5 shrink-0" />
        <span className="truncate">{endpoint || 'Configure the connection endpoint'}</span>
      </p>
      <div className="flex flex-wrap items-center gap-3">
        <Button
          className="h-7 px-3 text-xs"
          disabled={!ready || busy}
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
                className="flex cursor-help items-center gap-1.5 text-xs text-error focus-visible:outline-focus"
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
  const ready = useProvider((state) => (state.intent === 'edit' ? canSaveProvider(state) : canCreateProvider(state)))
  const busy = useProvider((state) => state.submitting)
  const error = useProvider((state) => state.error)
  return (
    <form
      className="mx-auto grid w-full max-w-4xl gap-6 px-4 py-8"
      onSubmit={(event) => {
        event.preventDefault()
        if (ready && !busy) {
          if (intent === 'edit') void controller.onSaveProvider()
          else void controller.onCreateProvider()
        }
      }}
    >
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
        <p role="alert" className="pl-7 text-sm text-error">
          {error}
        </p>
      ) : null}
      <div className="flex justify-end gap-2">
        {intent !== 'create' ? (
          <Button className="h-7 px-4 text-xs" disabled={busy} onClick={controller.onCancelProvider}>
            Cancel
          </Button>
        ) : null}
        <Button type="submit" variant="primary" className="h-7 w-32 text-xs" disabled={!ready} loading={busy}>
          {intent === 'edit' ? 'Save changes' : intent === 'clone' ? 'Create provider' : 'Add provider'}
        </Button>
      </div>
    </form>
  )
}
