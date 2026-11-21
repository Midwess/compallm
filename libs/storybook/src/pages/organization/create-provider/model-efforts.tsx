import { Skeleton } from '../../../components/loading'
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
      <h2 id={id} className="px-4 text-xsm font-medium">
        Thinking efforts
      </h2>
      <div className="grid gap-3 rounded-control bg-inset p-3">
        <div className="flex min-h-6 items-center gap-1">
          <label className="flex cursor-pointer items-center gap-2 text-xsm font-medium">
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
            Enable thinking
          </label>
          <FieldHelp label="About thinking efforts">
            Enable only if this model supports reasoning. Map each level to an effort value accepted by the provider. Leave
            unsupported levels empty. Higher effort can use more time and tokens; enabling this setting cannot add reasoning
            support to the upstream model. Turning it off clears the effort mappings.
          </FieldHelp>
        </div>
        {enabled ? (
          <div>
            <Skeleton>
              <EffortMapping name="modelDraft.effortMapping" variant="compact" />
            </Skeleton>
            <FormError name="modelDraft.effortMapping" />
          </div>
        ) : null}
      </div>
    </section>
  )
}
