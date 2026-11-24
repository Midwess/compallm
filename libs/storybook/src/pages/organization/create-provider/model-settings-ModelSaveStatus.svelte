<script lang="ts">
  import { Button } from '../../../components/buttons'

  let {
    form,
  }: {
    form: import('../../../components/form').FormModel<Partial<import('./types').CreateProviderState>>
  } = $props()
</script>

<div class="flex items-center gap-2" role="status" aria-live="polite">
  <span class="text-xs text-muted-foreground">
    {#if form.pending}Saving…{:else}{#if form.error}Changes could not be saved.{:else}{#if form.dirty}Unsaved changes{:else}Saved{/if}{/if}{/if}
  </span>{#if form.error}<Button class="h-7 w-fit px-3 text-xs" onclick={() => void form.retry()}>Retry</Button>{:else}{/if}
</div>
