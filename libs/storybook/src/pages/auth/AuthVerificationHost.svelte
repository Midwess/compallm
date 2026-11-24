<script lang="ts">
  import { getAuthController } from '../../context'
  import AccessVerificationDialog from './AccessVerificationDialog.svelte'
  const controller = getAuthController()
</script>

{#if controller.store.getState().verificationActive}<AccessVerificationDialog
    open
    onOpenChange={(open) => {
      if (!open) controller.cancelVerification?.()
    }}
  />{/if}
