<script lang="ts">
  import { tick } from 'svelte'
  import { getKeyValue } from './key-value-context'
  import TextField from './TextField.svelte'
  import DotsSixVerticalIcon from 'phosphor-svelte/lib/DotsSixVerticalIcon'
  import TrashIcon from 'phosphor-svelte/lib/TrashIcon'
  const scope = getKeyValue()
  let dragging: string | null = null
  let pointer: number | null = null
  let root: HTMLDivElement
  let announcement = $state('')
  function update(index: number, patch: { key?: string; value?: string }) {
    const next = { ...scope.entries[index], ...patch }
    if (scope.acceptEntry && !scope.acceptEntry(next)) return
    scope.change(scope.entries.map((entry, i) => (i === index ? next : entry)))
  }
  function move(from: number, to: number) {
    if (scope.disabled || from < 0 || to < 0 || to >= scope.entries.length || from === to) return
    const next = scope.entries.slice()
    next.splice(to, 0, next.splice(from, 1)[0])
    scope.change(next)
    announcement = `Header moved to position ${to + 1}`
  }
</script>

<div
  bind:this={root}
  role="table"
  aria-label="Custom headers"
  class="grid w-full min-w-0 max-w-full grid-cols-[minmax(0,1fr)] gap-2"
>
  <div
    role="row"
    class="grid grid-cols-[2rem_minmax(0,1fr)_minmax(0,1fr)_1.5rem] items-center gap-1 text-xs text-muted-foreground"
  >
    <span role="columnheader">Order</span><span role="columnheader" class="pl-1.5">Key</span><span
      role="columnheader"
      class="pl-1"
    >
      Value
    </span><span role="columnheader" class="sr-only">Remove</span>
  </div>
  {#each scope.entries as entry, index (entry.id)}<div
      role="row"
      data-header-id={entry.id}
      tabindex={-1}
      class="relative flex min-w-0 items-center gap-2"
      ondragover={(e) => e.preventDefault()}
      ondrop={(e) => {
        e.preventDefault()
        move(
          scope.entries.findIndex((item) => item.id === dragging),
          index,
        )
      }}
    >
      <span role="cell" class="w-6 shrink-0">
        <button
          type="button"
          disabled={scope.disabled}
          aria-label={`Reorder header ${index + 1}`}
          onpointerdown={(event) => {
            if (scope.disabled || event.button !== 0) return
            event.preventDefault()
            event.currentTarget.focus()
            dragging = entry.id
            pointer = event.pointerId
            event.currentTarget.setPointerCapture(event.pointerId)
          }}
          onpointermove={(event) => {
            if (pointer !== event.pointerId || !dragging) return
            const rows = Array.from(root.querySelectorAll<HTMLElement>('[data-header-id]'))
            const destination = rows.findIndex((row) => {
              const rect = row.getBoundingClientRect()
              return event.clientY >= rect.top && event.clientY <= rect.bottom
            })
            if (destination >= 0)
              move(
                scope.entries.findIndex((item) => item.id === dragging),
                destination,
              )
          }}
          onpointerup={(event) => {
            if (pointer === event.pointerId) {
              pointer = null
              dragging = null
              event.currentTarget.releasePointerCapture(event.pointerId)
            }
          }}
          onpointercancel={() => {
            pointer = null
            dragging = null
          }}
          onlostpointercapture={() => {
            pointer = null
            dragging = null
          }}
          onkeydown={(e) => {
            if (e.key === 'ArrowUp' || e.key === 'ArrowDown') {
              e.preventDefault()
              move(index, index + (e.key === 'ArrowUp' ? -1 : 1))
            }
          }}
          class="flex size-6 touch-none cursor-grab items-center justify-center text-muted-foreground hover:text-foreground active:cursor-grabbing focus-visible:outline-muted-foreground disabled:opacity-40"
        >
          <DotsSixVerticalIcon aria-hidden="true" class="size-3" />
        </button>
      </span>
      <div
        class="grid min-w-0 flex-1 grid-cols-[minmax(0,1fr)_minmax(0,1fr)_1.5rem] items-center gap-1 rounded-lg bg-elevated p-1"
      >
        <span role="cell" class="min-w-0">
          <TextField
            value={entry.key}
            maxlength={scope.keyMaxLength}
            disabled={scope.disabled}
            aria-label={`Header ${index + 1} key`}
            placeholder="Enter key"
            class="h-8 border-transparent text-xs"
            oninput={(e) => update(index, { key: e.currentTarget.value })}
          />
        </span><span role="cell" class="min-w-0">
          <TextField
            value={entry.value}
            disabled={scope.disabled}
            aria-label={`Header ${index + 1} value`}
            placeholder="Enter value"
            class="h-8 border-transparent text-xs"
            oninput={(e) => update(index, { value: e.currentTarget.value })}
          />
        </span><span role="cell">
          <button
            type="button"
            disabled={scope.disabled}
            aria-label={`Remove header ${index + 1}`}
            class="flex size-6 cursor-pointer items-center justify-center text-muted-foreground hover:text-foreground disabled:opacity-40"
            onclick={() => scope.change(scope.entries.filter((item) => item.id !== entry.id))}
          >
            <TrashIcon size={12} />
          </button>
        </span>
      </div>
    </div>{/each}
  <div role="row" class="flex min-w-0 gap-2">
    <span role="cell" class="w-6 shrink-0"></span>
    <div role="cell" class="min-w-0 flex-1 rounded-lg bg-elevated p-1">
      <TextField
        aria-label="New header key"
        maxlength={scope.keyMaxLength}
        placeholder="Enter key"
        value=""
        disabled={scope.disabled}
        class="h-8 border-transparent text-xs"
        oninput={async (e) => {
          const key = e.currentTarget.value
          if (!key) return
          const entry = { id: crypto.randomUUID(), key, value: '' }
          if (!scope.acceptEntry || scope.acceptEntry(entry)) {
            scope.change([...scope.entries, entry])
            e.currentTarget.value = ''
            await tick()
            const field = root.querySelector<HTMLInputElement>(`input[aria-label="Header ${scope.entries.length} key"]`)
            field?.focus()
            field?.setSelectionRange(key.length, key.length)
          }
        }}
      />
    </div>
  </div>
  <span role="status" class="sr-only">{announcement}</span>
</div>
