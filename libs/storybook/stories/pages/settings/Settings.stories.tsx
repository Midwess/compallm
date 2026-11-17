import { useEffect, useState } from 'react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { AuthControllerContext, SettingsControllerContext, UserPreferenceControllerContext } from '../../../src/context'
import { userPreferenceDefaults } from '@compallm/contract/preference'
import { loadedPreferenceDraft } from '../../../src/pages/settings'
import { SettingsPage } from '../../../src/pages/settings'
import type {
  SettingsState,
  SettingsDestination,
  RoleDraft,
  GroupDraft,
  UserPreferenceState,
  UserPreferenceValues,
} from '../../../src/pages/settings'
import { DemoAuthController } from '../demo-auth-controller'
import type { AuthUser } from '../../../src/pages/auth/types'
import { DemoSettingsController } from './demo-controller'
import { advancedSource, modelOperatorSource, initialSettings } from './fixtures'

function Demo({
  page = 'general',
  id,
  initial = {},
  failure = false,
  roleLoadingMs = 0,
  groupLoadingMs = 0,
  pagedGroup = false,
  preferenceStatus,
  savedPreferences,
  session,
}: {
  page?: SettingsDestination
  id?: string
  initial?: Partial<SettingsState> & { roleDraft?: RoleDraft | null; groupDraft?: GroupDraft | null }
  failure?: boolean | 'conflict'
  roleLoadingMs?: number
  groupLoadingMs?: number
  pagedGroup?: boolean
  preferenceStatus?: UserPreferenceState['status']
  savedPreferences?: UserPreferenceValues
  session?: Partial<AuthUser>
}) {
  const [auth] = useState(() => {
    const auth = new DemoAuthController('signin')
    auth.store.setState({
      status: 'authenticated',
      currentUser: {
        id: 'session',
        userId: 'alex',
        organizationId: 'workspace',
        organizationName: 'Workspace',
        ...session,
      },
    })
    return auth
  })
  const [controller] = useState(() => {
    const controller = new DemoSettingsController({ location: { page, id }, ...initial }, failure)
    const user = controller.pages.preferences.user
    if (savedPreferences) {
      user.store.setState({ saved: savedPreferences })
      user.store.form.reset({ draft: loadedPreferenceDraft(savedPreferences) })
    }
    if (preferenceStatus) user.store.setState({ status: preferenceStatus })
    return controller
  })
  useEffect(() => {
    controller.start()
    if (initial.error) controller.apiError({ info: { message: initial.error } })
    return controller.stop
  }, [controller, initial.error])
  useEffect(() => {
    if (!roleLoadingMs) return
    const draft = controller.pages.role.store.getState().draft
    controller.pages.role.store.setState({ loading: true, draft: null })
    const timer = setTimeout(() => {
      controller.pages.role.store.setState({ loading: false, draft })
    }, roleLoadingMs)
    return () => clearTimeout(timer)
  }, [controller, roleLoadingMs])
  useEffect(() => {
    if (!groupLoadingMs) return
    const draft = controller.group.store.getState().draft
    controller.group.store.setState({ loading: true, draft: null })
    const timer = setTimeout(() => controller.group.store.setState({ loading: false, draft }), groupLoadingMs)
    return () => clearTimeout(timer)
  }, [controller, groupLoadingMs])
  useEffect(() => {
    if (!pagedGroup) return
    const members = controller.store.getState().members
    controller.group.store.setState({
      members: members.slice(0, 1).map((member) => ({ ...member, available: true })),
      hasNextPage: members.length > 1,
      memberCount: controller.group.store.getState().draft?.memberIds.length,
    })
  }, [controller, pagedGroup])
  return (
    <AuthControllerContext value={auth}>
      <SettingsControllerContext value={controller}>
        <UserPreferenceControllerContext value={controller.pages.preferences.user}>
          <SettingsPage />
        </UserPreferenceControllerContext>
      </SettingsControllerContext>
    </AuthControllerContext>
  )
}
const meta = {
  title: 'Pages/Settings',
  parameters: {
    layout: 'fullscreen',
    docs: {
      description: {
        component:
          'Current settings designs with a shared in-memory preview controller. Actions do not write to backend services.',
      },
    },
  },
} satisfies Meta
export default meta
type Story = StoryObj<typeof meta>

