<script lang="ts">
  import { onMount } from 'svelte'
  import { RoleUpdateInput } from '@compallm/contract/validation'
  import { Form, createForm } from '../../src/components/form'
  import TextField from '../../src/components/TextField.svelte'
  let { edge = false }: { edge?: boolean } = $props()
  const form = createForm(
    { name: '' },
    {
      mode: 'button',
      command: true,
      parse: (values) => {
        const result = RoleUpdateInput.safeParse(values)
        return result.success ? { values: { name: result.data.name ?? '' } } : { issues: result.error.issues }
      },
      submit: (_values, patch) => patch,
    },
  )
  onMount(() => {
    void form.submit()
  })
</script>

<div class={edge ? 'flex min-h-48 justify-end p-2' : 'min-h-48 p-6'}>
  <Form.Root model={form} class="grid w-72 content-start gap-3">
    <label class="grid gap-2 text-sm">Role name<TextField name="name" /></label><Form.Error name="name" />
    <p class="text-xs text-muted-foreground">The message opens for five seconds. Hover or focus the warning to reopen it.</p>
    <Form.Submit>Validate</Form.Submit>
  </Form.Root>
</div>
