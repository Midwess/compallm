import { createFormStore } from '../../components/form'
import type { SettingsState } from './types'
import { initialPreferenceDraft } from './preference-values'

export function createSettingsStore(initial: Partial<SettingsState> = {}) {
  return createFormStore<SettingsState>(
    {
      profileUpload: null,
      organizationUpload: null,
      codeExamples: {},
      location: { page: 'general' },
      navigationQuery: '',
      query: '',
      filters: {},
      preferences: initialPreferenceDraft(),
      profile: { name: '', username: '', email: '', avatar: null },
      organization: {
        name: '',
        slug: '',
        logo: null,
        fiscalMonth: 'January',
        region: 'Not configured',
        welcome: '',
        announcements: '',
        home: 'Chat',
      },
      roles: [],
      policyActions: [],
      models: [],
      groups: [],
      members: [],
      passkeys: [],
      applications: [],
      apiKeys: [],
      hasOtherSessions: false,
      organizationLimitUsd: null,
      memberDefaultLimitUsd: null,
      resetFrequency: 'Monthly',
      timezone: 'Workspace timezone',
      usage: [],
      audit: [],
      dialog: null,
      busy: false,
      error: null,
      errorToast: null,
      notice: null,
      readonly: false,
      status: 'ready',
      ...initial,
    },
    ['profile', 'organization', 'preferences', 'dialog', 'profileUpload', 'organizationUpload'],
  )
}
