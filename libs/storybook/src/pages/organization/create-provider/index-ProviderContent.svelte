<script lang="ts">
  import { ProviderChooser } from './chooser'
  import { ProviderConfiguration } from './configuration'

  import { readStore } from '../../page-state.svelte'
  import { getCreateProviderController } from '../../../context'

  const __provider = getCreateProviderController()
  let step = $derived(readStore(__provider.store, (state) => state.step))
</script>

{#if step === 'choose'}<ProviderChooser></ProviderChooser>{:else}<ProviderConfiguration></ProviderConfiguration>{/if}
