<script lang="ts">
  import { Toast } from '@compallm/storybook/components/feedback'
  import { notifications, dismissNotice } from './toast'
  let records = $derived(notifications.getState().records)
</script>

<div
  class="pointer-events-none fixed top-4 right-4 z-50 grid max-w-[calc(100vw-2rem)] gap-3 font-sans text-xsm text-foreground"
>
  {#each records as record (record.id)}<div class="pointer-events-auto">
      <Toast tone={record.tone} message={record.message} action={record.action} onDismiss={() => dismissNotice(record.id)} />
    </div>{/each}
</div>
