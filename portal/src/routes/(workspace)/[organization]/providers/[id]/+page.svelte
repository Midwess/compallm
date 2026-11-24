<script lang="ts">
  import { page } from '$app/state'
  import ProviderScope from '../../../../../ProviderScope.svelte'
</script>

{#key page.params.id}<ProviderScope id={page.params.id} />{/key}
