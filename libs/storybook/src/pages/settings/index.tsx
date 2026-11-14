import { PermissionRequest } from '../../components/permission'
import type { PermissionAction } from '../../permission'
import { configureSettingsForm } from './form'
import { Form } from '../../components/form'
import { useEffect, useState } from 'react'
import { ListIcon } from '@phosphor-icons/react/dist/csr/List'
import { Button } from '../../components/buttons'
import { Toast } from '../../components/feedback'
import { SettingsNavigation } from './sidebar'
import { SettingsDialogs } from './dialogs'
import { GeneralSettings, MessagingSettings, CodeSettings } from './preferences/user/page'
import { UserPreferencePage } from './preferences/user'
import { ProfileSettings, AccessSettings, SecuritySettings, ApiKeySettings } from './account'
import { OrganizationSettings, OrganizationOnboarding } from './organization'
import { MembersSettings } from './members'
import { GroupEditorSettings } from './group'
import { RoleEditorSettings } from './role'
import { UsageLimitSettings, MemberLimitSettings, UsageExplorerSettings, AuditSettings } from './usage'
import { useSettings, useSettingsController } from './controller'
import { cx } from '../../components/shared'
import { Empty, SettingsContent } from './shared'

function SettingsScreen() {
  const page = useSettings((s) => s.location.page),
    status = useSettings((s) => s.status)
  if (status !== 'ready')
    return (
      <SettingsContent>
        <Empty title={status === 'loading' ? 'Loading settings…' : 'You do not have access'}>
          {status === 'denied' ? 'Ask an authorized administrator for access.' : 'Loading the current organization settings.'}
        </Empty>
      </SettingsContent>
    )
  switch (page) {
    case 'blank':
      return null
    case 'general':
      return <GeneralSettings />
    case 'messaging':
      return <MessagingSettings />
    case 'code':
      return <CodeSettings />
    case 'profile':
      return <ProfileSettings />
    case 'access':
      return <AccessSettings />
    case 'security':
      return <SecuritySettings />
    case 'api-keys':
      return <ApiKeySettings />
    case 'organization':
      return <OrganizationSettings />
    case 'organization-onboarding':
      return <OrganizationOnboarding />
    case 'members':
      return <MembersSettings />
    case 'role':
    case 'new-role':
      return <RoleEditorSettings />
    case 'group':
    case 'new-group':
      return <GroupEditorSettings />
    case 'usage-limits':
      return <UsageLimitSettings />
    case 'member-limits':
      return <MemberLimitSettings />
    case 'usage-explorer':
      return <UsageExplorerSettings />
    case 'audit':
      return <AuditSettings />
  }
}
function ProtectedSettingsScreen() {
  const page = useSettings((state) => state.location.page)
  const action: PermissionAction | undefined = (
    {
      organization: 'organization.settings.read',
      'organization-onboarding': 'organization.settings.read',
      members: 'member.read',
      role: 'role.read',
      'new-role': 'role.create',
      group: 'group.read',
      'new-group': 'group.create',
    } as Partial<Record<string, PermissionAction>>
  )[page]
  if (!action) return <SettingsScreen />
  return (
    <PermissionRequest
      actions={[action]}
      restricted="fallback"
      fallback={
        <SettingsContent>
          <Empty title="You do not have access">Return to an available page using the sidebar.</Empty>
        </SettingsContent>
      }
    >
      <SettingsScreen />
    </PermissionRequest>
  )
}
function SharedSettingsForm() {
  const c = useSettingsController()
  const page = useSettings((s) => s.location.page)
  const form = c.store.form
  useEffect(() => {
    return configureSettingsForm(c)
  }, [c, form, page])
  if (['general', 'messaging', 'code'].includes(page))
    return (
      <UserPreferencePage>
        <ProtectedSettingsScreen />
      </UserPreferencePage>
    )
  if (['role', 'new-role', 'group', 'new-group'].includes(page)) return <ProtectedSettingsScreen />
  return (
    <Form.Root model={form}>
      <ProtectedSettingsScreen />
    </Form.Root>
  )
}
export function SettingsPage() {
  const [open, setOpen] = useState(false),
    c = useSettingsController()
  const notice = useSettings((s) => s.notice)
  const errorToast = useSettings((s) => s.errorToast)
  useEffect(() => {
    const sources = [
      { store: c.store, clear: () => c.store.setState({ error: null }) },
      { store: c.role.store, clear: () => c.role.store.setState({ error: null }) },
      { store: c.group.store, clear: () => c.group.store.setState({ error: null }) },
    ]
    const subscriptions = sources.map(({ store, clear }) => {
      const report = () => {
        const error = store.getState().error
        if (!error) return
        clear()
        c.apiError(
          { info: { message: error } },
          store === c.store
            ? undefined
            : () => {
                void store.form.retry()
              },
        )
      }
      report()
      return store.subscribe(report)
    })
    return () => subscriptions.forEach((unsubscribe) => unsubscribe())
  }, [c])
  return (
    <div className="relative flex h-dvh min-h-0 overflow-hidden bg-background font-sans text-foreground">
      {open ? (
        <button
          aria-label="Close settings navigation"
          className="fixed inset-0 z-30 bg-background/60 md:hidden"
          onClick={() => setOpen(false)}
        />
      ) : null}
      <div className={cx('fixed inset-y-0 left-0 z-40 w-61 bg-background md:static md:block', open ? 'block' : 'hidden')}>
        <SettingsNavigation onNavigate={() => setOpen(false)} />
      </div>
      <main
        className="@container relative mt-4 mr-2 mb-2 ml-2 min-h-0 min-w-0 flex-1 overflow-x-hidden overflow-y-auto
overscroll-none
rounded-xl border-(length:--border-hairline) border-divider bg-surface pr-2 scrollbar-gutter-stable md:ml-0"
        aria-label="Settings content"
      >
        <button
          type="button"
          aria-label="Open settings navigation"
          onClick={() => setOpen(true)}
          className="absolute top-4 left-4 z-10 rounded p-1 text-muted-foreground hover:text-foreground md:hidden"
        >
          <ListIcon className="size-4" />
        </button>
        <SharedSettingsForm />
      </main>
      {notice ? (
        <div
          role="status"
          className="fixed right-6 bottom-6 z-50 flex max-w-[calc(100vw-3rem)] items-center gap-4
rounded-lg border border-border bg-elevated px-4 py-3 text-xsm shadow-menu"
        >
          {notice}
          <Button variant="ghost" className="h-6 p-0" onClick={() => c.dismissNotice()}>
            Dismiss
          </Button>
        </div>
      ) : null}
      {errorToast ? (
        <div className="fixed right-6 bottom-6 z-50 max-w-[calc(100vw-3rem)]">
          <Toast
            tone="error"
            message={errorToast.message}
            onDismiss={c.dismissErrorToast}
            action={
              errorToast.retry
                ? {
                    label: 'Retry',
                    onClick: () => {
                      c.dismissErrorToast()
                      errorToast.retry?.()
                    },
                  }
                : undefined
            }
          />
        </div>
      ) : null}
      <SettingsDialogs />
    </div>
  )
}
export { createSettingsStore } from './store'
export type * from './types'

export { createRoleStore } from './role/store'
export { createGroupStore } from './group/store'
export type { RoleController, RoleState } from './role/controller'
export type { GroupController, GroupState } from './group/controller'
export type { ProfileController } from './profile/controller'
export type { OrganizationController } from './organization/controller'

export { parsePreferencePatch, confirmedPreferenceDraft, preferenceSelection } from './preferences/user/values'

export { createUserPreferenceStore } from './preferences/user/store'
export { configureUserPreferenceForm } from './preferences/user/form'
export { loadedPreferenceDraft, initialPreferenceDraft } from './preferences/user/values'
export type { PreferenceDraft, UserPreferenceValues } from './preferences/user/values'
export type { UserPreferenceController, UserPreferenceState } from './preferences/user/controller'
