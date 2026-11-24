<script lang="ts">
  import Button, { type ButtonProps } from './Button.svelte'
  import { currentForm } from './form-context.svelte'
  let { children, onclick, ...props }: ButtonProps = $props()
  const form = currentForm()
</script>

<Button
  {...props}
  onclick={(e) => {
    onclick?.(e)
    if (!e.defaultPrevented) form?.reset()
  }}
>
  {#if children}{@render children()}{:else}Reset{/if}
</Button>
