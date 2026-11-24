<script lang="ts">
  import Button, { type ButtonProps } from './Button.svelte'
  import { getDialog } from './dialog-context'
  let { children, onclick, ...props }: ButtonProps = $props()
  const scope = getDialog()
</script>

<Button
  {...props}
  onclick={(event) => {
    onclick?.(event)
    if (!event.defaultPrevented && !props.disabled) scope.close()
  }}
>
  {#if children}{@render children()}{:else}Cancel{/if}
</Button>
