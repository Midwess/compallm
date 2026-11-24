<script lang="ts">
  import { getAuthController } from '../../context'
  import { Dialog } from '../../components/dialog'
  import AccessPanel from './AccessPanel.svelte'
  let {
    open = $bindable(false),
    onOpenChange,
    accountEmail,
    title,
    description,
  }: {
    open?: boolean
    onOpenChange?: (open: boolean) => void
    accountEmail?: string
    title?: string
    description?: string
  } = $props()
  const controller = getAuthController()
  let complete = $derived(controller.store.getState().step === 'complete')
</script>

<Dialog.Root
  open={open && !complete}
  onOpenChange={(next) => {
    open = next
    onOpenChange?.(next)
  }}
>
  <Dialog.Content class="relative w-72 max-w-[calc(100vw-2rem)] rounded-xl bg-inset p-4">
    <AccessPanel {accountEmail} {title} {description} intent="verify" />
    <div class="mt-2 text-center">
      <Dialog.Cancel variant="link" class="h-6 text-xs text-muted-foreground">Cancel</Dialog.Cancel>
    </div>
  </Dialog.Content>
</Dialog.Root>
