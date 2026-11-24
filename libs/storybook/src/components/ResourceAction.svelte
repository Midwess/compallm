<script lang="ts">
  import Button, { type ButtonProps } from './Button.svelte'
  import PlusIcon from 'phosphor-svelte/lib/PlusIcon'
  let { children, ...props }: ButtonProps = $props()
</script>

<Button variant="ghost" size="content" {...props}>
  <PlusIcon size={14} />{#if children}{@render children()}{:else}Add document or link…{/if}
</Button>
