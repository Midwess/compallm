<script lang="ts">
  import SearchboxRoot from './SearchboxRoot.svelte'
  import SearchboxInput from './SearchboxInput.svelte'
  import type { ComponentProps } from 'svelte'
  let { class: className, value = $bindable(''), ...props }: ComponentProps<typeof SearchboxInput> = $props()
</script>

<SearchboxRoot class={className}><SearchboxInput {...props} bind:value /></SearchboxRoot>
