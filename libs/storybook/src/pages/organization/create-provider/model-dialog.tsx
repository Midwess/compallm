import { EffortMapping } from '../../../components/effort-mapping'
import { modelPath, modelProtocol, isEndpointPath, textModelOptions, modelDraftFingerprint } from './model-options'
import { FormField } from '../../../components/form-field'
import { Button } from '../../../components/buttons'
import { Dialog } from '../../../components/dialog'

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
          draft.maxOutputTokens ||
          Object.values(draft.effortMapping ?? {}).some(Boolean) ||
          draft.maxContextTokens ||
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
                  aria-invalid={!isEndpointPath(modelPath(draft)) || undefined}
                  onChange={(event) => controller.onModelDraftChanged({ path: event.target.value })}
                />
                {!isEndpointPath(modelPath(draft)) ? (
                  <FormField.Hint>Enter a path beginning with /, without a host, query, or fragment.</FormField.Hint>
                ) : null}
              </FormField>
              <FormField>
                <FormField.Label>Model name</FormField.Label>
                <FormField.Input
                  value={draft.name}
                  required
                  placeholder="Enter model name"
                  onChange={(event) => controller.onModelDraftChanged({ name: event.target.value })}
                />
              </FormField>
              <div className="grid grid-cols-2 gap-2">
                <FormField>
                  <FormField.Label>Max content window</FormField.Label>
                  <FormField.Input
                    inputMode="numeric"
                    placeholder="Not set"
                    value={draft.maxContextTokens}
                    onChange={(event) => controller.onModelDraftChanged({ maxContextTokens: event.target.value })}
                  />
                </FormField>
                <FormField>
                  <FormField.Label>Max output tokens</FormField.Label>
                  <FormField.Input
                    inputMode="numeric"
                    placeholder="Not set"
                    value={draft.maxOutputTokens ?? ''}
                    onChange={(event) => controller.onModelDraftChanged({ maxOutputTokens: event.target.value })}
                  />
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
              <h3 className="text-sm font-medium">Pricing</h3>
              <div className="grid grid-cols-1 gap-2 sm:grid-cols-3">
                {(
                  [
                    { key: 'inputPrice', label: '1M Input' },
                    { key: 'outputPrice', label: '1M Output' },
                    { key: 'cachedPrice', label: '1M Cached' },
                  ] as const
                ).map(({ key, label }) => (
                  <FormField key={key}>
                    <FormField.Label>{label}</FormField.Label>
                    <FormField.Input
                      required
                      inputMode="decimal"
                      placeholder="Enter price USD"
                      value={draft[key]}
                      onChange={(event) => controller.onModelDraftChanged({ [key]: event.target.value })}
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
