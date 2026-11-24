import { redirect, error, type RequestEvent } from '@sveltejs/kit'
import { parseString } from 'set-cookie-parser'
import { requestGraphql } from './transport'
import { readResponse, NetworkError } from '../../network/service'
import type { Mutable } from '../../graphql-client'
import AuthSession2, {
  type AuthSession2$result,
  type AuthSession2$input,
} from '@compallm/contract/houdini/artifacts/AuthSession2.ts'
import PortalAuthority, {
  type PortalAuthority$result,
  type PortalAuthority$input,
} from '@compallm/contract/houdini/artifacts/PortalAuthority.ts'
import PreferenceUserUserPreferences, {
  type PreferenceUserUserPreferences$result,
  type PreferenceUserUserPreferences$input,
} from '@compallm/contract/houdini/artifacts/PreferenceUserUserPreferences.ts'
import OrganizationProviders, {
  type OrganizationProviders$result,
  type OrganizationProviders$input,
} from '@compallm/contract/houdini/artifacts/OrganizationProviders.ts'
import SettingsMeAndOrganizationAndRolesAndGroups, {
  type SettingsMeAndOrganizationAndRolesAndGroups$result,
  type SettingsMeAndOrganizationAndRolesAndGroups$input,
} from '@compallm/contract/houdini/artifacts/SettingsMeAndOrganizationAndRolesAndGroups.ts'
import SettingsProfileUserProfile, {
  type SettingsProfileUserProfile$result,
  type SettingsProfileUserProfile$input,
} from '@compallm/contract/houdini/artifacts/SettingsProfileUserProfile.ts'
import SettingsOrganizationOrganizationProfile, {
  type SettingsOrganizationOrganizationProfile$result,
  type SettingsOrganizationOrganizationProfile$input,
} from '@compallm/contract/houdini/artifacts/SettingsOrganizationOrganizationProfile.ts'
import SettingsOnboardingOrganizationOnboardingSettings, {
  type SettingsOnboardingOrganizationOnboardingSettings$result,
  type SettingsOnboardingOrganizationOnboardingSettings$input,
} from '@compallm/contract/houdini/artifacts/SettingsOnboardingOrganizationOnboardingSettings.ts'
import SettingsMembersMeAndOrganizationAndMembers, {
  type SettingsMembersMeAndOrganizationAndMembers$result,
  type SettingsMembersMeAndOrganizationAndMembers$input,
} from '@compallm/contract/houdini/artifacts/SettingsMembersMeAndOrganizationAndMembers.ts'
import SettingsMembersInvitations, {
  type SettingsMembersInvitations$result,
  type SettingsMembersInvitations$input,
} from '@compallm/contract/houdini/artifacts/SettingsMembersInvitations.ts'
import SettingsSessions, {
  type SettingsSessions$result,
  type SettingsSessions$input,
} from '@compallm/contract/houdini/artifacts/SettingsSessions.ts'
import SettingsGroupGroupAndRolesAndMembers, {
  type SettingsGroupGroupAndRolesAndMembers$result,
  type SettingsGroupGroupAndRolesAndMembers$input,
} from '@compallm/contract/houdini/artifacts/SettingsGroupGroupAndRolesAndMembers.ts'
import SettingsRoleRole, {
  type SettingsRoleRole$result,
  type SettingsRoleRole$input,
} from '@compallm/contract/houdini/artifacts/SettingsRoleRole.ts'
import SettingsRoleOrganizationPolicyActions, {
  type SettingsRoleOrganizationPolicyActions$result,
  type SettingsRoleOrganizationPolicyActions$input,
} from '@compallm/contract/houdini/artifacts/SettingsRoleOrganizationPolicyActions.ts'
import PasskeyPasskeysAndSession, {
  type PasskeyPasskeysAndSession$result,
  type PasskeyPasskeysAndSession$input,
} from '@compallm/contract/houdini/artifacts/PasskeyPasskeysAndSession.ts'
import AuthInvitationControllerInvitationContext, {
  type AuthInvitationControllerInvitationContext$result,
  type AuthInvitationControllerInvitationContext$input,
} from '@compallm/contract/houdini/artifacts/AuthInvitationControllerInvitationContext.ts'
import OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile, {
  type OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile$result,
  type OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile$input,
} from '@compallm/contract/houdini/artifacts/OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile.ts'
import ProviderProvider, {
  type ProviderProvider$result,
  type ProviderProvider$input,
} from '@compallm/contract/houdini/artifacts/ProviderProvider.ts'
import ProviderCatalogEffectiveProviderCatalogs, {
  type ProviderCatalogEffectiveProviderCatalogs$result,
  type ProviderCatalogEffectiveProviderCatalogs$input,
} from '@compallm/contract/houdini/artifacts/ProviderCatalogEffectiveProviderCatalogs.ts'
import ProviderCatalogEffectiveProviderCatalog, {
  type ProviderCatalogEffectiveProviderCatalog$result,
  type ProviderCatalogEffectiveProviderCatalog$input,
} from '@compallm/contract/houdini/artifacts/ProviderCatalogEffectiveProviderCatalog.ts'
import ProviderCatalogEffectiveModelCatalog, {
  type ProviderCatalogEffectiveModelCatalog$result,
  type ProviderCatalogEffectiveModelCatalog$input,
} from '@compallm/contract/houdini/artifacts/ProviderCatalogEffectiveModelCatalog.ts'
import ProviderCatalogModelCatalog, {
  type ProviderCatalogModelCatalog$result,
  type ProviderCatalogModelCatalog$input,
} from '@compallm/contract/houdini/artifacts/ProviderCatalogModelCatalog.ts'

