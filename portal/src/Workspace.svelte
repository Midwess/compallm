<script lang="ts">
  import type { Snippet } from 'svelte'
  import { getPortalControllers } from './context'
  import WorkspaceScope from './WorkspaceScope.svelte'
  let { children }: { children?: Snippet } = $props()
  const { auth } = getPortalControllers()
  let user = $derived(auth.store.getState().currentUser)
</script>

{#if user}{#key `${user.id}:${user.userId}:${user.organizationId}`}<WorkspaceScope {user}>
      {@render children?.()}
    </WorkspaceScope>{/key}{/if}
