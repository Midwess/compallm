<script lang="ts" generics="T">
  import { getVisibleColumns } from './table-context'

  import { untrack, tick } from 'svelte'
  import type { Snippet } from 'svelte'
  import CaretDownIcon from 'phosphor-svelte/lib/CaretDownIcon'
  import { cx, type Content as Value } from './shared'
  import Content from './Content.svelte'
  import CollectionRow from './CollectionRow.svelte'
  import type { CollectionPaging } from './collection-types'
  import TableLoadMore from './TableLoadMore.svelte'
  import { interaction, animation, transition } from './motion'
  import TableLoadingRows from './TableLoadingRows.svelte'
  import { getLoadingContext } from '../context'
  import { optionalContext } from './form-context.svelte'
  import { animate } from 'motion'
  import LoadingScope from './LoadingScope.svelte'
  let {
    variant = 'default',
    initialLoading = false,
    label,
    count,
    items,
    getKey,
    children,
    columns = 3,
    open = $bindable(true),
    defaultOpen = true,
    onOpenChange,
    action,
    paging,
  }: {
    variant?: 'default' | 'directory'
    initialLoading?: boolean
    label: Value
    count?: number | string
    items?: readonly T[]
    getKey?: (item: T) => string
    children?: Snippet<[T, number]> | Snippet
    columns?: number
    open?: boolean
    defaultOpen?: boolean
    onOpenChange?: (value: boolean) => void
    action?: Snippet
    paging?: CollectionPaging
  } = $props()
  const loading = optionalContext(getLoadingContext)
  untrack(() => {
    if (!defaultOpen) open = false
  })
  let initial = $derived(initialLoading || !!loading?.initial)
  let directory = $derived(variant === 'directory')
  const visibleColumns = getVisibleColumns()
  let body: HTMLTableSectionElement
  let previous = untrack(() => open)
  $effect(() => {
    const expanded = open
    if (items) return
    let cancelled = false
    const controls: ReturnType<typeof animate>[] = []
    void tick().then(() => {
      if (cancelled) return
      const rows = Array.from(body.querySelectorAll<HTMLTableRowElement>('tr[data-slot="table-row"]'))
      const changed = previous !== expanded
      previous = expanded
      for (const row of rows) {
        row.inert = !expanded
        row.hidden = !expanded && !changed
        if (!changed) {
          row.style.opacity = expanded ? '1' : '0'
          row.style.transform = ''
        }
      }
      if (!changed) return
      for (const row of rows)
        controls.push(
          animate(row, { opacity: expanded ? 1 : 0, y: expanded ? 0 : -animation.distance }, transition('disclosure')),
        )
      void Promise.all(controls).then(() => {
        if (cancelled) return
        if (!expanded)
          rows.forEach((row) => {
            row.hidden = true
          })
        controls.forEach((control) => control.stop())
      })
    })
    return () => {
      cancelled = true
      controls.forEach((control) => control.stop())
    }
  })
</script>

<LoadingScope ready={!initial}>
  <tbody
    bind:this={body}
    aria-busy={initial || undefined}
    inert={initial || undefined}
    data-state={open ? 'open' : 'closed'}
    data-slot="table-category"
    class="[&:not(:first-of-type)>tr:first-child]:table-row"
  >
    <tr aria-hidden="true" class="hidden h-2"><td colspan={visibleColumns() || columns} class="h-2 p-0"></td></tr><tr
      data-slot="table-category-header"
      class={directory ? 'h-8 border-0' : 'h-10 border-0'}
    >
      <td
        colspan={action ? Math.max(1, (visibleColumns() || columns) - 1) : visibleColumns() || columns}
        class={cx(
          'sticky top-8 z-10 rounded-l-md bg-elevated p-0 translate-y-(--category-offset,0px)',
          !action && 'rounded-r-md',
          directory && 'rounded-lg bg-inset',
        )}
      >
        <button
          type="button"
          aria-expanded={open}
          aria-label={`${open ? 'Collapse' : 'Expand'} ${typeof label === 'string' ? label : 'category'}`}
          class={cx(
            'flex w-full cursor-pointer items-center gap-2 px-2 text-left focus-visible:outline-2 focus-visible:outline-ring',
            directory ? 'h-8 text-xs text-muted-foreground' : 'min-h-10',
          )}
          onclick={() => {
            open = !open
            onOpenChange?.(open)
          }}
          use:interaction
        >
          <span class={cx('inline-flex size-2 shrink-0', !open && '-rotate-90')}>
            <CaretDownIcon weight="fill" aria-hidden="true" class="size-2" />
          </span><span class={directory ? 'min-w-0' : 'min-w-0 text-xsm font-medium'}>
            <Content value={label} />
          </span>{#if count !== undefined}<span class="text-xs font-normal text-muted-foreground">{count}</span>{/if}
        </button>
      </td>{#if action}<td class="sticky top-8 z-10 rounded-r-md bg-elevated px-2 py-0 translate-y-(--category-offset,0px)">
          {@render action()}
        </td>{/if}
    </tr>{#if open}{#if initial}<TableLoadingRows
        />{:else if items}{#each items as item, index (getKey?.(item) ?? index)}<CollectionRow
            rowKey={getKey?.(item) ?? String(index)}
            {index}
          >
            {@render (children as Snippet<[T, number]>)?.(item, index)}
          </CollectionRow>{/each}{/if}{#if paging}<TableLoadMore {...paging} initialLoading={initial} />{/if}{/if}
    {#if open && items}<tr class="sr-only">
        <td colspan={visibleColumns() || columns} aria-live="polite">{items.length} rows loaded</td>
      </tr>{/if}
    {#if !items}{@render (children as Snippet)?.()}{/if}
  </tbody>
</LoadingScope>
