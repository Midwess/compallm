import type { SettingsState } from '../../../src/pages/settings'
import { policyActions } from './policy-catalog'
export { policyActions } from './policy-catalog'

export function actionSource(action: string) {
  const condition = action === 'usage.read.own' ? ' when { resource.owner == principal }' : ''
  return (
    `permit(\n  principal,\n  action == Compallm::Action::${JSON.stringify(action)},\n` +
    `  resource in Compallm::Organization::"org-7"\n)${condition};`
  )
}
export const modelOperatorSource = `${actionSource('model.use')}\n\n${actionSource('usage.read.own')}`
export const advancedSource =
  '// Restrict model use to the production environment.\n' +
  modelOperatorSource.replace('org-7"\n);', 'org-7"\n) when { context.environment == "production" };')

export const initialSettings: Partial<SettingsState> = {
  codeExamples: {
    typescript: {
      code:
        'const config = {\n  apiUrl: "https://api.example.com",\n  timeout: 5000,\n  debug: true,\n' +
        '  headers: { "Content-Type": "application/json" },\n};\n' +
        'async function fetchUser(id: string): Promise<User> {\n  const url = `${config.apiUrl}/users/${id}`;\n' +
        'async function fetchUser(id: string): Promise<User | null> {\n  const url = `${config.apiUrl}/v2/users/${id}`;\n' +
        '  const res = await fetch(url);\n  return res.ok ? res.json() : null;',
      lineChanges: { 4: 'removed', 5: 'added', 7: 'removed', 8: 'removed', 9: 'added', 10: 'added', 12: 'added' },
    },
    javascript: {
      code:
        'const config = {\n  apiUrl: "https://api.example.com",\n  timeout: 5000,\n};\n\n' +
        'async function fetchUser(id) {\n  const res = await fetch(`${config.apiUrl}/users/${id}`);\n' +
        '  return res.ok ? res.json() : null;\n}',
    },
    json: { code: '{\n  "model": "general-chat",\n  "stream": true\n}' },
    bash: { code: 'curl https://api.example.com/v1/models \\\n  -H "Content-Type: application/json"' },
    cedar: { code: modelOperatorSource },
  },
  profile: { name: 'Alex Morgan', username: 'alex', email: 'alex@example.com', avatar: null },
  organization: {
    name: 'Compallm',
    slug: 'compallm',
    logo: null,
    fiscalMonth: 'January',
    region: 'Not configured',
    welcome: '',
    announcements: '',
    home: 'Chat',
  },
  hasOtherSessions: true,
  policyActions,
  models: ['general-chat', 'coding-assistant'],
  roles: [
    { id: 'owner', name: 'Owner', description: 'Organization ownership', source: '', builtin: true, color: '#98999C' },
    {
      id: 'admin',
      name: 'Administrator',
      description: 'Organization administration',
      source: '',
      builtin: true,
      color: '#98999C',
    },
    { id: 'member', name: 'Member', description: 'Member access', source: '', builtin: true, color: '#98999C' },
    {
      id: 'model-operator',
      name: 'Model operator',
      description: 'Use all models and view own usage.',
      source: modelOperatorSource,
      color: '#A49ADA',
    },
  ],
  groups: [
    {
      id: 'research',
      name: 'Research',
      description: 'People exploring models and research.',
      memberIds: ['jamie', 'sam'],
      roleIds: ['model-operator'],
      color: '#6EB8A8',
    },
  ],
  members: [
    {
      id: 'alex',
      name: 'Alex Morgan',
      email: 'alex@example.com',
      roleIds: ['owner'],
      limitUsd: null,
      currentUsageUsd: null,
      previousUsageUsd: null,
    },
    {
      id: 'jamie',
      name: 'Jamie Chen',
      email: 'jamie@example.com',
      roleIds: ['member'],
      limitUsd: null,
      currentUsageUsd: null,
      previousUsageUsd: null,
    },
    {
      id: 'sam',
      name: 'Sam Lee',
      email: 'sam@example.com',
      roleIds: ['member'],
      limitUsd: null,
      currentUsageUsd: null,
      previousUsageUsd: null,
    },
  ],
}
