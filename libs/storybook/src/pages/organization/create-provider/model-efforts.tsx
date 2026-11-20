import { EffortMapping } from '../../../components/effort-mapping'
import { FormError } from '../../../components/form'
import { ModelSection } from './model-sections'

export function ModelEfforts() {
  return (
    <ModelSection
      title="Thinking efforts"
      help={`
Reasoning models can spend more time and tokens thinking before they answer. Map Low through Max to the exact
effort values accepted by this model. Only one field is edited at a time. Select a level to configure it and leave
unsupported levels empty; the glow shows the order, not a price or token budget.
`}
    >
      <div className="pt-1">
        <EffortMapping name="modelDraft.effortMapping" variant="compact" />
      </div>
      <FormError name="modelDraft.effortMapping" />
    </ModelSection>
  )
}
