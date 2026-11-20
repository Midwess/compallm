import { useEffect, useState } from 'react'
import { DotsThreeIcon } from '@phosphor-icons/react/dist/csr/DotsThree'
import { ArrowCounterClockwiseIcon } from '@phosphor-icons/react/dist/csr/ArrowCounterClockwise'
import { Popup } from '../../../components/popup'
import { modelDraftRules } from './configuration'
import { ProviderModelFieldsConstraints } from '@compallm/contract/provider'
import { Form, FormError } from '../../../components/form'
import { FormField } from '../../../components/form-field'
import { PageHeader, Functions } from '../../../components/page-header'
import { Button } from '../../../components/buttons'
import { AlertDialog } from '../../../components/dialog'
import { FieldHelp, Tooltip } from '../../../components/feedback'
import { usePermission } from '../../../components/permission'
import {
  modelDraftFingerprint,
  modelPath,
  modelProtocol,
  textModelOptions,
  joinModelEndpoint,
  initializeModelDraft,
} from './model-options'
import { useController, useProvider } from './controller'
import { parseModelDraft } from './validation'
import type { ModelDraft } from './types'
import { ModelCapabilities, ModelContext, ModelPricing, ModelSection } from './model-sections'
import { ModelEfforts } from './model-efforts'
import { ModelRequestFields } from './model-request-fields'