const operations = {
  AuthSession2,
  PortalAuthority,
  PreferenceUserUserPreferences,
  OrganizationProviders,
  SettingsMeAndOrganizationAndRolesAndGroups,
  SettingsProfileUserProfile,
  SettingsOrganizationOrganizationProfile,
  SettingsOnboardingOrganizationOnboardingSettings,
  SettingsMembersMeAndOrganizationAndMembers,
  SettingsMembersInvitations,
  SettingsSessions,
  SettingsGroupGroupAndRolesAndMembers,
  SettingsRoleRole,
  SettingsRoleOrganizationPolicyActions,
  PasskeyPasskeysAndSession,
  AuthInvitationControllerInvitationContext,
  OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile,
  ProviderProvider,
  ProviderCatalogEffectiveProviderCatalogs,
  ProviderCatalogEffectiveProviderCatalog,
  ProviderCatalogEffectiveModelCatalog,
  ProviderCatalogModelCatalog,
}
type Results = {
  AuthSession2: AuthSession2$result
  PortalAuthority: PortalAuthority$result
  PreferenceUserUserPreferences: PreferenceUserUserPreferences$result
  OrganizationProviders: OrganizationProviders$result
  SettingsMeAndOrganizationAndRolesAndGroups: SettingsMeAndOrganizationAndRolesAndGroups$result
  SettingsProfileUserProfile: SettingsProfileUserProfile$result
  SettingsOrganizationOrganizationProfile: SettingsOrganizationOrganizationProfile$result
  SettingsOnboardingOrganizationOnboardingSettings: SettingsOnboardingOrganizationOnboardingSettings$result
  SettingsMembersMeAndOrganizationAndMembers: SettingsMembersMeAndOrganizationAndMembers$result
  SettingsMembersInvitations: SettingsMembersInvitations$result
  SettingsSessions: SettingsSessions$result
  SettingsGroupGroupAndRolesAndMembers: SettingsGroupGroupAndRolesAndMembers$result
  SettingsRoleRole: SettingsRoleRole$result
  SettingsRoleOrganizationPolicyActions: SettingsRoleOrganizationPolicyActions$result
  PasskeyPasskeysAndSession: PasskeyPasskeysAndSession$result
  AuthInvitationControllerInvitationContext: AuthInvitationControllerInvitationContext$result
  OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile: OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile$result
  ProviderProvider: ProviderProvider$result
  ProviderCatalogEffectiveProviderCatalogs: ProviderCatalogEffectiveProviderCatalogs$result
  ProviderCatalogEffectiveProviderCatalog: ProviderCatalogEffectiveProviderCatalog$result
  ProviderCatalogEffectiveModelCatalog: ProviderCatalogEffectiveModelCatalog$result
  ProviderCatalogModelCatalog: ProviderCatalogModelCatalog$result
}
type Inputs = {
  AuthSession2: AuthSession2$input
  PortalAuthority: PortalAuthority$input
  PreferenceUserUserPreferences: PreferenceUserUserPreferences$input
  OrganizationProviders: OrganizationProviders$input
  SettingsMeAndOrganizationAndRolesAndGroups: SettingsMeAndOrganizationAndRolesAndGroups$input
  SettingsProfileUserProfile: SettingsProfileUserProfile$input
  SettingsOrganizationOrganizationProfile: SettingsOrganizationOrganizationProfile$input
  SettingsOnboardingOrganizationOnboardingSettings: SettingsOnboardingOrganizationOnboardingSettings$input
  SettingsMembersMeAndOrganizationAndMembers: SettingsMembersMeAndOrganizationAndMembers$input
  SettingsMembersInvitations: SettingsMembersInvitations$input
  SettingsSessions: SettingsSessions$input
  SettingsGroupGroupAndRolesAndMembers: SettingsGroupGroupAndRolesAndMembers$input
  SettingsRoleRole: SettingsRoleRole$input
  SettingsRoleOrganizationPolicyActions: SettingsRoleOrganizationPolicyActions$input
  PasskeyPasskeysAndSession: PasskeyPasskeysAndSession$input
  AuthInvitationControllerInvitationContext: AuthInvitationControllerInvitationContext$input
  OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile: OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile$input
  ProviderProvider: ProviderProvider$input
  ProviderCatalogEffectiveProviderCatalogs: ProviderCatalogEffectiveProviderCatalogs$input
  ProviderCatalogEffectiveProviderCatalog: ProviderCatalogEffectiveProviderCatalog$input
  ProviderCatalogEffectiveModelCatalog: ProviderCatalogEffectiveModelCatalog$input
  ProviderCatalogModelCatalog: ProviderCatalogModelCatalog$input
}

