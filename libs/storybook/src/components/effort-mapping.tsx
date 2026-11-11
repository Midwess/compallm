import { animated } from './motion'
import { ProviderInputRules, ProviderInputRulesInput, ProviderInputRulesConstraints } from '@compallm/contract/provider'
import { useId, useRef, useState } from 'react'
import { CaretLeftIcon } from '@phosphor-icons/react/dist/csr/CaretLeft'
import { CaretRightIcon } from '@phosphor-icons/react/dist/csr/CaretRight'
import { TextField } from './fields'
import { FieldHelp } from './feedback'
import { cx } from './shared'

export const effortLevels = ['low', 'medium', 'high', 'xhigh', 'max'] as const
export type EffortLevel = (typeof effortLevels)[number]
export type EffortValues = Partial<Record<EffortLevel, string>>
export interface EffortMappingProps {
  value: EffortValues
  onValueChange(value: EffortValues): void
  disabled?: boolean
  defaultSelected?: EffortLevel
  defaultFocused?: EffortLevel
}
export function EffortMapping({
  value,
  onValueChange,
  disabled = false,
  defaultSelected = 'low',
  defaultFocused,
}: EffortMappingProps) {
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
    <fieldset disabled={disabled} className="w-full min-w-0">
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
                    'h-6 w-auto min-w-10 max-w-full text-center text-xs leading-4 [field-sizing:content]',
                    !editing && !current && 'border-transparent bg-transparent text-center placeholder:text-transparent',
                    (editing || current) && 'border border-muted/80 bg-inset focus:border focus:border-muted/80',
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
flex size-6 shrink-0 cursor-pointer items-center justify-center text-text focus-visible:outline-muted
disabled:opacity-30
`}
            onClick={() => choose(index - 1, true)}
          >
            <CaretLeftIcon weight="fill" className="size-4" aria-hidden="true" />
          </animated.button>
          <svg ref={rail} viewBox="0 0 560 14" preserveAspectRatio="none" className="h-3.5 min-w-0 flex-1" aria-hidden="true">
            <defs>
              <linearGradient id={`${id}-gradient`} x1="0" x2="1">
                <stop stopColor="#7762d8" />
                <stop offset="58%" stopColor="#9470d1" />
                <stop offset="100%" stopColor="#ee9b58" />
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
flex size-6 shrink-0 cursor-pointer items-center justify-center text-text focus-visible:outline-muted
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
focus-visible:outline-muted
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
