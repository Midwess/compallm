<script lang="ts">
  import type { Snippet } from 'svelte'
  import { setPermissionController } from '../src/context'
  import { createPermissionStore, permissionKey, type PermissionCheck } from '../src/permission'
  let { children, deniedActions = [] }: { children?: Snippet; deniedActions?: string[] } = $props()
  const store = createPermissionStore()
  setPermissionController({
    store,
    async evaluate(checks: PermissionCheck[]) {
      store.setState((state) => ({
        entries: {
          ...state.entries,
          ...Object.fromEntries(
            checks.map((check) => [
              permissionKey(check),
              { check, status: deniedActions.includes(check.action) ? ('denied' as const) : ('allowed' as const) },
            ]),
          ),
        },
      }))
    },
  })
</script>

<div class="min-h-dvh bg-background font-sans text-foreground">{@render children?.()}</div>
