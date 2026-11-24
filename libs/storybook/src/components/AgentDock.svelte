<script lang="ts">
  import Button, { type ButtonProps } from './Button.svelte'
  import SparkleIcon from 'phosphor-svelte/lib/SparkleIcon'
  let { children, ...props }: ButtonProps = $props()
</script>

<Button variant="ghost" {...props}>
  <SparkleIcon size={14} />{#if children}{@render children()}{:else}Agent{/if}
</Button>
