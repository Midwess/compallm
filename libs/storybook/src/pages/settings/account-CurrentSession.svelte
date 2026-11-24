<script lang="ts">
  import { describeSessionBrowser } from './session-browser'

  import type { AuthController } from '../auth/types'

  import { untrack } from 'svelte'

  import { readStore } from '../page-state.svelte'
  import { getSettingsController } from '../../context'
  import { SessionItem } from './account'

  let {
    controller,
  }: {
    controller: AuthController
  } = $props()
  const __settings = getSettingsController()
  let userAgent = $derived(readStore(controller.store, (state) => state.currentUser?.userAgent))
  let ipAddress = $derived(readStore(controller.store, (state) => state.currentUser?.ipAddress))
  let sessionId = $derived(readStore(controller.store, (state) => state.currentUser?.id))
  let signedInAt = $derived(readStore(controller.store, (state) => state.currentUser?.signedInAt))
  let lastSeenAt = $derived(readStore(controller.store, (state) => state.currentUser?.lastSeenAt))
  let saved = $derived(readStore(__settings.store, (state) => state.sessionLocation))
  const settings = untrack(() => getSettingsController())
  let location = $derived(saved && saved.sessionId === sessionId && saved.ipAddress === ipAddress ? saved.location : null)
  let browser = $derived((() => describeSessionBrowser(userAgent))())
</script>

<SessionItem {browser} {location} {ipAddress} {signedInAt} {lastSeenAt} current onRevoke={settings.signOut}></SessionItem>
