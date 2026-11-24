<script lang="ts">
  import FileTextIcon from 'phosphor-svelte/lib/FileTextIcon'
  let { path, added = 0, removed = 0 }: { path: string; added?: number; removed?: number } = $props()
</script>

<span
  class="inline-flex items-center gap-1 rounded-md bg-inset px-1 py-0.5 text-ui-11 [&>code]:wrap-anywhere *:data-[tone=success]:text-success *:data-[tone=error]:text-destructive"
>
  <FileTextIcon size={14} aria-hidden="true" /><code>{path}</code><span data-tone="success">+{added}</span><span
    data-tone="error"
  >
    −{removed}
  </span>
</span>
