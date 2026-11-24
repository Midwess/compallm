<script lang="ts">
  import { tick } from 'svelte'
  import { fieldModel } from './form-context.svelte'
  import MultilineField from './MultilineField.svelte'
  import type { ComponentProps } from 'svelte'
  import { cx } from './shared'
  import {
    transitionTokenEntry,
    tokenParts,
    type TokenEntryState,
    type TokenEntryEvent,
    type TokenSelection,
  } from './token-entry'
  let {
    name,
    parseToken,
    onfocus,
    onblur,
    ...props
  }: ComponentProps<typeof MultilineField> & {
    name: string
    parseToken: (text: string) => string | null
  } = $props()
  let input = $state<HTMLTextAreaElement>()
  let editorText = $state('')
  let entry = $state<TokenEntryState>({ kind: 'preview' })
  const form = fieldModel(() => name)
  let value = $derived(String(form?.get(name) ?? ''))
  let focused = $derived(entry.kind !== 'preview')
  let groupedUntil = $derived(entry.kind === 'preview' ? 0 : Math.min(entry.groupedUntil, value.length))
  let preview = $derived(!focused && value.length > 0)
  let parts = $derived(tokenParts(focused ? value.slice(0, groupedUntil) : value, parseToken))
  $effect(() => {
    if (entry.kind !== 'composing') editorText = value.slice(groupedUntil)
  })
  function selection(node: HTMLTextAreaElement) {
    return { start: node.selectionStart, end: node.selectionEnd }
  }
  function restore(range: TokenSelection) {
    void tick().then(() => {
      if (input === document.activeElement) input?.setSelectionRange(range.start, range.end)
    })
  }
  function dispatch(event: TokenEntryEvent) {
    const result = transitionTokenEntry(entry, event, value, parseToken)
    entry = result.state
    const next = result.value ?? value
    editorText = entry.kind === 'composing' ? entry.draft : next.slice(entry.kind === 'preview' ? 0 : entry.groupedUntil)
    if (result.value !== undefined && result.value !== value) form?.change(name, result.value)
    if (result.selection) restore(result.selection)
    if (result.validate) form?.blur(name)
    return result
  }
  async function edit(start?: number, end?: number) {
    if (props.disabled || props.readonly) return
    const result = dispatch({ type: 'focus', selection: start === undefined ? undefined : { start, end: end ?? start } })
    await tick()
    input?.focus({ preventScroll: true })
    if (result.selection) input?.setSelectionRange(result.selection.start, result.selection.end)
  }
</script>

<div
  data-form-field={name}
  data-entry-state={entry.kind}
  aria-invalid={form?.invalid(name) || undefined}
  class="relative flex max-h-40 min-h-18 w-full min-w-0 touch-pan-y flex-wrap content-start items-start gap-1 overflow-x-hidden overflow-y-auto overscroll-x-none rounded-lg border-(length:--border-hairline) border-input bg-inset p-1.5 text-left text-xsm text-foreground focus-within:border-muted-foreground/60 aria-invalid:border-destructive"
>
  <button
    type="button"
    tabindex={-1}
    aria-hidden="true"
    disabled={props.disabled || props.readonly}
    class="absolute inset-0 cursor-text disabled:cursor-not-allowed"
    onpointerdown={(event) => {
      event.preventDefault()
      if (focused) input?.focus({ preventScroll: true })
      else void edit()
    }}
  ></button>
  {#each parts as part (part.start)}<button
      type="button"
      tabindex={-1}
      aria-hidden="true"
      aria-label={`Edit ${part.text}`}
      disabled={props.disabled || props.readonly}
      data-token-start={part.start}
      data-token-end={part.end}
      class={cx(
        'relative w-fit min-w-0 max-w-full whitespace-pre-wrap break-all text-left disabled:opacity-50',
        part.token && 'rounded bg-foreground/10 px-1.5 py-0.5 hover:bg-foreground/15',
      )}
      onpointerdown={(event) => event.preventDefault()}
      onclick={() => edit(part.start, part.end)}
    >
      {part.text}
    </button>{/each}
  <MultilineField
    {...props}
    bind:element={input}
    bind:value={editorText}
    aria-invalid={form?.invalid(name) || undefined}
    aria-describedby={`${name}-error`}
    placeholder={groupedUntil ? undefined : props.placeholder}
    wrap="soft"
    onfocus={(event) => {
      if (!focused) dispatch({ type: 'focus' })
      onfocus?.(event)
    }}
    oncompositionstart={(event) => dispatch({ type: 'composition-start', text: event.currentTarget.value })}
    oncompositionend={(event) =>
      dispatch({ type: 'composition-end', text: event.currentTarget.value, selection: selection(event.currentTarget) })}
    onkeydown={(event) => {
      if (event.isComposing || props.readonly || event.key !== 'Backspace') return
      const result = dispatch({ type: 'backspace', selection: selection(event.currentTarget) })
      if (result.selection) event.preventDefault()
    }}
    class={cx(
      'field-sizing-content min-h-6 w-auto min-w-28 max-w-full flex-1 basis-28 resize-none overflow-x-hidden overflow-y-hidden whitespace-pre-wrap break-all rounded-none border-0 bg-transparent p-0 text-xsm',
      preview ? 'pointer-events-none absolute inset-0 m-0 h-full min-h-0 w-full min-w-0 basis-auto opacity-0' : 'relative',
    )}
    rows={1}
    oninput={(event) => {
      if (!props.readonly)
        dispatch({ type: 'input', text: event.currentTarget.value, selection: selection(event.currentTarget) })
    }}
    onblur={(event) => {
      dispatch({ type: 'blur', selection: selection(event.currentTarget) })
      onblur?.(event)
    }}
  />
</div>
