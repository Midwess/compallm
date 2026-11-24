<script lang="ts">
  import { untrack } from 'svelte'
  import { setDialog } from './dialog-context'
  import type { Snippet } from 'svelte'
  let {
    open = $bindable(false),
    defaultOpen = false,
    onOpenChange,
    children,
  }: { open?: boolean; defaultOpen?: boolean; onOpenChange?: (open: boolean) => void; children?: Snippet } = $props()
  untrack(() => {
    if (defaultOpen && open === false) open = true
  })
  let guard: (() => void) | undefined
  const id = $props.id()
  function setOpen(value: boolean) {
    if (value === open) return
    open = value
    onOpenChange?.(value)
  }
  setDialog({
    get open() {
      return open
    },
    setOpen,
    close() {
      if (guard) guard()
      else setOpen(false)
    },
    titleId: id + '-title',
    descriptionId: id + '-description',
    registerDiscard(value) {
      guard = value
    },
  })
</script>

{@render children?.()}
