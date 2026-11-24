<script lang="ts">
  import { Skeleton } from '../../../components/loading'

  import { effectiveContextTokens } from './model-options'

  import { readStore } from '../../page-state.svelte'
  import { getCreateProviderController } from '../../../context'

  import { modelTones } from './model-sections'

  const __provider = getCreateProviderController()
  const id = $props.id()
  let values = $derived(
    readStore(
      __provider.store,
      (s) => ({
        context: s.modelDraft ? effectiveContextTokens(s.modelDraft) : undefined,
        input: s.modelDraft?.maxInputTokens,
        output: s.modelDraft?.maxOutputTokens,
        inputEnabled: s.modelDraft?.inputLimitEnabled,
        outputEnabled: s.modelDraft?.outputLimitEnabled,
      }),
      [
        'modelDraft.maxContextTokens',
        'modelDraft.contextLimitEnabled',
        'modelDraft.providerContextTokens',
        'modelDraft.maxInputTokens',
        'modelDraft.maxOutputTokens',
        'modelDraft.inputLimitEnabled',
        'modelDraft.outputLimitEnabled',
      ],
    ),
  )
  let total = $derived(Math.max(1, Number(values.context) || 1))
  let output = $derived(Math.min(total, values.outputEnabled ? Number(values.output) || 0 : Math.round(total * 0.03)))
  let input = $derived(Math.min(total - output, values.inputEnabled ? Number(values.input) || 0 : Math.round(total * 0.67)))
  let free = $derived(Math.max(0, total - input - output))
</script>

<div class="grid min-w-0 gap-2.5 px-1">
  <Skeleton>
    <svg viewBox="0 0 608 12" preserveAspectRatio="none" aria-hidden="true" class="h-3 w-full rounded-sm">
      <defs>
        <pattern id={`${id}-dots`} width="10" height="5" patternUnits="userSpaceOnUse">
          <circle cx="2" cy="2" r="0.5" class="fill-muted-foreground/50"></circle>
        </pattern><pattern id={`${id}-hatch`} width="5" height="12" patternUnits="userSpaceOnUse">
          <path d="M0 12 6 0" fill="none" stroke="currentColor" stroke-width="1"></path>
        </pattern>
      </defs><rect width="608" height="12" class="fill-foreground/10"></rect><rect
        width={(608 * input) / total}
        height="12"
        class="fill-muted-foreground/55"
      ></rect><rect x={(608 * input) / total} width={(608 * free) / total} height="12" fill={`url(#${id}-dots)`}></rect><rect
        x={(608 * (input + free)) / total}
        width={(608 * output) / total}
        height="12"
        class="fill-muted-foreground/20"
      ></rect><rect
        x={(608 * (input + free)) / total}
        width={(608 * output) / total}
        height="12"
        fill={`url(#${id}-hatch)`}
        class="text-muted-foreground/65"
      ></rect>
    </svg>
  </Skeleton>
  <div class="grid grid-cols-3 gap-2 text-ui-11 tabular-nums">
    <span class={modelTones.input}>Input · {input.toLocaleString()}</span><span class="text-muted-foreground">
      Available · {free.toLocaleString()}
    </span><span class={modelTones.output}>Output · {output.toLocaleString()}</span>
  </div>
</div>
