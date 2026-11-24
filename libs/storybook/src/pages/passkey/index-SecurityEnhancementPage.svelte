<script lang="ts">
  import AccessLayout from '../auth/AccessLayout.svelte'
  import { Button } from '../../components/buttons'

  import { untrack } from 'svelte'

  import { readStore } from '../page-state.svelte'
  import { getPasskeyController } from '../../context'

  let {
    onSkip,
  }: {
    onSkip: () => void
  } = $props()

  const controller = untrack(() => getPasskeyController())
  let busy = $derived(readStore(controller.store, (s) => s.busy))
</script>

<AccessLayout>
  <div class="grid gap-6">
    <h1 class="m-0 text-ui-28 leading-10 font-medium">Enhance your security.</h1>
    <p class="m-0 text-sm leading-6 text-muted-foreground">
      Add a passkey to sign in with your fingerprint, face, or security key. You can also set this up later in settings.
    </p>
    <Button variant="primary" size="authentication" onclick={controller.openAdd} loading={busy}>Passkey</Button><Button
      variant="ghost"
      onclick={onSkip}
    >
      Skip for now
    </Button>
  </div>
</AccessLayout>
