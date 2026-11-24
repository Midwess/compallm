<script lang="ts">
  import Button, { type ButtonProps } from './Button.svelte'
  import { getDialog } from './dialog-context'
  let { children, onclick, ...props }: ButtonProps = $props()
  const scope = getDialog()
</script>

<Button
  {...props}
  aria-haspopup="dialog"
  aria-expanded={scope.open}
  data-state={scope.open ? 'open' : 'closed'}
  onclick={(event) => {
    onclick?.(event)
    if (!event.defaultPrevented && !props.disabled) {
      event.currentTarget.focus({ preventScroll: true })
      scope.setOpen(true)
    }
  }}
>
  {@render children?.()}
</Button>
