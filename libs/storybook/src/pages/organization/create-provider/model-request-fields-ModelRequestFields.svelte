<script lang="ts">
  import DotsSixVerticalIcon from 'phosphor-svelte/lib/DotsSixVerticalIcon'
  import TrashIcon from 'phosphor-svelte/lib/TrashIcon'
  import CaretDownIcon from 'phosphor-svelte/lib/CaretDownIcon'

  import { Popup } from '../../../components/popup'
  import { TextField } from '../../../components/fields'
  import { FormError } from '../../../components/form'
  import { useReorder } from '../../../components/use-reorder.svelte'
  import { ModelSection } from './model-sections'
  import type { HeaderEntry } from './types'
  import { untrack } from 'svelte'
  import { fieldState, toStyle, registerElement } from '../../page-state.svelte'
  import { getCreateProviderController } from '../../../context'
  import { headerOptions, bodyOptions } from './model-request-fields'
  import ContentValue from '../../../components/Content.svelte'
  import { interaction } from '../../../components/motion'

  let {
    kind,
  }: {
    kind: 'headers' | 'bodyDefaults'
  } = $props()

  const controller = untrack(() => getCreateProviderController())
  let name = $derived(`modelDraft.${kind}`)
  const field = fieldState<HeaderEntry[]>(() => name)!
  let entries = $derived((controller.store.form.get(name) ?? []) as HeaderEntry[])
  let isHeader = $derived(kind === 'headers')
  let options = $derived(isHeader ? headerOptions : bodyOptions)
  let label = $derived(isHeader ? 'header' : 'body field')
  const reorder = untrack(() =>
    useReorder(
      () => entries.map((entry) => entry.id),
      (order) => field.change(order.flatMap((id) => entries.filter((entry) => entry.id === id))),
      () => false,
      () => label,
    ),
  )
  let custom = $state(false)
  let customKey = $state('')
  let customIndex: number | undefined = $state(undefined)
  const add = (key: string) => {
    if (entries.some((entry) => (isHeader ? entry.key.toLowerCase() === key.toLowerCase() : entry.key === key))) return
    field.change([...entries, { id: crypto.randomUUID(), key, value: '' }])
  }
  const saveCustom = () => {
    if (!customKey) return
    if (customIndex === undefined) add(customKey)
    else controller.store.form.change(`${name}.${customIndex}.key`, customKey)
    custom = false
  }
</script>

