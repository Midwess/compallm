<script lang="ts">
  import Selection from './Selection.svelte'
  import type { ComponentProps } from 'svelte'
  let { checked = $bindable(false), ...props }: Omit<ComponentProps<typeof Selection>, 'kind'> = $props()
</script>

<Selection kind="checkbox" {...props} bind:checked />