export function ModelSettingsPage() {
  const controller = useController()
  const identity = useProvider((s) => s.modelDraft?.id, ['modelDraft.id'])
  const [confirm, setConfirm] = useState(false)
  const navigation = useProvider((s) => s.navigationRequest)
  const access = useModelEditAccess()
  const [resetDefaults] = useState(() => {
    const draft = controller.store.getState().modelDraft!
    return structuredClone(draft.defaults ?? initializeModelDraft(draft).defaults!)
  })
  const [form] = useState(() => controller.store.form.scope(['modelDraft']))
  useEffect(() => {
    const fields = {
      ...modelDraftRules,
      ...Object.fromEntries(
        Object.entries(controller.store.getState().modelDraft ?? {}).map(([key]) => [
          `modelDraft.${key}`,
          {
            ...modelDraftRules[`modelDraft.${key}` as keyof typeof modelDraftRules],
            mode: 'debounce' as const,
            debounceMs: 300,
            group: 'modelDraft',
          },
        ]),
      ),
      modelDraft: { mode: 'debounce' as const, debounceMs: 300, group: 'modelDraft' },
    }
    form.configure({
      mode: 'debounce',
      command: true,
      fields,
      validationNames: () => ['modelDraft'],
      changed: (name, value) => {
        if (name === 'modelDraft.path') controller.onModelDraftChanged({ path: '/' + String(value).replace(/^\/+/, '') })
        else
          controller.onModelDraftChanged({
            ...(controller.store.form.get('modelDraft') as ModelDraft),
            ...(name === 'modelDraft.effortMapping'
              ? { thinkingLevels: Object.keys(value as object) as ModelDraft['thinkingLevels'] }
              : {}),
          })
      },
      parse: (values) => {
        const state = controller.store.getState()
        const result = parseModelDraft(values.modelDraft ?? null, state.models, state.editingModelId)
        return result.issues ? { issues: result.issues } : { values }
      },
      submit: async (values) => {
        const state = controller.store.getState()
        if (!values.modelDraft || state.submitting) return false
        const saved = controller.onSaveModel(values.modelDraft)
        return saved ? { modelDraft: saved } : false
      },
    })
    const draft = controller.store.getState().modelDraft
    if (!draft) return
    const defaults = {
      path: modelPath(draft),
      effortMapping: draft.effortMapping ?? Object.fromEntries(draft.thinkingLevels.map((k) => [k, k])),
      contextLimitEnabled: draft.contextLimitEnabled ?? true,
      inputLimitEnabled: draft.inputLimitEnabled ?? false,
      outputLimitEnabled: draft.outputLimitEnabled ?? !!draft.maxOutputTokens,
      maxInputTokens: draft.maxInputTokens ?? '',
      maxOutputTokens: draft.maxOutputTokens ?? '',
      capabilities: draft.capabilities ?? [],
      headers: draft.headers ?? [],
      bodyDefaults: draft.bodyDefaults ?? [],
      inputPriceEnabled: draft.inputPriceEnabled ?? true,
      outputPriceEnabled: draft.outputPriceEnabled ?? true,
      cachedPriceEnabled: draft.cachedPriceEnabled ?? true,
      templatePrices: draft.templatePrices ?? {
        inputPrice: draft.inputPrice,
        outputPrice: draft.outputPrice,
        cachedPrice: draft.cachedPrice,
      },
    }
    for (const [key, value] of Object.entries(defaults)) controller.store.form.set(`modelDraft.${key}`, value)
    const state = controller.store.getState()
    const issues = parseModelDraft(state.modelDraft, state.models, state.editingModelId).issues
    if (issues?.some((issue) => ['maxInputTokens', 'maxOutputTokens'].includes(String(issue.path[1]))))
      form.blur('modelDraft.maxContextTokens')
  }, [controller, identity, form])
  const reset = () => {
    const current = controller.store.getState().modelDraft
    if (!current || !access) return
    const next = initializeModelDraft({
      ...structuredClone(resetDefaults),
      id: current.id,
      persistedId: current.persistedId,
      catalogRef: current.catalogRef,
      defaults: structuredClone(resetDefaults),
    })
    form.change('modelDraft', next)
    void form.commit(['modelDraft'], true)
  }
  const close = () => {
    void (async () => {
      const state = controller.store.getState(),
        draft = state.modelDraft
      if (!draft) return
      const existing = state.models.find((model) => model.id === state.editingModelId)
      if (
        (existing && modelDraftFingerprint(draft) === modelDraftFingerprint(existing)) ||
        (!existing && modelDraftFingerprint(draft) === modelDraftFingerprint(initializeModelDraft(resetDefaults)))
      ) {
        controller.onCancelModel()
        return
      }
      const result = parseModelDraft(draft, state.models, state.editingModelId)
      if (!result.issues) await form.commit(['modelDraft'], true)
      const latest = controller.store.getState(),
        current = latest.modelDraft
      const saved = latest.models.find((model) => model.id === latest.editingModelId)
      if (
        !current ||
        (saved && modelDraftFingerprint(current) === modelDraftFingerprint(saved)) ||
        (!saved && modelDraftFingerprint(current) === modelDraftFingerprint(initializeModelDraft(resetDefaults)))
      ) {
        controller.onCancelModel()
      } else setConfirm(true)
    })()
  }
  return (
    <>
      <Form.Root
        model={form}
        data-model-settings
        className="contents"
        onKeyDown={(event) => {
          if (event.key === 'Escape' && !event.defaultPrevented && event.target === event.currentTarget) close()
        }}
      >
        <PageHeader>
          <ModelBreadcrumb onBack={close} />
          <Functions>
            <Popup.Root>
              <Popup.Trigger aria-label="Model actions">
                <DotsThreeIcon aria-hidden="true" className="size-4" />
              </Popup.Trigger>
              <Popup.Content aria-label="Model actions">
                <Popup.Item disabled={!access} onSelect={reset} className="gap-2">
                  <ArrowCounterClockwiseIcon aria-hidden="true" className="size-3.5" />
                  Reset to defaults
                </Popup.Item>
              </Popup.Content>
            </Popup.Root>
          </Functions>
        </PageHeader>
        <fieldset disabled={!access} className="mx-auto grid w-full min-w-0 max-w-2xl gap-8 px-4 py-8">
          <ModelHeading />
          <ModelSection
            title="Configuration"
            help={`
Use the model identifier and endpoint accepted by this provider. The provider connection supplies the base URL
and credential. Valid edits are saved automatically into the provider draft. The provider’s final save is separate.
`}
          >
            <div className="grid gap-3 rounded-control bg-inset p-3">
              <ModelPathField />
              <FormField.Root>
                <div className="flex items-center gap-1">
                  <FormField.Label className="text-xsm font-normal">Name</FormField.Label>
                  <FieldHelp label="About the upstream model name">
                    Enter the exact model identifier the provider expects, including vendor prefixes, slashes, dots, or colons.
                    This is sent upstream; it is not a friendly display name. Spaces and control characters are not allowed.
                  </FieldHelp>
                </div>
                <FormField.Input
                  name="modelDraft.name"
                  required
                  className="rounded-lg"
                  placeholder="Enter model name"
                  maxLength={ProviderModelFieldsConstraints.name.maxLength}
                />
                <FormError name="modelDraft.name" />
              </FormField.Root>
            </div>
          </ModelSection>
          <ModelEfforts />
          <ModelContext />
          <ModelPricing />
          <ModelCapabilities />
          <ModelRequestFields kind="headers" />
          <ModelRequestFields kind="bodyDefaults" />
        </fieldset>
      </Form.Root>
      <AlertDialog.Root
        open={confirm || !!navigation}
        onOpenChange={(open) => {
          if (!open) {
            setConfirm(false)
            navigation?.keep()
          }
        }}
      >
        <AlertDialog.Content>
          <AlertDialog.Header>
            <AlertDialog.Title>Discard changes?</AlertDialog.Title>
          </AlertDialog.Header>
          <AlertDialog.Body>
            <AlertDialog.Description>Your unsaved model changes will be lost.</AlertDialog.Description>
          </AlertDialog.Body>
          <AlertDialog.Footer>
            <Button
              autoFocus
              onClick={() => {
                setConfirm(false)
                navigation?.keep()
              }}
            >
              Keep editing
            </Button>
            <Button
              variant="destructive"
              onClick={() => {
                setConfirm(false)
                if (navigation) navigation.discard()
                else controller.onCancelModel()
              }}
            >
              Discard changes
            </Button>
          </AlertDialog.Footer>
        </AlertDialog.Content>
      </AlertDialog.Root>
    </>
  )
}