export const Blank: Story = { render: () => <Demo page="blank" /> }
export const General: Story = { render: () => <Demo /> }
export const PreferencesLoading: Story = { render: () => <Demo preferenceStatus="loading" /> }
export const PreferencesLoadFailure: Story = { render: () => <Demo preferenceStatus="error" /> }
export const PreferencesDenied: Story = { render: () => <Demo preferenceStatus="denied" /> }
export const SavedPreferences: Story = {
  render: () => (
    <Demo
      savedPreferences={{
        ...userPreferenceDefaults,
        navigation: { home: 'MODELS' },
        memberDisplay: { nameFormat: 'USERNAME' },
        calendar: { firstDay: 'SUNDAY' },
        sidebar: {
          ...userPreferenceDefaults.sidebar,
          layout: userPreferenceDefaults.sidebar.layout.map((section) => ({ ...section, items: [...section.items] })),
          entries: [...userPreferenceDefaults.sidebar.entries],
        },
      }}
    />
  ),
}
export const GeneralNarrow: Story = {
  render: () => <Demo />,
  parameters: {
    viewport: { options: { settingsNarrow: { name: 'Narrow', styles: { width: '625px', height: '900px' } } } },
  },
  globals: { viewport: { value: 'settingsNarrow', isRotated: false } },
}
export const GeneralCompactDesktop: Story = {
  render: () => <Demo />,
  parameters: {
    viewport: { options: { settingsCompact: { name: 'Compact desktop', styles: { width: '900px', height: '900px' } } } },
  },
  globals: { viewport: { value: 'settingsCompact', isRotated: false } },
}
export const SidebarPreferences: Story = {
  render: () => <Demo page="general" initial={{ dialog: { kind: 'customize-sidebar', name: '', value: '', phrase: '' } }} />,
}
export const Messaging: Story = { render: () => <Demo page="messaging" /> }
export const CodeAppearance: Story = { render: () => <Demo page="code" /> }
export const PreferenceSaveFailure: Story = { render: () => <Demo page="general" failure /> }
export const Profile: Story = { render: () => <Demo page="profile" /> }
export const ProfileLoading: Story = { render: () => <Demo page="profile" initial={{ profileStatus: 'loading' }} /> }
export const ProfileMissing: Story = { render: () => <Demo page="profile" initial={{ profileStatus: 'missing' }} /> }
export const ProfileDenied: Story = { render: () => <Demo page="profile" initial={{ profileStatus: 'denied' }} /> }
export const ProfileLoadFailure: Story = { render: () => <Demo page="profile" initial={{ profileStatus: 'error' }} /> }
export const ProfileWithPhoto: Story = {
  render: () => (
    <Demo
      page="profile"
      initial={{
        profile: {
          name: 'Alex Morgan',
          username: 'alex',
          email: 'alex@example.com',
          avatar: [
            'data:image/svg+xml,%3Csvg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 40 40"%3E',
            '%3Crect width="40" height="40" fill="%236b879c"/%3E',
            '%3Ccircle cx="20" cy="15" r="8" fill="%23e8d6c8"/%3E',
            '%3Cpath d="M5 40c0-10 6-16 15-16s15 6 15 16" fill="%233c5065"/%3E%3C/svg%3E',
          ].join(''),
        },
      }}
    />
  ),
}
export const Access: Story = {
  render: () => (
    <Demo
      page="access"
      initial={{ sessionLocation: { sessionId: 'session', ipAddress: '203.0.113.42', location: 'Vietnam' } }}
      session={{
        ipAddress: '203.0.113.42',
        userAgent:
          'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) ' +
          'AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36',
      }}
    />
  ),
}
export const AccessUnknownSession: Story = { render: () => <Demo page="access" /> }
export const AccessMobileSession: Story = {
  render: () => (
    <Demo
      page="access"
      initial={{ sessionLocation: { sessionId: 'session', ipAddress: '2001:db8::42', location: 'Ho Chi Minh City, Vietnam' } }}
      session={{
        ipAddress: '2001:db8::42',
        userAgent:
          'Mozilla/5.0 (iPhone; CPU iPhone OS 18_0 like Mac OS X) ' +
          'AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.0 Mobile/15E148 Safari/604.1',
      }}
    />
  ),
}
export const AccessLight: Story = { ...Access, globals: { theme: 'BASIC_WHITE' } }
export const SessionIpFallback: Story = {
  render: () => (
    <Demo
      page="access"
      session={{
        ipAddress: '203.0.113.42',
        userAgent: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 Chrome/131.0.0.0 Safari/537.36',
      }}
    />
  ),
}
export const SessionsLoading: Story = { render: () => <Demo page="access" initial={{ sessionsLoading: true }} /> }
export const SessionsLoadFailure: Story = { render: () => <Demo page="access" initial={{ sessionsError: true }} /> }
export const SessionsReference: Story = {
  render: () => (
    <Demo
      page="security"
      session={{
        ipAddress: '203.0.113.42',
        signedInAt: '1794000000',
        lastSeenAt: '1794877200',
        userAgent: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 Chrome/131.0.0.0 Safari/537.36',
      }}
      initial={{
        sessionLocation: { sessionId: 'session', ipAddress: '203.0.113.42', location: 'Ho Chi Minh City, 61, VN' },
        otherSessions: [
          {
            id: 'arc',
            browser: 'Arc on macOS',
            location: 'Quận Mười, 61, VN',
            lastSeenAt: '1794812400',
            signedInAt: '1794000000',
          },
          {
            id: 'brave',
            browser: 'Brave on macOS',
            location: 'Quận Mười, 61, VN',
            lastSeenAt: '1794531600',
            signedInAt: '1793000000',
          },
        ],
      }}
    />
  ),
}
export const Security: Story = { render: () => <Demo page="security" /> }
export const AccessPopulated: Story = {
  render: () => (
    <Demo
      page="access"
      initial={{
        applications: [{ id: 'app', name: 'Example integration', scope: 'read your profile' }],
      }}
    />
  ),
}
export const ApiKeys: Story = { render: () => <Demo page="api-keys" /> }
export const ApiKeysPopulated: Story = {
  render: () => (
    <Demo
      page="api-keys"
      initial={{
        apiKeys: [{ id: 'key', name: 'Development', prefix: 'preview_key_••••' }],
      }}
    />
  ),
}
export const SecurityPopulated: Story = {
  render: () => <Demo page="security" initial={{ passkeys: [{ id: 'passkey', name: 'My passkey' }] }} />,
}
export const Organization: Story = { render: () => <Demo page="organization" /> }
export const OrganizationReadOnly: Story = {
  render: () => <Demo page="organization" initial={{ organizationCanUpdateProfile: false }} />,
}
export const OrganizationLoading: Story = {
  render: () => <Demo page="organization" initial={{ organizationProfileStatus: 'loading' }} />,
}
export const OrganizationLoadFailure: Story = {
  render: () => <Demo page="organization" initial={{ organizationProfileStatus: 'error' }} />,
}
export const OrganizationOnboarding: Story = { render: () => <Demo page="organization-onboarding" /> }
export const OrganizationOnboardingLight: Story = { ...OrganizationOnboarding, globals: { theme: 'BASIC_WHITE' } }

