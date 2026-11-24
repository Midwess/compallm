<script lang="ts">
  import { createStore } from '../../src/state.svelte'
  import { InvitationPage, type InvitationController } from '../../src/pages/invitation'

  import { untrack } from 'svelte'

  import { setInvitationController } from '../../src/context'

  let {
    step = 'continue',
    busy = false,
    error = null,
    unavailable = false,
    loading = false,
  }: {
    step?: 'continue' | 'code' | 'confirmed'
    busy?: boolean
    error?: string | null
    unavailable?: boolean
    loading?: boolean
  } = $props()

  let controller: InvitationController = $state(
    untrack(() => {
      const store = createStore<ReturnType<InvitationController['store']['getState']>>(() => ({
        context: unavailable
          ? null
          : { organizationName: 'Research', email: 'alex@example.com', expiresAt: 2000000000, signup: true },
        loading,
        step,
        error: unavailable ? 'Invitation is expired, revoked, consumed, or unavailable.' : error,
        busy,
      }))
      return {
        store,
        continue: async () => {
          store.setState({ step: 'code' })
          return true
        },
        confirm: async () => {
          store.setState({ error: 'Incorrect verification code' })
          return false
        },
      }
    }),
  )
  setInvitationController(controller)
</script>

<InvitationPage></InvitationPage>
