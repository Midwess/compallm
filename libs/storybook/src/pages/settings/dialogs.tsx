import { CustomizeSidebar } from './preferences/user/sidebar-layout'
import { parseSettingsDialog } from './validation'
import { FormError } from '../../components/form'
import { useState } from 'react'
import { Form } from '../../components/form'
import { Button } from '../../components/buttons'
import { Dialog } from '../../components/dialog'
import { TextField, MultilineField } from '../../components/fields'
import { Select } from '../../components/dropdowns'
import { useSettings, useSettingsController } from './controller'
import type { SettingsDialogKind } from './types'

function SettingsDialogContent() {
  const c = useSettingsController(),
    d = useSettings((s) => s.dialog, ['dialog']),
    busy = useSettings((s) => s.busy)
  const [dialogForm] = useState(() =>
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
  )
  const profile = useSettings((s) => s.profile, ['profile']),
    organization = useSettings((s) => s.organization, ['organization'])
  const members = useSettings((s) => s.members),
    roles = useSettings((s) => s.roles),
    groups = useSettings((s) => s.groups)
  const passkeys = useSettings((s) => s.passkeys),
    apps = useSettings((s) => s.applications),
    keys = useSettings((s) => s.apiKeys)
  if (!d) return null
  if (d.kind === 'customize-sidebar') return <CustomizeSidebar />
  const member = members.find((m) => m.id === d.targetId),
    role = roles.find((r) => r.id === d.targetId)
  const group = groups.find((g) => g.id === d.targetId),
    app = apps.find((a) => a.id === d.targetId)
  const passkey = passkeys.find((p) => p.id === d.targetId),
    key = keys.find((k) => k.id === d.targetId)
  const phrase =
    d.kind === 'delete-account'
      ? `I want to delete ${profile.email}`
      : d.kind === 'delete-organization'
        ? `I want to delete ${organization.name}`
        : null
  const named: SettingsDialogKind[] = ['add-passkey', 'create-key']
  const limit = d.kind === 'organization-limit' || d.kind === 'member-limit'
  const text = d.kind === 'welcome' || d.kind === 'announcements'
  const titles: Record<SettingsDialogKind, string> = {
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
  }
  const descriptions: Partial<Record<SettingsDialogKind, string>> = {
    'delete-account': 'To confirm, type the phrase below exactly.',
    'delete-organization': 'To confirm, type the phrase below exactly.',
    'add-passkey': 'Give this passkey a name so you can recognize it later.',
    'verify-passkey':
      'Follow your device’s instructions to create a passkey. Keep this window open until verification completes.',
    'remove-passkey': 'You will no longer be able to use this passkey to sign in.',
    'revoke-sessions': 'Your other active sessions will be signed out. Your current session will stay active.',
    application: `Granted access: ${app?.scope ?? '—'}. Revoke access if you no longer use this application.`,
    'create-key': `Create a key for your membership in ${organization.name}. Keep your key private.`,
    'reveal-key': 'Copy and store this key securely. Keep your API key private.',
    'revoke-key': 'Requests using this API key will no longer be authorized. This cannot be undone.',
    'remove-member': 'This member will lose membership and access to the organization.',
    'delete-role': 'Remove all members and groups assigned to this role before deleting it. This cannot be undone.',
    'delete-group': 'Members will lose grants inherited from this group. Direct role assignments will remain.',
    'remove-group-member': 'This member will lose grants inherited through this group. Direct roles will remain.',
  }
  const danger = [
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
  ].includes(d.kind)
  const submit =
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
                    : 'Save changes'
  const disabled = busy || Boolean(phrase && d.phrase !== phrase)
  return (
    <Dialog onClose={() => c.pages.dialogs.closeDialog()}>
      <Dialog.Content className="w-110 rounded-xl bg-elevated">
        <Form model={dialogForm}>
          <Dialog.Header className="border-0 px-6 pt-6">
            <Dialog.Title className="text-xl leading-7">{titles[d.kind]}</Dialog.Title>
            <Dialog.Close />
          </Dialog.Header>
          <Dialog.Body className="gap-4 px-6 pt-4 pb-6">
            {descriptions[d.kind] ? (
              <Dialog.Description className="text-xsm leading-5.5">{descriptions[d.kind]}</Dialog.Description>
            ) : null}
            {phrase ? (
              <>
                <code className="text-xsm wrap-break-word">{phrase}</code>
                <label className="sr-only" htmlFor="confirmation-phrase">
                  Confirmation phrase
                </label>
                <TextField autoFocus id="confirmation-phrase" placeholder="Enter confirmation phrase" name="dialog.phrase" />
                <FormError name="dialog.phrase" />
              </>
            ) : null}
            {named.includes(d.kind) ? (
              <label className="grid gap-2 text-xs text-muted-foreground">
                {d.kind === 'add-passkey' ? 'Passkey name' : 'Name'}
                <TextField autoFocus name="dialog.name" required />
                <FormError name="dialog.name" />
              </label>
            ) : null}
            {d.kind === 'verify-passkey' ? (
              <p role="status" className="text-xsm">
                Waiting for your device…
              </p>
            ) : null}
            {d.kind === 'reveal-key' ? (
              <div className="flex gap-2">
                <TextField aria-label="Generated API key" readOnly value={d.revealedKey ?? ''} />
                <Button onClick={() => c.pages.dialogs.copy(d.revealedKey ?? '')}>Copy</Button>
              </div>
            ) : null}
            {limit ? (
              <label className="grid gap-2 text-xs text-muted-foreground">
                Monthly limit · USD
                <TextField
                  aria-label="Monthly limit in USD"
                  type="number"
                  min="0"
                  step="0.01"
                  placeholder="No limit set"
                  name="dialog.value"
                />
                <FormError name="dialog.value" />
                <span>Leave empty to use {d.targetId ? 'the organization default' : 'no limit'}.</span>
              </label>
            ) : null}
            {d.kind === 'reset-schedule' ? (
              <>
                <label className="grid gap-2 text-xs text-muted-foreground">
                  Reset frequency
                  <Select
                    aria-label="Reset frequency"
                    value="Monthly"
                    className="w-full"
                    options={[{ label: 'Monthly', value: 'Monthly' }]}
                  />
                </label>
                <label className="grid gap-2 text-xs text-muted-foreground">
                  Timezone
                  <Select
                    aria-label="Timezone"
                    name="dialog.value"
                    className="w-full"
                    options={['Workspace timezone', 'UTC', 'Asia/Ho_Chi_Minh'].map((label) => ({ label, value: label }))}
                  />
                  <FormError name="dialog.value" />
                </label>
              </>
            ) : null}
            {text ? (
              <>
                <MultilineField aria-label={titles[d.kind]} className="min-h-32" name="dialog.value" />
                <FormError name="dialog.value" />
              </>
            ) : null}
          </Dialog.Body>
          <Dialog.Footer className="border-0 px-6 pb-6">
            {d.kind !== 'reveal-key' ? <Dialog.Cancel variant="ghost">Cancel</Dialog.Cancel> : null}
            {d.kind !== 'verify-passkey' ? (
              <Button type="submit" variant={danger ? 'destructive' : 'primary'} loading={busy} disabled={disabled}>
                {submit}
              </Button>
            ) : null}
          </Dialog.Footer>
        </Form>
      </Dialog.Content>
    </Dialog>
  )
}

export function SettingsDialogs() {
  const dialog = useSettings((state) => state.dialog, ['dialog'])
  if (!dialog) return null
  return <SettingsDialogContent key={`${dialog.kind}:${dialog.targetId ?? ''}`} />
}
