<script lang="ts">
  import { onDestroy, untrack } from 'svelte'
  import { Form, createForm, type CommitMode } from '../../src/components/form'
  import { TextField } from '../../src/components/fields'
  import { Switch } from '../../src/components/selection'
  import { Button } from '../../src/components/buttons'
  let { mode = 'button', reject = false }: { mode?: CommitMode; reject?: boolean } = $props()
  let saved = $state({ name: 'Research', enabled: true })
  const form = untrack(() =>
    createForm(
      { ...saved },
      {
        mode,
        submit: async (_values, patch, signal) => {
          await new Promise((resolve) => setTimeout(resolve, 800))
          if (signal.aborted) return false
          if (reject) throw new Error('Mock rejection')
          saved = { ...saved, ...patch }
          return patch
        },
      },
    ),
  )
  onDestroy(form.stop)
</script>

<div class="grid max-w-lg gap-4 p-6">
  <Form.Root model={form} class="grid gap-4">
    <label class="grid gap-2 text-sm">Name<TextField name="name" /></label><label class="flex items-center gap-2 text-sm">
      <Switch name="enabled" />Enabled
    </label>
    <div class="flex gap-3">
      <Form.Submit>Apply</Form.Submit><Form.Reset>Reset</Form.Reset><Form.Status>
        {#snippet children(status)}<span class="text-xs text-muted-foreground">
            {status.pending ? 'Saving…' : status.error ? 'Save failed' : status.dirty ? 'Unsaved' : 'Saved'}
          </span>{/snippet}
      </Form.Status>
    </div>
    <Button type="button" onclick={() => form.retry()}>Retry latest draft</Button>
  </Form.Root>
  <p class="text-xs text-muted-foreground">
    Draft: {String(form.get('name'))}. Confirmed: {saved.name}, {saved.enabled ? 'enabled' : 'disabled'}.
  </p>
  <p class="text-xs text-muted-foreground">
    Button mode suppresses blur and Enter. Other modes commit only at their configured trigger.
  </p>
  <TextField aria-label="Standalone input" placeholder="Outside the form" />
</div>
