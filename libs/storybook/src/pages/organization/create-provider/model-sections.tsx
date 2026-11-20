import { useId, type ReactNode } from 'react'
import { TrashIcon } from '@phosphor-icons/react/dist/csr/Trash'
import { PlusIcon } from '@phosphor-icons/react/dist/csr/Plus'
import { Popup } from '../../../components/popup'
import { TextField } from '../../../components/fields'
import { Checkbox } from '../../../components/selection'
import { RangeSlider } from '../../../components/range-slider'
import { CapabilityIcon } from '../../../components/capability-icon'
import { FieldHelp } from '../../../components/feedback'
import { FormError, useFormField } from '../../../components/form'
import {
  capabilityOptions,
  effectiveContextTokens,
  effectiveModelPrice,
  priceEnabledFields,
  type ModelPriceField,
} from './model-options'
import { useProvider } from './controller'
import type { ModelCapability } from './types'

export const modelTones = {
  context: 'text-[color-mix(in_srgb,var(--warning)_45%,var(--foreground))]',
  input: 'text-[color-mix(in_srgb,color-mix(in_srgb,var(--ring)_50%,var(--success))_45%,var(--foreground))]',
  output: 'text-[color-mix(in_srgb,var(--primary)_45%,var(--foreground))]',
}

export function ModelSection({
  title,
  children,
  action,
  description,
  help,
}: {
  title: string
  children: ReactNode
  action?: ReactNode
  description?: string
  help?: ReactNode
}) {
  const id = useId()
  return (
    <section aria-labelledby={id} className="grid min-w-0 gap-2">
      <div className="flex min-h-6 items-center justify-between gap-3 px-4">
        <div className="flex min-w-0 items-center gap-1">
          <h2 id={id} className="text-xsm font-medium">
            {title}
          </h2>
          {help ? <FieldHelp label={`About ${title.toLowerCase()}`}>{help}</FieldHelp> : null}
        </div>
        {action}
      </div>
      {description ? <p className="px-4 text-xs text-muted-foreground">{description}</p> : null}
      {children}
    </section>
  )
}

export function ModelContext() {
  return (
    <ModelSection
      title="Context windows"
      help={
        <div className="grid gap-2">
          <p>
            A token is a piece of text. The context window is the total space for a request and its reply, including
            instructions and conversation history.
          </p>
          <p>
            Actual input + actual output must fit inside that window. With 128k context, a 16k reply leaves at most 112k for
            input.
          </p>
          <p>
            Input and output limits are separate ceilings. Their configured values do not have to add up to the context window.
          </p>
          <p>
            Uncheck an override to use the provider default. Increasing a value cannot expand the upstream model’s real
            capacity.
          </p>
        </div>
      }
    >
      <div className="grid min-w-0 gap-3 rounded-control bg-inset px-3 py-4">
        <ContextExample />
        <div className="mx-1 border-t border-divider" />
        <LimitRow kind="context" label="Context window" valueKey="maxContextTokens" enabledKey="contextLimitEnabled" />
        <div className="mx-1 border-t border-divider" />
        <LimitRow kind="input" label="Input limit" valueKey="maxInputTokens" enabledKey="inputLimitEnabled" />
        <div className="mx-1 border-t border-divider" />
        <LimitRow kind="output" label="Output limit" valueKey="maxOutputTokens" enabledKey="outputLimitEnabled" />
      </div>
    </ModelSection>
  )
}

