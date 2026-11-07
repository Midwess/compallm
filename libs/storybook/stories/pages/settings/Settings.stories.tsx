import { useEffect, useState } from 'react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { SettingsControllerContext } from '../../../src/context'
import { SettingsPage } from '../../../src/pages/settings'
import type { SettingsState, SettingsDestination } from '../../../src/pages/settings'
import { DemoSettingsController } from './demo-controller'
import { advancedSource, modelOperatorSource } from './fixtures'

function Demo({
  page = 'general',
  id,
  initial = {},
  failure = false,
}: {
  page?: SettingsDestination
  id?: string
  initial?: Partial<SettingsState>
  failure?: boolean | 'conflict'
}) {
  const [controller] = useState(() => new DemoSettingsController({ location: { page, id }, ...initial }, failure))
  useEffect(() => {
    controller.start()
    return controller.stop
  }, [controller])
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

export const General: Story = { render: () => <Demo /> }
export const Appearance: Story = { render: () => <Demo page="appearance" /> }
export const Messaging: Story = { render: () => <Demo page="messaging" /> }
export const CodeAppearance: Story = { render: () => <Demo page="code" /> }
export const Profile: Story = { render: () => <Demo page="profile" /> }
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
export const GroupReadOnly: Story = {
  render: () => <Demo page="group" id="research" initial={{ readonly: true }} />,
}
export const EditRole: Story = { render: () => <Demo page="role" id="model-operator" /> }
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
            'org-7"\n);',
            'org-7"\n) when { ["general-chat"].contains(resource.name) && context.environment == "production" };',
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
