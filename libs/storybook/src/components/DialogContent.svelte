<script lang="ts">
  import { getDialog } from './dialog-context'
  import { setFloatingContainer } from '../context'
  import { cx } from './shared'
  import type { HTMLDialogAttributes } from 'svelte/elements'
  import { floatingMotion } from './motion'
  let {
    children,
    class: className,
    element = $bindable(),
    role = 'dialog',
    ...props
  }: HTMLDialogAttributes & { element?: HTMLDialogElement } = $props()
  const scope = getDialog()
  setFloatingContainer(() => element ?? null)
  $effect(() => {
    if (!scope.open || !element) return
    const previous = document.activeElement
    element.showModal()
    return () => {
      element?.close()
      if (previous instanceof HTMLElement && previous.isConnected) previous.focus()
    }
  })
</script>

{#if scope.open}<dialog
    {...props}
    bind:this={element}
    {role}
    data-state="open"
    aria-labelledby={scope.titleId}
    oncancel={(e) => {
      props.oncancel?.(e)
      const prevented = e.defaultPrevented
      e.preventDefault()
      if (!prevented) scope.close()
    }}
    class={cx(
      'fixed inset-0 m-auto max-h-11/12 w-160 max-w-11/12 overflow-y-auto overscroll-y-none rounded-xl border border-border bg-inset p-0 font-sans text-xsm text-foreground backdrop:bg-background/70',
      className,
    )}
    use:floatingMotion
  >
    {@render children?.()}
  </dialog>{/if}
