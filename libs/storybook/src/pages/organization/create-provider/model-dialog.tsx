import {
  ProviderModelFields,
  ProviderModelFieldsInput,
  ProviderModelFieldsConstraints,
  ProviderInputRules,
} from '@compallm/contract/provider'
import { EffortMapping } from '../../../components/effort-mapping'
import { modelPath, modelProtocol, isEndpointPath, textModelOptions, modelDraftFingerprint } from './model-options'
import { FormField } from '../../../components/form-field'
import { Button } from '../../../components/buttons'
import { Dialog } from '../../../components/dialog'
import { FieldHelp } from '../../../components/feedback'

import { useController, useProvider } from './controller'
import { isValidModelDraft } from './validation'

export function ModelDialog() {
  const controller = useController()
  const draft = useProvider((state) => state.modelDraft)
  const editing = useProvider((state) => state.editingModelId)
  const models = useProvider((state) => state.models)
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
  if (!draft) return null
  const duplicate = models.some((model) => model.id !== editing && model.name.trim() === draft.name.trim())
  const valid = isValidModelDraft(draft) && !duplicate
  return (
    <>
      <Dialog onClose={controller.onCancelModel}>
        <Dialog.Content>
          <form
            onSubmit={(event) => {
              event.preventDefault()
              if (valid) controller.onSaveModel()
            }}
          >
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
                  value={modelPath(draft)}
                  required
                  autoFocus
                  maxLength={ProviderModelFieldsConstraints.path.maxLength}
                  aria-invalid={!isEndpointPath(modelPath(draft)) || undefined}
                  onChange={(event) => {
                    const path = `/${event.target.value.replace(/^\/+/, '')}`
                    if (ProviderModelFieldsInput.path(path)) controller.onModelDraftChanged({ path })
                  }}
                />
                {!isEndpointPath(modelPath(draft)) ? (
                  <FormField.Hint>
                    Use up to {ProviderModelFieldsConstraints.path.maxLength} letters, digits, dashes, underscores, and slashes;
                    start with /.
                  </FormField.Hint>
                ) : null}
              </FormField>
              <FormField>
                <div className="flex items-center gap-1">
                  <FormField.Label>Model name</FormField.Label>
                  <FieldHelp label="About model names">
                    1–{ProviderModelFieldsConstraints.name.maxLength} letters, digits, dashes, or underscores. No spaces.
                  </FieldHelp>
                </div>
                <FormField.Input
                  value={draft.name}
                  required
                  placeholder="Enter model name"
                  maxLength={ProviderModelFieldsConstraints.name.maxLength}
                  aria-invalid={(!!draft.name && !ProviderModelFields.shape.name.safeParse(draft.name).success) || undefined}
                  onChange={(event) => {
                    if (ProviderModelFieldsInput.name(event.target.value))
                      controller.onModelDraftChanged({ name: event.target.value })
                  }}
                />
              </FormField>
              <div>
                <FormField>
                  <FormField.Label>Max context window</FormField.Label>
                  <FormField.Input
                    inputMode="numeric"
                    required
                    placeholder="250000"
                    value={draft.maxContextTokens}
                    aria-invalid={
                      (draft.maxContextTokens !== '' &&
                        (!ProviderInputRules.shape.positiveInteger.safeParse(draft.maxContextTokens).success ||
                          !ProviderModelFields.shape.maxContextTokens.safeParse(Number(draft.maxContextTokens)).success)) ||
                      undefined
                    }
                    onChange={(event) => {
                      if (ProviderModelFieldsInput.maxContextTokens(event.target.value))
                        controller.onModelDraftChanged({ maxContextTokens: event.target.value })
                    }}
                  />
                  <FormField.Hint>Tokens</FormField.Hint>
                </FormField>
              </div>
              <EffortMapping
                value={draft.effortMapping ?? Object.fromEntries(draft.thinkingLevels.map((level) => [level, level]))}
                onValueChange={(effortMapping) =>
                  controller.onModelDraftChanged({
                    effortMapping,
                    thinkingLevels: (Object.keys(effortMapping) as import('./types').ThinkingLevel[]).filter(
                      (level) => !!effortMapping[level],
                    ),
                  })
                }
              />
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
                      aria-invalid={
                        (!!draft[key] && !ProviderModelFields.shape.inputPrice.safeParse(draft[key]).success) || undefined
                      }
                      value={draft[key]}
                      onChange={(event) => {
                        if (ProviderModelFieldsInput.inputPrice(event.target.value))
                          controller.onModelDraftChanged({ [key]: event.target.value })
                      }}
                    />
                  </FormField>
                ))}
              </div>
              {duplicate ? (
                <p role="alert" className="text-xs text-error">
                  A model with this name already exists.
                </p>
              ) : null}
            </Dialog.Body>
            <Dialog.Footer>
              <Dialog.Cancel className="h-7 px-3 text-xs" />
              <Button type="submit" variant="primary" className="h-7 px-3 text-xs" disabled={!valid}>
                {editing ? 'Save model' : 'Add model'}
              </Button>
            </Dialog.Footer>
          </form>
        </Dialog.Content>
        {changed ? (
          <Dialog.Discard request={navigationRequest ?? undefined}>Your unsaved model changes will be lost.</Dialog.Discard>
        ) : null}
      </Dialog>
    </>
  )
}
