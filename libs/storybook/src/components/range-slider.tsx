import { useEffect, useId, useRef, type CSSProperties } from 'react'
import { cx } from './shared'
import { interactionFeedback } from './interaction-feedback'

export interface RangeSliderProps {
  label: string
  value: number
  min: number
  max: number
  onValueChange(value: number): void
  onBlur?(): void
  onValueCommit?(): void
  onTickFeedback?(value: number): void
  className?: string
}

export function RangeSlider({
  label,
  value,
  min,
  max,
  onValueChange,
  onBlur,
  onValueCommit,
  onTickFeedback,
  className,
}: RangeSliderProps) {
  const id = useId()
  const ratio = Math.max(0, Math.min(1, Math.log(Math.max(min, value) / min) / Math.log(max / min)))
  const tickCount = 50
  const lastTick = useRef(Math.floor((Math.round(ratio * 1000) * tickCount) / 1000))
  useEffect(() => {
    lastTick.current = Math.floor((Math.round(ratio * 1000) * tickCount) / 1000)
  }, [ratio])
  const change = (position: number) => {
    const tick = Math.floor((position * tickCount) / 1000)
    const direction = Math.sign(tick - lastTick.current)
    while (lastTick.current !== tick) {
      lastTick.current += direction
      const tickValue = Math.round(min * (max / min) ** (lastTick.current / tickCount))
      if (onTickFeedback) onTickFeedback(tickValue)
      else interactionFeedback('tick')
    }
    onValueChange(Math.round(min * (max / min) ** (position / 1000)))
  }
  return (
    <div className={cx('min-w-0', className)}>
      <div className="relative flex h-6 items-center">
        <div aria-hidden="true" className="h-1 w-full rounded-full bg-foreground/15">
          <div className="h-full rounded-full bg-current opacity-65" style={{ width: `${ratio * 100}%` }} />
        </div>
        <div
          aria-hidden="true"
          data-range-ruler
          className="pointer-events-none absolute inset-0 flex items-center justify-between opacity-40"
        >
          {Array.from({ length: tickCount + 1 }, (_, tick) => (
            <span key={tick} className={cx('w-0.5 bg-muted-foreground', tick % 5 === 0 ? 'h-2' : 'h-1')} />
          ))}
        </div>
        <input
          id={id}
          type="range"
          aria-label={label}
          aria-valuemin={min}
          aria-valuemax={max}
          aria-valuenow={value}
          aria-valuetext={`${value.toLocaleString()} tokens`}
          min={0}
          max={1000}
          step={1}
          value={Math.round(ratio * 1000)}
          onChange={(event) => change(Number(event.target.value))}
          onPointerUp={onValueCommit}
          onKeyUp={(event) => {
            if (['ArrowLeft', 'ArrowRight', 'ArrowUp', 'ArrowDown', 'Home', 'End', 'PageUp', 'PageDown'].includes(event.key))
              onValueCommit?.()
          }}
          onBlur={onBlur}
          className="absolute inset-0 h-full w-full cursor-ew-resize appearance-none bg-transparent text-inherit
          focus-visible:rounded focus-visible:outline-2 focus-visible:outline-ring
          [&::-webkit-slider-thumb]:size-3 [&::-webkit-slider-thumb]:appearance-none
          [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:border-2
          [&::-webkit-slider-thumb]:border-current [&::-webkit-slider-thumb]:bg-foreground
          [&::-moz-range-thumb]:size-2 [&::-moz-range-thumb]:rounded-full
          [&::-moz-range-thumb]:border-2 [&::-moz-range-thumb]:border-current [&::-moz-range-thumb]:bg-foreground"
        />
      </div>
      <div
        className="flex justify-between text-ui-11 text-muted-foreground"
        style={{ fontVariantNumeric: 'tabular-nums' } as CSSProperties}
      >
        <span>{min.toLocaleString()}</span>
        <span>{max.toLocaleString()}</span>
      </div>
    </div>
  )
}
