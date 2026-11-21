import { useFormField } from './form'
import { animated } from './motion'
import { ProviderInputRules, ProviderInputRulesInput, ProviderInputRulesConstraints } from '@compallm/contract/provider'
import { useEffect, useId, useRef, useState } from 'react'
import { CaretLeftIcon } from '@phosphor-icons/react/dist/csr/CaretLeft'
import { CaretRightIcon } from '@phosphor-icons/react/dist/csr/CaretRight'
import { TextField } from './fields'
import { FieldHelp, DisabledTooltip } from './feedback'
import { cx } from './shared'

export const effortLevels = ['low', 'medium', 'high', 'xhigh', 'max'] as const
export type EffortLevel = (typeof effortLevels)[number]
export type EffortValues = Partial<Record<EffortLevel, string>>
export interface EffortMappingProps {
  variant?: 'default' | 'compact'
  name?: string
  value?: EffortValues
  onValueChange?(value: EffortValues): void
  disabled?: boolean
  defaultSelected?: EffortLevel
  defaultFocused?: EffortLevel
}
function DefaultEffortMapping({
  name,
  value: suppliedValue,
  onValueChange: suppliedChange,
  disabled = false,
  defaultSelected = 'low',
  defaultFocused,
}: EffortMappingProps) {
  const field = useFormField<EffortValues>(name)
  const value = field ? (field.value ?? {}) : (suppliedValue ?? {})
  const onValueChange = (value: EffortValues) => {
    if (field) field.change(value)
    else suppliedChange?.(value)
  }
  const firstEmpty = effortLevels.findIndex((level) => !value[level]?.trim())
  const lastAvailable = firstEmpty === -1 ? effortLevels.length - 1 : firstEmpty
  const [selection, select] = useState<EffortLevel>(defaultFocused ?? defaultSelected)
  const selected = effortLevels[Math.min(effortLevels.indexOf(selection), lastAvailable)]
  const [focused, focus] = useState<EffortLevel | null>(defaultFocused ?? null)
  const root = useRef<HTMLDivElement>(null)
  const rail = useRef<SVGSVGElement>(null)
  const inputs = useRef<Array<HTMLInputElement | null>>([])
  const pointer = useRef<number | null>(null)
  const id = useId()
  const index = effortLevels.indexOf(selected)
  function choose(position: number, edit = false) {
    const next = Math.max(0, Math.min(lastAvailable, position))
    select(effortLevels[next])
    if (edit) {
      focus(effortLevels[next])
      inputs.current[next]?.focus()
    }
  }
  function drag(clientX: number) {
    const bounds = rail.current?.getBoundingClientRect()
    if (!bounds?.width) return
    choose(Math.round((((clientX - bounds.left) / bounds.width) * 560 - 32) / 124), true)
  }
  return (
    <fieldset
      data-form-field={name}
      aria-invalid={field?.invalid || undefined}
      aria-describedby={field ? `${name}-error` : undefined}
      onBlur={(event) => {
        if (!event.currentTarget.contains(event.relatedTarget)) field?.blur()
      }}
      disabled={disabled}
      className="w-full min-w-0"
    >
      <legend className="mb-2 text-sm font-medium">
        <span className="inline-flex items-center gap-1">
          Thinking efforts
          <FieldHelp label="About thinking efforts">
            Map Low → Max to the thinking-effort values your model accepts (for example, low, medium, high). Higher effort can
            use more time and tokens. Use up to {ProviderInputRulesConstraints.effortValue.maxLength} letters, digits, dashes,
            or underscores, without spaces. Fill levels in order; leave unused levels empty.
          </FieldHelp>
        </span>
      </legend>
      <div
        ref={root}
        className="relative h-16 min-w-0"
        onBlur={(event) => {
          if (!event.currentTarget.contains(event.relatedTarget)) focus(null)
        }}
      >
        <div className="absolute inset-x-0 top-0 grid grid-cols-5 gap-1 sm:gap-3">
          {effortLevels.map((level, position) => {
            const current = level === selected,
              editing = focused === level,
              mapped = value[level] ?? ''
            return (
              <div key={level} className="relative flex min-w-0 flex-col items-center">
                <TextField
                  ref={(element) => {
                    inputs.current[position] = element
                  }}
                  aria-label={`${level} effort mapping`}
                  value={mapped}
                  disabled={disabled || position > lastAvailable}
                  placeholder="None"
                  maxLength={ProviderInputRulesConstraints.effortValue.maxLength}
                  aria-invalid={(mapped !== '' && !ProviderInputRules.shape.effortValue.safeParse(mapped).success) || undefined}
                  autoFocus={defaultFocused === level}
                  size={Math.max(4, mapped.length || 4)}
                  className={cx(
                    'h-6 w-auto min-w-10 max-w-full text-center text-xs leading-4 field-sizing-content',
                    !editing && !current && 'border-transparent bg-transparent text-center placeholder:text-transparent',
                    (editing || current) &&
                      'border border-muted-foreground/80 bg-inset focus:border focus:border-muted-foreground/80',
                  )}
                  onFocus={() => {
                    focus(level)
                    select(level)
                  }}
                  onBlur={() => focus(null)}
                  onChange={(event) => {
                    if (!ProviderInputRulesInput.effortValue(event.target.value)) return
                    const next = { ...value }
                    if (event.target.value) next[level] = event.target.value
                    else delete next[level]
                    if (!event.target.value.trim()) {
                      for (const later of effortLevels.slice(position + 1)) delete next[later]
                    }
                    onValueChange(next)
                  }}
                  onKeyDown={(event) => {
                    if (event.key === 'Enter') {
                      event.preventDefault()
                      focus(null)
                      event.currentTarget.blur()
                    }
                  }}
                />
                {current ? (
                  <svg
                    aria-hidden="true"
                    viewBox="0 0 8 6"
                    className="absolute top-full left-1/2 h-1.5 w-2 -translate-x-1/2 fill-primary"
                  >
                    <path d="M0 0H8L4 6Z" />
                  </svg>
                ) : null}
              </div>
            )
          })}
        </div>
        <div className="absolute inset-x-0 top-7 flex items-center gap-1">
          <animated.button
            type="button"
            disabled={disabled || index === 0}
            aria-label="Previous effort level"
            className={`
flex size-6 shrink-0 cursor-pointer items-center justify-center text-foreground focus-visible:outline-muted-foreground
disabled:opacity-30
`}
            onClick={() => choose(index - 1, true)}
          >
            <CaretLeftIcon weight="fill" className="size-4" aria-hidden="true" />
          </animated.button>
          <svg ref={rail} viewBox="0 0 560 14" preserveAspectRatio="none" className="h-3.5 min-w-0 flex-1" aria-hidden="true">
            <defs>
              <linearGradient id={`${id}-gradient`} x1="0" x2="1">
                <stop stopColor="var(--effort-low)" />
                <stop offset="58%" stopColor="var(--effort-medium)" />
                <stop offset="100%" stopColor="var(--effort-high)" />
              </linearGradient>
            </defs>
            <path
              d="M2 5C136 4.8 360 3.5 550 2C556 2 560 3.8 560 7C560 10.2 556 12 550 12C360 10.5 136 9.2 2 9Q0 9 0 7Q0 5 2 5Z"
              fill={`url(#${id}-gradient)`}
            />
          </svg>
          <animated.button
            type="button"
            disabled={disabled || index === lastAvailable}
            aria-label="Next effort level"
            className={`
flex size-6 shrink-0 cursor-pointer items-center justify-center text-foreground focus-visible:outline-muted-foreground
disabled:opacity-30
`}
            onClick={() => choose(index + 1, true)}
          >
            <CaretRightIcon weight="fill" className="size-4" aria-hidden="true" />
          </animated.button>
        </div>
        <div
          role="slider"
          tabIndex={disabled ? -1 : 0}
          aria-label="Selected effort level"
          aria-valuemin={0}
          aria-valuemax={lastAvailable}
          aria-valuenow={index}
          aria-valuetext={selected}
          aria-disabled={disabled || undefined}
          className={`
absolute inset-x-7 top-7 h-6 touch-none cursor-ew-resize rounded focus-visible:outline-1
focus-visible:outline-muted-foreground
`}
          onPointerDown={(event) => {
            if (disabled) return
            event.preventDefault()
            pointer.current = event.pointerId
            event.currentTarget.setPointerCapture(event.pointerId)
            drag(event.clientX)
          }}
          onPointerMove={(event) => {
            if (pointer.current === event.pointerId) drag(event.clientX)
          }}
          onPointerUp={() => {
            pointer.current = null
            focus(selected)
            inputs.current[effortLevels.indexOf(selected)]?.focus()
          }}
          onPointerCancel={() => {
            pointer.current = null
          }}
          onKeyDown={(event) => {
            if (disabled) return
            const delta = event.key === 'ArrowLeft' ? -1 : event.key === 'ArrowRight' ? 1 : 0
            if (delta || event.key === 'Home' || event.key === 'End') {
              event.preventDefault()
              choose(event.key === 'Home' ? 0 : event.key === 'End' ? 4 : index + delta, true)
            }
          }}
        />
      </div>
    </fieldset>
  )
}