function ContextExample() {
  const id = useId()
  const values = useProvider(
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
  )
  const total = Math.max(1, Number(values.context) || 250000)
  const output = Math.min(total, values.outputEnabled ? Number(values.output) || 0 : Math.round(total * 0.03))
  const input = Math.min(total - output, values.inputEnabled ? Number(values.input) || 0 : Math.round(total * 0.67))
  const free = Math.max(0, total - input - output)
  return (
    <div className="grid min-w-0 gap-2.5 px-1">
      <div className="flex items-center gap-1">
        <p className="text-ui-11 text-muted-foreground">Example usage</p>
        <FieldHelp label="About the context example">
          This is an illustration, not live usage. Solid input, hatched output, and dotted available space share the total
          context. Changing a limit updates the example; it does not reserve a fixed split for every request.
        </FieldHelp>
      </div>
      <svg viewBox="0 0 608 8" preserveAspectRatio="none" aria-hidden="true" className="h-2 w-full rounded-sm">
        <defs>
          <pattern id={`${id}-dots`} width="10" height="5" patternUnits="userSpaceOnUse">
            <circle cx="2" cy="2" r="0.5" className="fill-muted-foreground/50" />
          </pattern>
          <pattern id={`${id}-hatch`} width="5" height="8" patternUnits="userSpaceOnUse">
            <path d="M0 8 4 0" fill="none" stroke="currentColor" strokeWidth="1" />
          </pattern>
        </defs>
        <rect width="608" height="8" className="fill-foreground/10" />
        <rect
          width={(608 * input) / total}
          height="8"
          className="fill-[color-mix(in_srgb,color-mix(in_srgb,var(--ring)_50%,var(--success))_65%,var(--muted-foreground))]"
        />
        <rect x={(608 * input) / total} width={(608 * free) / total} height="8" fill={`url(#${id}-dots)`} />
        <rect x={(608 * (input + free)) / total} width={(608 * output) / total} height="8" className="fill-primary/30" />
        <rect
          x={(608 * (input + free)) / total}
          width={(608 * output) / total}
          height="8"
          fill={`url(#${id}-hatch)`}
          className={modelTones.output}
        />
      </svg>
      <div className="grid grid-cols-3 gap-2 text-ui-11 tabular-nums">
        <span className={modelTones.input}>Input · {input.toLocaleString()}</span>
        <span className="text-muted-foreground">Available · {free.toLocaleString()}</span>
        <span className={modelTones.output}>Output · {output.toLocaleString()}</span>
      </div>
    </div>
  )
}

function LimitRow({
  kind,
  label,
  valueKey,
  enabledKey,
}: {
  kind: keyof typeof modelTones
  label: string
  valueKey: string
  enabledKey: string
}) {
  const id = useId()
  const field = useFormField<string>(`modelDraft.${valueKey}`)!
  const enabled = useFormField<boolean>(`modelDraft.${enabledKey}`)!
  const context = useProvider(
    (s) => (s.modelDraft ? effectiveContextTokens(s.modelDraft) : undefined),
    ['modelDraft.maxContextTokens', 'modelDraft.contextLimitEnabled', 'modelDraft.providerContextTokens'],
  )
  const maximum = kind === 'context' ? Math.max(1000000, Number(context) || 0) : Math.min(1000000, Number(context) || 1000000)
  const help = {
    context: `
The total token budget shared by the request and reply. Uncheck to use the provider’s context value. If you
reduce it below an enabled input or output cap, adjust that cap to resume autosave; entered values are not changed
automatically.
`,
    input: `
The largest request you allow, including instructions, messages, and tool definitions. Enable an override from
100 to 1,000,000 tokens, no greater than context. A long reply may require a shorter actual input. Uncheck to
use the provider default.
`,
    output: `
The most tokens allowed in a reply. Enable an override from 100 to 1,000,000 tokens, no greater than context.
The reply also has to fit the space left after the actual input. Uncheck to use the provider default.
`,
  }[kind]
  return (
    <div className="grid min-w-0 gap-1">
      <div className="flex h-8 min-w-0 items-center justify-between gap-3">
        <div className="flex min-w-0 items-center gap-0.5">
          <label
            htmlFor={id}
            className={`flex min-w-0 cursor-pointer items-center gap-1.5 text-xs sm:gap-2 sm:text-xsm ${modelTones[kind]}`}
          >
            <Checkbox
              id={id}
              size="compact"
              checked={!!enabled.value}
              onChange={(e) => {
                if (e.target.checked && !field.value)
                  field.change(String(Math.min(kind === 'output' ? 32768 : 1000000, maximum)))
                enabled.change(e.target.checked)
              }}
            />
            {label}
          </label>
          <FieldHelp label={`About ${label.toLowerCase()}`}>{help}</FieldHelp>
        </div>
        {enabled.value ? (
          <div className="relative w-36 shrink-0 sm:w-40.5" data-form-field={`modelDraft.${valueKey}`}>
            <TextField
              aria-label={`${label} tokens`}
              aria-invalid={field.invalid || undefined}
              aria-describedby={`modelDraft.${valueKey}-error`}
              value={(field.value ?? '').replace(/\B(?=(\d{3})+(?!\d))/g, ',')}
              onChange={(event) => field.change(event.target.value.replaceAll(',', ''))}
              onBlur={field.blur}
              inputMode="numeric"
              className="rounded-lg pr-12"
            />
            <span
              aria-hidden="true"
              className="pointer-events-none absolute inset-y-0 right-2 flex items-center text-ui-11 text-muted-foreground"
            >
              tokens
            </span>
            <FormError name={`modelDraft.${valueKey}`} />
          </div>
        ) : (
          <span className="text-xs text-muted-foreground">Provider default</span>
        )}
      </div>
      {enabled.value ? (
        <RangeSlider
          label={`${label} capacity`}
          min={100}
          max={Math.max(101, maximum)}
          value={Number(field.value) || 100}
          onValueChange={(value) => field.change(String(value))}
          onBlur={field.blur}
          className={`pl-7 pr-2 ${modelTones[kind]}`}
        />
      ) : null}
    </div>
  )
}

