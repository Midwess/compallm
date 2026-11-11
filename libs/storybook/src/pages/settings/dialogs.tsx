import { motion } from 'motion/react'
import { useTransition } from '../../components/motion'
import { Button } from '../../components/buttons'
import { Dialog } from '../../components/dialog'
import { TextField, MultilineField } from '../../components/fields'
import { Select } from '../../components/dropdowns'
import { useReorder } from '../../components/use-reorder'
import { cx } from '../../components/shared'
import { DotsSixVerticalIcon } from '@phosphor-icons/react/dist/csr/DotsSixVertical'
import { HardDrivesIcon } from '@phosphor-icons/react/dist/csr/HardDrives'
import { CubeIcon } from '@phosphor-icons/react/dist/csr/Cube'
import { NotebookIcon } from '@phosphor-icons/react/dist/csr/Notebook'
import { PlugsIcon } from '@phosphor-icons/react/dist/csr/Plugs'
import { useSettings, useSettingsController } from './controller'
import type { SettingsDialogKind } from './types'

const sidebarIcons = { Providers: HardDrivesIcon, Models: CubeIcon, Notebooks: NotebookIcon, Plugins: PlugsIcon }
const badgeOptions = [
  {
    value: 'Dot',
    label: 'Dot',
    icon: <span aria-hidden="true" className="mx-1 size-1.5 shrink-0 rounded-full bg-muted" />,
  },
  {
    value: 'Count',
    label: 'Count',
    icon: (
      <span aria-hidden="true" className="flex size-3.5 shrink-0 items-center justify-center text-xs text-muted">
        1
      </span>
    ),
  },
]
const visibilityOptions = ['Always show', 'Show when badged', "Don't show"].map((value) => ({ label: value, value }))

function CustomizeSidebar() {
  const timing = useTransition('reorder')
  const controller = useSettingsController()
  const preferences = useSettings((state) => state.preferences)
  const order = String(preferences.sidebarOrder).split(',')
  const { rowNodes, drag, preview, announcement, id, keyboardMove, startDrag, pointerMove, endDrag } = useReorder(
    order,
    (items) => controller.pages.preferences.setPreference('sidebarOrder', items.join(',')),
    false,
    'Sidebar item',
  )
  return (
    <Dialog onClose={controller.pages.dialogs.closeDialog}>
      <Dialog.Content className="w-120 rounded-xl bg-inset">
        <Dialog.Header className="h-14 px-6">
          <Dialog.Title className="text-[15px]">Customize sidebar</Dialog.Title>
          <Dialog.Close />
        </Dialog.Header>
        <Dialog.Body className="gap-6 px-6 pt-4 pb-6">
          <div className="flex h-11 items-center justify-between rounded-lg bg-elevated px-3 text-xsm">
            <span>Default badge style</span>
            <Select
              aria-label="Default badge style"
              className="w-26"
              value={String(preferences.sidebarBadgeStyle)}
              options={badgeOptions}
              onValueChange={(value) => controller.pages.preferences.setPreference('sidebarBadgeStyle', value)}
            />
          </div>
          <div className="grid gap-2.5">
            <h3 className="text-xsm font-medium">Workspace</h3>
            <div className="rounded-lg bg-elevated py-0.5">
              {order.map((name, index) => {
                const Icon = sidebarIcons[name as keyof typeof sidebarIcons]
                return (
                  <motion.div
                    key={name}
                    initial={false}
                    animate={{ y: preview?.offsets[name] ?? 0 }}
                    transition={timing}
                    ref={(node) => {
                      if (node) rowNodes.current.set(name, node)
                      else rowNodes.current.delete(name)
                    }}
                    className={cx('relative flex h-9 items-center px-2', preview?.key === name && 'opacity-0')}
                  >
                    <button
                      type="button"
                      aria-label={`Reorder ${name}`}
                      aria-describedby={`${id}-reorder`}
                      className={`
flex size-6 shrink-0 touch-none cursor-grab items-center justify-center text-muted hover:text-text
active:cursor-grabbing focus-visible:outline-focus
`}
                      onPointerDown={(event) => startDrag(name, event)}
                      onPointerMove={pointerMove}
                      onPointerUp={(event) => {
                        if (drag.current?.pointerId === event.pointerId) {
                          endDrag(true)
                          event.currentTarget.releasePointerCapture(event.pointerId)
                        }
                      }}
                      onPointerCancel={() => endDrag(false)}
                      onKeyDown={(event) => {
                        if (event.key === 'Escape' && drag.current) {
                          event.preventDefault()
                          endDrag(false)
                        } else if (event.key === 'ArrowUp' || event.key === 'ArrowDown') {
                          event.preventDefault()
                          keyboardMove(index, index + (event.key === 'ArrowUp' ? -1 : 1))
                        }
                      }}
                    >
                      <DotsSixVerticalIcon aria-hidden="true" className="size-3" />
                    </button>
                    {Icon ? <Icon aria-hidden="true" className="mr-2 size-4 shrink-0 text-muted" /> : null}
                    <span className="min-w-0 flex-1 text-xsm">{name}</span>
                    <Select
                      aria-label={`${name} visibility`}
                      className="w-35"
                      value={String(preferences[`sidebar${name}`])}
                      options={visibilityOptions}
                      onValueChange={(value) => controller.pages.preferences.setPreference(`sidebar${name}`, value)}
                    />
                  </motion.div>
                )
              })}
            </div>
            <p id={`${id}-reorder`} className="sr-only">
              Drag to reorder, or focus the grip and use Up and Down arrow keys.
            </p>
            <span role="status" className="sr-only">
              {announcement}
            </span>
          </div>
        </Dialog.Body>
      </Dialog.Content>
    </Dialog>
  )
}

