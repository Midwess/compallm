<script lang="ts">
  import { untrack } from 'svelte'
  import { cx, type ElementProps } from './shared'
  import { interactionFeedback, type InteractionFeedback } from './interaction-feedback'
  let {
    children,
    class: className,
    navigationLabel = 'Workspace navigation',
    resizable = true,
    width = $bindable(244),
    defaultWidth = 244,
    minWidth = 200,
    maxWidth = 400,
    onWidthChange,
    onResizeFeedback,
    ...props
  }: ElementProps & {
    navigationLabel?: string
    resizable?: boolean
    width?: number
    defaultWidth?: number
    minWidth?: number
    maxWidth?: number
    onResizeFeedback?: (kind: InteractionFeedback, width: number) => void
    onWidthChange?: (width: number) => void
  } = $props()
  untrack(() => {
    if (width === 244 && defaultWidth !== 244) width = defaultWidth
  })
  const id = $props.id()
  let root: HTMLElement
  let grip = $state<HTMLDivElement>()
  let drag: { pointer: number; x: number; width: number; current: number } | null = null
  const bound = (next: number) => Math.round(Math.max(Math.max(160, minWidth), Math.min(Math.max(minWidth, maxWidth), next)))
  function feedback(kind: InteractionFeedback, next: number) {
    if (onResizeFeedback) onResizeFeedback(kind, next)
    else interactionFeedback(kind)
  }
  function paint(next: number) {
    root.style.width = `${next}px`
    grip?.setAttribute('aria-valuenow', String(next))
    grip?.setAttribute('aria-valuetext', `${next} pixels`)
  }
  function finish(cancel = false) {
    if (!drag) return
    const current = drag
    drag = null
    delete root.dataset.resizing
    paint(cancel ? current.width : current.current)
    if (!cancel) {
      change(current.current)
      if (current.current !== current.width) feedback('commit', current.current)
    }
  }
  function change(next: number) {
    width = bound(next)
    onWidthChange?.(width)
  }
</script>

<aside
  bind:this={root}
  {...props}
  {id}
  style={`width:${bound(width)}px`}
  class={cx(
    'group/sidebar relative flex h-full min-h-0 shrink-0 flex-col bg-sidebar px-3 py-1 select-none [&_input]:select-text [&_textarea]:select-text **:[[contenteditable=true]]:select-text',
    className,
  )}
>
  <nav
    aria-label={navigationLabel}
    class="-mx-0.5 flex min-h-0 min-w-0 flex-1 flex-col gap-2.5 overflow-y-scroll scrollbar-gutter-stable overscroll-y-none px-0.5"
  >
    {@render children?.()}
  </nav>
  {#if resizable}<!-- svelte-ignore a11y_no_noninteractive_tabindex, a11y_no_noninteractive_element_interactions -->
    <div
      bind:this={grip}
      role="separator"
      aria-label="Resize sidebar"
      aria-orientation="vertical"
      aria-controls={id}
      aria-describedby={`${id}-resize-hint`}
      aria-valuetext={`${bound(width)} pixels`}
      aria-valuemin={minWidth}
      aria-valuemax={maxWidth}
      aria-valuenow={width}
      tabindex="0"
      class="group/grip absolute inset-y-0 -right-1 z-20 flex w-2 touch-none cursor-col-resize items-center justify-center outline-none"
      onpointerdown={(e) => {
        if (e.button !== 0) return
        e.preventDefault()
        e.currentTarget.focus()
        const current = root.getBoundingClientRect().width
        drag = { pointer: e.pointerId, x: e.clientX, width: current, current }
        root.dataset.resizing = ''
        e.currentTarget.setPointerCapture(e.pointerId)
      }}
      onpointermove={(e) => {
        if (drag && drag.pointer === e.pointerId) {
          const raw = bound(drag.width + e.clientX - drag.x)
          const snap = [minWidth, defaultWidth, maxWidth].find((point) => Math.abs(raw - point) <= 6)
          const next = snap ?? raw
          if (snap !== undefined && next !== drag.current) feedback('snap', next)
          drag.current = next
          paint(next)
        }
      }}
      onpointerup={(e) => {
        if (drag?.pointer !== e.pointerId) return
        finish()
        e.currentTarget.releasePointerCapture(e.pointerId)
      }}
      onlostpointercapture={() => finish(true)}
      onpointercancel={() => finish(true)}
      ondblclick={() => {
        paint(bound(defaultWidth))
        change(defaultWidth)
        feedback('snap', bound(defaultWidth))
      }}
      onkeydown={(e) => {
        const next =
          e.key === 'ArrowLeft'
            ? width - (e.shiftKey ? 24 : 8)
            : e.key === 'ArrowRight'
              ? width + (e.shiftKey ? 24 : 8)
              : e.key === 'Home'
                ? minWidth
                : e.key === 'End'
                  ? maxWidth
                  : e.key === 'Enter'
                    ? defaultWidth
                    : null
        if (e.key === 'Escape' && drag) {
          e.preventDefault()
          finish(true)
          return
        }
        if (next !== null) {
          e.preventDefault()
          paint(bound(next))
          change(next)
          feedback('snap', bound(next))
        }
      }}
    >
      <span
        aria-hidden="true"
        class="pointer-events-none h-10 w-0.5 rounded-full bg-divider group-hover/grip:bg-muted-foreground/60 group-focus-visible/grip:bg-ring group-data-resizing/sidebar:bg-ring"
      ></span>
    </div>{/if}
  {#if resizable}<span id={`${id}-resize-hint`} class="sr-only">
      Drag to resize, or use Left and Right arrow keys. Shift moves faster. Home and End select the limits. Enter or
      double-click resets the width. Escape cancels a drag.
    </span>{/if}
</aside>
