<script lang="ts">
  import {
    setPermissionController,
    createPermissionStore,
    permissionKey,
    publishPermissions,
    PermissionRequest,
    Button,
    DisabledTooltip,
    type PermissionStatus,
  } from '../../src'
  let { scenario = 'Allowed' }: { scenario?: string } = $props()
  let restricted = $state(false)
  let status = $derived(scenario === 'Failed' ? 'error' : scenario === 'Denied' || restricted ? 'denied' : 'allowed')
  const store = createPermissionStore()
  const controller = {
    store,
    async evaluate(checks: import('../../src').PermissionCheck[]) {
      store.setState((state) => ({
        entries: {
          ...state.entries,
          ...Object.fromEntries(checks.map((check) => [permissionKey(check), { check, status: status as PermissionStatus }])),
        },
      }))
    },
  }
  setPermissionController(controller)
  function toggle() {
    restricted = !restricted
    publishPermissions(
      controller,
      Object.fromEntries(
        Object.entries(store.getState().entries).map(([key, entry]) => [
          key,
          { ...entry, status: restricted ? 'denied' : 'allowed' },
        ]),
      ),
    )
  }
</script>

<div class="grid justify-items-start gap-4 bg-background p-6 text-foreground">
  {#if scenario === 'Reactive'}<Button onclick={toggle}>
      {restricted ? 'Restore fixture access' : 'Revoke fixture access'}
    </Button>
    <PermissionRequest actions={['role.create']}>
      {#snippet children(allowed)}<DisabledTooltip
          disabled={!allowed}
          reason="You do not have permission to perform this action."
        >
          <Button disabled={!allowed}>Create role</Button>
        </DisabledTooltip>{/snippet}
    </PermissionRequest>
    <PermissionRequest actions={['role.read']}>
      {#snippet children(allowed)}{#if allowed}<p>Visible role content</p>{/if}{/snippet}
    </PermissionRequest>
  {:else}<p>Permission decision: {status}</p>
    <PermissionRequest actions={['role.create']}>
      {#snippet children(allowed)}{#if allowed}<Button>Create role (hidden when restricted)</Button>{/if}{/snippet}
    </PermissionRequest>
    <PermissionRequest actions={['role.metadata.update']} resource={{ type: 'Role', id: '12' }}>
      {#snippet children(allowed)}<DisabledTooltip
          disabled={!allowed}
          reason="You do not have permission to perform this action."
        >
          <Button disabled={!allowed}>Edit role</Button>
        </DisabledTooltip>{/snippet}
    </PermissionRequest>
    <PermissionRequest actions={['role.read']}>
      {#snippet children(allowed)}<p>{allowed ? 'Role list content' : 'You do not have access to roles.'}</p>{/snippet}
    </PermissionRequest>
  {/if}
</div>