export const Members: Story = { render: () => <Demo page="members" /> }
export const EditGroup: Story = { render: () => <Demo page="group" id="research" /> }
export const NewGroup: Story = { render: () => <Demo page="new-group" /> }
export const ConfiguredGroup: Story = {
  render: () => (
    <Demo
      page="new-group"
      initial={{
        groupDraft: {
          id: null,
          name: 'Product team',
          description: 'People building the product experience.',
          memberIds: ['jamie', 'sam'],
          roleIds: ['model-operator'],
        },
      }}
    />
  ),
}
export const GroupMemberSearch: Story = {
  render: () => <Demo page="group" id="research" initial={{ filters: { 'group-members-query': 'Jamie' } }} />,
}
export const GroupRoleSearch: Story = {
  render: () => <Demo page="group" id="research" initial={{ filters: { 'group-roles-query': 'Model' } }} />,
}
export const GroupSearchEmpty: Story = {
  render: () => <Demo page="group" id="research" initial={{ filters: { 'group-members-query': 'no member matches' } }} />,
}
export const GroupRolesEmpty: Story = {
  render: () => <Demo page="new-group" initial={{ roles: [] }} />,
}
export const GroupReadOnly: Story = {
  render: () => <Demo page="group" id="research" initial={{ readonly: true }} />,
}
export const EditRole: Story = { render: () => <Demo page="role" id="model-operator" /> }
export const RoleLoading: Story = {
  render: () => <Demo page="role" id="model-operator" roleLoadingMs={1200} />,
}
export const DeleteRole: Story = {
  render: () => (
    <Demo
      page="role"
      id="model-operator"
      initial={{ dialog: { kind: 'delete-role', targetId: 'model-operator', name: '', value: '', phrase: '' } }}
    />
  ),
}
export const BuiltInRole: Story = { render: () => <Demo page="role" id="owner" /> }
export const NewRole: Story = { render: () => <Demo page="new-role" /> }
export const ConfiguredRole: Story = {
  render: () => (
    <Demo
      page="new-role"
      initial={{
        roleDraft: {
          id: null,
          name: 'Model contributor',
          description: 'Use all models and view own usage.',
          source: modelOperatorSource,
          mode: 'visual',
          projection: [],
          diagnostics: [],
          parsing: false,
          builtin: false,
        },
      }}
    />
  ),
}
export const FilteredPolicies: Story = {
  render: () => <Demo page="role" id="model-operator" initial={{ filters: { 'policy-resource': 'Providers' } }} />,
}
export const SelectedPolicies: Story = {
  render: () => <Demo page="role" id="model-operator" initial={{ filters: { 'policy-selection': 'enabled' } }} />,
}
export const PolicySearchEmpty: Story = {
  render: () => <Demo page="role" id="model-operator" initial={{ query: 'no-matching-policy' }} />,
}
export const AdvancedVisual: Story = {
  render: () => (
    <Demo
      page="role"
      id="model-operator"
      initial={{
        roleDraft: {
          id: 'model-operator',
          name: 'Model operator',
          description: 'Use models in production.',
          source: advancedSource,
          mode: 'visual',
          projection: [],
          diagnostics: [],
          parsing: false,
          builtin: false,
        },
      }}
    />
  ),
}
export const ScopedCedarRetained: Story = {
  render: () => (
    <Demo
      page="role"
      id="model-operator"
      initial={{
        roleDraft: {
          id: 'model-operator',
          name: 'Model operator',
          description: 'An imported resource-restricted policy.',
          source: modelOperatorSource.replace(
            '?resource\n);',
            '?resource\n) when { ["general-chat"].contains(resource.name) && context.environment == "production" };',
          ),
          mode: 'visual',
          projection: [],
          diagnostics: [],
          parsing: false,
          builtin: false,
        },
      }}
    />
  ),
}
export const CedarSource: Story = {
  render: () => (
    <Demo
      page="role"
      id="model-operator"
      initial={{
        roleDraft: {
          id: 'model-operator',
          name: 'Model operator',
          description: 'Use all models and view own usage.',
          source: modelOperatorSource,
          mode: 'cedar',
          projection: [],
          diagnostics: [],
          parsing: false,
          builtin: false,
        },
      }}
    />
  ),
}
export const AdvancedCedar: Story = {
  render: () => (
    <Demo
      page="role"
      id="model-operator"
      initial={{
        roleDraft: {
          id: 'model-operator',
          name: 'Model operator',
          description: 'Use models in production.',
          source: advancedSource,
          mode: 'cedar',
          projection: [],
          diagnostics: [],
          parsing: false,
          builtin: false,
        },
      }}
    />
  ),
}
export const InvalidCedar: Story = {
  render: () => (
    <Demo
      page="role"
      id="model-operator"
      initial={{
        roleDraft: {
          id: 'model-operator',
          name: 'Model operator',
          description: 'An incomplete source draft.',
          source: modelOperatorSource.replace('when {', 'when '),
          mode: 'cedar',
          projection: [],
          diagnostics: ['Syntax error · Expected “{” after when.'],
          parsing: false,
          builtin: false,
        },
      }}
    />
  ),
}
export const UsageAndLimits: Story = { render: () => <Demo page="usage-limits" /> }
export const MemberLimitOverrides: Story = { render: () => <Demo page="member-limits" /> }
export const UsageExplorer: Story = { render: () => <Demo page="usage-explorer" /> }
export const AuditLog: Story = { render: () => <Demo page="audit" /> }
export const Loading: Story = { render: () => <Demo initial={{ status: 'loading' }} /> }
export const AccessDenied: Story = { render: () => <Demo page="role" id="model-operator" initial={{ status: 'denied' }} /> }
export const ReadOnly: Story = { render: () => <Demo page="role" id="model-operator" initial={{ readonly: true }} /> }
export const SaveFailure: Story = { render: () => <Demo page="role" id="model-operator" failure /> }
export const ConcurrentChange: Story = { render: () => <Demo page="role" id="model-operator" failure="conflict" /> }
export const PasskeyVerification: Story = {
  render: () => (
    <Demo
      page="security"
      initial={{
        dialog: { kind: 'verify-passkey', name: 'My passkey', value: '', phrase: '' },
      }}
    />
  ),
}
export const KeyCreated: Story = {
  render: () => (
    <Demo
      page="api-keys"
      initial={{
        dialog: {
          kind: 'reveal-key',
          name: 'Development',
          value: '',
          phrase: '',
          revealedKey: 'preview_key_not_a_real_credential',
        },
      }}
    />
  ),
}

