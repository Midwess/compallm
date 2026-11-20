import { EffortMapping } from '../../../components/effort-mapping'
import { FormError } from '../../../components/form'
import { ModelSection } from './model-sections'

export function ModelEfforts() {
  return (
    <ModelSection
      title="Thinking efforts"
      help={`
Reasoning models can spend more time and tokens thinking before they answer. Map Low through Max to the exact
effort values accepted by this model. Only one field is edited at a time. Fill levels in order and leave
unsupported levels empty; the glow shows the order, not a price or token budget.
`}
    >
      <EffortMapping name="modelDraft.effortMapping" variant="compact" />
      <FormError name="modelDraft.effortMapping" />
    </ModelSection>
  )
}
