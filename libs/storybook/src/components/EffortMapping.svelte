<script lang="ts">
  import { untrack, tick } from 'svelte'
  import { fieldModel } from './form-context.svelte'
  import TextField from './TextField.svelte'
  import { ProviderInputRules, ProviderInputRulesInput, ProviderInputRulesConstraints } from '@compallm/contract/provider'
  import FieldHelp from './FieldHelp.svelte'
  import DisabledTooltip from './DisabledTooltip.svelte'
  import CaretLeftIcon from 'phosphor-svelte/lib/CaretLeftIcon'
  import CaretRightIcon from 'phosphor-svelte/lib/CaretRightIcon'
  import { cx } from './shared'
  import { interaction } from './motion'
  import { effortLevels, type EffortLevel, type EffortValues } from './effort-mapping'
  type Level = EffortLevel
  type Values = EffortValues
  let {
    name,
    value = $bindable<Values>({}),
    onValueChange,
    disabled = false,
    variant = 'default',
    defaultSelected = 'low',
    defaultFocused,
  }: {
    name?: string
    value?: Values
    onValueChange?: (value: Values) => void
    disabled?: boolean
    variant?: 'default' | 'compact'
    defaultSelected?: Level
    defaultFocused?: Level
  } = $props()
  const form = fieldModel(() => name)
  let selection = $state<Level>(untrack(() => defaultFocused ?? defaultSelected))
  let focused = $state<Level | null>(untrack(() => defaultFocused ?? null))
  const inputs: HTMLInputElement[] = []
  let rail = $state<SVGSVGElement>()
  let pointer: number | null = null
  const id = $props.id()
  let values = $derived(form && name ? ((form.get(name) as Values) ?? {}) : value)
  let firstEmpty = $derived(effortLevels.findIndex((level) => !values[level]?.trim()))
  let lastAvailable = $derived(firstEmpty < 0 ? 4 : firstEmpty)
  let index = $derived(
    variant === 'compact' ? effortLevels.indexOf(selection) : Math.min(effortLevels.indexOf(selection), lastAvailable),
  )
  let selected = $derived(effortLevels[index])
  function change(level: Level, text: string) {
    if (!ProviderInputRulesInput.effortValue(text)) return
    const next = { ...values }
    if (text) next[level] = text
    else for (const later of effortLevels.slice(effortLevels.indexOf(level))) delete next[later]
    if (form && name) form.change(name, next)
    else {
      value = next
      onValueChange?.(next)
    }
  }
  async function choose(position: number, edit = false) {
    const next = Math.max(0, Math.min(variant === 'compact' ? 4 : lastAvailable, position))
    selection = effortLevels[next]
    if (edit || variant === 'compact') {
      focused = selection
      await tick()
      inputs[next]?.focus()
    }
  }
  function drag(clientX: number) {
    const bounds = rail?.getBoundingClientRect()
    if (bounds?.width) void choose(Math.round((((clientX - bounds.left) / bounds.width) * 560 - 32) / 124), true)
  }
</script>

