import { advancedSource, modelOperatorSource, initialSettings } from './fixtures'
import { userPreferenceDefaults } from '@compallm/contract/preference'
import type { Meta, StoryObj } from '@storybook/svelte-vite'
import Preview from './Settings.stories-Demo.svelte'
import SessionsRecentActivityPreview from './Settings.stories-StorySessionsRecentActivity.svelte'
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
  component: Preview,
} satisfies Meta<typeof Preview>
export default meta
type Story = StoryObj<typeof meta>
export const Blank: Story = { args: { page: 'blank' } }
export const General: Story = { args: {} }
export const PreferencesLoading: Story = { args: { preferenceStatus: 'loading' } }
export const PreferencesLoadFailure: Story = { args: { preferenceStatus: 'error' } }
export const PreferencesDenied: Story = { args: { preferenceStatus: 'denied' } }
export const SavedPreferences: Story = {
  args: {
    savedPreferences: {
      ...userPreferenceDefaults,
      navigation: { home: 'MODELS' },
      memberDisplay: { nameFormat: 'USERNAME' },
      calendar: { firstDay: 'SUNDAY' },
      sidebar: {
        ...userPreferenceDefaults.sidebar,
        layout: userPreferenceDefaults.sidebar.layout.map((section) => ({ ...section, items: [...section.items] })),
        entries: [...userPreferenceDefaults.sidebar.entries],
      },
    },
  },
}
export const GeneralNarrow: Story = {
  parameters: {
    viewport: { options: { settingsNarrow: { name: 'Narrow', styles: { width: '625px', height: '900px' } } } },
  },
  globals: { viewport: { value: 'settingsNarrow', isRotated: false } },
  args: {},
}
export const GeneralCompactDesktop: Story = {
  parameters: {
    viewport: { options: { settingsCompact: { name: 'Compact desktop', styles: { width: '900px', height: '900px' } } } },
  },
  globals: { viewport: { value: 'settingsCompact', isRotated: false } },
  args: {},
}
export const SidebarPreferences: Story = {
  args: { page: 'general', initial: { dialog: { kind: 'customize-sidebar', name: '', value: '', phrase: '' } } },
}
export const Messaging: Story = { args: { page: 'messaging' } }
export const CodeAppearance: Story = { args: { page: 'code' } }
export const PreferenceSaveFailure: Story = { args: { page: 'general', failure: true } }
export const Profile: Story = { args: { page: 'profile' } }
export const ProfileLoading: Story = { args: { page: 'profile', initial: { profileStatus: 'loading' } } }
export const ProfileMissing: Story = { args: { page: 'profile', initial: { profileStatus: 'missing' } } }
export const ProfileDenied: Story = { args: { page: 'profile', initial: { profileStatus: 'denied' } } }
export const ProfileLoadFailure: Story = { args: { page: 'profile', initial: { profileStatus: 'error' } } }
export const ProfileWithPhoto: Story = {
  args: {
    page: 'profile',
    initial: {
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
    },
  },
}
export const Access: Story = {
  args: {
    page: 'access',
    initial: { sessionLocation: { sessionId: 'session', ipAddress: '203.0.113.42', location: 'Vietnam' } },
    session: {
      ipAddress: '203.0.113.42',
      userAgent:
        'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) ' +
        'AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36',
    },
  },
}
export const AccessUnknownSession: Story = { args: { page: 'access' } }
export const AccessMobileSession: Story = {
  args: {
    page: 'access',
    initial: { sessionLocation: { sessionId: 'session', ipAddress: '2001:db8::42', location: 'Ho Chi Minh City, Vietnam' } },
    session: {
      ipAddress: '2001:db8::42',
      userAgent:
        'Mozilla/5.0 (iPhone; CPU iPhone OS 18_0 like Mac OS X) ' +
        'AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.0 Mobile/15E148 Safari/604.1',
    },
  },
}
export const AccessLight: Story = { ...Access, globals: { theme: 'basic-white' } }
export const SessionIpFallback: Story = {
  args: {
    page: 'access',
    session: {
      ipAddress: '203.0.113.42',
      userAgent: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 Chrome/131.0.0.0 Safari/537.36',
    },
  },
}
export const SessionsLoading: Story = {
  args: { page: 'access', initial: { sessionsLoading: true, sessionsLoaded: false, otherSessions: [] } },
}
export const SessionsLoadFailure: Story = { args: { page: 'access', initial: { sessionsError: true } } }
export const SessionsReference: Story = {
  args: {
    page: 'security',
    session: {
      ipAddress: '203.0.113.42',
      signedInAt: '1794000000',
      lastSeenAt: '1794877200',
      userAgent: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 Chrome/131.0.0.0 Safari/537.36',
    },
    initial: {
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
    },
  },
}
export const Security: Story = { args: { page: 'security' } }
export const AccessPopulated: Story = {
  args: {
    page: 'access',
    initial: {
      applications: [{ id: 'app', name: 'Example integration', scope: 'read your profile' }],
    },
  },
}
export const ApiKeys: Story = { args: { page: 'api-keys' } }
export const ApiKeysPopulated: Story = {
  args: {
    page: 'api-keys',
    initial: {
      apiKeys: [{ id: 'key', name: 'Development', prefix: 'preview_key_••••' }],
    },
  },
}
export const SecurityPopulated: Story = {
  args: { page: 'security', initial: { passkeys: [{ id: 'passkey', name: 'My passkey' }] } },
}
export const Organization: Story = { args: { page: 'organization' } }
export const OrganizationReadOnly: Story = { args: { page: 'organization', initial: { organizationCanUpdateProfile: false } } }
export const OrganizationLoading: Story = { args: { page: 'organization', initial: { organizationProfileStatus: 'loading' } } }
export const OrganizationLoadFailure: Story = {
  args: { page: 'organization', initial: { organizationProfileStatus: 'error' } },
}
export const OrganizationOnboarding: Story = { args: { page: 'organization-onboarding' } }
export const OrganizationOnboardingLight: Story = { ...OrganizationOnboarding, globals: { theme: 'basic-white' } }
export const Members: Story = { args: { page: 'members' } }
export const EditGroup: Story = { args: { page: 'group', id: 'research' } }
export const NewGroup: Story = { args: { page: 'new-group' } }
export const ConfiguredGroup: Story = {
  args: {
    page: 'new-group',
    initial: {
      groupDraft: {
        id: null,
        name: 'Product team',
        description: 'People building the product experience.',
        memberIds: ['jamie', 'sam'],
        roleIds: ['model-operator'],
      },
    },
  },
}
export const GroupMemberSearch: Story = {
  args: { page: 'group', id: 'research', initial: { filters: { 'group-members-query': 'Jamie' } } },
}
export const GroupRoleSearch: Story = {
  args: { page: 'group', id: 'research', initial: { filters: { 'group-roles-query': 'Model' } } },
}
export const GroupSearchEmpty: Story = {
  args: { page: 'group', id: 'research', initial: { filters: { 'group-members-query': 'no member matches' } } },
}
export const GroupRolesEmpty: Story = { args: { page: 'new-group', initial: { roles: [] } } }
export const GroupReadOnly: Story = { args: { page: 'group', id: 'research', initial: { readonly: true } } }
export const EditRole: Story = { args: { page: 'role', id: 'model-operator' } }
export const RoleLoading: Story = { args: { page: 'role', id: 'model-operator', roleLoadingMs: 1200 } }
export const DeleteRole: Story = {
  args: {
    page: 'role',
    id: 'model-operator',
    initial: { dialog: { kind: 'delete-role', targetId: 'model-operator', name: '', value: '', phrase: '' } },
  },
}
export const BuiltInRole: Story = { args: { page: 'role', id: 'owner' } }
export const NewRole: Story = { args: { page: 'new-role' } }
export const ConfiguredRole: Story = {
  args: {
    page: 'new-role',
    initial: {
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
    },
  },
}
export const FilteredPolicies: Story = {
  args: { page: 'role', id: 'model-operator', initial: { filters: { 'policy-resource': 'Providers' } } },
}
export const SelectedPolicies: Story = {
  args: { page: 'role', id: 'model-operator', initial: { filters: { 'policy-selection': 'enabled' } } },
}
export const PolicySearchEmpty: Story = {
  args: { page: 'role', id: 'model-operator', initial: { query: 'no-matching-policy' } },
}
export const AdvancedVisual: Story = {
  args: {
    page: 'role',
    id: 'model-operator',
    initial: {
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
    },
  },
}
export const ScopedCedarRetained: Story = {
  args: {
    page: 'role',
    id: 'model-operator',
    initial: {
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
    },
  },
}
export const CedarSource: Story = {
  args: {
    page: 'role',
    id: 'model-operator',
    initial: {
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
    },
  },
}
export const AdvancedCedar: Story = {
  args: {
    page: 'role',
    id: 'model-operator',
    initial: {
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
    },
  },
}
export const InvalidCedar: Story = {
  args: {
    page: 'role',
    id: 'model-operator',
    initial: {
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
    },
  },
}
export const UsageAndLimits: Story = { args: { page: 'usage-limits' } }
export const MemberLimitOverrides: Story = { args: { page: 'member-limits' } }
export const UsageExplorer: Story = { args: { page: 'usage-explorer' } }
export const AuditLog: Story = { args: { page: 'audit' } }
export const Loading: Story = { args: { initial: { status: 'loading' } } }
export const AccessDenied: Story = { args: { page: 'role', id: 'model-operator', initial: { status: 'denied' } } }
export const ReadOnly: Story = { args: { page: 'role', id: 'model-operator', initial: { readonly: true } } }
export const SaveFailure: Story = { args: { page: 'role', id: 'model-operator', failure: true } }
export const ConcurrentChange: Story = { args: { page: 'role', id: 'model-operator', failure: 'conflict' } }
export const PasskeyVerification: Story = {
  args: {
    page: 'security',
    initial: {
      dialog: { kind: 'verify-passkey', name: 'My passkey', value: '', phrase: '' },
    },
  },
}
export const KeyCreated: Story = {
  args: {
    page: 'api-keys',
    initial: {
      dialog: {
        kind: 'reveal-key',
        name: 'Development',
        value: '',
        phrase: '',
        revealedKey: 'preview_key_not_a_real_credential',
      },
    },
  },
}
export const ApiErrorToast: Story = {
  args: { page: 'role', id: 'model-operator', initial: { error: 'Name already exists in this organization' } },
}
export const GroupLoading: Story = { args: { page: 'group', id: 'research', groupLoadingMs: 1200 } }
export const GroupPagedMembers: Story = { args: { page: 'group', id: 'research', pagedGroup: true } }
export const GroupSaveFailure: Story = { args: { page: 'group', id: 'research', failure: true } }
export const DeleteGroup: Story = {
  args: {
    page: 'group',
    id: 'research',
    initial: { dialog: { kind: 'delete-group', targetId: 'research', name: '', value: '', phrase: '' } },
  },
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
export const PermissionHiddenPolicies: Story = { ...EditRole, parameters: { deniedPermissionActions: ['role.policy.read'] } }
export const MembersEmpty: Story = { args: { page: 'members', initial: { members: [] } } }
export const MembersLoading: Story = {
  args: { page: 'members', initial: { membersLoading: true, membersLoaded: false, members: [] } },
}
export const MembersPaged: Story = { args: { page: 'members', initial: { membersHasNextPage: true, membersComplete: false } } }
export const MembersLight: Story = { ...Members, globals: { theme: 'basic-white' } }
export const MembersLongLabels: Story = {
  args: {
    page: 'members',
    initial: {
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
    },
  },
}
export const MembersAsAdmin: Story = { args: { page: 'members', initial: { actorIsOwner: false } } }
export const MembersMultipleOwners: Story = {
  args: { page: 'members', initial: { actorIsOwner: true, ownerMemberIds: ['alex', 'jamie'] } },
}
export const MemberOwnerDemotion: Story = {
  args: { page: 'members', initial: { actorIsOwner: true, currentMemberId: 'other', ownerMemberIds: ['alex', 'other'] } },
}
export const MemberOwnerDemotionRetry: Story = {
  args: {
    page: 'members',
    failure: true,
    initial: { actorIsOwner: true, currentMemberId: 'other', ownerMemberIds: ['alex', 'other'] },
  },
}
export const MembersOwnRoleReadOnly: Story = {
  args: { page: 'members', initial: { actorIsOwner: true, currentMemberId: 'alex', ownerMemberIds: ['alex'] } },
}
export const MembershipInvitations: Story = {
  args: {
    page: 'members',
    initial: {
      invitations: [
        { id: 'pending', email: 'alex@example.com', status: 'pending', expiresAt: 2000000000 },
        { id: 'expired', email: 'expired@example.com', status: 'expired', expiresAt: 1700000000 },
      ],
      invitationLink: 'https://compallm.example/api/invitations/open?token=preview',
      invitationEmailAdmitted: false,
      invitationsOpen: false,
      membersHasNextPage: true,
    },
  },
}
export const InviteDialog: Story = { args: { page: 'members', initial: { invitationsOpen: true } } }
export const MembersScrollFailure: Story = {
  args: { page: 'members', initial: { membersHasNextPage: true, membersComplete: false, membersLoadError: true } },
}
export const InvitationsScroll: Story = {
  args: {
    page: 'members',
    initial: {
      invitations: Array.from({ length: 20 }, (_, index) => ({
        id: `invite-${index}`,
        email: `person${index}@example.com`,
        status: 'pending',
        expiresAt: 2000000000,
      })),
      invitationsHasNextPage: true,
    },
  },
}
export const RecentInvitationDates: Story = {
  args: {
    page: 'members',
    initial: {
      invitations: [30, 300, 3600, 7200, 90000].map((age, index) => ({
        id: `recent-${index}`,
        email: `person${index}@example.com`,
        status: 'pending',
        issuedAt: Math.floor(Date.now() / 1000) - age,
        expiresAt: Math.floor(Date.now() / 1000) - age + 604800,
      })),
    },
  },
}
export const MembersRenderingBatches: Story = {
  args: {
    page: 'members',
    initial: {
      members: Array.from({ length: 20 }, (_, index) => ({
        ...initialSettings.members![1],
        id: `batch-${index}`,
        name: `Member ${index}`,
        email: `member${index}@example.com`,
      })),
    },
  },
}
export const MembersRoleUpdates: Story = {
  args: {
    page: 'members',
    initial: {
      members: Array.from({ length: 1000 }, (_, index) => ({
        ...initialSettings.members![1],
        id: `editable-${index}`,
        name: `Member ${index}`,
        email: `member${index}@example.com`,
      })),
    },
  },
}
export const SessionsMinimalMobile: Story = {
  ...SessionsReference,
  parameters: { viewport: { options: { sessionMobile: { name: 'Mobile', styles: { width: '390px', height: '844px' } } } } },
  globals: { viewport: { value: 'sessionMobile', isRotated: false } },
}
export const SessionsMinimalLight: Story = { ...SessionsReference, globals: { theme: 'basic-white' } }
export const SessionsRecentActivity: StoryObj<typeof SessionsRecentActivityPreview> = {
  render: () => ({ Component: SessionsRecentActivityPreview, props: {} }),
}
export const NavigationLoading: Story = {
  args: { initial: { navigationLoaded: false, navigationLoading: true, roles: [], groups: [] } },
}
export const NavigationLoadFailure: Story = {
  args: { initial: { navigationLoaded: false, navigationLoadError: true, roles: [], groups: [] } },
}
export const MembersRefreshing: Story = { args: { page: 'members', initial: { membersLoading: true, membersLoaded: true } } }
export const InvitationsLoading: Story = {
  args: { page: 'members', initial: { invitationsLoaded: false, invitationsLoading: true, invitations: [] } },
}
