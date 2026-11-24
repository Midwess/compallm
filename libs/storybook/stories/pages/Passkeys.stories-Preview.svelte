<script lang="ts">
  import { DemoAuthController } from './demo-auth-controller'
  import { AuthVerificationHost } from '../../src/pages/auth'

  import { createStore } from '../../src/state.svelte'

  import {
    SecurityEnhancementPage,
    PasskeySettings,
    PasskeyDialogs,
    type PasskeyController,
    type PasskeyState,
  } from '../../src/pages/passkey'
  import { untrack } from 'svelte'

  import { setAuthController, setPasskeyController } from '../../src/context'

  let {
    screen = false,
    initial = {},
  }: {
    screen?: boolean
    initial?: Partial<PasskeyState>
  } = $props()

  let controller: PasskeyController = $state(
    untrack(() => {
      const store = createStore<PasskeyState>(() => ({
        records: [],
        enrollment: 'name',
        editingId: null,
        currentPasskeyId: null,
        status: 'ready',
        dialog: null,
        busy: false,
        error: null,
        reauthentication: false,
        emailSent: false,
        ...initial,
      }))
      return {
        store,
        load: async () => {
          store.setState({ status: 'ready', error: null })
        },
        openAdd: () => store.setState({ dialog: { kind: 'add' } }),
        openRemove: (id) => store.setState({ dialog: { kind: 'remove', id } }),
        close: () => store.setState({ dialog: null }),
        prepare: async () => {
          store.setState({ enrollment: 'name' })
          return true
        },
        edit: (id) => store.setState({ editingId: id }),
        rename: async (id, name) => {
          store.setState((state) => ({
            records: state.records.map((p) => (p.id === id ? { ...p, name } : p)),
            editingId: null,
          }))
          return true
        },
        add: async () => false,
        remove: async () => false,
      }
    }),
  )
  let auth = $state(
    untrack(() => {
      const result = new DemoAuthController({ mode: 'signin' })
      Object.assign(result, { requestVerification: async () => false })
      result.store.setState({ verificationActive: initial.reauthentication, step: initial.emailSent ? 'code' : 'choose' })
      return result
    }),
  )
  setAuthController(auth)
  setPasskeyController(controller)
</script>

{#if screen}<SecurityEnhancementPage onSkip={() => {}}></SecurityEnhancementPage>{:else}<main
    class="min-h-dvh bg-surface p-8 text-foreground"
  >
    <div class="mx-auto max-w-160"><PasskeySettings></PasskeySettings></div>
  </main>{/if}<PasskeyDialogs></PasskeyDialogs><AuthVerificationHost></AuthVerificationHost>
