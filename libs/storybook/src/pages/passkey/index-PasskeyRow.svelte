<script lang="ts">
  import { Button } from '../../components/buttons'
  import { TextField } from '../../components/fields'
  import { Form, managedForm, contractResult } from '../../components/form'

  import { PasskeyRegistrationInput } from '@compallm/contract/validation'
  import { Row } from '../settings/shared'
  import { Popup } from '../../components/popup'
  import { SettingsDivider } from '../../components/settings'
  import PencilSimpleIcon from 'phosphor-svelte/lib/PencilSimpleIcon'
  import TrashIcon from 'phosphor-svelte/lib/TrashIcon'
  import DotsThreeIcon from 'phosphor-svelte/lib/DotsThreeIcon'
  import { untrack } from 'svelte'
  import { readStore } from '../page-state.svelte'
  import { getPasskeyController } from '../../context'

  import { credentialDescription } from './index'

  let {
    record,
    current,
    last,
  }: {
    record: import('./types').PasskeyState['records'][number]
    current: boolean
    last: boolean
  } = $props()

  const controller = untrack(() => getPasskeyController())
  let editing = $derived(readStore(controller.store, (s) => s.editingId === record.id))
  let busy = $derived(readStore(controller.store, (s) => s.busy))
  let error = $derived(readStore(controller.store, (s) => s.error))
  const model = untrack(() =>
    managedForm(
      { name: record.name },
      {
        mode: 'button',
        command: true,
        parse: (values) => contractResult(PasskeyRegistrationInput.safeParse(values), values),
        submit: async (
          values,
        ): Promise<
          | {
              name: string
            }
          | false
        > => {
          if (!(await controller.rename(record.id, values.name))) return false
          if (controller.store.getState().editingId === record.id && model.values.name === values.name) controller.edit(null)
          return values
        },
      },
    ),
  )
</script>

{#if editing}<li>
    <div class="p-4">
      <Form.Root
        {model}
        class="flex flex-wrap items-start gap-3"
        onkeydown={(event) => {
          if (event.key === 'Escape') {
            event.preventDefault()
            controller.edit(null)
          }
          if (event.key === 'Enter' && !event.isComposing) {
            event.preventDefault()
            void model.submit()
          }
        }}
      >
        <label class="min-w-0 flex-1">
          <span class="sr-only">Passkey name</span><TextField name="name" autofocus class="w-full"></TextField><Form.Error
            name="name"
          ></Form.Error>
        </label><Button variant="ghost" class="px-3" onclick={() => controller.edit(null)}>Cancel</Button><Button
          type="submit"
          variant="neutral"
          class="h-8 px-3"
          loading={busy}
        >
          Save
        </Button>{#if error}<p role="alert" class="w-full text-xs text-destructive">{error}</p>{:else}{/if}
      </Form.Root>
    </div>
    {#if !last}<SettingsDivider class="mx-4"></SettingsDivider>{:else}{/if}
  </li>{:else}<li>
    <Row
      label={record.name}
      description={current ? 'Used with current session' : credentialDescription(record.registeredAt, record.lastUsedAt)}
      {last}
    >
      <Popup.Root>
        <Popup.Trigger aria-label={`Actions for ${record.name}`} class="size-8 p-0">
          <DotsThreeIcon size={16} aria-hidden="true"></DotsThreeIcon>
        </Popup.Trigger><Popup.Content class="w-44" aria-label={`Actions for ${record.name}`}>
          <Popup.Item
            onSelect={() => {
              model.reset({ name: record.name })
              controller.edit(record.id)
            }}
          >
            <PencilSimpleIcon size={16} aria-hidden="true"></PencilSimpleIcon>Rename…
          </Popup.Item><Popup.Item class="text-destructive" onSelect={() => controller.openRemove(record.id)}>
            <TrashIcon size={16} aria-hidden="true"></TrashIcon>Revoke
          </Popup.Item>
        </Popup.Content>
      </Popup.Root>
    </Row>
  </li>{/if}
