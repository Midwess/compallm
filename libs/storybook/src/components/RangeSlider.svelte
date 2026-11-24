<script lang="ts">
  import { cx } from './shared'
  import { interactionFeedback } from './interaction-feedback'
  let {
    label,
    value = $bindable(1),
    min,
    max,
    onValueChange,
    onBlur,
    onValueCommit,
    onTickFeedback,
    class: className,
  }: {
    label: string
    value: number
    min: number
    max: number
    onValueChange?: (value: number) => void
    onBlur?: () => void
    onValueCommit?: () => void
    onTickFeedback?: (value: number) => void
    class?: string
  } = $props()
  let ratio = $derived(Math.max(0, Math.min(1, Math.log(Math.max(min, value) / min) / Math.log(max / min))))
  let lastTick = 0
  $effect(() => {
    lastTick = Math.floor(ratio * 50)
  })
  function change(position: number) {
    const tick = Math.floor((position * 50) / 1000)
    const direction = Math.sign(tick - lastTick)
    while (lastTick !== tick) {
      lastTick += direction
      const next = Math.round(min * (max / min) ** (lastTick / 50))
      if (onTickFeedback) onTickFeedback(next)
      else interactionFeedback('tick')
    }
    value = Math.round(min * (max / min) ** (position / 1000))
    onValueChange?.(value)
  }
</script>

<div class={cx('min-w-0', className)}>
  <div class="relative flex h-6 items-center">
    <div aria-hidden="true" class="h-1 w-full rounded-full bg-foreground/15">
      <div class="h-full rounded-full bg-current opacity-65" style={`width:${ratio * 100}%`}></div>
    </div>
    <div aria-hidden="true" class="pointer-events-none absolute inset-0 flex items-center justify-between opacity-40">
      {#each Array(51) as _, tick (tick)}<span class={cx('w-0.5 bg-muted-foreground', tick % 5 === 0 ? 'h-2' : 'h-1')}
        ></span>{/each}
    </div>
    <input
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
      oninput={(e) => change(Number(e.currentTarget.value))}
      onpointerup={onValueCommit}
      onkeyup={(e) => {
        if (['ArrowLeft', 'ArrowRight', 'ArrowUp', 'ArrowDown', 'Home', 'End', 'PageUp', 'PageDown'].includes(e.key))
          onValueCommit?.()
      }}
      onblur={onBlur}
      class="absolute inset-0 h-full w-full cursor-ew-resize appearance-none bg-transparent text-inherit focus-visible:rounded focus-visible:outline-2 focus-visible:outline-ring [&::-webkit-slider-thumb]:size-3 [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:border-2 [&::-webkit-slider-thumb]:border-current [&::-webkit-slider-thumb]:bg-foreground [&::-moz-range-thumb]:size-2 [&::-moz-range-thumb]:rounded-full [&::-moz-range-thumb]:border-2 [&::-moz-range-thumb]:border-current [&::-moz-range-thumb]:bg-foreground"
    />
  </div>
  <div class="flex justify-between text-ui-11 text-muted-foreground" style="font-variant-numeric:tabular-nums">
    <span>{min.toLocaleString()}</span><span>{max.toLocaleString()}</span>
  </div>
</div>