export function ModelPricing() {
  return (
    <ModelSection
      title="Custom Pricing"
      action={<span className="text-xs text-muted-foreground">USD / 1M tokens</span>}
      help={
        <div className="grid gap-2">
          <p>
            These rates describe the model’s input, cached-input, and output costs in US dollars per one million tokens. They do
            not change what the provider charges.
          </p>
          <p>
            Enable a row to set a custom rate from $0 to $10,000. An explicit $0 means free. Uncheck it to use the retained
            template rate; an unknown rate stays unset and pauses autosave.
          </p>
          <p>
            For example, 1,000 input tokens at $2 per million cost $0.002 before output costs. Comparison hints are calculated;
            they do not link or change the prices.
          </p>
        </div>
      }
    >
      <div className="grid min-w-0 gap-3 rounded-control bg-inset p-4">
        <div className="grid gap-2">
          <h3 className={`text-xs ${modelTones.input}`}>Input</h3>
          <PriceRow label="Standard" name="inputPrice" />
          <PriceRow label="Cache read" name="cachedPrice" />
        </div>
        <div className="border-t border-divider" />
        <div className="grid gap-2">
          <h3 className={`text-xs ${modelTones.output}`}>Output</h3>
          <PriceRow label="Generated" name="outputPrice" />
        </div>
      </div>
    </ModelSection>
  )
}

function PriceRow({ label, name }: { label: string; name: ModelPriceField }) {
  const id = useId()
  const enabled = useFormField<boolean>(`modelDraft.${priceEnabledFields[name]}`)!
  const template = useProvider((state) => state.modelDraft?.templatePrices?.[name], ['modelDraft.templatePrices'])
  const help = {
    inputPrice: `
The rate for new input tokens that are not billed as cache reads. Enter US dollars per one million tokens.
Enable to override the template; uncheck to use its rate. Zero is an explicit free rate.
`,
    cachedPrice: `
The rate for input tokens that the provider reports as served from its prompt cache. A cheaper rate does not
enable caching by itself. Uncheck to use the template rate; unknown or unsupported is not the same as $0.
`,
    outputPrice: `
The rate for tokens generated by the model. Provider billing may also include reasoning tokens. This rate is
independent of the input price. Uncheck to use the template rate.
`,
  }[name]
  return (
    <div className="grid min-h-8 min-w-0 grid-cols-[minmax(0,1fr)_10.125rem] items-center gap-3">
      <div className="flex min-w-0 items-center gap-3">
        <div className="flex min-w-0 items-center gap-0.5">
          <label htmlFor={id} className="flex min-w-0 cursor-pointer items-center gap-1.5 text-xs sm:gap-2 sm:text-xsm">
            <Checkbox id={id} size="compact" name={`modelDraft.${priceEnabledFields[name]}`} />
            <span className="truncate">{label}</span>
          </label>
          <FieldHelp label={`About ${label.toLowerCase()} pricing`}>{help}</FieldHelp>
        </div>
        <PriceComparison name={name} />
      </div>
      <div className="relative min-w-0" data-form-field={`modelDraft.${name}`}>
        {enabled.value !== false ? (
          <>
            <span
              aria-hidden="true"
              className="pointer-events-none absolute inset-y-0 left-2 z-1 flex items-center text-xsm text-muted-foreground"
            >
              $
            </span>
            <TextField
              name={`modelDraft.${name}`}
              aria-label={`${label} price`}
              inputMode="decimal"
              required
              placeholder="Enter rate"
              className="rounded-lg pl-6.5"
            />
          </>
        ) : (
          <span
            className="flex h-8 items-center justify-end text-xs text-muted-foreground"
            title={template === undefined || template === '' ? 'No template rate is available.' : `$${template} per 1M tokens`}
          >
            {template === undefined || template === '' ? 'Not set' : 'Provider default'}
          </span>
        )}
        <FormError name={`modelDraft.${name}`} />
      </div>
    </div>
  )
}

