<script lang="ts">
  import { Skeleton } from '../../../components/loading'
  import { EffortMapping } from '../../../components/effort-mapping'
  import { FormError } from '../../../components/form'
  import { Checkbox } from '../../../components/selection'
  import { FieldHelp } from '../../../components/feedback'
  import type { ModelCapability, ThinkingLevel } from './types'

  import { fieldState } from '../../page-state.svelte'

  const id = $props.id()
  const capabilities = fieldState<ModelCapability[]>(() => 'modelDraft.capabilities')!
  const mapping = fieldState<Record<string, string>>(() => 'modelDraft.effortMapping')!
  const levels = fieldState<ThinkingLevel[]>(() => 'modelDraft.thinkingLevels')!
  let enabled = $derived((capabilities.value ?? []).includes('reasoning') || (levels.value ?? []).length > 0)
</script>

<section class="grid min-w-0 gap-2" aria-labelledby={id}>
  <h2 {id} class="px-4 text-xsm font-medium">Thinking efforts</h2>
  <div class="grid gap-3 rounded-control bg-inset p-3">
    <div class="flex min-h-6 items-center gap-1">
      <label class="flex cursor-pointer items-center gap-2 text-xsm font-medium">
        <Checkbox
          size="compact"
          aria-label="Enable thinking efforts"
          checked={enabled}
          onchange={(event) => {
            const next = (capabilities.value ?? []).filter((capability) => capability !== 'reasoning')
            capabilities.change(event.currentTarget.checked ? [...next, 'reasoning'] : next)
            if (!event.currentTarget.checked) {
              mapping.change({})
              levels.change([])
            }
          }}
        ></Checkbox>Enable thinking
      </label><FieldHelp label="About thinking efforts">
        Enable only if this model supports reasoning. Map each level to an effort value accepted by the provider. Leave
        unsupported levels empty. Higher effort can use more time and tokens; enabling this setting cannot add reasoning support
        to the upstream model. Turning it off clears the effort mappings.
      </FieldHelp>
    </div>
    {#if enabled}<div>
        <Skeleton><EffortMapping name="modelDraft.effortMapping" variant="compact"></EffortMapping></Skeleton><FormError
          name="modelDraft.effortMapping"
        ></FormError>
      </div>{:else}{/if}
  </div>
</section>