export const ApiErrorToast: Story = {
  render: () => <Demo page="role" id="model-operator" initial={{ error: 'Name already exists in this organization' }} />,
}

export const GroupLoading: Story = { render: () => <Demo page="group" id="research" groupLoadingMs={1200} /> }
export const GroupPagedMembers: Story = { render: () => <Demo page="group" id="research" pagedGroup /> }
export const GroupSaveFailure: Story = { render: () => <Demo page="group" id="research" failure /> }
export const DeleteGroup: Story = {
  render: () => (
    <Demo
      page="group"
      id="research"
      initial={{ dialog: { kind: 'delete-group', targetId: 'research', name: '', value: '', phrase: '' } }}
    />
  ),
}

export const PermissionRestrictedRole: Story = {
  ...EditRole,
  parameters: {
    deniedPermissionActions: ['role.metadata.update', 'role.policy.update', 'role.delete', 'role.create', 'group.create'],
  },
}
export const PermissionRestrictedGroup: Story = {
  ...EditGroup,
  parameters: {
    deniedPermissionActions: [
      'group.metadata.update',
      'group.member.add',
      'group.member.remove',
      'group.role.assign',
      'group.role.unassign',
      'group.delete',
    ],
  },
}
export const PermissionHiddenPolicies: Story = {
  ...EditRole,
  parameters: { deniedPermissionActions: ['role.policy.read'] },
}

