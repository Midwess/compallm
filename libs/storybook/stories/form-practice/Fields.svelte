<script lang="ts">
  import { getPracticeController } from './controller.svelte'
  import { Form } from '../../src/components/form'
  import { FormField } from '../../src/components/form-field'
  import { Button } from '../../src/components/buttons'
  import MultilineField from '../../src/components/MultilineField.svelte'
  const controller = getPracticeController()
</script>

<Form.Root model={controller.form}>
  <div class="grid gap-5">
    <FormField.Root>
      <FormField.Label>Name</FormField.Label><FormField.Input name="name" /><FormField.Error name="name" />
    </FormField.Root>
    <FormField.Root>
      <FormField.Label>Description</FormField.Label><MultilineField name="description" /><FormField.Error name="description" />
    </FormField.Root>
    <div class="flex items-center gap-2">
      <Form.Submit>Save changes</Form.Submit><Button onclick={() => controller.form.reset()}>Reset</Button><Button
        onclick={() => controller.form.retry()}
        disabled={!controller.form.error}
      >
        Retry
      </Button>
    </div>
    <Form.Status>
      {#snippet children(status)}<p role="status" class="text-xs text-muted-foreground">
          {status.pending
            ? 'Saving…'
            : status.error
              ? 'Save failed; draft retained'
              : status.dirty
                ? 'Unsaved changes'
                : 'Saved'}
        </p>{/snippet}
    </Form.Status>
  </div>
</Form.Root>
