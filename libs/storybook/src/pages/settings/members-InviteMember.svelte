<script lang="ts">
  import { zodResolver } from '../../components/form'

  import { Form, managedForm } from '../../components/form'
  import { TokenField, TextField } from '../../components/fields'
  import { Button } from '../../components/buttons'

  import { Select } from '../../components/dropdowns'

  import { InvitationCreateInput } from '@compallm/contract/validation'
  import { invitationTextSchema } from '@compallm/contract/invitation'

  import { untrack } from 'svelte'
  import { readStore } from '../page-state.svelte'
  import { getSettingsController } from '../../context'

  const __settings = getSettingsController()
  const c = untrack(() => getSettingsController())
  let busy = $derived(readStore(__settings.store, (s) => s.invitationBusy))
  let link = $derived(readStore(__settings.store, (s) => s.invitationLink))
  let admitted = $derived(readStore(__settings.store, (s) => s.invitationEmailAdmitted))
  let links = $derived(readStore(__settings.store, (s) => s.invitationLinks ?? []))
  const form = untrack(() =>
    managedForm(
      { email: '' },
      {
        resolver: zodResolver(invitationTextSchema),
        submit: async (values) => ((await c.pages.members.invite?.(values.email)) ? values : false),
      },
    ),
  )
</script>

<div class="space-y-4">
  <Form.Root model={form} class="space-y-4">
    <label class="grid gap-1 text-xsm font-medium">
      Email <div class="relative">
        <TokenField
          name="email"
          parseToken={(email) => {
            const result = InvitationCreateInput.safeParse({ email })
            return result.success ? result.data.email : null
          }}
          aria-label="Invite by email"
          placeholder="email@example.com, another@example.com…"
        ></TokenField><Form.Error name="email"></Form.Error>
      </div>
    </label><label class="grid gap-1 text-xsm font-medium">
      Role <Select.Root value="member">
        <Select.Trigger aria-label="Invitation role" class="h-8 w-full rounded-lg bg-elevated text-left">
          <Select.Value>
            Member <span class="font-normal text-muted-foreground">— Standard organization access</span>
          </Select.Value>
        </Select.Trigger><Select.Content><Select.Item value="member">Member</Select.Item></Select.Content>
      </Select.Root>
    </label>{#if link}<div class="space-y-2">
        <p class="text-xs text-muted-foreground">
          {#if admitted}Invitation email queued.{:else}Email was not queued. Share this link or resend explicitly.{/if}
        </p>
        {#if links.length}{#each links as item (item.email)}<div class="flex min-w-0 items-center gap-2">
              <span class="min-w-0 flex-1 truncate text-xs">{item.email}</span><Button
                onclick={() => c.pages.dialogs.copy(item.url)}
              >
                Copy link
              </Button>
            </div>{/each}{:else}<TextField aria-label="New invitation link" variant="constant" value={link}></TextField><Button
            onclick={() => c.pages.dialogs.copy(link)}
          >
            Copy invitation link
          </Button>{/if}
      </div>{:else}{/if}
    <div class="flex justify-end pt-6">
      <Button type="submit" variant="primary" loading={busy} class="h-8 px-3">Send invitations</Button>
    </div>
  </Form.Root>
</div>
