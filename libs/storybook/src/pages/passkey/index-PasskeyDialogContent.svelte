<script lang="ts">
  import { Button } from '../../components/buttons'
  import { TextField } from '../../components/fields'
  import { Form, managedForm, contractResult } from '../../components/form'
  import { Dialog } from '../../components/dialog'
  import { PasskeyRegistrationInput } from '@compallm/contract/validation'

  import { untrack } from 'svelte'
  import { readStore } from '../page-state.svelte'
  import { getPasskeyController } from '../../context'

  let {
    suspended = false,
  }: {
    suspended?: boolean
  } = $props()

  const controller = untrack(() => getPasskeyController())
  let busy = $derived(readStore(controller.store, (s) => s.busy))
  let error = $derived(readStore(controller.store, (s) => s.error))
  let dialog = $derived(readStore(controller.store, (s) => s.dialog)!)
  let enrollment = $derived(readStore(controller.store, (s) => s.enrollment))
  let passkeyName = $derived(
    dialog.kind === 'remove' ? controller.store.getState().records.find((record) => record.id === dialog.id)?.name : undefined,
  )
  const form = untrack(() =>
    managedForm(
      { name: '', code: '' },
      {
        mode: 'button',
        command: true,
        parse: (values) => {
          const state = controller.store.getState()
          return state.dialog?.kind === 'add' && state.enrollment === 'name'
            ? contractResult(PasskeyRegistrationInput.safeParse({ name: values.name }), values)
            : { values }
        },
        submit: async (values) => {
          const state = controller.store.getState()
          if (state.dialog?.kind === 'add' && state.enrollment === 'device') {
            await controller.prepare()
            return false as const
          }
          if (state.dialog?.kind === 'add') await controller.add(values.name)
          else if (state.dialog?.kind === 'remove') await controller.remove(state.dialog.id)
          return false as const
        },
      },
    ),
  )
</script>

<Dialog.Root
  open={!suspended}
  onOpenChange={(open) => {
    if (!open) controller.close()
  }}
>
  <Dialog.Content
    class={dialog.kind === 'remove'
      ? 'w-120 max-w-[calc(100vw-2rem)] rounded-xl bg-inset md:top-[28.2svh] md:right-2 md:bottom-auto md:left-61'
      : 'w-120 max-w-[calc(100vw-2rem)] rounded-xl bg-inset'}
  >
    <Form.Root model={form}>
      <Dialog.Header class="border-0 px-6 pt-6 pb-0">
        <Dialog.Title class="text-ui-15 leading-6 font-semibold">
          {#if dialog.kind === 'add'}{#if enrollment === 'name'}Name your passkey{:else}Create a passkey{/if}{:else}{`Revoke access for "${passkeyName ?? 'passkey'}"?`}{/if}
        </Dialog.Title>{#if dialog.kind === 'add'}<Dialog.Close></Dialog.Close>{:else}{/if}
      </Dialog.Header><Dialog.Body class={dialog.kind === 'remove' ? 'grid gap-4 px-6 pt-1 pb-1' : 'grid gap-4 px-6 pt-1 pb-4'}>
        <Dialog.Description class="text-ui-15 leading-6">
          {#if dialog.kind === 'add'}{#if enrollment === 'name'}Give your passkey a name so you can recognize it later.{:else}Follow
              your device’s instructions to create a passkey.{/if}{:else}You cannot undo this action.{/if}
        </Dialog.Description>{#if dialog.kind === 'add' && enrollment === 'name'}<label class="grid gap-2 text-sm">
            Passkey name <TextField name="name" placeholder="For example, Personal laptop" readonly={busy}
            ></TextField><Form.Error name="name"></Form.Error>
          </label>{:else}{/if}{#if busy}<p role="status" class="text-sm text-muted-foreground">
            {#if dialog.kind === 'add' && enrollment === 'name'}Saving passkey…{:else}Complete verification on your device…{/if}
          </p>{:else}{/if}{#if error}<p role="alert" class="text-center text-sm text-destructive">{error}</p>{:else}{/if}
      </Dialog.Body><Dialog.Footer class={dialog.kind === 'remove' ? 'border-0 px-6 pt-4 pb-6' : 'border-0 px-6 pt-0 pb-5'}>
        <Dialog.Cancel variant="neutral" class="h-8 px-3">Cancel</Dialog.Cancel><Button
          type="submit"
          variant={dialog.kind === 'remove' ? 'destructive' : 'primary'}
          loading={busy}
          class="px-3 text-primary-foreground"
        >
          {#if dialog.kind === 'add'}{#if enrollment === 'name'}Save{:else}Try again{/if}{:else}Revoke{/if}
        </Button>
      </Dialog.Footer>
    </Form.Root>
  </Dialog.Content>
</Dialog.Root>