function PriceComparison({ name }: { name: ModelPriceField }) {
  const values = useProvider(
    (s) =>
      s.modelDraft
        ? { input: effectiveModelPrice(s.modelDraft, 'inputPrice'), rate: effectiveModelPrice(s.modelDraft, name) }
        : {},
    [
      'modelDraft.inputPrice',
      `modelDraft.${name}`,
      'modelDraft.inputPriceEnabled',
      `modelDraft.${priceEnabledFields[name]}`,
      'modelDraft.templatePrices',
    ],
  )
  const base = Number(values.input),
    rate = Number(values.rate)
  const hint =
    values.input && values.rate && base > 0 && Number.isFinite(rate)
      ? name === 'cachedPrice'
        ? `${Math.round(Math.abs(1 - rate / base) * 100)}% ${rate <= base ? 'less' : 'more'}`
        : name === 'outputPrice'
          ? `${Number((rate / base).toFixed(2))}× input`
          : ''
      : ''
  return <span className="hidden shrink-0 text-ui-11 text-muted-foreground sm:block">{hint}</span>
}

export const capabilityDescriptions: Record<ModelCapability, string> = {
  text: 'Understand and generate text.',
  vision: 'Understand images in a request.',
  pdf: 'Read PDF documents.',
  audio: 'Understand audio in a request.',
  video: 'Understand video in a request.',
  image_generation: 'Generate images.',
  audio_generation: 'Generate audio.',
  search: 'Search the web for current information.',
  reasoning: 'Use additional reasoning before responding.',
  tool_call: 'Call functions and external tools.',
  forced_tool_choice: 'Require a specific tool to be called.',
  structured_output: 'Return responses matching a schema.',
}

export function ModelCapabilities() {
  const field = useFormField<ModelCapability[]>('modelDraft.capabilities')!
  const selected = field.value ?? []
  return (
    <ModelSection
      title="Capabilities"
      help={`
Declare the features this model actually supports, such as images or tool calling. These labels help discovery
and filtering; adding a capability does not enable it at the provider. Use the provider’s model documentation to
confirm support.
`}
      action={
        <Popup.Root>
          <Popup.Trigger className="flex h-8 w-auto items-center gap-1 rounded-lg bg-elevated px-2 text-xs">
            <PlusIcon className="size-3" />
            Add capability
          </Popup.Trigger>
          <Popup.Content align="start" className="w-80 text-left" aria-label="Add capability">
            {capabilityOptions.map((option) => (
              <Popup.Item
                key={option.value}
                disabled={selected.includes(option.value)}
                className="h-auto min-h-10 justify-start gap-2 py-1.5 text-left"
                onSelect={() => field.change([...selected, option.value])}
              >
                <CapabilityIcon capability={option.value} className="size-3.5" />
                <span className="min-w-0">
                  <span className="block">{option.label}</span>
                  <span className="block text-ui-11 text-muted-foreground">{capabilityDescriptions[option.value]}</span>
                </span>
              </Popup.Item>
            ))}
          </Popup.Content>
        </Popup.Root>
      }
    >
      <div className="divide-y divide-divider rounded-control bg-inset">
        {selected.length ? (
          selected.map((value) => (
            <div key={value} className="flex min-h-14 items-center justify-between gap-3 px-4 py-2">
              <div className="flex min-w-0 items-center gap-3">
                <CapabilityIcon capability={value} className="size-4" />
                <div className="min-w-0">
                  <p className="text-xsm">{capabilityOptions.find((o) => o.value === value)?.label}</p>
                  <p className="text-xs text-muted-foreground">{capabilityDescriptions[value]}</p>
                </div>
              </div>
              <button
                type="button"
                aria-label={`Remove ${capabilityOptions.find((o) => o.value === value)?.label}`}
                className={`
flex size-6 shrink-0 items-center justify-center rounded text-muted-foreground hover:bg-hover
focus-visible:outline-ring
`}
                onClick={() => field.change(selected.filter((item) => item !== value))}
              >
                <TrashIcon className="size-3" />
              </button>
            </div>
          ))
        ) : (
          <p className="px-4 py-5 text-xs text-muted-foreground">No capabilities added.</p>
        )}
      </div>
    </ModelSection>
  )
}