export function EffortMapping({ variant = 'default', ...props }: EffortMappingProps) {
  return variant === 'compact' ? <CompactEffortMapping {...props} /> : <DefaultEffortMapping {...props} />
}

function CompactEffortMapping({
  name,
  value: suppliedValue,
  onValueChange: suppliedChange,
  disabled = false,
  defaultSelected = 'low',
  defaultFocused,
}: EffortMappingProps) {
  const field = useFormField<EffortValues>(name)
  const values = field?.value ?? suppliedValue ?? {}
  const [selection, select] = useState(effortLevels.indexOf(defaultFocused ?? defaultSelected))
  const inputs = useRef<Array<HTMLInputElement | null>>([])
  const pendingFocus = useRef(false)
  const last = effortLevels.length - 1
  const active = selection
  useEffect(() => {
    if (!pendingFocus.current) return
    inputs.current[active]?.focus()
    pendingFocus.current = false
  }, [active])
  const choose = (next: number) => {
    pendingFocus.current = true
    select(Math.max(0, Math.min(last, next)))
  }
  const change = (level: EffortLevel, value: string) => {
    if (!ProviderInputRulesInput.effortValue(value)) return
    const next = { ...values }
    if (value) next[level] = value
    else for (const later of effortLevels.slice(effortLevels.indexOf(level))) delete next[later]
    if (field) field.change(next)
    else suppliedChange?.(next)
  }
  return (
    <div
      className="flex min-h-10 min-w-0 items-center gap-2"
      data-form-field={name}
      aria-invalid={field?.invalid || undefined}
      aria-describedby={name ? `${name}-error` : undefined}
    >
      <DisabledTooltip disabled={disabled || active === 0} reason="This is the first effort level.">
        <animated.button
          type="button"
          aria-label="Previous effort level"
          disabled={disabled || active === 0}
          onClick={() => choose(active - 1)}
          className="flex size-7 shrink-0 cursor-pointer items-center justify-center rounded-lg bg-elevated
          text-muted-foreground hover:bg-hover hover:text-foreground disabled:opacity-30 focus-visible:outline-ring"
        >
          <CaretLeftIcon className="size-3.5" aria-hidden="true" />
        </animated.button>
      </DisabledTooltip>
      <div className="relative grid min-h-10 min-w-0 flex-1 grid-cols-5 gap-1 overflow-hidden rounded-lg px-1 pb-2">
        <div
          aria-hidden="true"
          data-effort-glow
          className="pointer-events-none absolute inset-x-0 bottom-0
          bg-linear-to-r from-(--effort-low) via-(--effort-medium) to-(--effort-high) blur-sm"
          style={{ height: 4 + active * 0.5, opacity: 0.18 + active * 0.02 }}
        />
        <div
          aria-hidden="true"
          className="pointer-events-none absolute inset-x-0 bottom-0 h-0.5
          bg-linear-to-r from-(--effort-low) via-(--effort-medium) to-(--effort-high) opacity-70"
        />
        {effortLevels.map((level, index) => (
          <div key={level} className="relative z-1 flex min-w-0 items-center justify-center">
            {index === active ? (
              <TextField
                ref={(element) => {
                  inputs.current[index] = element
                }}
                aria-label={`${level} effort mapping`}
                value={values[level] ?? ''}
                placeholder={level}
                autoFocus={defaultFocused === level}
                disabled={disabled}
                maxLength={ProviderInputRulesConstraints.effortValue.maxLength}
                className="h-7 w-full rounded-lg border-ring/50 bg-elevated px-1 text-center text-xs"
                onChange={(event) => change(level, event.target.value)}
                onBlur={field?.blur}
                onKeyDown={(event) => {
                  if (event.key === 'Tab') {
                    const next = active + (event.shiftKey ? -1 : 1)
                    if (next >= 0 && next <= last) {
                      event.preventDefault()
                      choose(next)
                    }
                  }
                  if (event.key === 'ArrowLeft' && (event.ctrlKey || event.metaKey)) {
                    event.preventDefault()
                    choose(active - 1)
                  }
                  if (event.key === 'ArrowRight' && (event.ctrlKey || event.metaKey)) {
                    event.preventDefault()
                    choose(active + 1)
                  }
                }}
              />
            ) : (
              <DisabledTooltip disabled={disabled} reason="Editing thinking efforts is unavailable.">
                <animated.button
                  type="button"
                  disabled={disabled}
                  aria-label={`Edit ${level} effort mapping`}
                  onFocus={() => choose(index)}
                  onClick={() => choose(index)}
                  className="h-7 w-full min-w-0 truncate rounded-lg text-xs text-muted-foreground hover:bg-hover
                disabled:cursor-not-allowed disabled:opacity-40 focus-visible:outline-ring"
                >
                  {values[level] || '\u00a0'}
                </animated.button>
              </DisabledTooltip>
            )}
          </div>
        ))}
      </div>
      <DisabledTooltip disabled={disabled || active === last} reason="This is the last effort level.">
        <animated.button
          type="button"
          aria-label="Next effort level"
          disabled={disabled || active === last}
          onClick={() => choose(active + 1)}
          className="flex size-7 shrink-0 cursor-pointer items-center justify-center rounded-lg bg-elevated
          text-muted-foreground hover:bg-hover hover:text-foreground disabled:opacity-30 focus-visible:outline-ring"
        >
          <CaretRightIcon className="size-3.5" aria-hidden="true" />
        </animated.button>
      </DisabledTooltip>
    </div>
  )
}
