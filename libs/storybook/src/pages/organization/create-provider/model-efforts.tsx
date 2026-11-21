import { useId } from 'react'
import { EffortMapping } from '../../../components/effort-mapping'
import { FormError, useFormField } from '../../../components/form'
import { Checkbox } from '../../../components/selection'
import { FieldHelp } from '../../../components/feedback'
import type { ModelCapability, ThinkingLevel } from './types'

export function ModelEfforts() {
  const id = useId()
  const capabilities = useFormField<ModelCapability[]>('modelDraft.capabilities')!
  const mapping = useFormField<Record<string, string>>('modelDraft.effortMapping')!
  const levels = useFormField<ThinkingLevel[]>('modelDraft.thinkingLevels')!
  const enabled = (capabilities.value ?? []).includes('reasoning') || (levels.value ?? []).length > 0
  return (
    <section className="grid min-w-0 gap-2" aria-labelledby={id}>
      <div className="flex min-h-6 items-center gap-1 px-4">
        <label id={id} className="flex cursor-pointer items-center gap-2 text-xsm font-medium">
          <Checkbox
            size="compact"
            aria-label="Enable thinking efforts"
            checked={enabled}
            onChange={(event) => {
              const next = (capabilities.value ?? []).filter((capability) => capability !== 'reasoning')
              capabilities.change(event.target.checked ? [...next, 'reasoning'] : next)
              if (!event.target.checked) {
                mapping.change({})
                levels.change([])
              }
            }}
          />
          Thinking efforts
        </label>
        <FieldHelp label="About thinking efforts">
          Enable only if this model supports reasoning. Map each level to an effort value accepted by the provider. Leave
          unsupported levels empty. Higher effort can use more time and tokens; enabling this setting cannot add reasoning
          support to the upstream model. Turning it off clears the effort mappings.
        </FieldHelp>
      </div>
      {enabled ? (
        <div className="rounded-control bg-inset p-3">
          <EffortMapping name="modelDraft.effortMapping" variant="compact" />
          <FormError name="modelDraft.effortMapping" />
        </div>
      ) : null}
    </section>
  )
}