export const MembersEmpty: Story = { render: () => <Demo page="members" initial={{ members: [] }} /> }
export const MembersLoading: Story = { render: () => <Demo page="members" initial={{ membersLoading: true }} /> }
export const MembersPaged: Story = {
  render: () => <Demo page="members" initial={{ membersHasNextPage: true, membersComplete: false }} />,
}
export const MembersLight: Story = { ...Members, globals: { theme: 'BASIC_WHITE' } }
export const MembersLongLabels: Story = {
  render: () => (
    <Demo
      page="members"
      initial={{
        members: [
          {
            id: 'long',
            name: 'Organization member with an exceptionally long display name',
            email: 'long.email.address@example-organization.com',
            role: { id: 'custom', name: 'Custom role with a long display name', description: '', color: '', owner: false },
            groups: [{ id: 'research', name: 'Research and development group with a long name', description: '' }],
            joinedAt: 1794000000,
            limitUsd: null,
            currentUsageUsd: null,
            previousUsageUsd: null,
          },
        ],
      }}
    />
  ),
}

export const MembersAsAdmin: Story = { render: () => <Demo page="members" initial={{ actorIsOwner: false }} /> }
export const MembersMultipleOwners: Story = {
  render: () => <Demo page="members" initial={{ actorIsOwner: true, ownerMemberIds: ['alex', 'jamie'] }} />,
}
export const MemberOwnerDemotion: Story = {
  render: () => (
    <Demo page="members" initial={{ actorIsOwner: true, currentMemberId: 'other', ownerMemberIds: ['alex', 'other'] }} />
  ),
}
export const MemberOwnerDemotionRetry: Story = {
  render: () => (
    <Demo
      page="members"
      failure
      initial={{ actorIsOwner: true, currentMemberId: 'other', ownerMemberIds: ['alex', 'other'] }}
    />
  ),
}
export const MembersOwnRoleReadOnly: Story = {
  render: () => <Demo page="members" initial={{ actorIsOwner: true, currentMemberId: 'alex', ownerMemberIds: ['alex'] }} />,
}

