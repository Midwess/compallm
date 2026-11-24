<script lang="ts">
  import { onMount, untrack } from 'svelte'
  import { page } from '$app/state'
  import { InvitationPage } from '@compallm/storybook/pages/invitation'
  import { setInvitationController } from '@compallm/storybook/context'
  import { getPortalControllers } from '../../context'
  import { InvitationController } from '../../auth/invitation-controller'
  const { auth, url } = getPortalControllers()
  const controller = new InvitationController(auth, url)
  untrack(() => controller.seed(page.data.invitationRead ?? null))
  setInvitationController(controller)
  onMount(() => controller.start())
</script>

<InvitationPage />
