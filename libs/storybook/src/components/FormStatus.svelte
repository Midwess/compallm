<script lang="ts">
  import { currentForm } from './form-context.svelte'
  import type { Snippet } from 'svelte'
  let { children }: { children?: Snippet<[{ pending: boolean; dirty: boolean; error: unknown }]> } = $props()
  const form = currentForm()
</script>

{#if children}{@render children({
    pending: form?.pending ?? false,
    dirty: form?.dirty ?? false,
    error: form?.error,
  })}{:else}<span role="status">
    {form?.error ? 'Couldn’t save' : form?.pending ? 'Saving…' : form?.dirty ? 'Unsaved' : 'Saved'}
  </span>{/if}
