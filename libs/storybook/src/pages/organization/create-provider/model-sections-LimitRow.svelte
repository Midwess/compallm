<script lang="ts">
  import { ModelTemplateLimitsInputConstraints } from '@compallm/contract/validation'

  import { TextField } from '../../../components/fields'
  import { Checkbox } from '../../../components/selection'
  import { RangeSlider } from '../../../components/range-slider'

  import { FieldHelp } from '../../../components/feedback'
  import { FormError } from '../../../components/form'

  import { fieldState } from '../../page-state.svelte'

  import { modelTones } from './model-sections'
  import ContentValue from '../../../components/Content.svelte'

  let {
    kind,
    label,
    valueKey,
    enabledKey,
  }: {
    kind: keyof typeof modelTones
    label: string
    valueKey: string
    enabledKey: string
  } = $props()

  const id = $props.id()
  const field = fieldState<string>(() => `modelDraft.${valueKey}`)!
  const enabled = fieldState<boolean>(() => `modelDraft.${enabledKey}`)!
  let minimum = $derived(ModelTemplateLimitsInputConstraints.context.minimum)
  let maximum = $derived(ModelTemplateLimitsInputConstraints.context.maximum)
  let help = $derived(
    {
      context: `
The total token budget shared by the request and reply. Uncheck to use the provider’s context value. If you
reduce it below an enabled input or output cap, adjust that cap to resume autosave; entered values are not changed
automatically.
`,
      input: `
The largest request you allow, including instructions, messages, and tool definitions. Enable an override from
100 to 10,000,000 tokens, no greater than context. A long reply may require a shorter actual input. Uncheck to
use the provider default.
`,
      output: `
The most tokens allowed in a reply. Enable an override from 100 to 10,000,000 tokens, no greater than context.
The reply also has to fit the space left after the actual input. Uncheck to use the provider default.
`,
    }[kind],
  )
</script>

<div class="grid min-w-0 gap-1">
  <div class="flex h-8 min-w-0 items-center justify-between gap-3">
    <div class="flex min-w-0 items-center gap-0.5">
      <label
        for={id}
        class={`flex min-w-0 cursor-pointer items-center gap-1.5 text-xs sm:gap-2 sm:text-xsm ${modelTones[kind]}`}
      >
        <Checkbox
          {id}
          size="compact"
          checked={!!enabled.value}
          onchange={(e) => {
            if (e.currentTarget.checked && !field.value)
              field.change(String(Math.min(kind === 'output' ? 32768 : 1000000, maximum)))
            enabled.change(e.currentTarget.checked)
          }}
        ></Checkbox><ContentValue value={label} />
      </label><FieldHelp label={`About ${label.toLowerCase()}`}>{help}</FieldHelp>
    </div>
    {#if enabled.value}<div class="relative w-36 shrink-0 sm:w-40.5" data-form-field={`modelDraft.${valueKey}`}>
        <TextField
          aria-label={`${label} tokens`}
          aria-invalid={field.invalid || undefined}
          aria-describedby={`modelDraft.${valueKey}-error`}
          value={(field.value ?? '').replace(/\B(?=(\d{3})+(?!\d))/g, ',')}
          oninput={(event) => field.change(event.currentTarget.value.replaceAll(',', ''))}
          onblur={field.blur}
          inputmode="numeric"
          class="rounded-lg pr-12"
        ></TextField><span
          aria-hidden="true"
          class="pointer-events-none absolute inset-y-0 right-2 flex items-center text-ui-11 text-muted-foreground"
        >
          tokens
        </span><FormError name={`modelDraft.${valueKey}`}></FormError>
      </div>{:else}<span class="text-xs text-muted-foreground">Provider default</span>{/if}
  </div>
  {#if enabled.value}<RangeSlider
      label={`${label} capacity`}
      min={minimum}
      max={maximum}
      value={Number(field.value) || 100}
      onValueChange={(value) => field.change(String(value))}
      onBlur={field.blur}
      onValueCommit={field.blur}
      class={`pl-7 pr-2 ${modelTones[kind]}`}
    ></RangeSlider>{:else}{/if}
</div>
