<script lang="ts">
  import { onDestroy, onMount } from 'svelte'
  import { FormPracticeController, setPracticeController } from './controller.svelte'
  import Fields from './Fields.svelte'
  import Button from '../../src/components/Button.svelte'
  import CheckboxOption from '../../src/components/CheckboxOption.svelte'
  import Dropdown from '../../src/components/Dropdown.svelte'
  import CodeEntry from '../../src/components/CodeEntry.svelte'
  import TextField from '../../src/components/TextField.svelte'
  import { Form } from '../../src/components/form'
  import { createForm, type CommitMode } from '../../src/components/form-model.svelte'
  import { Dialog } from '../../src/components/dialog'
  const controller = new FormPracticeController()
  setPracticeController(controller)
  onDestroy(() => controller.stop())
  onMount(controller.form.mount)
  let mode = $state<CommitMode>('blur')
  let show = $state(true)
  let large = $state(false)
  let standalone = $state('Standalone value')
  const largeForm = createForm(Object.fromEntries(Array.from({ length: 1000 }, (_, i) => ['field' + i, 'Value ' + i])))
  const grouped = createForm(
    { first: 'Alpha', second: 'Beta' },
    { mode: 'blur', fields: { first: { group: 'pair' }, second: { group: 'pair' } }, submit: (values) => values },
  )
</script>

<main class="min-h-dvh bg-background p-6 font-sans text-foreground">
  <div class="mx-auto grid max-w-5xl gap-6">
    <h1 class="text-2xl font-medium">Form practice</h1>
    <p class="text-xsm text-muted-foreground">
      Explore commit modes, partial saves, normalization, delayed responses, and recovery. All operations are offline.
    </p>
    <section class="grid gap-4 rounded-control bg-inset p-4">
      <div class="flex flex-wrap items-center gap-4">
        <Dropdown
          label="Commit mode"
          value={mode}
          onValueChange={(v) => {
            mode = v as CommitMode
            controller.configureMode(mode)
          }}
          options={['button', 'change', 'blur', 'debounce', 'complete'].map((value) => ({ value, label: value }))}
        /><CheckboxOption checked={controller.slow} onchange={(e) => (controller.slow = e.currentTarget.checked)}>
          Slow response (3 seconds)
        </CheckboxOption><Button onclick={() => (controller.rejectNext = !controller.rejectNext)}>
          {controller.rejectNext ? 'Next save will fail' : 'Reject next save'}
        </Button><Button onclick={() => (show = !show)}>{show ? 'Detach fields' : 'Reattach fields'}</Button>
      </div>
      {#if show}<Fields />{/if}
    </section>
    <section class="grid gap-3 rounded-control bg-inset p-4">
      <h2 class="text-sm font-medium">Confirmed state and submitted snapshot</h2>
      <pre class="overflow-auto font-mono text-xs">{JSON.stringify(
          {
            confirmed: controller.confirmed,
            request: controller.request,
            response: controller.response,
            commits: controller.commits,
          },
          null,
          2,
        )}</pre>
    </section>
    <section class="grid gap-4 rounded-control bg-inset p-4">
      <h2 class="text-sm font-medium">Completion commit</h2>
      <Form.Root model={controller.otp}>
        <CodeEntry name="code" aria-label="Verification code" /><Form.Error name="code" />
      </Form.Root>
      <h2 class="text-sm font-medium">Standalone field</h2>
      <TextField bind:value={standalone} aria-label="Standalone field" />
      <h2 class="text-sm font-medium">Independent dialog scope</h2>
      <Dialog.Root>
        <Dialog.Trigger>Open editable dialog</Dialog.Trigger><Dialog.Content>
          <Dialog.Header><Dialog.Title>Scoped edit</Dialog.Title></Dialog.Header><Dialog.Body>
            <Form.Root model={grouped}>
              <TextField name="first" aria-label="First grouped value" /><TextField
                name="second"
                aria-label="Second grouped value"
              /><Form.Submit>Save group</Form.Submit>
            </Form.Root><Dialog.Discard dirty={grouped.dirty} />
          </Dialog.Body><Dialog.Footer><Dialog.Close>Close</Dialog.Close></Dialog.Footer>
        </Dialog.Content>
      </Dialog.Root><Button onclick={() => (large = !large)}>
        {large ? 'Hide' : 'Show'} 1,000 fields
      </Button>{#if large}<Form.Root model={largeForm}>
          <div class="grid grid-cols-2 gap-2">
            {#each Array(1000) as _, index (index)}<TextField name={'field' + index} aria-label={`Field ${index + 1}`} />{/each}
          </div>
        </Form.Root>{/if}
    </section>
  </div>
</main>