export const MembershipInvitations: Story = {
  render: () => (
    <Demo
      page="members"
      initial={{
        invitations: [
          { id: 'pending', email: 'alex@example.com', status: 'pending', expiresAt: 2000000000 },
          { id: 'expired', email: 'expired@example.com', status: 'expired', expiresAt: 1700000000 },
        ],
        invitationLink: 'https://compallm.example/api/invitations/open?token=preview',
        invitationEmailAdmitted: false,
        invitationsOpen: false,
        membersHasNextPage: true,
      }}
    />
  ),
}

export const InviteDialog: Story = { render: () => <Demo page="members" initial={{ invitationsOpen: true }} /> }

export const MembersScrollFailure: Story = {
  render: () => <Demo page="members" initial={{ membersHasNextPage: true, membersComplete: false, membersLoadError: true }} />,
}
export const InvitationsScroll: Story = {
  render: () => (
    <Demo
      page="members"
      initial={{
        invitations: Array.from({ length: 20 }, (_, index) => ({
          id: `invite-${index}`,
          email: `person${index}@example.com`,
          status: 'pending',
          expiresAt: 2000000000,
        })),
        invitationsHasNextPage: true,
      }}
    />
  ),
}

export const RecentInvitationDates: Story = {
  render: () => (
    <Demo
      page="members"
      initial={{
        invitations: [30, 300, 3600, 7200, 90000].map((age, index) => ({
          id: `recent-${index}`,
          email: `person${index}@example.com`,
          status: 'pending',
          issuedAt: Math.floor(Date.now() / 1000) - age,
          expiresAt: Math.floor(Date.now() / 1000) - age + 604800,
        })),
      }}
    />
  ),
}

export const MembersRenderingBatches: Story = {
  render: () => (
    <Demo
      page="members"
      initial={{
        members: Array.from({ length: 20 }, (_, index) => ({
          ...initialSettings.members![1],
          id: `batch-${index}`,
          name: `Member ${index}`,
          email: `member${index}@example.com`,
        })),
      }}
    />
  ),
}

export const MembersRoleUpdates: Story = {
  render: () => (
    <Demo
      page="members"
      initial={{
        members: Array.from({ length: 1000 }, (_, index) => ({
          ...initialSettings.members![1],
          id: `editable-${index}`,
          name: `Member ${index}`,
          email: `member${index}@example.com`,
        })),
      }}
    />
  ),
}

export const SessionsMinimalMobile: Story = {
  ...SessionsReference,
  parameters: { viewport: { options: { sessionMobile: { name: 'Mobile', styles: { width: '390px', height: '844px' } } } } },
  globals: { viewport: { value: 'sessionMobile', isRotated: false } },
}

export const SessionsMinimalLight: Story = {
  ...SessionsReference,
  globals: { theme: 'BASIC_WHITE' },
}

export const SessionsRecentActivity: Story = {
  render: () => {
    const now = Math.floor(Date.now() / 1000)
    return (
      <Demo
        page="access"
        session={{
          ipAddress: '203.0.113.42',
          userAgent: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 Chrome/131.0.0.0 Safari/537.36',
          signedInAt: String(now - 7200),
          lastSeenAt: String(now - 60),
        }}
        initial={{
          otherSessions: [
            {
              id: 'recent',
              browser: 'Firefox on macOS',
              location: 'Ho Chi Minh City, VN',
              lastSeenAt: String(now - 3600),
              signedInAt: String(now - 86400 * 3),
            },
            {
              id: 'older',
              browser: 'Safari on iOS',
              location: null,
              ipAddress: '203.0.113.43',
              lastSeenAt: String(now - 86400),
              signedInAt: String(now - 86400 * 7),
            },
          ],
        }}
      />
    )
  },
}
