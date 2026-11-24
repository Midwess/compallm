<script lang="ts">
  import { optionalContext } from '../../components/form-context.svelte'
  import { untrack } from 'svelte'

  import { readStore } from '../page-state.svelte'
  import { getPasskeyController, getAuthController } from '../../context'
  import { PasskeyDialogContent } from './index'

  const controller = untrack(() => getPasskeyController())
  const auth = untrack(() => optionalContext(getAuthController))
  let dialog = $derived(readStore(controller.store, (s) => s.dialog))
  let enrollment = $derived(readStore(controller.store, (s) => s.enrollment))
  let reauthentication = $derived(readStore(controller.store, (s) => s.reauthentication))
  let error = $derived(readStore(controller.store, (s) => s.error))
</script>

{#if reauthentication && auth?.requestVerification}<PasskeyDialogContent suspended
  ></PasskeyDialogContent>{:else}{#if dialog?.kind === 'add' && enrollment === 'device' && !reauthentication && !error}{:else}{#if dialog}<PasskeyDialogContent
      ></PasskeyDialogContent>{:else}{/if}{/if}{/if}
