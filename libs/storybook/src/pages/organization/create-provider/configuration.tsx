import { FormSection } from '../../../components/form-section'
import { SectionSurface } from '../../../components/surfaces'
import { FormField } from '../../../components/form-field'
import { CheckIcon } from '@phosphor-icons/react/dist/csr/Check'
import { XCircleIcon } from '@phosphor-icons/react/dist/csr/XCircle'
import { LinkSimpleIcon } from '@phosphor-icons/react/dist/csr/LinkSimple'
import { Button } from '../../../components/buttons'
import { Select } from '../../../components/dropdowns'
import { Tooltip } from '../../../components/feedback'

import { useController, useProvider } from './controller'
import { ProviderModels } from './model-table'
import { canCreateProvider, canSaveProvider } from './validation'
import { protocolLabels } from './chooser'
import type { ApiProtocol, EndpointMode } from './types'

export const modes: { value: EndpointMode; label: string; path: string }[] = [
  { value: 'chat_completions', label: 'Chat completions', path: '/chat/completions' },
  { value: 'responses', label: 'Responses', path: '/responses' },
  { value: 'messages', label: 'Messages', path: '/messages' },
  { value: 'video', label: 'Video', path: '/videos' },
  { value: 'realtime', label: 'Realtime', path: '/realtime' },
]
function ConnectionFields() {
  const controller = useController()
  const baseUrl = useProvider((state) => state.baseUrl)
  const apiKey = useProvider((state) => state.apiKey)
  const editing = useProvider((state) => state.intent === 'edit')
  const mode = useProvider((state) => state.mode)
  const protocol = useProvider((state) => state.protocol)
  const busy = useProvider((state) => state.submitting)
  return (
    <fieldset disabled={busy} className="grid min-w-0 grid-cols-1 gap-x-2 gap-y-4 rounded-xl bg-inset p-4 sm:grid-cols-2">
      <FormField>
        <FormField.Label>Base URL</FormField.Label>
        <FormField.Input
          value={baseUrl}
          placeholder="https://api.example.com/v1"
          onChange={(event) => controller.onConnectionChanged('baseUrl', event.target.value)}
        />
      </FormField>
      <div className="grid content-start gap-1">
        <span className="text-sm font-medium">Mode</span>
        <Select
          aria-label="Mode"
          options={modes}
          value={mode}
          onValueChange={(value) => controller.onModeChanged(value as EndpointMode)}
          className="w-full max-w-none [&>button]:h-8 [&>button]:px-1.5 [&>button]:text-sm [&_[role=option]]:text-sm"
        />
        <span className="text-xs text-focus">{modes.find((item) => item.value === mode)?.path}</span>
      </div>
      <FormField>
        <FormField.Label>{editing ? 'API key' : 'API key *'}</FormField.Label>
        <FormField.Input
          type="password"
          autoComplete="off"
          value={apiKey}
          placeholder={editing ? 'Leave empty to keep current key' : 'Enter your API key'}
          onChange={(event) => controller.onConnectionChanged('apiKey', event.target.value)}
        />
      </FormField>
      <div className="grid content-start gap-1">
        <span className="text-sm font-medium">API protocol</span>
        <Select
          aria-label="API protocol"
          value={protocol}
          options={Object.entries(protocolLabels).map(([value, label]) => ({ value, label }))}
          onValueChange={(value) => controller.onProtocolChanged(value as ApiProtocol)}
          className="w-full max-w-none [&>button]:h-8 [&>button]:px-1.5 [&>button]:text-sm [&_[role=option]]:text-sm"
        />
      </div>
    </fieldset>
  )
}
function ConnectionName() {
  const controller = useController()
  const name = useProvider((state) => state.name)
  const busy = useProvider((state) => state.submitting)
  return (
    <SectionSurface>
      <FormField>
        <FormField.Label>Connection name</FormField.Label>
        <FormField.Input
          value={name}
          disabled={busy}
          placeholder="Enter connection name"
          onChange={(event) => controller.onConnectionChanged('name', event.target.value)}
        />
      </FormField>
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
      className="grid gap-4 px-3 py-6"
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
        <FormSection.Heading>Name</FormSection.Heading>
        <ConnectionName />
      </FormSection>
      <FormSection>
        <FormSection.Marker>2</FormSection.Marker>
        <FormSection.Heading>Connection details</FormSection.Heading>
        <ConnectionFields />
      </FormSection>
      <FormSection>
        <FormSection.Marker>3</FormSection.Marker>
        <FormSection.Heading>Enabled models</FormSection.Heading>
        <ProviderModels />
      </FormSection>
      <FormSection>
        <FormSection.Marker>4</FormSection.Marker>
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
