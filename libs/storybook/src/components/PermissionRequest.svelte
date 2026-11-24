<script lang="ts">
  import { getPermissionController } from '../context'
  import { permissionChecks, permissionKey, type PermissionRequestInput } from '../permission'
  import { untrack, type Snippet } from 'svelte'
  let { children, pending, ...request }: PermissionRequestInput & { children: Snippet<[boolean]>; pending?: Snippet } = $props()
  const controller = getPermissionController()
  let checks = $derived(permissionChecks(request))
  let entries = $derived(controller.store.getState().entries)
  $effect(() => {
    const pendingChecks = checks
    untrack(() => {
      void controller.evaluate(pendingChecks)
    })
  })
  let allowed = $derived(
    checks.length > 0 &&
      (request.match === 'any'
        ? checks.some((check) => entries[permissionKey(check)]?.status === 'allowed')
        : checks.every((check) => entries[permissionKey(check)]?.status === 'allowed')),
  )
</script>

{#if pending && checks.some((check) => !entries[permissionKey(check)])}{@render pending()}{:else}{@render children(
    allowed,
  )}{/if}