function ModelPathField() {
  return (
    <FormField.Root>
      <div className="flex items-center gap-1">
        <FormField.Label className="text-xsm font-normal">Path</FormField.Label>
        <FieldHelp label="About the model endpoint path">
          This path is appended to the provider’s base URL, keeping any base path such as /v1. For example,
          https://api.openai.com/v1 plus /responses becomes https://api.openai.com/v1/responses. Enter only the path, beginning
          with /. Hover or focus the base URL prefix to see the complete endpoint.
        </FieldHelp>
      </div>
      <FormField.PrefixedInput
        name="modelDraft.path"
        required
        autoFocus
        spellCheck={false}
        autoCapitalize="none"
        autoComplete="off"
        maxLength={ProviderModelFieldsConstraints.path.maxLength}
        prefix={
          <Tooltip.Root>
            <Tooltip.Trigger
              type="button"
              aria-label="Show provider base URL and complete endpoint"
              className="flex h-full items-center px-2 font-mono text-ui-11 text-muted-foreground
            hover:text-foreground focus-visible:outline-1 focus-visible:outline-ring focus-visible:-outline-offset-2"
            >
              {'{{base_url}}'}
            </Tooltip.Trigger>
            <Tooltip.Content className="max-w-[min(32rem,calc(100vw-1rem))] rounded-lg px-3 py-2 text-left">
              <ModelEndpointPreview />
            </Tooltip.Content>
          </Tooltip.Root>
        }
      />
      <FormError name="modelDraft.path" />
    </FormField.Root>
  )
}

function ModelEndpointPreview() {
  const baseUrl = useProvider((s) => s.baseUrl, ['baseUrl'])
  const path = useProvider((s) => s.modelDraft?.path ?? '', ['modelDraft.path'])
  return (
    <div className="grid min-w-0 gap-2">
      <div>
        <p className="text-ui-11 text-muted-foreground">Provider base URL</p>
        <p className="break-all text-xs">{baseUrl || 'Not configured'}</p>
      </div>
      {baseUrl ? (
        <div>
          <p className="text-ui-11 text-muted-foreground">Complete endpoint</p>
          <p className="break-all text-xs">{joinModelEndpoint(baseUrl, path)}</p>
        </div>
      ) : null}
    </div>
  )
}

function ModelBreadcrumb({ onBack }: { onBack(): void }) {
  const provider = useProvider((s) => s.name || 'New provider', ['name'])
  const editing = useProvider((s) => !!s.editingModelId)
  return (
    <nav aria-label="Model settings breadcrumb" className="flex min-w-0 flex-1 items-center gap-4 text-xs">
      <button
        type="button"
        onClick={onBack}
        className="min-w-0 truncate text-muted-foreground hover:text-foreground focus-visible:outline-ring"
      >
        {provider}
      </button>
      <span aria-hidden="true" className="text-muted-foreground">
        /
      </span>
      <span className="shrink-0 whitespace-nowrap">{editing ? 'Edit model' : 'Add model'}</span>
    </nav>
  )
}
function ModelHeading() {
  const draft = useProvider(
    (s) => (s.modelDraft ? { name: s.modelDraft.name, protocol: s.modelDraft.protocol } : null),
    ['modelDraft.name', 'modelDraft.protocol'],
  )
  const provider = useProvider((s) => s.name || 'New provider', ['name'])
  const editing = useProvider((s) => !!s.editingModelId)
  return (
    <div className="grid gap-1 px-4">
      <h1 className="text-2xl font-medium">{editing ? draft?.name || 'Edit model' : 'Add model'}</h1>
      <p className="text-xsm text-muted-foreground">
        {provider} · {textModelOptions.find((o) => o.value === modelProtocol(draft ?? {}))?.label}
      </p>
    </div>
  )
}
function useModelEditAccess() {
  const saved = useProvider((s) => s.intent === 'edit')
  const model = useProvider((s) => s.editingModelId)
  const provider = useProvider((s) => s.savedProvider?.id)
  const permission = usePermission({
    actions: [saved ? (model ? 'provider.model.configuration.update' : 'provider.model.create') : 'provider.create'],
    resource: saved ? (model ? { type: 'ProviderModel', id: model } : { type: 'Provider', id: provider }) : undefined,
    enabled: !saved || !!provider,
  })
  return permission.allowed
}
