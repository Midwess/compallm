// @ts-nocheck

import type { AuthEmailSigninConfirmStore } from '$houdini/plugins/houdini-svelte/stores/AuthEmailSigninConfirm'
import type { AuthEmailSigninConfirm2Store } from '$houdini/plugins/houdini-svelte/stores/AuthEmailSigninConfirm2'
import type { AuthEmailSigninRequestStore } from '$houdini/plugins/houdini-svelte/stores/AuthEmailSigninRequest'
import type { AuthEmailSignupConfirmStore } from '$houdini/plugins/houdini-svelte/stores/AuthEmailSignupConfirm'
import type { AuthEmailSignupRequestStore } from '$houdini/plugins/houdini-svelte/stores/AuthEmailSignupRequest'
import type { AuthInvitationControllerEmailSigninConfirmStore } from '$houdini/plugins/houdini-svelte/stores/AuthInvitationControllerEmailSigninConfirm'
import type { AuthInvitationControllerEmailSigninRequestStore } from '$houdini/plugins/houdini-svelte/stores/AuthInvitationControllerEmailSigninRequest'
import type { AuthInvitationControllerEmailSignupConfirmStore } from '$houdini/plugins/houdini-svelte/stores/AuthInvitationControllerEmailSignupConfirm'
import type { AuthInvitationControllerEmailSignupRequestStore } from '$houdini/plugins/houdini-svelte/stores/AuthInvitationControllerEmailSignupRequest'
import type { AuthInvitationControllerInvitationContextStore } from '$houdini/plugins/houdini-svelte/stores/AuthInvitationControllerInvitationContext'
import type { AuthPasskeyAuthenticationFinishStore } from '$houdini/plugins/houdini-svelte/stores/AuthPasskeyAuthenticationFinish'
import type { AuthPasskeyAuthenticationFinish2Store } from '$houdini/plugins/houdini-svelte/stores/AuthPasskeyAuthenticationFinish2'
import type { AuthPasskeyAuthenticationStartStore } from '$houdini/plugins/houdini-svelte/stores/AuthPasskeyAuthenticationStart'
import type { AuthPasskeyAuthenticationStart2Store } from '$houdini/plugins/houdini-svelte/stores/AuthPasskeyAuthenticationStart2'
import type { AuthSessionStore } from '$houdini/plugins/houdini-svelte/stores/AuthSession'
import type { AuthSession2Store } from '$houdini/plugins/houdini-svelte/stores/AuthSession2'
import type { AuthSessionRevokeStore } from '$houdini/plugins/houdini-svelte/stores/AuthSessionRevoke'
import type { AuthUserProfileStore } from '$houdini/plugins/houdini-svelte/stores/AuthUserProfile'
import type { OnboardingAnalyticUserProfilePutStore } from '$houdini/plugins/houdini-svelte/stores/OnboardingAnalyticUserProfilePut'
import type { OnboardingGenerateUrlAvatarUrlStore } from '$houdini/plugins/houdini-svelte/stores/OnboardingGenerateUrlAvatarUrl'
import type { OnboardingOnboardingAdvanceStore } from '$houdini/plugins/houdini-svelte/stores/OnboardingOnboardingAdvance'
import type { OnboardingOnboardingCompleteStore } from '$houdini/plugins/houdini-svelte/stores/OnboardingOnboardingComplete'
import type { OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfileStore } from '$houdini/plugins/houdini-svelte/stores/OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile'
import type { OnboardingOnboardingPresentStore } from '$houdini/plugins/houdini-svelte/stores/OnboardingOnboardingPresent'
import type { OnboardingOrganizationProfileUpdateStore } from '$houdini/plugins/houdini-svelte/stores/OnboardingOrganizationProfileUpdate'
import type { OnboardingUserProfileUpdateStore } from '$houdini/plugins/houdini-svelte/stores/OnboardingUserProfileUpdate'
import type { OrganizationProvidersStore } from '$houdini/plugins/houdini-svelte/stores/OrganizationProviders'
import type { PasskeyPasskeyRegistrationFinishStore } from '$houdini/plugins/houdini-svelte/stores/PasskeyPasskeyRegistrationFinish'
import type { PasskeyPasskeyRegistrationStartStore } from '$houdini/plugins/houdini-svelte/stores/PasskeyPasskeyRegistrationStart'
import type { PasskeyPasskeyRemoveStore } from '$houdini/plugins/houdini-svelte/stores/PasskeyPasskeyRemove'
import type { PasskeyPasskeyUpdateStore } from '$houdini/plugins/houdini-svelte/stores/PasskeyPasskeyUpdate'
import type { PasskeyPasskeysStore } from '$houdini/plugins/houdini-svelte/stores/PasskeyPasskeys'
import type { PasskeyPasskeys2Store } from '$houdini/plugins/houdini-svelte/stores/PasskeyPasskeys2'
import type { PasskeyPasskeysAndSessionStore } from '$houdini/plugins/houdini-svelte/stores/PasskeyPasskeysAndSession'
import type { PermissionMeStore } from '$houdini/plugins/houdini-svelte/stores/PermissionMe'
import type { PreferenceUserUserPreferencesStore } from '$houdini/plugins/houdini-svelte/stores/PreferenceUserUserPreferences'
import type { PreferenceUserUserPreferencesUpdateStore } from '$houdini/plugins/houdini-svelte/stores/PreferenceUserUserPreferencesUpdate'
import type { ProviderCatalogEffectiveModelTemplateStore } from '$houdini/plugins/houdini-svelte/stores/ProviderCatalogEffectiveModelTemplate'
import type { ProviderCatalogEffectiveProviderTemplateStore } from '$houdini/plugins/houdini-svelte/stores/ProviderCatalogEffectiveProviderTemplate'
import type { ProviderCatalogEffectiveProviderTemplatesStore } from '$houdini/plugins/houdini-svelte/stores/ProviderCatalogEffectiveProviderTemplates'
import type { ProviderCatalogModelTemplateStore } from '$houdini/plugins/houdini-svelte/stores/ProviderCatalogModelTemplate'
import type { ProviderModelTemplateCreateStore } from '$houdini/plugins/houdini-svelte/stores/ProviderModelTemplateCreate'
import type { ProviderModelTemplateCustomizeStore } from '$houdini/plugins/houdini-svelte/stores/ProviderModelTemplateCustomize'
import type { ProviderModelTemplateDeleteStore } from '$houdini/plugins/houdini-svelte/stores/ProviderModelTemplateDelete'
import type { ProviderModelTemplateResetStore } from '$houdini/plugins/houdini-svelte/stores/ProviderModelTemplateReset'
import type { ProviderProviderStore } from '$houdini/plugins/houdini-svelte/stores/ProviderProvider'
import type { ProviderProviderCreateStore } from '$houdini/plugins/houdini-svelte/stores/ProviderProviderCreate'
import type { ProviderProviderLogoUploadStore } from '$houdini/plugins/houdini-svelte/stores/ProviderProviderLogoUpload'
import type { ScaffoldCheckStore } from '$houdini/plugins/houdini-svelte/stores/ScaffoldCheck'
import type { SettingsDialogsGroupDeleteStore } from '$houdini/plugins/houdini-svelte/stores/SettingsDialogsGroupDelete'
import type { SettingsDialogsRoleDeleteStore } from '$houdini/plugins/houdini-svelte/stores/SettingsDialogsRoleDelete'
import type { SettingsGroupCreateStore } from '$houdini/plugins/houdini-svelte/stores/SettingsGroupCreate'
import type { SettingsGroupGroupAndRolesAndMembersStore } from '$houdini/plugins/houdini-svelte/stores/SettingsGroupGroupAndRolesAndMembers'
import type { SettingsGroupGroupUpdateStore } from '$houdini/plugins/houdini-svelte/stores/SettingsGroupGroupUpdate'
import type { SettingsGroupMembersStore } from '$houdini/plugins/houdini-svelte/stores/SettingsGroupMembers'
import type { SettingsMeAndOrganizationAndRolesAndGroupsStore } from '$houdini/plugins/houdini-svelte/stores/SettingsMeAndOrganizationAndRolesAndGroups'
import type { SettingsMembersGroupUpdateStore } from '$houdini/plugins/houdini-svelte/stores/SettingsMembersGroupUpdate'
import type { SettingsMembersInvitationCreateStore } from '$houdini/plugins/houdini-svelte/stores/SettingsMembersInvitationCreate'
import type { SettingsMembersInvitationCreateManyStore } from '$houdini/plugins/houdini-svelte/stores/SettingsMembersInvitationCreateMany'
import type { SettingsMembersInvitationRevokeStore } from '$houdini/plugins/houdini-svelte/stores/SettingsMembersInvitationRevoke'
import type { SettingsMembersInvitationsStore } from '$houdini/plugins/houdini-svelte/stores/SettingsMembersInvitations'
import type { SettingsMembersMeAndOrganizationAndMembersStore } from '$houdini/plugins/houdini-svelte/stores/SettingsMembersMeAndOrganizationAndMembers'
import type { SettingsMembersRoleAssignStore } from '$houdini/plugins/houdini-svelte/stores/SettingsMembersRoleAssign'
import type { SettingsOnboardingOrganizationOnboardingSettingsStore } from '$houdini/plugins/houdini-svelte/stores/SettingsOnboardingOrganizationOnboardingSettings'
import type { SettingsOnboardingOrganizationOnboardingSettingsUpdateStore } from '$houdini/plugins/houdini-svelte/stores/SettingsOnboardingOrganizationOnboardingSettingsUpdate'
import type { SettingsOrganizationGenerateOrgAvatarUrlStore } from '$houdini/plugins/houdini-svelte/stores/SettingsOrganizationGenerateOrgAvatarUrl'
import type { SettingsOrganizationOrganizationProfileStore } from '$houdini/plugins/houdini-svelte/stores/SettingsOrganizationOrganizationProfile'
import type { SettingsOrganizationOrganizationProfileUpdateStore } from '$houdini/plugins/houdini-svelte/stores/SettingsOrganizationOrganizationProfileUpdate'
import type { SettingsProfileGenerateUrlAvatarUrlStore } from '$houdini/plugins/houdini-svelte/stores/SettingsProfileGenerateUrlAvatarUrl'
import type { SettingsProfileUserProfileStore } from '$houdini/plugins/houdini-svelte/stores/SettingsProfileUserProfile'
import type { SettingsProfileUserProfileUpdateStore } from '$houdini/plugins/houdini-svelte/stores/SettingsProfileUserProfileUpdate'
import type { SettingsRoleCreateStore } from '$houdini/plugins/houdini-svelte/stores/SettingsRoleCreate'
import type { SettingsRoleOrganizationPolicyActionsStore } from '$houdini/plugins/houdini-svelte/stores/SettingsRoleOrganizationPolicyActions'
import type { SettingsRoleRoleStore } from '$houdini/plugins/houdini-svelte/stores/SettingsRoleRole'
import type { SettingsRoleRoleUpdateStore } from '$houdini/plugins/houdini-svelte/stores/SettingsRoleRoleUpdate'
import type { SettingsRoleRoleUpdate2Store } from '$houdini/plugins/houdini-svelte/stores/SettingsRoleRoleUpdate2'
import type { SettingsSessionRevokeOtherStore } from '$houdini/plugins/houdini-svelte/stores/SettingsSessionRevokeOther'
import type { SettingsSessionRevokeOthersStore } from '$houdini/plugins/houdini-svelte/stores/SettingsSessionRevokeOthers'
import type { SettingsSessionsStore } from '$houdini/plugins/houdini-svelte/stores/SettingsSessions'

