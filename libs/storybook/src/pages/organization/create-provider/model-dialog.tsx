import { FormError } from '../../../components/form'
import { useEffect, useState } from 'react'
import { Form } from '../../../components/form'
import { ProviderModelFieldsConstraints } from '@compallm/contract/provider'
import { EffortMapping } from '../../../components/effort-mapping'
import { modelPath, modelProtocol, textModelOptions, modelDraftFingerprint } from './model-options'
import { FormField } from '../../../components/form-field'
import { Button } from '../../../components/buttons'
import { Dialog } from '../../../components/dialog'
import { FieldHelp } from '../../../components/feedback'

import { useController, useProvider } from './controller'
import { parseModelDraft } from './validation'

export function ModelDialog() {
  const controller = useController()
  const draft = useProvider(
    (state) => (state.modelDraft ? { id: state.modelDraft.id, protocol: state.modelDraft.protocol } : null),
    ['modelDraft.id', 'modelDraft.protocol'],
  )
  const editing = useProvider((state) => state.editingModelId)
  useEffect(() => {
    const current = controller.store.getState().modelDraft
    if (current && current.path === undefined) controller.store.form.set('modelDraft.path', modelPath(current))
    if (current && !current.effortMapping)
      controller.store.form.set(
        'modelDraft.effortMapping',
        Object.fromEntries(current.thinkingLevels.map((level) => [level, level])),
      )
  }, [controller, draft?.id])
  const [modelForm] = useState(() =>
    controller.store.form.scope(['modelDraft'], {
      mode: 'button',
      command: true,
      validationNames: () => ['modelDraft'],
      parse: (values) => {
        const state = controller.store.getState()
        const result = parseModelDraft(values.modelDraft ?? null, state.models, state.editingModelId)
        if (result.issues) return { issues: result.issues }
        return {
          values: values.modelDraft
            ? { ...values, modelDraft: { ...values.modelDraft, name: result.data!.name, path: result.data!.path } }
            : values,
        }
      },
      submit: async (values) => {
        if (!values.modelDraft) return false
        controller.onSaveModel(values.modelDraft)
        return {}
      },
    }),
  )
  if (!draft) return null
  return (
    <>
      <Dialog onClose={controller.onCancelModel}>
        <Dialog.Content>
          <Form model={modelForm}>
            <Dialog.Header>
              <Dialog.Title>
                {editing
                  ? 'Edit model'
                  : `Add ${textModelOptions.find((option) => option.value === modelProtocol(draft))!.label}`}
              </Dialog.Title>
              <Dialog.Close aria-label="Close model popup" className="focus-visible:outline-muted" />
            </Dialog.Header>
            <Dialog.Body className="py-4">
              <FormField>
                <FormField.Label>Path</FormField.Label>
                <FormField.Input
                  name="modelDraft.path"
                  required
                  autoFocus
                  maxLength={ProviderModelFieldsConstraints.path.maxLength}
                />
                <FormError name="modelDraft.path" />
              </FormField>
              <FormField>
                <div className="flex items-center gap-1">
                  <FormField.Label>Model name</FormField.Label>
                  <FieldHelp label="About model names">
                    1–{ProviderModelFieldsConstraints.name.maxLength} letters, digits, dashes, or underscores. No spaces.
                  </FieldHelp>
                </div>
                <FormField.Input
                  name="modelDraft.name"
                  required
                  placeholder="Enter model name"
                  maxLength={ProviderModelFieldsConstraints.name.maxLength}
                />
                <FormError name="modelDraft.name" />
              </FormField>
              <div>
                <FormField>
                  <FormField.Label>Max context window</FormField.Label>
                  <FormField.Input inputMode="numeric" required placeholder="250000" name="modelDraft.maxContextTokens" />
                  <FormError name="modelDraft.maxContextTokens" />
                  <FormField.Hint>Tokens</FormField.Hint>
                </FormField>
              </div>
              <EffortMapping name="modelDraft.effortMapping" />
              <FormError name="modelDraft.effortMapping" />
              <div className="flex items-center gap-1">
                <h3 className="text-sm font-medium">Pricing · USD per 1M tokens</h3>
                <FieldHelp label="About model pricing">
                  Enter a non-negative price with up to 8 decimal places. Use 0 for free.
                </FieldHelp>
              </div>
              <div className="grid grid-cols-1 gap-2 sm:grid-cols-3">
                {(
                  [
                    { key: 'inputPrice', label: 'Input' },
                    { key: 'outputPrice', label: 'Output' },
                    { key: 'cachedPrice', label: 'Cache read' },
                  ] as const
                ).map(({ key, label }) => (
                  <FormField key={key}>
                    <FormField.Label>{label}</FormField.Label>
                    <FormField.Input
                      required
                      inputMode="decimal"
                      placeholder="0.00"

                      name={`modelDraft.${key}`}
                    />
                    <FormError name={`modelDraft.${key}`} />
                  </FormField>
                ))}
              </div>
            </Dialog.Body>
            <Dialog.Footer>
              <Dialog.Cancel className="h-7 px-3 text-xs" />
              <Button type="submit" variant="primary" className="h-7 px-3 text-xs">
                {editing ? 'Save model' : 'Add model'}
              </Button>
            </Dialog.Footer>
          </Form>
        </Dialog.Content>
        <ModelDiscard />
      </Dialog>
    </>
  )
}

function ModelDiscard() {
  const draft = useProvider((state) => state.modelDraft, ['modelDraft'])
  const editing = useProvider((state) => state.editingModelId)
  const models = useProvider((state) => state.models, ['models'])
  const navigationRequest = useProvider((state) => state.navigationRequest)
  const originalDraft = editing ? models.find((model) => model.id === editing) : null
  const changed =
    draft &&
    (originalDraft
      ? modelDraftFingerprint(draft) !== modelDraftFingerprint(originalDraft)
      : !!(
          draft.name ||
          (draft.path !== undefined &&
            draft.path !== textModelOptions.find((option) => option.value === modelProtocol(draft))!.path) ||
          Object.values(draft.effortMapping ?? {}).some(Boolean) ||
          draft.maxContextTokens !== '250000' ||
          draft.thinkingLevels.length ||
          draft.inputPrice ||
          draft.outputPrice ||
          draft.cachedPrice
        ))
  return changed ? (
    <Dialog.Discard request={navigationRequest ?? undefined}>Your unsaved model changes will be lost.</Dialog.Discard>
  ) : null
}
