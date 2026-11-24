<script lang="ts">
  import { readStore } from '../page-state.svelte'
  import { getSettingsController } from '../../context'
  import { SettingsDialogContent } from './dialogs'

  const __settings = getSettingsController()
  let dialog = $derived(readStore(__settings.store, (state) => state.dialog, ['dialog']))
</script>

{#if !dialog}{:else}<SettingsDialogContent></SettingsDialogContent>{/if}
