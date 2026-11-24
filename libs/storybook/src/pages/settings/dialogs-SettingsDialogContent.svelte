<script lang="ts">
  import { CustomizeSidebar } from './preferences/user/sidebar-layout'
  import { parseSettingsDialog } from './validation'
  import { FormError } from '../../components/form'
  import { Form } from '../../components/form'
  import { Button } from '../../components/buttons'
  import { Dialog } from '../../components/dialog'
  import { TextField, MultilineField } from '../../components/fields'
  import { Select } from '../../components/dropdowns'
  import type { SettingsDialogKind } from './types'
  import { untrack } from 'svelte'
  import { readStore } from '../page-state.svelte'
  import { getSettingsController } from '../../context'

  const __settings = getSettingsController()
  const c = untrack(() => getSettingsController())
  let d = $derived(readStore(__settings.store, (s) => s.dialog, ['dialog']))
  let busy = $derived(readStore(__settings.store, (s) => s.busy))
  let dialogForm = $state(
    untrack(() =>
      c.store.form.scope(['dialog'], {
        mode: 'button',
        command: true,
        validationNames: () => ['dialog'],
        parse: parseSettingsDialog,
        error: (error, retry) => c.apiError(error, retry),
        submit: (values) => {
          if (values.dialog) c.pages.dialogs.confirmDialog(values.dialog)
          return false
        },
      }),
    ),
  )
  let profile = $derived(readStore(__settings.store, (s) => s.profile, ['profile']))
  let organization = $derived(readStore(__settings.store, (s) => s.organization, ['organization']))
  let members = $derived(readStore(__settings.store, (s) => s.members))
  let roles = $derived(readStore(__settings.store, (s) => s.roles))
  let groups = $derived(readStore(__settings.store, (s) => s.groups))
  let passkeys = $derived(readStore(__settings.store, (s) => s.passkeys))
  let apps = $derived(readStore(__settings.store, (s) => s.applications))
  let keys = $derived(readStore(__settings.store, (s) => s.apiKeys))
</script>

