<script lang="ts">
  import { onMount } from 'svelte'
  import { DemoAuthController } from './demo-auth-controller'
  import { setAuthController } from '../../src/context'
  import { Form } from '../../src/components/form'
  import EmailField from '../../src/components/EmailField.svelte'
  import CodeEntry from '../../src/components/CodeEntry.svelte'
  import Button from '../../src/components/Button.svelte'
  import SubscriptionProbe from './SubscriptionProbe.svelte'
  const controller = new DemoAuthController({ mode: 'signup' })
  setAuthController(controller)
  onMount(controller.start)
  let remaining = $derived(controller.store.getState().remaining)
</script>

<main class="grid max-w-180 gap-4 p-4 font-sans text-foreground">
  <h1>Controller subscriptions</h1>
  <p>One controller, independent draft and workflow subscriptions. Counts record observed value changes.</p>
  <SubscriptionProbe label="email" value={controller.form.get('email')} /><SubscriptionProbe
    label="pending"
    value={controller.store.getState().pending}
  /><SubscriptionProbe label="code" value={controller.form.get('code')} /><SubscriptionProbe
    label="countdown"
    value={remaining}
  />
  <Form.Root model={controller.form} class="grid gap-4">
    <label for="subscription-email">Email address</label><EmailField id="subscription-email" name="email" /><CodeEntry
      name="code"
      aria-label="Email code"
    />
  </Form.Root>
  <p>Remaining: {remaining}</p>
  <Button onclick={() => controller.store.setState({ remaining: remaining + 1 })}>Advance countdown</Button>
</main>
