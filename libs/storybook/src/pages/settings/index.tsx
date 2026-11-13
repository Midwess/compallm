import { useState } from 'react'
import { ListIcon } from '@phosphor-icons/react/dist/csr/List'
import { Button } from '../../components/buttons'
import { InlineError } from '../../components/feedback'
import { SettingsNavigation } from './sidebar'
import { SettingsDialogs } from './dialogs'
import { GeneralSettings, AppearanceSettings, MessagingSettings, CodeSettings } from './preferences'
import { ProfileSettings, SecuritySettings, ApiKeySettings, DangerSettings } from './account'
import { OrganizationSettings } from './organization'
import { MembersSettings } from './members'
import { GroupEditorSettings } from './group'
import { RoleEditorSettings } from './role'
import { UsageLimitSettings, MemberLimitSettings, UsageExplorerSettings, AuditSettings } from './usage'
import { useSettings, useSettingsController } from './controller'
import { cx } from '../../components/shared'
import { Empty, SettingsContent } from './shared'
import { useRole } from './role/controller'
import { useGroup } from './group/controller'

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
    case 'appearance':
      return <AppearanceSettings />
    case 'messaging':
      return <MessagingSettings />
    case 'code':
      return <CodeSettings />
    case 'profile':
      return <ProfileSettings />
    case 'security':
      return <SecuritySettings />
    case 'api-keys':
      return <ApiKeySettings />
    case 'danger':
      return <DangerSettings />
    case 'organization':
      return <OrganizationSettings />
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
export function SettingsPage() {
  const [open, setOpen] = useState(false),
    c = useSettingsController()
  const notice = useSettings((s) => s.notice),
    sharedError = useSettings((s) => s.error),
    page = useSettings((s) => s.location.page),
    roleError = useRole((s) => s.error),
    groupError = useGroup((s) => s.error)
  const error =
    (page === 'role' || page === 'new-role' ? roleError : page === 'group' || page === 'new-group' ? groupError : null) ??
    sharedError
  return (
    <div className="relative flex h-dvh min-h-0 overflow-hidden bg-background font-sans text-text">
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
        className="@container relative my-2 mr-2 ml-2 min-h-0 min-w-0 flex-1 overflow-x-hidden overflow-y-auto overscroll-none
rounded-xl border-[0.5px] border-divider bg-surface pr-2 scrollbar-gutter-stable md:ml-0"
        aria-label="Settings content"
      >
        <button
          type="button"
          aria-label="Open settings navigation"
          onClick={() => setOpen(true)}
          className="absolute top-4 left-4 z-10 rounded p-1 text-muted hover:text-text md:hidden"
        >
          <ListIcon className="size-4" />
        </button>
        {error ? <InlineError className="mx-auto mt-4 max-w-160 px-4">{error}</InlineError> : null}
        <SettingsScreen />
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