{#if !d}{:else}{#if d.kind === 'customize-sidebar'}<CustomizeSidebar></CustomizeSidebar>{:else}{@const member = members.find(
      (m) => m.id === d.targetId,
    )}{@const role = roles.find((r) => r.id === d.targetId)}{@const group = groups.find(
      (g) => g.id === d.targetId,
    )}{@const app = apps.find((a) => a.id === d.targetId)}{@const passkey = passkeys.find(
      (p) => p.id === d.targetId,
    )}{@const key = keys.find((k) => k.id === d.targetId)}{@const phrase =
      d.kind === 'delete-account'
        ? `I want to delete ${profile.email}`
        : d.kind === 'delete-organization'
          ? `I want to delete ${organization.name}`
          : null}{@const named: SettingsDialogKind[] = ['add-passkey', 'create-key']}{@const limit =
      d.kind === 'organization-limit' || d.kind === 'member-limit'}{@const text =
      d.kind === 'welcome' ||
      d.kind ===
        'announcements'}{@const titles: Record<SettingsDialogKind, string> = {
    'delete-account': 'Delete account?',
    'delete-organization': 'Delete organization?',
    'add-passkey': 'Add passkey',
    'verify-passkey': 'Complete on your device',
    'remove-passkey': `Remove ${passkey?.name ?? 'passkey'}?`,
    'revoke-sessions': 'Revoke other sessions?',
    application: app?.name ?? 'Authorized application',
    'create-key': 'Create API key',
    'reveal-key': 'API key created',
    'revoke-key': `Revoke ${key?.name ?? 'key'}?`,
    'remove-member': `Remove ${member?.name ?? 'member'}?`,
    'delete-role': `Delete ${role?.name ?? 'role'}?`,
    'delete-group': `Delete ${group?.name ?? 'group'}?`,
    'remove-group-member': `Remove ${member?.name ?? 'member'}?`,
    'organization-limit': 'Organization spending limit',
    'member-limit': member ? `${member.name} · Monthly limit` : 'Per-member spending limit',
    'reset-schedule': 'Reset schedule',
    welcome: 'Welcome message',
    announcements: 'Announcements',
    'customize-sidebar': 'Customize sidebar',
}}{@const descriptions: Partial<Record<SettingsDialogKind, string>> = {
    'delete-account': 'To confirm, type the phrase below exactly.',
    'delete-organization': 'To confirm, type the phrase below exactly.',
    'add-passkey': 'Give this passkey a name so you can recognize it later.',
    'verify-passkey': 'Follow your device’s instructions to create a passkey. Keep this window open until verification completes.',
    'remove-passkey': 'You will no longer be able to use this passkey to sign in.',
    'revoke-sessions': 'Your other active sessions will be signed out. Your current session will stay active.',
    application: `Granted access: ${app?.scope ?? '—'}. Revoke access if you no longer use this application.`,
    'create-key': `Create a key for your membership in ${organization.name}. Keep your key private.`,
    'reveal-key': 'Copy and store this key securely. Keep your API key private.',
    'revoke-key': 'Requests using this API key will no longer be authorized. This cannot be undone.',
    'remove-member': 'This member will lose membership and access to the organization.',
    'delete-role': 'Remove all members and groups assigned to this role before deleting it. This cannot be undone.',
    'delete-group': 'Remove all members before deleting this group. Its role links will be removed.',
    'remove-group-member': 'This member will lose grants inherited through this group. Direct roles will remain.',
}}{@const danger =
      [
        'delete-account',
        'delete-organization',
        'remove-passkey',
        'revoke-sessions',
        'application',
        'revoke-key',
        'remove-member',
        'delete-role',
        'delete-group',
        'remove-group-member',
      ].includes(d.kind)}{@const submit =
      d.kind === 'add-passkey'
        ? 'Continue'
        : d.kind === 'create-key'
          ? 'Create key'
          : d.kind === 'reveal-key'
            ? 'Done'
            : d.kind === 'application'
              ? 'Revoke access'
              : d.kind === 'revoke-sessions'
                ? 'Revoke all'
                : d.kind.includes('delete')
                  ? 'Delete'
                  : d.kind.includes('remove')
                    ? 'Remove'
                    : d.kind === 'revoke-key'
                      ? 'Revoke key'
                      : 'Save changes'}{@const disabled = busy || Boolean(phrase && d.phrase !== phrase)}<Dialog.Root
      open
      onOpenChange={(open) => {
        if (!open) c.pages.dialogs.closeDialog()
      }}
    >
      <Dialog.Content class="w-110 rounded-xl bg-elevated">
        <Form.Root model={dialogForm}>
          <Dialog.Header class="border-0 px-6 pt-6">
            <Dialog.Title class="text-xl leading-7">{titles[d.kind]}</Dialog.Title><Dialog.Close></Dialog.Close>
          </Dialog.Header><Dialog.Body class="gap-4 px-6 pt-4 pb-6">
            {#if descriptions[d.kind]}<Dialog.Description class="text-xsm leading-5.5">
                {descriptions[d.kind]}
              </Dialog.Description>{:else}{/if}{#if phrase}<code class="text-xsm wrap-break-word">{phrase}</code><label
                class="sr-only"
                for="confirmation-phrase"
              >
                Confirmation phrase
              </label><TextField autofocus id="confirmation-phrase" placeholder="Enter confirmation phrase" name="dialog.phrase"
              ></TextField><FormError name="dialog.phrase"></FormError>{:else}{/if}{#if named.includes(d.kind)}<label
                class="grid gap-2 text-xs text-muted-foreground"
              >
                {#if d.kind === 'add-passkey'}Passkey name{:else}Name{/if}<TextField autofocus name="dialog.name" required
                ></TextField><FormError name="dialog.name"></FormError>
              </label>{:else}{/if}{#if d.kind === 'verify-passkey'}<p role="status" class="text-xsm">
                Waiting for your device…
              </p>{:else}{/if}{#if d.kind === 'reveal-key'}<div class="flex gap-2">
                <TextField aria-label="Generated API key" readonly value={d.revealedKey ?? ''}></TextField><Button
                  onclick={() => c.pages.dialogs.copy(d.revealedKey ?? '')}
                >
                  Copy
                </Button>
              </div>{:else}{/if}{#if limit}<label class="grid gap-2 text-xs text-muted-foreground">
                Monthly limit · USD <TextField
                  aria-label="Monthly limit in USD"
                  type="number"
                  min="0"
                  step="0.01"
                  placeholder="No limit set"
                  name="dialog.value"
                ></TextField><FormError name="dialog.value"></FormError><span>
                  Leave empty to use {#if d.targetId}the organization default{:else}no limit{/if}.
                </span>
              </label>{:else}{/if}{#if d.kind === 'reset-schedule'}<label class="grid gap-2 text-xs text-muted-foreground">
                Reset frequency <Select.Control
                  aria-label="Reset frequency"
                  value="Monthly"
                  class="w-full"
                  options={[{ label: 'Monthly', value: 'Monthly' }]}
                ></Select.Control>
              </label><label class="grid gap-2 text-xs text-muted-foreground">
                Timezone <Select.Control
                  aria-label="Timezone"
                  name="dialog.value"
                  class="w-full"
                  options={['Workspace timezone', 'UTC', 'Asia/Ho_Chi_Minh'].map((label) => ({ label, value: label }))}
                ></Select.Control><FormError name="dialog.value"></FormError>
              </label>{:else}{/if}{#if text}<MultilineField aria-label={titles[d.kind]} class="min-h-32" name="dialog.value"
              ></MultilineField><FormError name="dialog.value"></FormError>{:else}{/if}
          </Dialog.Body><Dialog.Footer class="border-0 px-6 pb-6">
            {#if d.kind !== 'reveal-key'}<Dialog.Cancel variant="ghost">
                Cancel
              </Dialog.Cancel>{:else}{/if}{#if d.kind !== 'verify-passkey'}<Button
                type="submit"
                variant={danger ? 'destructive' : 'primary'}
                loading={busy}
                {disabled}
              >
                {submit}
              </Button>{:else}{/if}
          </Dialog.Footer>
        </Form.Root>
      </Dialog.Content>
    </Dialog.Root>{/if}{/if}