{#snippet selector(entry: HeaderEntry | undefined, index: number | undefined)}<Popup.Root>
    <Popup.Trigger
      aria-label={entry ? `Change ${label} ${entry.key}` : `Add ${label}`}
      class={`
flex h-8 w-full min-w-0 justify-between gap-2 rounded-md bg-foreground/10 px-2 text-left text-xsm
hover:bg-foreground/15
`}
    >
      <span class={`truncate ${entry ? '' : 'text-muted-foreground'}`}>{entry?.key ?? `Select ${label}…`}</span><CaretDownIcon
        class="size-2.5 shrink-0"
      ></CaretDownIcon>
    </Popup.Trigger><Popup.Content align="start" class="w-80" aria-label={`Available ${label}s`}>
      {#each options as option (option.key)}<Popup.Item
          class="h-auto min-h-10 flex-col items-start py-2"
          disabled={entries.some(
            (item) =>
              item.id !== entry?.id &&
              (isHeader ? item.key.toLowerCase() === option.key.toLowerCase() : item.key === option.key),
          )}
          onSelect={() => (entry ? controller.store.form.change(`${name}.${index}.key`, option.key) : add(option.key))}
        >
          {option.key}<span class="text-ui-11 text-muted-foreground">{option.description}</span>
        </Popup.Item>{/each}<Popup.Separator></Popup.Separator><Popup.Item
        onSelect={() => {
          custom = true
          customKey = entry?.key ?? ''
          customIndex = index
        }}
      >
        Custom <ContentValue value={label} />
      </Popup.Item>
    </Popup.Content>
  </Popup.Root>{/snippet}
<ModelSection
  title={isHeader ? 'Headers' : 'Body defaults'}
  help={isHeader
    ? `
Optional HTTP headers for this model’s requests, such as a project identifier or provider beta flag. The
provider connection supplies the base URL and credential. Choose a known header or add a custom name. Names are
unique ignoring case. Remove a row to remove its override. Header values may contain secrets, so do not share
them.
`
    : `
Optional JSON body fields used when the caller leaves them unset. Explicit request values take precedence.
Choose a known field or add a custom one. Numbers and booleans use JSON syntax; strings need quotes, and arrays
or objects must be valid JSON. Model, messages, input, and stream are managed separately. Remove a row to stop
supplying its default.
`}
  description={isHeader ? undefined : 'Used when the request leaves these fields unset.'}
>
  <div class="grid min-w-0 gap-2 rounded-control bg-inset px-6 py-4" data-form-field={name}>
    <div class="flex min-w-0 items-center gap-6 text-xs text-muted-foreground">
      <span class="w-6 shrink-0">Order</span>
      <div class="grid min-w-0 flex-1 grid-cols-[minmax(0,1fr)_minmax(0,1.2fr)_1.75rem] gap-1 px-1">
        <span class="px-2">Key</span><span class="px-1.5">Value</span><span></span>
      </div>
    </div>
    {#each entries as entry, index (entry.id)}<div
        class="relative flex min-w-0 items-center gap-6"
        style={toStyle({ transform: `translateY(${reorder.preview?.offsets[entry.id] ?? 0}px)` })}
        use:registerElement={(node) => {
          if (node) reorder.rowNodes.current.set(entry.id, node)
          else reorder.rowNodes.current.delete(entry.id)
        }}
      >
        <button
          use:interaction
          type="button"
          aria-label={`Reorder ${label} ${index + 1}`}
          aria-describedby={`${reorder.id}-hint`}
          class={`
flex size-6 shrink-0 touch-none cursor-grab items-center justify-center text-muted-foreground
focus-visible:outline-ring
`}
          onpointerdown={(event) => reorder.startDrag(entry.id, event)}
          onpointermove={reorder.pointerMove}
          onpointerup={() => reorder.endDrag(true)}
          onpointercancel={() => reorder.endDrag(false)}
          onkeydown={(event) => {
            if (event.key === 'ArrowUp' || event.key === 'ArrowDown') {
              event.preventDefault()
              reorder.keyboardMove(index, index + (event.key === 'ArrowUp' ? -1 : 1))
            }
          }}
        >
          <DotsSixVerticalIcon class="size-3"></DotsSixVerticalIcon>
        </button>
        <div
          class={`grid min-w-0 flex-1 grid-cols-[minmax(0,1fr)_minmax(0,1.2fr)_1.75rem]
              items-start gap-1 rounded-lg bg-elevated p-1`}
        >
          <div class="relative min-w-0">
            {@render selector(entry, index)}<FormError name={`${name}.${index}.key`}></FormError>
          </div>
          <div class="relative min-w-0">
            {#if isHeader}<TextField
                name={`${name}.${index}.value`}
                aria-label={`${entry.key} value`}
                placeholder="Enter value"
                class="h-8 rounded-md border-transparent text-xsm"
                autocomplete="off"
              ></TextField>{:else}<TextField
                name={`${name}.${index}.value`}
                aria-label={`${entry.key} value`}
                placeholder="JSON value"
                class="h-8 rounded-md border-transparent text-xsm"
              ></TextField>{/if}<FormError name={`${name}.${index}.value`}></FormError>
          </div>
          <button
            use:interaction
            type="button"
            aria-label={`Remove ${entry.key}`}
            class={`
mt-1 flex size-6 items-center justify-center rounded text-muted-foreground hover:bg-hover
focus-visible:outline-ring
`}
            onclick={() => field.change(entries.filter((item) => item.id !== entry.id))}
          >
            <TrashIcon class="size-3"></TrashIcon>
          </button>
        </div>
      </div>{/each}
    <div class="ml-12 grid min-w-0 grid-cols-[minmax(0,1fr)_minmax(0,1.2fr)_1.75rem] gap-1 rounded-lg bg-elevated p-1">
      <div class="min-w-0">{@render selector(undefined, undefined)}</div>
    </div>
    {#if custom}<div class="ml-12 flex min-w-0 items-center gap-2">
        <TextField
          aria-label={`Custom ${label} name`}
          placeholder={`Enter ${label} name`}
          value={customKey}
          oninput={(event) => (customKey = event.currentTarget.value)}
          onkeydown={(event) => {
            if (event.key === 'Enter') {
              event.preventDefault()
              if (customKey) {
                saveCustom()
              }
            }
            if (event.key === 'Escape') custom = false
          }}
        ></TextField><button
          use:interaction
          type="button"
          class="h-8 rounded-md bg-elevated px-2 text-xs focus-visible:outline-ring"
          onclick={() => {
            if (customKey) {
              saveCustom()
            }
          }}
        >
          {#if customIndex === undefined}Add{:else}Apply{/if}
        </button><button
          use:interaction
          type="button"
          aria-label="Cancel custom field"
          class="h-8 px-1 text-xs text-muted-foreground"
          onclick={() => (custom = false)}
        >
          Cancel
        </button>
      </div>{:else}{/if}
    <p id={`${reorder.id}-hint`} class="sr-only">Drag to reorder or use Up and Down on a grip.</p>
    <span role="status" class="sr-only">{reorder.announcement}</span><FormError {name}></FormError>
  </div>
</ModelSection>
