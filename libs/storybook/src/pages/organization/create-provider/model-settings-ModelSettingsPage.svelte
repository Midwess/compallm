<script lang="ts">
  import { cloneFormValue } from '../../../components/form-model.svelte'
  import { LoadingRegion } from '../../../components/loading'
  import { providerPageLayout } from './layout'
  import DotsThreeIcon from 'phosphor-svelte/lib/DotsThreeIcon'
  import ArrowCounterClockwiseIcon from 'phosphor-svelte/lib/ArrowCounterClockwiseIcon'
  import { Popup } from '../../../components/popup'
  import { CatalogStatus } from './catalog-status'
  import { modelDraftRules } from './configuration'
  import { ProviderModelFieldsConstraints } from '@compallm/contract/provider'
  import { Form, FormError } from '../../../components/form'
  import { FormField } from '../../../components/form-field'
  import { PageHeader, Functions } from '../../../components/page-header'
  import { Button } from '../../../components/buttons'
  import { AlertDialog } from '../../../components/dialog'
  import { FieldHelp } from '../../../components/feedback'

  import { modelDraftFingerprint, modelPath, initializeModelDraft } from './model-options'
  import { parseModelDraft } from './validation'
  import type { ModelDraft } from './types'
  import { ModelCapabilities, ModelContext, ModelPricing, ModelSection } from './model-sections'
  import { ModelEfforts } from './model-efforts'
  import { ModelRequestFields } from './model-request-fields'
  import { untrack } from 'svelte'
  import { readStore } from '../../page-state.svelte'
  import { getCreateProviderController } from '../../../context'
  import { ModelPathField, ModelBreadcrumb, ModelHeading, ModelSaveStatus } from './model-settings'
  import { useModelEditAccess } from './model-settings'

  const __provider = getCreateProviderController()
  const controller = untrack(() => getCreateProviderController())
  let identity = $derived(readStore(__provider.store, (s) => s.modelDraft?.id, ['modelDraft.id']))
  let confirm = $state(false)
  let confirmReset = $state(false)
  let navigation = $derived(readStore(__provider.store, (s) => s.navigationRequest))
  const readAccess = useModelEditAccess()
  let access = $derived(readAccess())
  let ready = $derived(readStore(__provider.store, (s) => !s.templatePreview || !!s.modelLoaded))
  let loading = $derived(readStore(__provider.store, (s) => s.modelLoading || s.providerLoading))
  let receivedDefaults = $derived(readStore(__provider.store, (s) => s.modelDraft?.defaults, ['modelDraft.defaults']))
  const initialDefaults = untrack(() => {
    const draft = controller.store.getState().modelDraft!
    return ready ? cloneFormValue(draft.defaults ?? initializeModelDraft(draft).defaults!) : undefined
  })
  let resetDefaults = $derived(receivedDefaults ?? initialDefaults)
  let editedFields = $state(untrack(() => new Set<string>()))
  let form = $state(untrack(() => controller.store.form.scope(['modelDraft'])))
  $effect(() => {
    void [controller, identity, form, editedFields, ready]
    return untrack(() => {
      if (!ready) return
      const fields = {
        ...modelDraftRules,
        ...Object.fromEntries(
          Object.entries(controller.store.getState().modelDraft ?? {}).map(([key]) => [
            `modelDraft.${key}`,
            {
              ...modelDraftRules[`modelDraft.${key}` as keyof typeof modelDraftRules],
              mode: ([
                'headers',
                'bodyDefaults',
                'capabilities',
                'contextLimitEnabled',
                'inputLimitEnabled',
                'outputLimitEnabled',
                'inputPriceEnabled',
                'outputPriceEnabled',
                'cachedPriceEnabled',
              ].includes(key)
                ? 'change'
                : 'blur') as 'change' | 'blur',
              group: 'modelDraft',
            },
          ]),
        ),
        modelDraft: { mode: 'blur' as const, group: 'modelDraft' },
      }
      form.configure({
        mode: 'blur',
        command: true,
        fields,
        validationNames: () => ['modelDraft'],
        showError: (name) =>
          [...editedFields].some(
            (field) =>
              name === field ||
              name.startsWith(field + '.') ||
              ([
                'maxContextTokens',
                'maxInputTokens',
                'maxOutputTokens',
                'contextLimitEnabled',
                'inputLimitEnabled',
                'outputLimitEnabled',
              ].some((key) => field === `modelDraft.${key}`) &&
                ['modelDraft.maxContextTokens', 'modelDraft.maxInputTokens', 'modelDraft.maxOutputTokens'].includes(name)),
          ),
        changed: (name, value) => {
          const relatedFields: Record<string, string> = {
            contextLimitEnabled: 'maxContextTokens',
            inputLimitEnabled: 'maxInputTokens',
            outputLimitEnabled: 'maxOutputTokens',
            inputPriceEnabled: 'inputPrice',
            outputPriceEnabled: 'outputPrice',
            cachedPriceEnabled: 'cachedPrice',
          }
          editedFields.add(name)
          const related = relatedFields[name.replace('modelDraft.', '')]
          if (related) editedFields.add(`modelDraft.${related}`)
          if (name === 'modelDraft.path') controller.onModelDraftChanged({ path: '/' + String(value).replace(/^\/+/, '') })
          else
            controller.onModelDraftChanged({
              ...(controller.store.form.get('modelDraft') as ModelDraft),
              ...(name === 'modelDraft.effortMapping'
                ? { thinkingLevels: Object.keys(value as object) as ModelDraft['thinkingLevels'] }
                : {}),
            })
        },
        parse: (values, names) => {
          const state = controller.store.getState()
          if (controller.validateModel && values.modelDraft) {
            const issues = controller.validateModel(values.modelDraft, names)
            return issues?.length ? { issues } : { values }
          }
          if (state.templatePreview) return { values }
          const result = parseModelDraft(values.modelDraft ?? null, state.models, state.editingModelId)
          return result.issues ? { issues: result.issues } : { values }
        },
        submit: async (values, _patch, _signal, names) => {
          const state = controller.store.getState()
          if (!values.modelDraft || state.submitting) return false
          const saved = await controller.onSaveModel(values.modelDraft, names)
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
        catalogPrices: draft.catalogPrices ?? {
          inputPrice: draft.inputPrice,
          outputPrice: draft.outputPrice,
          cachedPrice: draft.cachedPrice,
        },
      }
      for (const [key, value] of Object.entries(defaults)) controller.store.form.set(`modelDraft.${key}`, value)
      form.reset(controller.store.getState())
    })
  })
  const reset = () => {
    const current = controller.store.getState().modelDraft
    if (!current || !access || !resetDefaults) return
    if (controller.onResetModel) {
      form.stop()
      void controller
        .onResetModel()
        .then(() => form.reset(controller.store.getState()))
        .catch(() => undefined)
      return
    }
    const next = initializeModelDraft({
      ...cloneFormValue(resetDefaults),
      id: current.id,
      persistedId: current.persistedId,
      catalogRef: current.catalogRef,
      defaults: cloneFormValue(resetDefaults),
    })
    form.change('modelDraft', next)
    void form.commit(['modelDraft'], true)
  }
  const close = () => {
    if (controller.store.getState().templatePreview) {
      form.stop()
      controller.onCancelModel()
      return
    }
    void (async () => {
      const state = controller.store.getState(),
        draft = state.modelDraft
      if (!draft || !resetDefaults) return
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
      } else confirm = true
    })()
  }
</script>

<Form.Root
  model={form}
  data-model-settings
  class="contents"
  onkeydown={(event) => {
    if (event.key === 'Escape' && !event.defaultPrevented && event.target === event.currentTarget) close()
  }}
>
  <PageHeader class="rounded-t-xl">
    <ModelBreadcrumb onBack={close}></ModelBreadcrumb><Functions>
      {#if controller.validateModel}<ModelSaveStatus {form}></ModelSaveStatus>{:else}{/if}<Popup.Root>
        <Popup.Trigger aria-label="Model actions">
          <DotsThreeIcon aria-hidden="true" class="size-4"></DotsThreeIcon>
        </Popup.Trigger><Popup.Content aria-label="Model actions">
          <Popup.Item
            disabled={!access}
            onSelect={() => {
              if (controller.store.getState().modelDraft?.origin === 'CUSTOM') confirm = true
              else confirmReset = true
            }}
            class="gap-2"
          >
            <ArrowCounterClockwiseIcon aria-hidden="true" class="size-3.5"
            ></ArrowCounterClockwiseIcon>{#if controller.store.getState().modelDraft?.origin === 'CUSTOM'}Delete custom model{:else}Reset
              to defaults{/if}
          </Popup.Item>
        </Popup.Content>
      </Popup.Root>
    </Functions>
  </PageHeader><CatalogStatus></CatalogStatus><LoadingRegion
    class="contents"
    {ready}
    pending={loading}
    controller={controller.loading?.model}
    label="Loading model configuration"
  >
    <fieldset disabled={!access} class={`${providerPageLayout} gap-8`}>
      <ModelHeading></ModelHeading><ModelSection
        title="Configuration"
        help={`
Use the model identifier and endpoint accepted by this provider. The provider connection supplies the base URL
and credential. Valid edits are saved automatically into the provider draft. The provider’s final save is separate.
`}
      >
        <div class="grid gap-3 rounded-control bg-inset p-3">
          <ModelPathField></ModelPathField><FormField.Root>
            <div class="flex items-center gap-1">
              <FormField.Label class="text-xsm font-normal">Name</FormField.Label><FieldHelp
                label="About the upstream model name"
              >
                Enter the exact model identifier the provider expects, including vendor prefixes, slashes, dots, or colons. This
                is sent upstream; it is not a friendly display name. Spaces and control characters are not allowed.
              </FieldHelp>
            </div>
            <FormField.Input
              name="modelDraft.name"
              required
              class="rounded-lg"
              placeholder="Enter model name"
              maxlength={ProviderModelFieldsConstraints.name.maxLength}
            ></FormField.Input><FormError name="modelDraft.name"></FormError>
          </FormField.Root>
        </div>
      </ModelSection><ModelEfforts></ModelEfforts><ModelContext></ModelContext><ModelPricing></ModelPricing><ModelCapabilities
      ></ModelCapabilities><ModelRequestFields kind="headers"></ModelRequestFields><ModelRequestFields kind="bodyDefaults"
      ></ModelRequestFields>
    </fieldset>
  </LoadingRegion>
</Form.Root><AlertDialog.Root open={confirmReset} onOpenChange={(value) => (confirmReset = value)}>
  <AlertDialog.Content>
    <AlertDialog.Header><AlertDialog.Title>Reset model to defaults?</AlertDialog.Title></AlertDialog.Header><AlertDialog.Body>
      <AlertDialog.Description>
        This removes your personal overrides and restores the global model defaults. Saved providers are unchanged.
      </AlertDialog.Description>
    </AlertDialog.Body><AlertDialog.Footer>
      <Button autofocus onclick={() => (confirmReset = false)}>Cancel</Button><Button
        variant="destructive"
        onclick={() => {
          confirmReset = false
          reset()
        }}
      >
        Reset to defaults
      </Button>
    </AlertDialog.Footer>
  </AlertDialog.Content>
</AlertDialog.Root><AlertDialog.Root
  open={confirm || !!navigation}
  onOpenChange={(open) => {
    if (!open) {
      confirm = false
      navigation?.keep()
    }
  }}
>
  <AlertDialog.Content>
    <AlertDialog.Header>
      <AlertDialog.Title>
        {#if controller.store.getState().modelDraft?.origin === 'CUSTOM' && !navigation}Delete custom model?{:else}Discard
          changes?{/if}
      </AlertDialog.Title>
    </AlertDialog.Header><AlertDialog.Body>
      <AlertDialog.Description>Your unsaved model changes will be lost.</AlertDialog.Description>
    </AlertDialog.Body><AlertDialog.Footer>
      <Button
        autofocus
        onclick={() => {
          confirm = false
          navigation?.keep()
        }}
      >
        Keep editing
      </Button><Button
        variant="destructive"
        onclick={() => {
          confirm = false
          if (controller.store.getState().modelDraft?.origin === 'CUSTOM' && !navigation) {
            form.stop()
            void controller.onDeleteModel?.().catch(() => undefined)
            return
          }
          if (navigation) navigation.discard()
          else controller.onCancelModel()
        }}
      >
        Discard changes
      </Button>
    </AlertDialog.Footer>
  </AlertDialog.Content>
</AlertDialog.Root>
