<script lang="ts">
  import { page } from '$app/state'
  import RouteGate from '../../../RouteGate.svelte'
  import Workspace from '../../../Workspace.svelte'
  let { children } = $props()
</script>

<RouteGate organization={page.params.organization}><Workspace>{@render children()}</Workspace></RouteGate>