export function SettingsDialogs() {
  const c = useSettingsController(),
    d = useSettings((s) => s.dialog),
    busy = useSettings((s) => s.busy)
  const profile = useSettings((s) => s.profile),
    organization = useSettings((s) => s.organization)
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
    'delete-role': 'Assignments will lose the grants from this role. Review the effect before deleting it.',
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
  const disabled = busy || Boolean(phrase && d.phrase !== phrase) || (named.includes(d.kind) && !d.name.trim())
  return (
    <Dialog onClose={() => c.pages.dialogs.closeDialog()}>
      <Dialog.Content className="w-110 rounded-xl bg-elevated">
        <form
          onSubmit={(e) => {
            e.preventDefault()
            c.pages.dialogs.confirmDialog()
          }}
        >
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
                <TextField
                  autoFocus
                  id="confirmation-phrase"
                  placeholder="Enter confirmation phrase"
                  value={d.phrase}
                  onChange={(e) => c.pages.dialogs.updateDialog({ phrase: e.target.value })}
                />
              </>
            ) : null}
            {named.includes(d.kind) ? (
              <label className="grid gap-2 text-xs text-muted">
                {d.kind === 'add-passkey' ? 'Passkey name' : 'Name'}
                <TextField
                  autoFocus
                  value={d.name}
                  required
                  onChange={(e) => c.pages.dialogs.updateDialog({ name: e.target.value })}
                />
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
              <label className="grid gap-2 text-xs text-muted">
                Monthly limit · USD
                <TextField
                  aria-label="Monthly limit in USD"
                  type="number"
                  min="0"
                  step="0.01"
                  placeholder="No limit set"
                  value={d.value}
                  onChange={(e) => c.pages.dialogs.updateDialog({ value: e.target.value })}
                />
                <span>Leave empty to use {d.targetId ? 'the organization default' : 'no limit'}.</span>
              </label>
            ) : null}
            {d.kind === 'reset-schedule' ? (
              <>
                <label className="grid gap-2 text-xs text-muted">
                  Reset frequency
                  <Select
                    aria-label="Reset frequency"
                    value="Monthly"
                    className="w-full"
                    options={[{ label: 'Monthly', value: 'Monthly' }]}
                  />
                </label>
                <label className="grid gap-2 text-xs text-muted">
                  Timezone
                  <Select
                    aria-label="Timezone"
                    value={d.value}
                    className="w-full"
                    options={['Workspace timezone', 'UTC', 'Asia/Ho_Chi_Minh'].map((label) => ({ label, value: label }))}
                    onValueChange={(value) => c.pages.dialogs.updateDialog({ value })}
                  />
                </label>
              </>
            ) : null}
            {text ? (
              <MultilineField
                aria-label={titles[d.kind]}
                className="min-h-32"
                value={d.value}
                onChange={(e) => c.pages.dialogs.updateDialog({ value: e.target.value })}
              />
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
        </form>
      </Dialog.Content>
    </Dialog>
  )
}
