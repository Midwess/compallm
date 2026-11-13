import { useEffect, useState } from 'react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { SettingsControllerContext } from '../../../src/context'
import { SettingsPage } from '../../../src/pages/settings'
import type { SettingsState, SettingsDestination, RoleDraft, GroupDraft } from '../../../src/pages/settings'
import { DemoSettingsController } from './demo-controller'
import { advancedSource, modelOperatorSource } from './fixtures'

function Demo({
  page = 'general',
  id,
  initial = {},
  failure = false,
  roleLoadingMs = 0,
}: {
  page?: SettingsDestination
  id?: string
  initial?: Partial<SettingsState> & { roleDraft?: RoleDraft | null; groupDraft?: GroupDraft | null }
  failure?: boolean | 'conflict'
  roleLoadingMs?: number
}) {
  const [controller] = useState(() => {
    const controller = new DemoSettingsController({ location: { page, id }, ...initial }, failure)
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
  return (
    <SettingsControllerContext value={controller}>
      <SettingsPage />
    </SettingsControllerContext>
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
export const Appearance: Story = { render: () => <Demo page="appearance" /> }
export const SidebarPreferences: Story = {
  render: () => <Demo page="appearance" initial={{ dialog: { kind: 'customize-sidebar', name: '', value: '', phrase: '' } }} />,
}
export const Messaging: Story = { render: () => <Demo page="messaging" /> }
export const CodeAppearance: Story = { render: () => <Demo page="code" /> }
export const Profile: Story = { render: () => <Demo page="profile" /> }
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
export const SecurityAndAccess: Story = { render: () => <Demo page="security" /> }
export const SecurityPopulated: Story = {
  render: () => (
    <Demo
      page="security"
      initial={{
        passkeys: [{ id: 'passkey', name: 'My passkey' }],
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
export const Danger: Story = { render: () => <Demo page="danger" /> }
export const Organization: Story = { render: () => <Demo page="organization" /> }
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
