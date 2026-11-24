<script lang="ts">
  import { untrack } from 'svelte'
  import { InvitationCreateInput } from '@compallm/contract/validation'
  import { invitationTextSchema } from '@compallm/contract/invitation'
  import { createForm, Form, TokenField } from '../../src'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
  let { initial = 'alex@example.com, sam@example.com; incomplete' }: { initial?: string } = $props()
  const form = createForm(
    { email: untrack(() => initial) },
    {
      parse(values) {
        const result = invitationTextSchema.safeParse(values)
        return result.success ? { values: result.data } : { issues: result.error.issues }
      },
    },
  )
</script>

<Preview>
  <Sample label="Recipients · delimiters validate addresses" wide>
    <Form.Root model={form}>
      <div class="relative">
        <TokenField
          name="email"
          aria-label="Invitation recipients"
          placeholder="email@example.com, another@example.com…"
          parseToken={(email) => {
            const result = InvitationCreateInput.safeParse({ email })
            return result.success ? result.data.email : null
          }}
        /><Form.Error name="email" />
      </div>
    </Form.Root>
  </Sample>
</Preview>