export class PortalRequest {
  private readonly reads = new Map<string, Promise<unknown>>()
  private identity?: ReturnType<PortalRequest['readIdentity']>
  readonly preparationVersion: string | undefined
  constructor(private readonly event: RequestEvent) {
    this.preparationVersion = event.cookies.get('compallm-read-version')
  }
  async query<K extends keyof Results>(name: K, variables: Inputs[K] = {} as Inputs[K]): Promise<Mutable<Results[K]>> {
    const key = JSON.stringify([name, variables])
    const existing = this.reads.get(key)
    if (existing) return existing as Promise<Mutable<Results[K]>>
    const result = this.execute(name, variables)
    this.reads.set(key, result)
    return result
  }
  private async execute<K extends keyof Results>(name: K, variables: Inputs[K]): Promise<Mutable<Results[K]>> {
    const { event } = this
    const headers = new Headers({ 'Content-Type': 'application/json', Origin: event.url.origin, Host: event.url.host })
    headers.set(
      'Cookie',
      event.cookies
        .getAll()
        .map(({ name, value }) => `${name}=${encodeURIComponent(value)}`)
        .join('; '),
    )
    const agent = event.request.headers.get('user-agent')
    if (agent) headers.set('User-Agent', agent)
    const origin = process.env.RUST_API_ORIGIN || 'http://127.0.0.1:3000'
    const response = await requestGraphql(
      new URL('/api/graphql', origin),
      headers,
      JSON.stringify({ query: operations[name].raw, operationName: name, variables }),
      AbortSignal.any([event.request.signal, AbortSignal.timeout(45000)]),
    )
    for (const header of response.headers.getSetCookie()) {
      const cookie = parseString(header)
      if (!cookie) continue
      event.cookies.set(cookie.name, cookie.value, {
        path: cookie.path || '/',
        httpOnly: cookie.httpOnly,
        secure: Boolean(cookie.secure),
        sameSite: cookie.sameSite?.toLowerCase() as 'lax' | 'strict' | 'none' | undefined,
        ...(cookie.maxAge === undefined ? {} : { maxAge: cookie.maxAge }),
        ...(cookie.expires ? { expires: cookie.expires } : {}),
      })
    }
    const result = await readResponse(response)
    return result.data as Mutable<Results[K]>
  }
  session() {
    return (this.identity ??= this.readIdentity())
  }
  private async readIdentity() {
    try {
      return (await this.query('AuthSession2')).session.node
    } catch (failure) {
      if (failure instanceof NetworkError && failure.status === 401) return null
      console.error(
        'SSR session verification failed',
        failure instanceof NetworkError
          ? { status: failure.status, code: failure.code }
          : failure instanceof Error
            ? failure.name
            : 'unknown',
      )
      error(503, 'Session could not be verified. Please retry.')
    }
  }
  async requireUser(organization?: string) {
    const user = await this.session()
    if (!user) redirect(303, '/login?' + new URLSearchParams({ return_to: this.event.url.pathname + this.event.url.search }))
    if (organization && user.organizationName !== organization) redirect(303, '/' + encodeURIComponent(user.organizationName))
    return user
  }
}
