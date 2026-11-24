<script lang="ts">
  import type { ComponentProps } from 'svelte'
  import Section from './SidebarSection.svelte'
  let { open = $bindable(true), ...props }: Omit<ComponentProps<typeof Section>, 'folder'> = $props()
</script>

<Section {...props} bind:open folder />
