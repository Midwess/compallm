import { Checkbox } from '../../../components/selection'
import { FormField } from '../../../components/form-field'
import { Button } from '../../../components/buttons'
import { Dialog } from '../../../components/dialog'

import { useController, useProvider } from './controller'
import { isValidModelDraft } from './validation'
import type { ThinkingLevel } from './types'

const levels: { value: ThinkingLevel; label: string }[] = [
  { value: 'low', label: 'Low' },
  { value: 'medium', label: 'Medium' },
  { value: 'high', label: 'High' },
  { value: 'xhigh', label: 'Xhigh' },
  { value: 'max', label: 'Max' },
]
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
      ? JSON.stringify([
          draft.name,
          draft.maxContextTokens,
          draft.thinkingLevels,
          draft.inputPrice,
          draft.outputPrice,
          draft.cachedPrice,
        ]) !==
        JSON.stringify([
          originalDraft.name,
          originalDraft.maxContextTokens,
          originalDraft.thinkingLevels,
          originalDraft.inputPrice,
          originalDraft.outputPrice,
          originalDraft.cachedPrice,
        ])
      : !!(
          draft.name ||
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
              <Dialog.Title>{editing ? 'Edit model' : 'Add model'}</Dialog.Title>
              <Dialog.Close aria-label="Close model popup" />
            </Dialog.Header>
            <Dialog.Body>
              <FormField>
                <FormField.Label>Model name</FormField.Label>
                <FormField.Input
                  value={draft.name}
                  required
                  autoFocus
                  placeholder="Enter model name"
                  onChange={(event) => controller.onModelDraftChanged({ name: event.target.value })}
                />
              </FormField>
              <FormField>
                <FormField.Label>Max content window</FormField.Label>
                <FormField.Input
                  inputMode="numeric"
                  placeholder="Not set"
                  value={draft.maxContextTokens}
                  onChange={(event) => controller.onModelDraftChanged({ maxContextTokens: event.target.value })}
                />
              </FormField>
              <fieldset className="min-w-0">
                <legend className="mb-2 text-sm font-medium">Supported thinking levels</legend>
                <div className="flex flex-wrap items-center justify-between gap-4">
                  {levels.map(({ value, label }) => (
                    <label key={value} className="flex items-center gap-1.5 text-sm">
                      <Checkbox
                        aria-label={label}
                        checked={draft.thinkingLevels.includes(value)}
                        onChange={(event) =>
                          controller.onModelDraftChanged({
                            thinkingLevels: event.target.checked
                              ? [...draft.thinkingLevels, value]
                              : draft.thinkingLevels.filter((item) => item !== value),
                          })
                        }
                      />
                      {label}
                    </label>
                  ))}
                </div>
              </fieldset>
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
              <p className="text-xs text-muted">All three prices are required. Enter 0 for a free rate.</p>
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
