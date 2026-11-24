<script lang="ts">
  import { untrack } from 'svelte'

  import { getUserPreferenceController } from '../../context'
  import { CustomizableDestinations, PreferenceDestinations } from './index'

  const preferences = untrack(() => getUserPreferenceController())
</script>

{#if preferences}<PreferenceDestinations controller={preferences}></PreferenceDestinations>{:else}<CustomizableDestinations
  ></CustomizableDestinations>{/if}