{#if variant === 'compact'}<div
    class="flex min-h-10 min-w-0 items-center gap-2"
    data-form-field={name}
    aria-invalid={form && name ? form.invalid(name) || undefined : undefined}
  >
    <DisabledTooltip disabled={disabled || index === 0} reason="This is the first effort level.">
      <button
        type="button"
        aria-label="Previous effort level"
        disabled={disabled || index === 0}
        onclick={() => choose(index - 1)}
        class="flex size-7 shrink-0 cursor-pointer items-center justify-center rounded-lg bg-elevated text-muted-foreground hover:bg-hover hover:text-foreground disabled:opacity-30 focus-visible:outline-ring"
        use:interaction
      >
        <CaretLeftIcon class="size-3.5" />
      </button>
    </DisabledTooltip>
    <div class="relative grid min-h-10 min-w-0 flex-1 grid-cols-5 gap-1 overflow-hidden rounded-lg px-1 pb-2">
      <div
        aria-hidden="true"
        data-effort-glow
        class="pointer-events-none absolute inset-x-0 bottom-0 bg-linear-to-r from-(--effort-low) via-(--effort-medium) to-(--effort-high) blur-sm"
        style={`height:${4 + index * 0.5}px;opacity:${0.18 + index * 0.02}`}
      ></div>
      <div
        aria-hidden="true"
        class="pointer-events-none absolute inset-x-0 bottom-0 h-0.5 bg-linear-to-r from-(--effort-low) via-(--effort-medium) to-(--effort-high) opacity-70"
      ></div>
      {#each effortLevels as level, i (level)}<div class="relative z-1 flex min-w-0 items-center justify-center">
          {#if i === index}<TextField
              bind:element={inputs[i]}
              aria-label={`${level} effort mapping`}
              value={values[level] ?? ''}
              placeholder={level}
              {disabled}
              autofocus={defaultFocused === level}
              maxlength={ProviderInputRulesConstraints.effortValue.maxLength}
              class="h-7 w-full rounded-lg border-ring/50 bg-elevated px-1 text-center text-xs"
              oninput={(e) => change(level, e.currentTarget.value)}
              onblur={() => name && form?.blur(name)}
              onkeydown={(e) => {
                if (e.key === 'Tab') {
                  const next = index + (e.shiftKey ? -1 : 1)
                  if (next >= 0 && next <= 4) {
                    e.preventDefault()
                    void choose(next)
                  }
                }
                if ((e.ctrlKey || e.metaKey) && (e.key === 'ArrowLeft' || e.key === 'ArrowRight')) {
                  e.preventDefault()
                  void choose(index + (e.key === 'ArrowLeft' ? -1 : 1))
                }
              }}
            />{:else}<DisabledTooltip {disabled} reason="Editing thinking efforts is unavailable.">
              <button
                type="button"
                {disabled}
                aria-label={`Edit ${level} effort mapping`}
                onfocus={() => choose(i)}
                onclick={() => choose(i)}
                class="h-7 w-full min-w-0 truncate rounded-lg text-xs text-muted-foreground hover:bg-hover disabled:cursor-not-allowed disabled:opacity-40 focus-visible:outline-ring"
              >
                {values[level] || '\u00a0'}
              </button>
            </DisabledTooltip>{/if}
        </div>{/each}
    </div>
    <DisabledTooltip disabled={disabled || index === 4} reason="This is the last effort level.">
      <button
        type="button"
        aria-label="Next effort level"
        disabled={disabled || index === 4}
        onclick={() => choose(index + 1)}
        class="flex size-7 shrink-0 cursor-pointer items-center justify-center rounded-lg bg-elevated text-muted-foreground hover:bg-hover hover:text-foreground disabled:opacity-30 focus-visible:outline-ring"
        use:interaction
      >
        <CaretRightIcon class="size-3.5" />
      </button>
    </DisabledTooltip>
  </div>
{:else}<fieldset
    {disabled}
    data-form-field={name}
    class="w-full min-w-0"
    onfocusout={(e) => {
      if (!e.currentTarget.contains(e.relatedTarget as Node) && name) form?.blur(name)
    }}
  >
    <legend class="mb-2 text-sm font-medium">
      <span class="inline-flex items-center gap-1">
        Thinking efforts<FieldHelp label="About thinking efforts">
          Map Low → Max to the thinking-effort values your model accepts (for example, low, medium, high). Higher effort can use
          more time and tokens. Use up to {ProviderInputRulesConstraints.effortValue.maxLength} letters, digits, dashes, or underscores,
          without spaces. Fill levels in order; leave unused levels empty.
        </FieldHelp>
      </span>
    </legend>
    <div
      class="relative h-16 min-w-0"
      onfocusout={(e) => {
        if (!e.currentTarget.contains(e.relatedTarget as Node)) focused = null
      }}
      role="group"
      aria-label="Effort values"
    >
      <div class="absolute inset-x-0 top-0 grid grid-cols-5 gap-1 sm:gap-3">
        {#each effortLevels as level, i (level)}{@const current = level === selected}{@const editing =
            focused === level}{@const mapped = values[level] ?? ''}
          <div class="relative flex min-w-0 flex-col items-center">
            <TextField
              bind:element={inputs[i]}
              aria-label={`${level} effort mapping`}
              value={mapped}
              disabled={disabled || i > lastAvailable}
              autofocus={defaultFocused === level}
              placeholder="None"
              maxlength={ProviderInputRulesConstraints.effortValue.maxLength}
              aria-invalid={(mapped !== '' && !ProviderInputRules.shape.effortValue.safeParse(mapped).success) || undefined}
              size={Math.max(4, mapped.length || 4)}
              class={cx(
                'h-6 w-auto min-w-10 max-w-full text-center text-xs leading-4 field-sizing-content',
                !editing && !current && 'border-transparent bg-transparent text-center placeholder:text-transparent',
                (editing || current) &&
                  'border border-muted-foreground/80 bg-inset focus:border focus:border-muted-foreground/80',
              )}
              onfocus={() => {
                focused = level
                selection = level
              }}
              onblur={() => (focused = null)}
              oninput={(e) => change(level, e.currentTarget.value)}
              onkeydown={(e) => {
                if (e.key === 'Enter') {
                  e.preventDefault()
                  focused = null
                  e.currentTarget.blur()
                }
              }}
            />{#if current}<svg
                aria-hidden="true"
                viewBox="0 0 8 6"
                class="absolute top-full left-1/2 h-1.5 w-2 -translate-x-1/2 fill-primary"
              >
                <path d="M0 0H8L4 6Z" />
              </svg>{/if}
          </div>{/each}
      </div>
      <div class="absolute inset-x-0 top-7 flex items-center gap-1">
        <button
          type="button"
          disabled={disabled || index === 0}
          aria-label="Previous effort level"
          class="flex size-6 shrink-0 cursor-pointer items-center justify-center text-foreground focus-visible:outline-muted-foreground disabled:opacity-30"
          onclick={() => choose(index - 1, true)}
          use:interaction
        >
          <CaretLeftIcon weight="fill" class="size-4" />
        </button><svg
          bind:this={rail}
          viewBox="0 0 560 14"
          preserveAspectRatio="none"
          class="h-3.5 min-w-0 flex-1"
          aria-hidden="true"
        >
          <defs>
            <linearGradient id={`${id}-gradient`} x1="0" x2="1">
              <stop stop-color="var(--effort-low)" /><stop offset="58%" stop-color="var(--effort-medium)" /><stop
                offset="100%"
                stop-color="var(--effort-high)"
              />
            </linearGradient>
          </defs><path
            d="M2 5C136 4.8 360 3.5 550 2C556 2 560 3.8 560 7C560 10.2 556 12 550 12C360 10.5 136 9.2 2 9Q0 9 0 7Q0 5 2 5Z"
            fill={`url(#${id}-gradient)`}
          />
        </svg><button
          type="button"
          disabled={disabled || index === lastAvailable}
          aria-label="Next effort level"
          class="flex size-6 shrink-0 cursor-pointer items-center justify-center text-foreground focus-visible:outline-muted-foreground disabled:opacity-30"
          onclick={() => choose(index + 1, true)}
          use:interaction
        >
          <CaretRightIcon weight="fill" class="size-4" />
        </button>
      </div>
      <div
        role="slider"
        tabindex={disabled ? -1 : 0}
        aria-label="Selected effort level"
        aria-valuemin={0}
        aria-valuemax={lastAvailable}
        aria-valuenow={index}
        aria-valuetext={selected}
        aria-disabled={disabled || undefined}
        class="absolute inset-x-7 top-7 h-6 touch-none cursor-ew-resize rounded focus-visible:outline-1 focus-visible:outline-muted-foreground"
        onpointerdown={(e) => {
          if (disabled) return
          e.preventDefault()
          pointer = e.pointerId
          e.currentTarget.setPointerCapture(e.pointerId)
          drag(e.clientX)
        }}
        onpointermove={(e) => {
          if (pointer === e.pointerId) drag(e.clientX)
        }}
        onpointerup={() => {
          pointer = null
          focused = selected
          inputs[index]?.focus()
        }}
        onpointercancel={() => (pointer = null)}
        onkeydown={(e) => {
          if (disabled) return
          const delta = e.key === 'ArrowLeft' ? -1 : e.key === 'ArrowRight' ? 1 : 0
          if (delta || e.key === 'Home' || e.key === 'End') {
            e.preventDefault()
            void choose(e.key === 'Home' ? 0 : e.key === 'End' ? 4 : index + delta, true)
          }
        }}
      ></div>
    </div>
  </fieldset>{/if}
