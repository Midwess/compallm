<script lang="ts">
  import FileTextIcon from 'phosphor-svelte/lib/FileTextIcon'
  let { size = 24 }: { size?: number } = $props()
</script>

<FileTextIcon {size} aria-hidden="true" />