import type { Cache as InternalCache } from 'houdini/runtime/cache'

import _cache from './cache.js'
import type { CacheTypeDef } from './generated.js'
import { Cache } from './public/index.js'

export { CachePolicy, PendingValue, isPending } from 'houdini/runtime'
export type { QueryArtifact, GraphQLVariables } from 'houdini/runtime'

export * from './client.js'
export { getCurrentConfig } from './config.js'

// this template tag gets removed by the preprocessor so it should never be invoked.
// this function must return any so that we can assign it a type in a variable declaration (ie an inline store)
// ideally we would be able to parse the input for values but typescript does not yet support that kind of matches in template args
export function graphql(str: `mutation AuthEmailSigninConfirm($emailSigninConfirmInput: EmailCodeInput!) {
  emailSigninConfirm(input: $emailSigninConfirmInput) {
    node {
      purpose
    }
  }
}
`): AuthEmailSigninConfirmStore;
export function graphql(str: `mutation AuthEmailSigninConfirm2($emailSigninConfirmInput: EmailCodeInput!) {
  emailSigninConfirm(input: $emailSigninConfirmInput) {
    node {
      accessLifetime {
        expiresInSeconds
      }
    }
  }
}
`): AuthEmailSigninConfirm2Store;
export function graphql(str: `mutation AuthEmailSigninRequest($emailSigninRequestInput: EmailInput!) {
  emailSigninRequest(input: $emailSigninRequestInput) {
    node {
      lifetime {
        expiresInSeconds
        throttleSeconds
      }
    }
  }
}
`): AuthEmailSigninRequestStore;
export function graphql(str: `mutation AuthEmailSignupConfirm($emailSignupConfirmInput: EmailCodeInput!) {
  emailSignupConfirm(input: $emailSignupConfirmInput) {
    node {
      accessLifetime {
        expiresInSeconds
      }
    }
  }
}
`): AuthEmailSignupConfirmStore;
export function graphql(str: `mutation AuthEmailSignupRequest($emailSignupRequestInput: EmailInput!) {
  emailSignupRequest(input: $emailSignupRequestInput) {
    node {
      lifetime {
        expiresInSeconds
        throttleSeconds
      }
    }
  }
}
`): AuthEmailSignupRequestStore;
export function graphql(str: `mutation AuthInvitationControllerEmailSigninConfirm($emailSigninConfirmInput: EmailCodeInput!) {
  emailSigninConfirm(input: $emailSigninConfirmInput) {
    node {
      accessLifetime {
        expiresInSeconds
      }
    }
  }
}
`): AuthInvitationControllerEmailSigninConfirmStore;
export function graphql(str: `mutation AuthInvitationControllerEmailSigninRequest($emailSigninRequestInput: EmailInput!) {
  emailSigninRequest(input: $emailSigninRequestInput) {
    node {
      lifetime {
        expiresInSeconds
      }
    }
  }
}
`): AuthInvitationControllerEmailSigninRequestStore;
export function graphql(str: `mutation AuthInvitationControllerEmailSignupConfirm($emailSignupConfirmInput: EmailCodeInput!) {
  emailSignupConfirm(input: $emailSignupConfirmInput) {
    node {
      accessLifetime {
        expiresInSeconds
      }
    }
  }
}
`): AuthInvitationControllerEmailSignupConfirmStore;
export function graphql(str: `mutation AuthInvitationControllerEmailSignupRequest($emailSignupRequestInput: EmailInput!) {
  emailSignupRequest(input: $emailSignupRequestInput) {
    node {
      lifetime {
        expiresInSeconds
      }
    }
  }
}
`): AuthInvitationControllerEmailSignupRequestStore;
export function graphql(str: `query AuthInvitationControllerInvitationContext {
  invitationContext {
    node {
      organizationName
      email
      expiresAt
      signup
    }
  }
}
`): AuthInvitationControllerInvitationContextStore;
export function graphql(str: `mutation AuthPasskeyAuthenticationFinish($passkeyAuthenticationFinishInput: PasskeyFinishInput!) {
  passkeyAuthenticationFinish(input: $passkeyAuthenticationFinishInput) {
    node {
      purpose
    }
  }
}
`): AuthPasskeyAuthenticationFinishStore;
export function graphql(str: `mutation AuthPasskeyAuthenticationFinish2($passkeyAuthenticationFinishInput: PasskeyFinishInput!) {
  passkeyAuthenticationFinish(input: $passkeyAuthenticationFinishInput) {
    node {
      accessLifetime {
        expiresInSeconds
      }
    }
  }
}
`): AuthPasskeyAuthenticationFinish2Store;
export function graphql(str: `mutation AuthPasskeyAuthenticationStart {
  passkeyAuthenticationStart(purpose: PASSKEY_CHANGE) {
    node {
      token
      expiresAt
      options {
        challenge
        rpId
        rpName
        userId
        userName
        userDisplayName
        algorithms
        credentials {
          id
          transports
        }
        timeout
        userVerification
        residentKey
        extensions {
          credProps
          credentialProtectionPolicy
          enforceCredentialProtectionPolicy
        }
      }
    }
  }
}
`): AuthPasskeyAuthenticationStartStore;
export function graphql(str: `mutation AuthPasskeyAuthenticationStart2 {
  passkeyAuthenticationStart {
    node {
      token
      expiresAt
      options {
        challenge
        rpId
        rpName
        userId
        userName
        userDisplayName
        algorithms
        credentials {
          id
          transports
        }
        timeout
        userVerification
        residentKey
        extensions {
          credProps
          credentialProtectionPolicy
          enforceCredentialProtectionPolicy
        }
      }
    }
  }
}
`): AuthPasskeyAuthenticationStart2Store;
export function graphql(str: `query AuthSession {
  session {
    node {
      passkey {
        credentialId
      }
    }
  }
}
`): AuthSessionStore;
export function graphql(str: `query AuthSession2 {
  session {
    node {
      id
      userId
      organizationId
      organizationName
      avatarUrl
      ipAddress
      userAgent
      signedInAt
      lastSeenAt
    }
  }
}
`): AuthSession2Store;
export function graphql(str: `mutation AuthSessionRevoke {
  sessionRevoke {
    node
  }
}
`): AuthSessionRevokeStore;
export function graphql(str: `query AuthUserProfile($userProfileId: ID!) {
  userProfile(id: $userProfileId) {
    node {
      email
    }
  }
}
`): AuthUserProfileStore;
export function graphql(str: `mutation OnboardingAnalyticUserProfilePut($analyticUserProfilePutInput: AnalyticUserProfilePutInput!) {
  analyticUserProfilePut(input: $analyticUserProfilePutInput) {
    node {
      value
    }
  }
}
`): OnboardingAnalyticUserProfilePutStore;
export function graphql(str: `mutation OnboardingGenerateUrlAvatarUrl($generateUrlAvatarUrlInput: ImageUploadInput!) {
  generateUrlAvatarUrl(input: $generateUrlAvatarUrlInput) {
    node {
      url
      fields {
        name
        value
      }
    }
  }
}
`): OnboardingGenerateUrlAvatarUrlStore;
export function graphql(str: `mutation OnboardingOnboardingAdvance($onboardingAdvanceBack: Boolean!) {
  onboardingAdvance(back: $onboardingAdvanceBack) {
    node {
      required
      hasBeenPresented
      flow
      step
      settings {
        welcome
        links {
          label
          href
        }
        destination
        artworkColor
        requireFullName
        requireAvatar
      }
    }
  }
}
`): OnboardingOnboardingAdvanceStore;
export function graphql(str: `mutation OnboardingOnboardingComplete {
  onboardingComplete {
    node {
      required
      hasBeenPresented
      flow
      step
      settings {
        welcome
        links {
          label
          href
        }
        destination
        artworkColor
        requireFullName
        requireAvatar
      }
    }
  }
}
`): OnboardingOnboardingCompleteStore;
export function graphql(str: `query OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile($userProfileId: ID!) {
  onboardingContext {
    node {
      required
      hasBeenPresented
      flow
      step
      settings {
        welcome
        links {
          label
          href
        }
        destination
        artworkColor
        requireFullName
        requireAvatar
      }
    }
  }
  userProfile(id: $userProfileId) {
    node {
      displayName
      username
      email
      avatarUrl
    }
  }
  analyticUserProfile(key: "work") {
    node {
      value
    }
  }
}
`): OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfileStore;
export function graphql(str: `mutation OnboardingOnboardingPresent($onboardingPresentManual: Boolean!) {
  onboardingPresent(manual: $onboardingPresentManual) {
    node {
      required
      hasBeenPresented
      flow
      step
      settings {
        welcome
        links {
          label
          href
        }
        destination
        artworkColor
        requireFullName
        requireAvatar
      }
    }
  }
}
`): OnboardingOnboardingPresentStore;
export function graphql(str: `mutation OnboardingOrganizationProfileUpdate($organizationProfileUpdateInput: OrganizationProfileUpdateInput!) {
  organizationProfileUpdate(input: $organizationProfileUpdateInput) {
    node {
      name
    }
  }
}
`): OnboardingOrganizationProfileUpdateStore;
export function graphql(str: `mutation OnboardingUserProfileUpdate($userProfileUpdateInput: UserProfileUpdateInput!) {
  userProfileUpdate(input: $userProfileUpdateInput) {
    node {
      displayName
      username
      email
      avatarUrl
    }
  }
}
`): OnboardingUserProfileUpdateStore;
export function graphql(str: `query OrganizationProviders($providersAfter: String) {
  providers(first: 100, after: $providersAfter) {
    edges {
      node {
        id
        name
      }
    }
    pageInfo {
      endCursor
      hasNextPage
    }
  }
}
`): OrganizationProvidersStore;
export function graphql(str: `mutation PasskeyPasskeyRegistrationFinish(
  $passkeyRegistrationFinishInput: PasskeyFinishInput!
  $passkeyRegistrationFinishName: PasskeyRegistrationInput!
) {
  passkeyRegistrationFinish(input: $passkeyRegistrationFinishInput, name: $passkeyRegistrationFinishName) {
    node {
      credentialId
      id
      name
      registeredAt
      lastUsedAt
    }
  }
}
`): PasskeyPasskeyRegistrationFinishStore;
export function graphql(str: `mutation PasskeyPasskeyRegistrationStart {
  passkeyRegistrationStart {
    node {
      token
      expiresAt
      options {
        challenge
        rpId
        rpName
        userId
        userName
        userDisplayName
        algorithms
        credentials {
          id
          transports
        }
        timeout
        userVerification
        residentKey
        extensions {
          credProps
          credentialProtectionPolicy
          enforceCredentialProtectionPolicy
        }
      }
    }
  }
}
`): PasskeyPasskeyRegistrationStartStore;
export function graphql(str: `mutation PasskeyPasskeyRemove($passkeyRemoveId: ID!) {
  passkeyRemove(id: $passkeyRemoveId) {
    node
  }
}
`): PasskeyPasskeyRemoveStore;
export function graphql(str: `mutation PasskeyPasskeyUpdate($passkeyUpdateId: ID!, $passkeyUpdateInput: PasskeyRegistrationInput!) {
  passkeyUpdate(id: $passkeyUpdateId, input: $passkeyUpdateInput) {
    node {
      credentialId
      id
      name
      registeredAt
      lastUsedAt
    }
  }
}
`): PasskeyPasskeyUpdateStore;
export function graphql(str: `query PasskeyPasskeys {
  passkeys {
    edges {
      node {
        credentialId
        id
        name
        registeredAt
        lastUsedAt
      }
    }
  }
}
`): PasskeyPasskeysStore;
export function graphql(str: `query PasskeyPasskeys2 {
  passkeys {
    edges {
      node {
        credentialId
        id
        name
        registeredAt
        lastUsedAt
      }
    }
  }
}
`): PasskeyPasskeys2Store;
export function graphql(str: `query PasskeyPasskeysAndSession {
  passkeys {
    edges {
      node {
        credentialId
        id
        name
        registeredAt
        lastUsedAt
      }
    }
  }
  session {
    node {
      passkey {
        id
      }
    }
  }
}
`): PasskeyPasskeysAndSessionStore;
export function graphql(str: `query PermissionMe {
  me {
    node {
      id
      organizationId
      cedarSource
      cedarEntities
    }
  }
}
`): PermissionMeStore;
export function graphql(str: `query PreferenceUserUserPreferences {
  userPreferences {
    node {
      navigation {
        home
      }
      memberDisplay {
        nameFormat
      }
      calendar {
        firstDay
      }
      theme {
        mode
        palettes {
          light
          dark
        }
      }
      typography {
        family
        size
      }
      accessibility {
        underlineLinks
        reduceMotion
        pointerCursors
      }
      messageComposition {
        convertEmoticons
        sendShortcut
      }
      codeAppearance {
        theme
        font {
          family
          size
        }
      }
      sidebar {
        badgeStyle
        entries {
          item
          visibility
        }
        layout {
          category
          items
        }
      }
    }
  }
}
`): PreferenceUserUserPreferencesStore;
export function graphql(str: `mutation PreferenceUserUserPreferencesUpdate(
  $userPreferencesUpdateInput: UserPreferencesPatchInput!
  $includeNavigation: Boolean! = false
  $includeMemberDisplay: Boolean! = false
  $includeCalendar: Boolean! = false
  $includeTheme: Boolean! = false
  $includeTypography: Boolean! = false
  $includeAccessibility: Boolean! = false
  $includeMessageComposition: Boolean! = false
  $includeCodeAppearance: Boolean! = false
  $includeSidebar: Boolean! = false
) {
  userPreferencesUpdate(input: $userPreferencesUpdateInput) {
    node {
      navigation @include(if: $includeNavigation) {
        home
      }
      memberDisplay @include(if: $includeMemberDisplay) {
        nameFormat
      }
      calendar @include(if: $includeCalendar) {
        firstDay
      }
      theme @include(if: $includeTheme) {
        mode
        palettes {
          light
          dark
        }
      }
      typography @include(if: $includeTypography) {
        family
        size
      }
      accessibility @include(if: $includeAccessibility) {
        underlineLinks
        reduceMotion
        pointerCursors
      }
      messageComposition @include(if: $includeMessageComposition) {
        convertEmoticons
        sendShortcut
      }
      codeAppearance @include(if: $includeCodeAppearance) {
        theme
        font {
          family
          size
        }
      }
      sidebar @include(if: $includeSidebar) {
        badgeStyle
        entries {
          item
          visibility
        }
        layout {
          category
          items
        }
      }
    }
  }
}
`): PreferenceUserUserPreferencesUpdateStore;
export function graphql(str: `query ProviderCatalogEffectiveModelTemplate($effectiveModelTemplateProviderTemplateId: ID!, $effectiveModelTemplateId: ID!) {
  effectiveModelTemplate(providerTemplateId: $effectiveModelTemplateProviderTemplateId, id: $effectiveModelTemplateId) {
    node {
      id
      providerTemplateId
      origin
      customFields
      name
      capabilities
      maxContextTokens
      inputPrice
      outputPrice
      cachedPrice
      pricingMode
      cacheBilling
      connections {
        protocol
        configuration {
          baseUrl
          path
          timeoutMs
          query {
            key
            value
          }
          authentication {
            method
            configuration {
              placement {
                location
                name
                prefix
              }
              flow
              authorizationUrl
              tokenUrl
              deviceAuthorizationUrl
              scopes
              requiredInputs {
                name
                kind
                required
                allowedValues
              }
              adapter
              parameters {
                key
                value
              }
            }
          }
          endpoint {
            mode
            configuration {
              urlTemplate
              requiredInputs {
                name
                kind
                required
                allowedValues
              }
            }
          }
        }
      }
      headers {
        key
        value
      }
      bodyDefaults {
        key
        value
      }
      text {
        input {
          configuration {
            maxTokens
          }
        }
        output {
          configuration {
            maxTokens
            thinking {
              configuration {
                effortMapping {
                  level
                  value
                }
              }
            }
          }
        }
      }
    }
  }
}
`): ProviderCatalogEffectiveModelTemplateStore;
export function graphql(str: `query ProviderCatalogEffectiveProviderTemplate($effectiveProviderTemplateId: ID!) {
  effectiveProviderTemplate(id: $effectiveProviderTemplateId) {
    node {
      id
      name
      baseUrl
      logoUrl
      description
      connection {
        baseUrl
        path
        timeoutMs
        query {
          key
          value
        }
        authentication {
          method
          configuration {
            placement {
              location
              name
              prefix
            }
            flow
            authorizationUrl
            tokenUrl
            deviceAuthorizationUrl
            scopes
            requiredInputs {
              name
              kind
              required
              allowedValues
            }
            adapter
            parameters {
              key
              value
            }
          }
        }
        endpoint {
          mode
          configuration {
            urlTemplate
            requiredInputs {
              name
              kind
              required
              allowedValues
            }
          }
        }
      }
      headers {
        key
        value
      }
      authentication {
        method
        configuration {
          placement {
            location
            name
            prefix
          }
          flow
          authorizationUrl
          tokenUrl
          deviceAuthorizationUrl
          scopes
          requiredInputs {
            name
            kind
            required
            allowedValues
          }
          adapter
          parameters {
            key
            value
          }
        }
      }
      models {
        edges {
          node {
            id
            name
            displayName
            description
            serviceKind
            capabilities
            maxContextTokens
            maxInputTokens
            maxOutputTokens
            inputPrice
            outputPrice
            cachedPrice
            pricingMode
            billingModel
            cacheBilling
            operations {
              operation
              path
            }
          }
        }
        pageInfo {
          hasNextPage
        }
      }
    }
  }
}
`): ProviderCatalogEffectiveProviderTemplateStore;
export function graphql(str: `query ProviderCatalogEffectiveProviderTemplates(
  $effectiveProviderTemplatesAfter: String
  $effectiveProviderTemplatesSearch: String
) {
  effectiveProviderTemplates(first: 20, after: $effectiveProviderTemplatesAfter, search: $effectiveProviderTemplatesSearch) {
    edges {
      node {
        id
        name
        baseUrl
        logoUrl
      }
    }
    pageInfo {
      endCursor
      hasNextPage
    }
  }
}
`): ProviderCatalogEffectiveProviderTemplatesStore;
export function graphql(str: `query ProviderCatalogModelTemplate($modelTemplateProviderTemplateId: ID!, $modelTemplateId: ID!) {
  modelTemplate(providerTemplateId: $modelTemplateProviderTemplateId, id: $modelTemplateId) {
    node {
      id
      providerTemplateId
      origin
      customFields
      name
      capabilities
      maxContextTokens
      inputPrice
      outputPrice
      cachedPrice
      pricingMode
      cacheBilling
      connections {
        protocol
        configuration {
          baseUrl
          path
          timeoutMs
          query {
            key
            value
          }
          authentication {
            method
            configuration {
              placement {
                location
                name
                prefix
              }
              flow
              authorizationUrl
              tokenUrl
              deviceAuthorizationUrl
              scopes
              requiredInputs {
                name
                kind
                required
                allowedValues
              }
              adapter
              parameters {
                key
                value
              }
            }
          }
          endpoint {
            mode
            configuration {
              urlTemplate
              requiredInputs {
                name
                kind
                required
                allowedValues
              }
            }
          }
        }
      }
      headers {
        key
        value
      }
      bodyDefaults {
        key
        value
      }
      text {
        input {
          configuration {
            maxTokens
          }
        }
        output {
          configuration {
            maxTokens
            thinking {
              configuration {
                effortMapping {
                  level
                  value
                }
              }
            }
          }
        }
      }
    }
  }
}
`): ProviderCatalogModelTemplateStore;
export function graphql(str: `mutation ProviderModelTemplateCreate(
  $modelTemplateCreateProviderTemplateId: ID!
  $modelTemplateCreateInput: ModelTemplateCreateInput!
) {
  modelTemplateCreate(providerTemplateId: $modelTemplateCreateProviderTemplateId, input: $modelTemplateCreateInput) {
    node {
      id
      providerTemplateId
      origin
      customFields
      name
      capabilities
      maxContextTokens
      inputPrice
      outputPrice
      cachedPrice
      pricingMode
      cacheBilling
      connections {
        protocol
        configuration {
          baseUrl
          path
          timeoutMs
          query {
            key
            value
          }
          authentication {
            method
            configuration {
              placement {
                location
                name
                prefix
              }
              flow
              authorizationUrl
              tokenUrl
              deviceAuthorizationUrl
              scopes
              requiredInputs {
                name
                kind
                required
                allowedValues
              }
              adapter
              parameters {
                key
                value
              }
            }
          }
          endpoint {
            mode
            configuration {
              urlTemplate
              requiredInputs {
                name
                kind
                required
                allowedValues
              }
            }
          }
        }
      }
      headers {
        key
        value
      }
      bodyDefaults {
        key
        value
      }
      text {
        input {
          configuration {
            maxTokens
          }
        }
        output {
          configuration {
            maxTokens
            thinking {
              configuration {
                effortMapping {
                  level
                  value
                }
              }
            }
          }
        }
      }
    }
  }
}
`): ProviderModelTemplateCreateStore;
export function graphql(str: `mutation ProviderModelTemplateCustomize(
  $modelTemplateCustomizeProviderTemplateId: ID!
  $modelTemplateCustomizeId: ID!
  $modelTemplateCustomizeInput: ModelTemplateCustomizeInput!
) {
  modelTemplateCustomize(
    providerTemplateId: $modelTemplateCustomizeProviderTemplateId
    id: $modelTemplateCustomizeId
    input: $modelTemplateCustomizeInput
  ) {
    node {
      id
      providerTemplateId
      origin
      customFields
      name
      capabilities
      maxContextTokens
      inputPrice
      outputPrice
      cachedPrice
      pricingMode
      cacheBilling
      connections {
        protocol
        configuration {
          baseUrl
          path
          timeoutMs
          query {
            key
            value
          }
          authentication {
            method
            configuration {
              placement {
                location
                name
                prefix
              }
              flow
              authorizationUrl
              tokenUrl
              deviceAuthorizationUrl
              scopes
              requiredInputs {
                name
                kind
                required
                allowedValues
              }
              adapter
              parameters {
                key
                value
              }
            }
          }
          endpoint {
            mode
            configuration {
              urlTemplate
              requiredInputs {
                name
                kind
                required
                allowedValues
              }
            }
          }
        }
      }
      headers {
        key
        value
      }
      bodyDefaults {
        key
        value
      }
      text {
        input {
          configuration {
            maxTokens
          }
        }
        output {
          configuration {
            maxTokens
            thinking {
              configuration {
                effortMapping {
                  level
                  value
                }
              }
            }
          }
        }
      }
    }
  }
}
`): ProviderModelTemplateCustomizeStore;
export function graphql(str: `mutation ProviderModelTemplateDelete($modelTemplateDeleteProviderTemplateId: ID!, $modelTemplateDeleteId: ID!) {
  modelTemplateDelete(providerTemplateId: $modelTemplateDeleteProviderTemplateId, id: $modelTemplateDeleteId) {
    node
  }
}
`): ProviderModelTemplateDeleteStore;
export function graphql(str: `mutation ProviderModelTemplateReset($modelTemplateResetProviderTemplateId: ID!, $modelTemplateResetId: ID!) {
  modelTemplateReset(providerTemplateId: $modelTemplateResetProviderTemplateId, id: $modelTemplateResetId) {
    node {
      id
      providerTemplateId
      origin
      customFields
      name
      capabilities
      maxContextTokens
      inputPrice
      outputPrice
      cachedPrice
      pricingMode
      cacheBilling
      connections {
        protocol
        configuration {
          baseUrl
          path
          timeoutMs
          query {
            key
            value
          }
          authentication {
            method
            configuration {
              placement {
                location
                name
                prefix
              }
              flow
              authorizationUrl
              tokenUrl
              deviceAuthorizationUrl
              scopes
              requiredInputs {
                name
                kind
                required
                allowedValues
              }
              adapter
              parameters {
                key
                value
              }
            }
          }
          endpoint {
            mode
            configuration {
              urlTemplate
              requiredInputs {
                name
                kind
                required
                allowedValues
              }
            }
          }
        }
      }
      headers {
        key
        value
      }
      bodyDefaults {
        key
        value
      }
      text {
        input {
          configuration {
            maxTokens
          }
        }
        output {
          configuration {
            maxTokens
            thinking {
              configuration {
                effortMapping {
                  level
                  value
                }
              }
            }
          }
        }
      }
    }
  }
}
`): ProviderModelTemplateResetStore;
export function graphql(str: `query ProviderProvider($providerId: ID!) {
  provider(id: $providerId) {
    node {
      id
      name
      description
      logoUrl
      connection {
        baseUrl
      }
      models {
        id
        name
        enabled
        capabilities
        inputPrice
        outputPrice
        cachedPrice
        protocol
        connection {
          path
        }
        contextTokens
        text {
          input {
            configuration {
              maxTokens
            }
          }
          output {
            configuration {
              maxTokens
            }
          }
        }
      }
    }
  }
}
`): ProviderProviderStore;
export function graphql(str: `mutation ProviderProviderCreate($providerCreateInput: ProviderCreateFromTemplateInput!) {
  providerCreate(input: $providerCreateInput) {
    node {
      id
      name
      logoUrl
    }
  }
}
`): ProviderProviderCreateStore;
export function graphql(str: `mutation ProviderProviderLogoUpload($providerLogoUploadInput: ImageUploadInput!) {
  providerLogoUpload(input: $providerLogoUploadInput) {
    node {
      id
      upload {
        url
        fields {
          name
          value
        }
      }
    }
  }
}
`): ProviderProviderLogoUploadStore;
export function graphql(str: `query ScaffoldCheck {
  __typename
}
`): ScaffoldCheckStore;
export function graphql(str: `mutation SettingsDialogsGroupDelete($groupDeleteId: ID!) {
  groupDelete(id: $groupDeleteId) {
    node
  }
}
`): SettingsDialogsGroupDeleteStore;
export function graphql(str: `mutation SettingsDialogsRoleDelete($roleDeleteId: ID!) {
  roleDelete(id: $roleDeleteId) {
    node
  }
}
`): SettingsDialogsRoleDeleteStore;
export function graphql(str: `mutation SettingsGroupCreate($groupCreateInput: GroupCreateInput!) {
  groupCreate(input: $groupCreateInput) {
    node {
      id
      name
    }
  }
}
`): SettingsGroupCreateStore;
export function graphql(str: `query SettingsGroupGroupAndRolesAndMembers(
  $groupId: ID!
  $includeGroupNode: Boolean! = false
  $includeGroupNodeMemberIds: Boolean! = false
) {
  group(id: $groupId) {
    node {
      id
      name
      description
      roles @include(if: $includeGroupNode) {
        edges {
          node {
            id
            name
            description
            color
            owner
          }
        }
      }
      memberIds @include(if: $includeGroupNodeMemberIds)
    }
  }
  roles @include(if: $includeGroupNode) {
    edges {
      node {
        id
        name
        description
        color
        owner
      }
    }
  }
  members(first: 30) @include(if: $includeGroupNodeMemberIds) {
    edges {
      node {
        id
        displayName
        email
        available
        effectiveRoleIds
        groupIds
      }
    }
    pageInfo {
      endCursor
      hasNextPage
    }
  }
}
`): SettingsGroupGroupAndRolesAndMembersStore;
export function graphql(str: `mutation SettingsGroupGroupUpdate(
  $groupUpdateId: ID!
  $groupUpdateInput: GroupUpdateInput!
  $includeGroupUpdateNodeMemberIds: Boolean! = false
  $includeGroupUpdateNode: Boolean! = false
) {
  groupUpdate(id: $groupUpdateId, input: $groupUpdateInput) {
    node {
      id
      name
      description
      memberIds @include(if: $includeGroupUpdateNodeMemberIds)
      roles @include(if: $includeGroupUpdateNode) {
        edges {
          node {
            id
            name
            description
            color
          }
        }
      }
    }
  }
}
`): SettingsGroupGroupUpdateStore;
export function graphql(str: `query SettingsGroupMembers($membersAfter: String, $membersFilter: MemberFilterInput) {
  members(first: 30, after: $membersAfter, filter: $membersFilter) {
    edges {
      node {
        id
        displayName
        email
        available
        effectiveRoleIds
        groupIds
      }
    }
    pageInfo {
      endCursor
      hasNextPage
    }
  }
}
`): SettingsGroupMembersStore;
export function graphql(str: `query SettingsMeAndOrganizationAndRolesAndGroups(
  $includeFields: Boolean! = false
  $includeFields2: Boolean! = false
  $includeFields3: Boolean! = false
) {
  me {
    node {
      id
      organizationOwner
    }
  }
  organization @include(if: $includeFields) {
    node {
      ownerMemberIds
    }
  }
  roles @include(if: $includeFields2) {
    edges {
      node {
        id
        name
        color
        owner
      }
    }
  }
  groups @include(if: $includeFields3) {
    edges {
      node {
        id
        name
        roles @include(if: $includeFields2) {
          edges {
            node {
              id
            }
          }
        }
      }
    }
  }
}
`): SettingsMeAndOrganizationAndRolesAndGroupsStore;
export function graphql(str: `mutation SettingsMembersGroupUpdate($groupUpdateId: ID!, $groupUpdateInput: GroupUpdateInput!) {
  groupUpdate(id: $groupUpdateId, input: $groupUpdateInput) {
    node {
      id
      memberIds
    }
  }
}
`): SettingsMembersGroupUpdateStore;
export function graphql(str: `mutation SettingsMembersInvitationCreate($invitationCreateInput: InvitationCreateInput!) {
  invitationCreate(input: $invitationCreateInput) {
    node {
      invitation {
        id
        email
        status
        expiresAt
        issuedAt
      }
      url
      emailAdmitted
    }
  }
}
`): SettingsMembersInvitationCreateStore;
export function graphql(str: `mutation SettingsMembersInvitationCreateMany($invitationCreateManyInput: InvitationBatchCreateInput!) {
  invitationCreateMany(input: $invitationCreateManyInput) {
    edges {
      node {
        invitation {
          id
          email
          status
          expiresAt
          issuedAt
        }
        url
        emailAdmitted
      }
    }
  }
}
`): SettingsMembersInvitationCreateManyStore;
export function graphql(str: `mutation SettingsMembersInvitationRevoke($invitationRevokeId: ID!) {
  invitationRevoke(id: $invitationRevokeId) {
    node {
      id
      email
      status
      expiresAt
      issuedAt
    }
  }
}
`): SettingsMembersInvitationRevokeStore;
export function graphql(str: `query SettingsMembersInvitations($invitationsAfter: String) {
  invitations(first: 20, after: $invitationsAfter) {
    edges {
      node {
        id
        email
        status
        expiresAt
        issuedAt
      }
    }
    pageInfo {
      endCursor
      hasNextPage
    }
  }
}
`): SettingsMembersInvitationsStore;
export function graphql(str: `query SettingsMembersMeAndOrganizationAndMembers(
  $membersAfter: String
  $membersFilter: MemberFilterInput
  $membersOrderBy: [MemberOrderInput!]
  $includeMembersEdgesNode: Boolean! = false
  $includeMembersEdgesNode2: Boolean! = false
  $includeMembersEdgesNode3: Boolean! = false
) {
  me {
    node {
      id
      organizationOwner
    }
  }
  organization {
    node {
      ownerMemberIds
    }
  }
  members(first: 20, after: $membersAfter, filter: $membersFilter, currentUserFirst: true, orderBy: $membersOrderBy) {
    edges {
      node {
        id
        displayName
        email
        available
        joinedAt
        role @include(if: $includeMembersEdgesNode) {
          id
          name
          description
          color
          owner
        }
        groups @include(if: $includeMembersEdgesNode2) {
          id
          name
          description
        }
        effectiveRoleIds @include(if: $includeMembersEdgesNode3)
      }
    }
    pageInfo {
      endCursor
      hasNextPage
    }
  }
}
`): SettingsMembersMeAndOrganizationAndMembersStore;
export function graphql(str: `mutation SettingsMembersRoleAssign($roleAssignId: ID!, $roleAssignMemberId: ID!) {
  roleAssign(id: $roleAssignId, memberId: $roleAssignMemberId) {
    node {
      id
      name
      description
      color
      owner
    }
  }
}
`): SettingsMembersRoleAssignStore;
export function graphql(str: `query SettingsOnboardingOrganizationOnboardingSettings {
  organizationOnboardingSettings {
    node {
      welcome
      links {
        label
        href
      }
      destination
      artworkColor
      requireFullName
      requireAvatar
    }
  }
}
`): SettingsOnboardingOrganizationOnboardingSettingsStore;
export function graphql(str: `mutation SettingsOnboardingOrganizationOnboardingSettingsUpdate(
  $organizationOnboardingSettingsUpdateInput: OrganizationOnboardingSettingsInput!
) {
  organizationOnboardingSettingsUpdate(input: $organizationOnboardingSettingsUpdateInput) {
    node {
      welcome
      links {
        label
        href
      }
      destination
      artworkColor
      requireFullName
      requireAvatar
    }
  }
}
`): SettingsOnboardingOrganizationOnboardingSettingsUpdateStore;
export function graphql(str: `mutation SettingsOrganizationGenerateOrgAvatarUrl($generateOrgAvatarUrlInput: ImageUploadInput!) {
  generateOrgAvatarUrl(input: $generateOrgAvatarUrlInput) {
    node {
      url
      fields {
        name
        value
      }
      expiresAt
    }
  }
}
`): SettingsOrganizationGenerateOrgAvatarUrlStore;
export function graphql(str: `query SettingsOrganizationOrganizationProfile($organizationProfileId: ID!) {
  organizationProfile(id: $organizationProfileId) {
    node {
      id
      name
      avatarUrl
      canUpdateProfile
    }
  }
}
`): SettingsOrganizationOrganizationProfileStore;
export function graphql(str: `mutation SettingsOrganizationOrganizationProfileUpdate($organizationProfileUpdateInput: OrganizationProfileUpdateInput!) {
  organizationProfileUpdate(input: $organizationProfileUpdateInput) {
    node {
      id
      name
      avatarUrl
      canUpdateProfile
    }
  }
}
`): SettingsOrganizationOrganizationProfileUpdateStore;
export function graphql(str: `mutation SettingsProfileGenerateUrlAvatarUrl($generateUrlAvatarUrlInput: ImageUploadInput!) {
  generateUrlAvatarUrl(input: $generateUrlAvatarUrlInput) {
    node {
      url
      fields {
        name
        value
      }
      expiresAt
    }
  }
}
`): SettingsProfileGenerateUrlAvatarUrlStore;
export function graphql(str: `query SettingsProfileUserProfile($userProfileId: ID!) {
  userProfile(id: $userProfileId) {
    node {
      id
      displayName
      username
      email
      avatarUrl
    }
  }
}
`): SettingsProfileUserProfileStore;
export function graphql(str: `mutation SettingsProfileUserProfileUpdate($userProfileUpdateInput: UserProfileUpdateInput!) {
  userProfileUpdate(input: $userProfileUpdateInput) {
    node {
      id
      displayName
      username
      email
      avatarUrl
    }
  }
}
`): SettingsProfileUserProfileUpdateStore;
export function graphql(str: `mutation SettingsRoleCreate($roleCreateInput: RoleCreateInput!) {
  roleCreate(input: $roleCreateInput) {
    node {
      id
      name
      description
      color
    }
  }
}
`): SettingsRoleCreateStore;
export function graphql(str: `query SettingsRoleOrganizationPolicyActions {
  organizationPolicyActions {
    edges {
      node {
        name
        displayName
        description
        entityType
        resourceType
      }
    }
  }
}
`): SettingsRoleOrganizationPolicyActionsStore;
export function graphql(str: `query SettingsRoleRole($roleId: ID!, $includeRoleNodeCedarSource: Boolean! = false) {
  role(id: $roleId) {
    node {
      id
      name
      description
      color
      cedarSource @include(if: $includeRoleNodeCedarSource)
      owner
    }
  }
}
`): SettingsRoleRoleStore;
export function graphql(str: `mutation SettingsRoleRoleUpdate($roleUpdateId: ID!, $roleUpdateInput: RoleUpdateInput!) {
  roleUpdate(id: $roleUpdateId, input: $roleUpdateInput) {
    node {
      id
      name
      description
      color
    }
  }
}
`): SettingsRoleRoleUpdateStore;
export function graphql(str: `mutation SettingsRoleRoleUpdate2($roleUpdateId: ID!, $roleUpdateInput: RoleUpdateInput!) {
  roleUpdate(id: $roleUpdateId, input: $roleUpdateInput) {
    node {
      id
    }
  }
}
`): SettingsRoleRoleUpdate2Store;
export function graphql(str: `mutation SettingsSessionRevokeOther($sessionRevokeOtherId: ID!) {
  sessionRevokeOther(id: $sessionRevokeOtherId) {
    node
  }
}
`): SettingsSessionRevokeOtherStore;
export function graphql(str: `mutation SettingsSessionRevokeOthers {
  sessionRevokeOthers {
    node
  }
}
`): SettingsSessionRevokeOthersStore;
export function graphql(str: `query SettingsSessions($sessionsAfter: String) {
  sessions(first: 20, after: $sessionsAfter) {
    edges {
      node {
        id
        ipAddress
        userAgent
        signedInAt
        lastSeenAt
      }
    }
    pageInfo {
      endCursor
      hasNextPage
    }
  }
}
`): SettingsSessionsStore;
export function graphql<_Payload, _Result = _Payload>(str: string): _Result
export function graphql(str: string): never {
	// if we are executing this function as part of the plugin, we need to return
	// the query instead of throwing an error. We don't want to bundle the graphql
	// module into the runtime so all we can do is return the query string
	if (globalThis?.process?.env?.HOUDINI_PLUGIN) {
		// @ts-expect-error: this is a totally internal/hidden value. user will never see it and we won't
		//             and ever get a typed value of this since it's only used in the result of a dynamic
		//             import from the plugin which gives Record<string, any>
		return str
	}

	// if this is executed, the preprocessor is not enabled
	throw new Error(`! graphql template was invoked at runtime. This should never happen and usually means that your project isn't properly configured.

Please make sure you have the appropriate plugin/preprocessor enabled. For more information, visit this link: https://houdinigraphql.com/guides/setting-up-your-project
`)
}

export const cache = new Cache<CacheTypeDef>(_cache)

export function getCache(): InternalCache {
	return _cache
}
