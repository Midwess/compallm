<script lang="ts">
  import ResourceAction from './ResourceAction.svelte'
  import type { ButtonProps } from './Button.svelte'
  let { children, ...props }: ButtonProps = $props()
</script>

<ResourceAction {...props}>
  {#if children}{@render children()}{:else}Milestone{/if}
</ResourceAction>
