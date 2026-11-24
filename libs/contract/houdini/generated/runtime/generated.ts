// @ts-nocheck
import type { Record } from "./public/record";
import type { AuthInvitationControllerInvitationContext$result, AuthInvitationControllerInvitationContext$input, AuthInvitationControllerInvitationContext$artifact } from "../artifacts/AuthInvitationControllerInvitationContext";
import type { AuthSession$result, AuthSession$input, AuthSession$artifact } from "../artifacts/AuthSession";
import type { AuthSession2$result, AuthSession2$input, AuthSession2$artifact } from "../artifacts/AuthSession2";
import type { AuthUserProfile$result, AuthUserProfile$input, AuthUserProfile$artifact } from "../artifacts/AuthUserProfile";
import type { OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile$result, OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile$input, OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile$artifact } from "../artifacts/OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile";
import type { OrganizationProviders$result, OrganizationProviders$input, OrganizationProviders$artifact } from "../artifacts/OrganizationProviders";
import type { PasskeyPasskeys$result, PasskeyPasskeys$input, PasskeyPasskeys$artifact } from "../artifacts/PasskeyPasskeys";
import type { PasskeyPasskeys2$result, PasskeyPasskeys2$input, PasskeyPasskeys2$artifact } from "../artifacts/PasskeyPasskeys2";
import type { PasskeyPasskeysAndSession$result, PasskeyPasskeysAndSession$input, PasskeyPasskeysAndSession$artifact } from "../artifacts/PasskeyPasskeysAndSession";
import type { PermissionMe$result, PermissionMe$input, PermissionMe$artifact } from "../artifacts/PermissionMe";
import type { PortalAuthority$result, PortalAuthority$input, PortalAuthority$artifact } from "../artifacts/PortalAuthority";
import type { PreferenceUserUserPreferences$result, PreferenceUserUserPreferences$input, PreferenceUserUserPreferences$artifact } from "../artifacts/PreferenceUserUserPreferences";
import type { ProviderCatalogEffectiveModelTemplate$result, ProviderCatalogEffectiveModelTemplate$input, ProviderCatalogEffectiveModelTemplate$artifact } from "../artifacts/ProviderCatalogEffectiveModelTemplate";
import type { ProviderCatalogEffectiveProviderTemplate$result, ProviderCatalogEffectiveProviderTemplate$input, ProviderCatalogEffectiveProviderTemplate$artifact } from "../artifacts/ProviderCatalogEffectiveProviderTemplate";
import type { ProviderCatalogEffectiveProviderTemplates$result, ProviderCatalogEffectiveProviderTemplates$input, ProviderCatalogEffectiveProviderTemplates$artifact } from "../artifacts/ProviderCatalogEffectiveProviderTemplates";
import type { ProviderCatalogModelTemplate$result, ProviderCatalogModelTemplate$input, ProviderCatalogModelTemplate$artifact } from "../artifacts/ProviderCatalogModelTemplate";
import type { ProviderProvider$result, ProviderProvider$input, ProviderProvider$artifact } from "../artifacts/ProviderProvider";
import type { SettingsGroupGroupAndRolesAndMembers$result, SettingsGroupGroupAndRolesAndMembers$input, SettingsGroupGroupAndRolesAndMembers$artifact } from "../artifacts/SettingsGroupGroupAndRolesAndMembers";
import type { SettingsGroupMembers$result, SettingsGroupMembers$input, SettingsGroupMembers$artifact } from "../artifacts/SettingsGroupMembers";
import type { SettingsMeAndOrganizationAndRolesAndGroups$result, SettingsMeAndOrganizationAndRolesAndGroups$input, SettingsMeAndOrganizationAndRolesAndGroups$artifact } from "../artifacts/SettingsMeAndOrganizationAndRolesAndGroups";
import type { SettingsMembersInvitations$result, SettingsMembersInvitations$input, SettingsMembersInvitations$artifact } from "../artifacts/SettingsMembersInvitations";
import type { SettingsMembersMeAndOrganizationAndMembers$result, SettingsMembersMeAndOrganizationAndMembers$input, SettingsMembersMeAndOrganizationAndMembers$artifact } from "../artifacts/SettingsMembersMeAndOrganizationAndMembers";
import type { SettingsOnboardingOrganizationOnboardingSettings$result, SettingsOnboardingOrganizationOnboardingSettings$input, SettingsOnboardingOrganizationOnboardingSettings$artifact } from "../artifacts/SettingsOnboardingOrganizationOnboardingSettings";
import type { SettingsOrganizationOrganizationProfile$result, SettingsOrganizationOrganizationProfile$input, SettingsOrganizationOrganizationProfile$artifact } from "../artifacts/SettingsOrganizationOrganizationProfile";
import type { SettingsProfileUserProfile$result, SettingsProfileUserProfile$input, SettingsProfileUserProfile$artifact } from "../artifacts/SettingsProfileUserProfile";
import type { SettingsRoleOrganizationPolicyActions$result, SettingsRoleOrganizationPolicyActions$input, SettingsRoleOrganizationPolicyActions$artifact } from "../artifacts/SettingsRoleOrganizationPolicyActions";
import type { SettingsRoleRole$result, SettingsRoleRole$input, SettingsRoleRole$artifact } from "../artifacts/SettingsRoleRole";
import type { SettingsSessions$result, SettingsSessions$input, SettingsSessions$artifact } from "../artifacts/SettingsSessions";
import type { AuthenticationMethod$options } from "$houdini/graphql/enums";
import type { AuthenticationPurpose$options } from "$houdini/graphql/enums";
import type { BillingModel$options } from "$houdini/graphql/enums";
import type { CacheBilling$options } from "$houdini/graphql/enums";
import type { CapabilityCacheMode$options } from "$houdini/graphql/enums";
import type { CapabilityPriceMode$options } from "$houdini/graphql/enums";
import type { CapabilityPricingKind$options } from "$houdini/graphql/enums";
import type { CapabilityStatus$options } from "$houdini/graphql/enums";
import type { ConfigurationInputKind$options } from "$houdini/graphql/enums";
import type { CredentialDelivery$options } from "$houdini/graphql/enums";
import type { CredentialLocation$options } from "$houdini/graphql/enums";
import type { CreditUsageDirection$options } from "$houdini/graphql/enums";
import type { EndpointMode$options } from "$houdini/graphql/enums";
import type { FilterIs$options } from "$houdini/graphql/enums";
import type { HomeDestination$options } from "$houdini/graphql/enums";
import type { ImageMediaType$options } from "$houdini/graphql/enums";
import type { ModelBillingKind$options } from "$houdini/graphql/enums";
import type { ModelServiceKind$options } from "$houdini/graphql/enums";
import type { ModelTemplateOrigin$options } from "$houdini/graphql/enums";
import type { OAuthFlow$options } from "$houdini/graphql/enums";
import type { OnboardingDestination$options } from "$houdini/graphql/enums";
import type { OnboardingFlow$options } from "$houdini/graphql/enums";
import type { OnboardingStep$options } from "$houdini/graphql/enums";
import type { OrderByDirection$options } from "$houdini/graphql/enums";
import type { PreferenceAppearanceMode$options } from "$houdini/graphql/enums";
import type { PreferenceBadgeStyle$options } from "$houdini/graphql/enums";
import type { PreferenceCodeTheme$options } from "$houdini/graphql/enums";
import type { PreferenceDarkPalette$options } from "$houdini/graphql/enums";
import type { PreferenceDestination$options } from "$houdini/graphql/enums";
import type { PreferenceFontFamily$options } from "$houdini/graphql/enums";
import type { PreferenceKeyCombination$options } from "$houdini/graphql/enums";
import type { PreferenceLightPalette$options } from "$houdini/graphql/enums";
import type { PreferenceNamingFormat$options } from "$houdini/graphql/enums";
import type { PreferencePixelFontSize$options } from "$houdini/graphql/enums";
import type { PreferenceRelativeFontSize$options } from "$houdini/graphql/enums";
import type { PreferenceSidebarCategory$options } from "$houdini/graphql/enums";
import type { PreferenceSidebarVisibility$options } from "$houdini/graphql/enums";
import type { PreferenceWeekday$options } from "$houdini/graphql/enums";
import type { PriceBillingPeriod$options } from "$houdini/graphql/enums";
import type { PriceTierBasis$options } from "$houdini/graphql/enums";
import type { PriceUnit$options } from "$houdini/graphql/enums";
import type { PricingMode$options } from "$houdini/graphql/enums";
import type { ProviderApiProtocol$options } from "$houdini/graphql/enums";
import type { ProviderModelCapability$options } from "$houdini/graphql/enums";
import type { TextModelProtocol$options } from "$houdini/graphql/enums";
import type { ThinkingEffortLevel$options } from "$houdini/graphql/enums";
import type { UpstreamProtocol$options } from "$houdini/graphql/enums";
import type { UserPreferenceGroup$options } from "$houdini/graphql/enums";
import type { AccessibilityPatchInput } from "$houdini/graphql/inputs";
import type { AnalyticUserProfilePutInput } from "$houdini/graphql/inputs";
import type { CalendarPatchInput } from "$houdini/graphql/inputs";
import type { ChatCreateInput } from "$houdini/graphql/inputs";
import type { CodeAppearancePatchInput } from "$houdini/graphql/inputs";
import type { CodeFontPatchInput } from "$houdini/graphql/inputs";
import type { CredentialCreateInput } from "$houdini/graphql/inputs";
import type { EmailCodeInput } from "$houdini/graphql/inputs";
import type { EmailInput } from "$houdini/graphql/inputs";
import type { GroupCreateInput } from "$houdini/graphql/inputs";
import type { GroupUpdateInput } from "$houdini/graphql/inputs";
import type { IDFilter } from "$houdini/graphql/inputs";
import type { ImageUploadInput } from "$houdini/graphql/inputs";
import type { InvitationBatchCreateInput } from "$houdini/graphql/inputs";
import type { InvitationCreateInput } from "$houdini/graphql/inputs";
import type { MemberDisplayPatchInput } from "$houdini/graphql/inputs";
import type { MemberFilterInput } from "$houdini/graphql/inputs";
import type { MemberMatchInput } from "$houdini/graphql/inputs";
import type { MemberOrderInput } from "$houdini/graphql/inputs";
import type { MessageCompositionPatchInput } from "$houdini/graphql/inputs";
import type { ModelTemplateCreateInput } from "$houdini/graphql/inputs";
import type { ModelTemplateCustomizeInput } from "$houdini/graphql/inputs";
import type { ModelTemplateEffectiveLimitsInput } from "$houdini/graphql/inputs";
import type { ModelTemplateFilterInput } from "$houdini/graphql/inputs";
import type { ModelTemplateLimitsInput } from "$houdini/graphql/inputs";
import type { ModelTemplateMatchInput } from "$houdini/graphql/inputs";
import type { ModelTemplatePriceInput } from "$houdini/graphql/inputs";
import type { NavigationPatchInput } from "$houdini/graphql/inputs";
import type { OnboardingLinkInput } from "$houdini/graphql/inputs";
import type { OrganizationOnboardingSettingsInput } from "$houdini/graphql/inputs";
import type { OrganizationProfileUpdateInput } from "$houdini/graphql/inputs";
import type { OrganizationSettingsUpdateInput } from "$houdini/graphql/inputs";
import type { PalettePatchInput } from "$houdini/graphql/inputs";
import type { PasskeyFinishInput } from "$houdini/graphql/inputs";
import type { PasskeyRegistrationInput } from "$houdini/graphql/inputs";
import type { ProfileUpdateInput } from "$houdini/graphql/inputs";
import type { ProviderBodyDefault } from "$houdini/graphql/inputs";
import type { ProviderCatalogRef } from "$houdini/graphql/inputs";
import type { ProviderCreateFromTemplateInput } from "$houdini/graphql/inputs";
import type { ProviderCreateInput } from "$houdini/graphql/inputs";
import type { ProviderEffortMapping } from "$houdini/graphql/inputs";
import type { ProviderHeader } from "$houdini/graphql/inputs";
import type { ProviderHeaderUpdate } from "$houdini/graphql/inputs";
import type { ProviderModelConfiguration } from "$houdini/graphql/inputs";
import type { ProviderModelWrite } from "$houdini/graphql/inputs";
import type { ProviderPatchInput } from "$houdini/graphql/inputs";
import type { ProviderReplaceInput } from "$houdini/graphql/inputs";
import type { ProviderTemplateFilterInput } from "$houdini/graphql/inputs";
import type { ProviderTemplateMatchInput } from "$houdini/graphql/inputs";
import type { ProviderTemplateModelInput } from "$houdini/graphql/inputs";
import type { ProviderTemplateOrderInput } from "$houdini/graphql/inputs";
import type { ProvidersCreateInput } from "$houdini/graphql/inputs";
import type { ReportingScheduleInput } from "$houdini/graphql/inputs";
import type { RoleCreateInput } from "$houdini/graphql/inputs";
import type { RoleUpdateInput } from "$houdini/graphql/inputs";
import type { SidebarLayoutSectionInput } from "$houdini/graphql/inputs";
import type { SidebarPatchInput } from "$houdini/graphql/inputs";
import type { SidebarVisibilityPatchInput } from "$houdini/graphql/inputs";
import type { SpendingLimitInput } from "$houdini/graphql/inputs";
import type { StringFilter } from "$houdini/graphql/inputs";
import type { ThemePatchInput } from "$houdini/graphql/inputs";
import type { TypographyPatchInput } from "$houdini/graphql/inputs";
import type { UserPreferencesPatchInput } from "$houdini/graphql/inputs";
import type { UserProfileUpdateInput } from "$houdini/graphql/inputs";

export declare type CacheTypeDef = {
		types: {
			__ROOT__: {
				idFields: {};
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					analyticUserProfile: {
						type: Record<CacheTypeDef, "AnalyticUserProfilePayload"> | null;
						args: {
							key: string;
						};
					};
					effectiveModelTemplate: {
						type: Record<CacheTypeDef, "ModelTemplatePayload"> | null;
						args: {
							id: string | number;
							providerTemplateId: string | number;
						};
					};
					effectiveProviderTemplate: {
						type: Record<CacheTypeDef, "ProviderTemplatePayload"> | null;
						args: {
							id: string | number;
						};
					};
					effectiveProviderTemplates: {
						type: Record<CacheTypeDef, "ProviderTemplateConnection">;
						args: {
							after?: string | null | undefined;
							first?: number | null | undefined;
							search?: string | null | undefined;
						};
					};
					group: {
						type: Record<CacheTypeDef, "GroupPayload"> | null;
						args: {
							id: string | number;
						};
					};
					groups: {
						type: Record<CacheTypeDef, "GroupConnection">;
						args: never;
					};
					invitationContext: {
						type: Record<CacheTypeDef, "InvitationContextPayload">;
						args: never;
					};
					invitations: {
						type: Record<CacheTypeDef, "InvitationConnection">;
						args: {
							after?: string | null | undefined;
							first?: number | null | undefined;
						};
					};
					me: {
						type: Record<CacheTypeDef, "MePayload">;
						args: never;
					};
					members: {
						type: Record<CacheTypeDef, "MemberConnection">;
						args: {
							after?: string | null | undefined;
							before?: string | null | undefined;
							currentUserFirst?: boolean | null | undefined;
							filter?: MemberFilterInput | null | undefined;
							first?: number | null | undefined;
							last?: number | null | undefined;
							offset?: number | null | undefined;
							orderBy?: (MemberOrderInput)[] | null | undefined;
						};
					};
					modelTemplate: {
						type: Record<CacheTypeDef, "ModelTemplatePayload"> | null;
						args: {
							id: string | number;
							providerTemplateId: string | number;
						};
					};
					onboardingContext: {
						type: Record<CacheTypeDef, "OnboardingContextPayload"> | null;
						args: never;
					};
					organization: {
						type: Record<CacheTypeDef, "OrganizationPayload">;
						args: never;
					};
					organizationOnboardingSettings: {
						type: Record<CacheTypeDef, "OrganizationOnboardingSettingsPayload">;
						args: never;
					};
					organizationPolicyActions: {
						type: Record<CacheTypeDef, "PolicyActionConnection">;
						args: never;
					};
					organizationProfile: {
						type: Record<CacheTypeDef, "OrganizationPayload"> | null;
						args: {
							id: string | number;
						};
					};
					passkeys: {
						type: Record<CacheTypeDef, "PasskeySummaryConnection">;
						args: never;
					};
					provider: {
						type: Record<CacheTypeDef, "ProviderPayload"> | null;
						args: {
							id: string | number;
						};
					};
					providerTemplate: {
						type: Record<CacheTypeDef, "ProviderTemplatePayload"> | null;
						args: {
							id: string | number;
						};
					};
					providerTemplates: {
						type: Record<CacheTypeDef, "ProviderTemplateConnection">;
						args: {
							after?: string | null | undefined;
							before?: string | null | undefined;
							filter?: ProviderTemplateFilterInput | null | undefined;
							first?: number | null | undefined;
							last?: number | null | undefined;
							offset?: number | null | undefined;
							orderBy?: (ProviderTemplateOrderInput)[] | null | undefined;
							search?: string | null | undefined;
						};
					};
					providers: {
						type: Record<CacheTypeDef, "ProviderConnection">;
						args: {
							after?: string | null | undefined;
							first?: number | null | undefined;
						};
					};
					role: {
						type: Record<CacheTypeDef, "RolePayload"> | null;
						args: {
							id: string | number;
							includeUnseen: boolean;
						};
					};
					roles: {
						type: Record<CacheTypeDef, "RoleConnection">;
						args: {
							includeUnseen: boolean;
						};
					};
					session: {
						type: Record<CacheTypeDef, "SessionPayload">;
						args: never;
					};
					sessions: {
						type: Record<CacheTypeDef, "SessionConnection">;
						args: {
							after?: string | null | undefined;
							first?: number | null | undefined;
						};
					};
					userPreferences: {
						type: Record<CacheTypeDef, "UserPreferencesPayload">;
						args: never;
					};
					userProfile: {
						type: Record<CacheTypeDef, "UserPayload"> | null;
						args: {
							id: string | number;
						};
					};
				};
				fragments: [];
			};
			AccessibilityPreferences: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					pointerCursors: {
						type: boolean;
						args: never;
					};
					reduceMotion: {
						type: boolean;
						args: never;
					};
					underlineLinks: {
						type: boolean;
						args: never;
					};
				};
				fragments: [];
			};
			AnalyticUserProfile: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					key: {
						type: string;
						args: never;
					};
					value: {
						type: string;
						args: never;
					};
				};
				fragments: [];
			};
			AnalyticUserProfilePayload: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "AnalyticUserProfile">;
						args: never;
					};
				};
				fragments: [];
			};
			AudioCapabilities: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					input: {
						type: Record<CacheTypeDef, "AudioSupport">;
						args: never;
					};
					output: {
						type: Record<CacheTypeDef, "AudioSupport">;
						args: never;
					};
				};
				fragments: [];
			};
			AudioConfiguration: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					formats: {
						type: (string)[] | null;
						args: never;
					};
					maxBytes: {
						type: number | null;
						args: never;
					};
					maxDurationMs: {
						type: number | null;
						args: never;
					};
				};
				fragments: [];
			};
			AudioSupport: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					configuration: {
						type: Record<CacheTypeDef, "AudioConfiguration"> | null;
						args: never;
					};
					pricing: {
						type: Record<CacheTypeDef, "CapabilityPricing"> | null;
						args: never;
					};
					status: {
						type: CapabilityStatus$options;
						args: never;
					};
				};
				fragments: [];
			};
			AuthenticationConfiguration: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					configuration: {
						type: Record<CacheTypeDef, "AuthenticationParameters">;
						args: never;
					};
					method: {
						type: AuthenticationMethod$options;
						args: never;
					};
				};
				fragments: [];
			};
			AuthenticationParameters: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					adapter: {
						type: string | null;
						args: never;
					};
					authorizationUrl: {
						type: string | null;
						args: never;
					};
					deviceAuthorizationUrl: {
						type: string | null;
						args: never;
					};
					flow: {
						type: OAuthFlow$options | null;
						args: never;
					};
					parameters: {
						type: (Record<CacheTypeDef, "ConfigurationEntry">)[];
						args: never;
					};
					placement: {
						type: Record<CacheTypeDef, "CredentialPlacement"> | null;
						args: never;
					};
					requiredInputs: {
						type: (Record<CacheTypeDef, "ConfigurationInput">)[];
						args: never;
					};
					scopes: {
						type: (string)[];
						args: never;
					};
					tokenUrl: {
						type: string | null;
						args: never;
					};
				};
				fragments: [];
			};
			AvatarUpload: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					expiresAt: {
						type: string;
						args: never;
					};
					fields: {
						type: (Record<CacheTypeDef, "UploadFormField">)[];
						args: never;
					};
					url: {
						type: string;
						args: never;
					};
				};
				fragments: [];
			};
			AvatarUploadPayload: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "AvatarUpload">;
						args: never;
					};
				};
				fragments: [];
			};
			BillingAllowance: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					quota: {
						type: string | null;
						args: never;
					};
					resetPeriod: {
						type: PriceBillingPeriod$options | null;
						args: never;
					};
					unit: {
						type: PriceUnit$options;
						args: never;
					};
				};
				fragments: [];
			};
			BillingFee: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					amount: {
						type: string | null;
						args: never;
					};
					currency: {
						type: string | null;
						args: never;
					};
					period: {
						type: PriceBillingPeriod$options | null;
						args: never;
					};
				};
				fragments: [];
			};
			BodyDefaultEntry: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					key: {
						type: string;
						args: never;
					};
					value: {
						type: string;
						args: never;
					};
				};
				fragments: [];
			};
			BooleanPayload: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					node: {
						type: boolean;
						args: never;
					};
				};
				fragments: [];
			};
			CalendarPreferences: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					firstDay: {
						type: PreferenceWeekday$options;
						args: never;
					};
				};
				fragments: [];
			};
			CapabilityCachePrice: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					amount: {
						type: string | null;
						args: never;
					};
					currency: {
						type: string | null;
						args: never;
					};
					mode: {
						type: CapabilityCacheMode$options;
						args: never;
					};
					quantity: {
						type: string | null;
						args: never;
					};
				};
				fragments: [];
			};
			CapabilityPricing: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					kind: {
						type: CapabilityPricingKind$options;
						args: never;
					};
					usage: {
						type: Record<CacheTypeDef, "CapabilityUsage"> | null;
						args: never;
					};
				};
				fragments: [];
			};
			CapabilityRate: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					amount: {
						type: string | null;
						args: never;
					};
					cache: {
						type: Record<CacheTypeDef, "CapabilityCachePrice"> | null;
						args: never;
					};
					conditions: {
						type: (Record<CacheTypeDef, "ConfigurationEntry">)[];
						args: never;
					};
					credits: {
						type: string | null;
						args: never;
					};
					currency: {
						type: string | null;
						args: never;
					};
					direction: {
						type: CreditUsageDirection$options | null;
						args: never;
					};
					quantity: {
						type: string | null;
						args: never;
					};
					unit: {
						type: PriceUnit$options;
						args: never;
					};
				};
				fragments: [];
			};
			CapabilitySlot: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					fromHour: {
						type: number;
						args: never;
					};
					pricing: {
						type: Record<CacheTypeDef, "CapabilityUsage">;
						args: never;
					};
					toHour: {
						type: number;
						args: never;
					};
				};
				fragments: [];
			};
			CapabilityTier: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					fromTokens: {
						type: string;
						args: never;
					};
					rate: {
						type: Record<CacheTypeDef, "CapabilityRate">;
						args: never;
					};
					toTokens: {
						type: string | null;
						args: never;
					};
				};
				fragments: [];
			};
			CapabilityUsage: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					mode: {
						type: CapabilityPriceMode$options;
						args: never;
					};
					rates: {
						type: (Record<CacheTypeDef, "CapabilityRate">)[];
						args: never;
					};
					slots: {
						type: (Record<CacheTypeDef, "CapabilitySlot">)[];
						args: never;
					};
					tierBasis: {
						type: PriceTierBasis$options | null;
						args: never;
					};
					tiers: {
						type: (Record<CacheTypeDef, "CapabilityTier">)[];
						args: never;
					};
					timezone: {
						type: string | null;
						args: never;
					};
				};
				fragments: [];
			};
			CodeAppearancePreferences: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					font: {
						type: Record<CacheTypeDef, "CodeFontPreferences">;
						args: never;
					};
					theme: {
						type: PreferenceCodeTheme$options;
						args: never;
					};
				};
				fragments: [];
			};
			CodeFontPreferences: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					family: {
						type: PreferenceFontFamily$options;
						args: never;
					};
					size: {
						type: PreferencePixelFontSize$options;
						args: never;
					};
				};
				fragments: [];
			};
			ConfigurationEntry: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					key: {
						type: string;
						args: never;
					};
					value: {
						type: string;
						args: never;
					};
				};
				fragments: [];
			};
			ConfigurationInput: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					allowedValues: {
						type: (string)[] | null;
						args: never;
					};
					kind: {
						type: ConfigurationInputKind$options;
						args: never;
					};
					name: {
						type: string;
						args: never;
					};
					required: {
						type: boolean;
						args: never;
					};
				};
				fragments: [];
			};
			ConnectionConfiguration: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					authentication: {
						type: Record<CacheTypeDef, "AuthenticationConfiguration"> | null;
						args: never;
					};
					baseUrl: {
						type: string | null;
						args: never;
					};
					endpoint: {
						type: Record<CacheTypeDef, "EndpointRequirements">;
						args: never;
					};
					path: {
						type: string | null;
						args: never;
					};
					query: {
						type: (Record<CacheTypeDef, "ConfigurationEntry">)[];
						args: never;
					};
					timeoutMs: {
						type: number | null;
						args: never;
					};
				};
				fragments: [];
			};
			CredentialPlacement: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					location: {
						type: CredentialLocation$options;
						args: never;
					};
					name: {
						type: string;
						args: never;
					};
					prefix: {
						type: string | null;
						args: never;
					};
				};
				fragments: [];
			};
			EmailChallenge: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					lifetime: {
						type: Record<CacheTypeDef, "TokenLifetime">;
						args: never;
					};
					token: {
						type: string | null;
						args: never;
					};
				};
				fragments: [];
			};
			EmailChallengePayload: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "EmailChallenge">;
						args: never;
					};
				};
				fragments: [];
			};
			EndpointConfiguration: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					requiredInputs: {
						type: (Record<CacheTypeDef, "ConfigurationInput">)[];
						args: never;
					};
					urlTemplate: {
						type: string | null;
						args: never;
					};
				};
				fragments: [];
			};
			EndpointRequirements: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					configuration: {
						type: Record<CacheTypeDef, "EndpointConfiguration">;
						args: never;
					};
					mode: {
						type: EndpointMode$options;
						args: never;
					};
				};
				fragments: [];
			};
			FeatureSupport: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					pricing: {
						type: Record<CacheTypeDef, "CapabilityPricing"> | null;
						args: never;
					};
					status: {
						type: CapabilityStatus$options;
						args: never;
					};
				};
				fragments: [];
			};
			Group: {
				idFields: {
					id: any;
				};
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					description: {
						type: string;
						args: never;
					};
					id: {
						type: string;
						args: never;
					};
					memberIds: {
						type: (string)[];
						args: never;
					};
					name: {
						type: string;
						args: never;
					};
					roles: {
						type: Record<CacheTypeDef, "RoleConnection">;
						args: never;
					};
				};
				fragments: [];
			};
			GroupConnection: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					edges: {
						type: (Record<CacheTypeDef, "GroupEdge">)[];
						args: never;
					};
					pageInfo: {
						type: Record<CacheTypeDef, "PageInfo">;
						args: never;
					};
				};
				fragments: [];
			};
			GroupEdge: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					cursor: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "Group">;
						args: never;
					};
				};
				fragments: [];
			};
			GroupPayload: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "Group">;
						args: never;
					};
				};
				fragments: [];
			};
			IDPayload: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					node: {
						type: string;
						args: never;
					};
				};
				fragments: [];
			};
			ImageGenerationConfiguration: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					formats: {
						type: (string)[] | null;
						args: never;
					};
					maxImages: {
						type: number | null;
						args: never;
					};
				};
				fragments: [];
			};
			ImageGenerationSupport: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					configuration: {
						type: Record<CacheTypeDef, "ImageGenerationConfiguration"> | null;
						args: never;
					};
					pricing: {
						type: Record<CacheTypeDef, "CapabilityPricing"> | null;
						args: never;
					};
					status: {
						type: CapabilityStatus$options;
						args: never;
					};
				};
				fragments: [];
			};
			Invitation: {
				idFields: {
					id: any;
				};
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					email: {
						type: string;
						args: never;
					};
					expiresAt: {
						type: number;
						args: never;
					};
					id: {
						type: string;
						args: never;
					};
					inviterId: {
						type: string;
						args: never;
					};
					issuedAt: {
						type: number;
						args: never;
					};
					organizationId: {
						type: string;
						args: never;
					};
					status: {
						type: string;
						args: never;
					};
				};
				fragments: [];
			};
			InvitationConnection: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					edges: {
						type: (Record<CacheTypeDef, "InvitationEdge">)[];
						args: never;
					};
					pageInfo: {
						type: Record<CacheTypeDef, "PageInfo">;
						args: never;
					};
				};
				fragments: [];
			};
			InvitationContext: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					email: {
						type: string;
						args: never;
					};
					expiresAt: {
						type: number;
						args: never;
					};
					organizationId: {
						type: string;
						args: never;
					};
					organizationName: {
						type: string;
						args: never;
					};
					signup: {
						type: boolean;
						args: never;
					};
				};
				fragments: [];
			};
			InvitationContextPayload: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "InvitationContext">;
						args: never;
					};
				};
				fragments: [];
			};
			InvitationEdge: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					cursor: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "Invitation">;
						args: never;
					};
				};
				fragments: [];
			};
			InvitationIssued: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					emailAdmitted: {
						type: boolean;
						args: never;
					};
					invitation: {
						type: Record<CacheTypeDef, "Invitation">;
						args: never;
					};
					url: {
						type: string;
						args: never;
					};
				};
				fragments: [];
			};
			InvitationIssuedConnection: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					edges: {
						type: (Record<CacheTypeDef, "InvitationIssuedEdge">)[];
						args: never;
					};
					pageInfo: {
						type: Record<CacheTypeDef, "PageInfo">;
						args: never;
					};
				};
				fragments: [];
			};
			InvitationIssuedEdge: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					cursor: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "InvitationIssued">;
						args: never;
					};
				};
				fragments: [];
			};
			InvitationIssuedPayload: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "InvitationIssued">;
						args: never;
					};
				};
				fragments: [];
			};
			InvitationPayload: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "Invitation">;
						args: never;
					};
				};
				fragments: [];
			};
			Me: {
				idFields: {
					id: any;
				};
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					cedarEntities: {
						type: string;
						args: never;
					};
					cedarSource: {
						type: string;
						args: never;
					};
					id: {
						type: string;
						args: never;
					};
					organizationId: {
						type: string;
						args: never;
					};
					organizationOwner: {
						type: boolean;
						args: never;
					};
					organizationPermissions: {
						type: (Record<CacheTypeDef, "OrganizationPermission">)[];
						args: never;
					};
					sessionId: {
						type: string;
						args: never;
					};
				};
				fragments: [];
			};
			MePayload: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "Me">;
						args: never;
					};
				};
				fragments: [];
			};
			Member: {
				idFields: {
					id: any;
				};
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					available: {
						type: boolean;
						args: never;
					};
					displayName: {
						type: string;
						args: never;
					};
					effectiveRoleIds: {
						type: (string)[];
						args: never;
					};
					email: {
						type: string;
						args: never;
					};
					groupIds: {
						type: (string)[];
						args: never;
					};
					groups: {
						type: (Record<CacheTypeDef, "Group">)[];
						args: never;
					};
					id: {
						type: string;
						args: never;
					};
					joinedAt: {
						type: number;
						args: never;
					};
					role: {
						type: Record<CacheTypeDef, "Role">;
						args: never;
					};
				};
				fragments: [];
			};
			MemberConnection: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					edges: {
						type: (Record<CacheTypeDef, "MemberEdge">)[];
						args: never;
					};
					pageInfo: {
						type: Record<CacheTypeDef, "PageInfo">;
						args: never;
					};
				};
				fragments: [];
			};
			MemberDisplayPreferences: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					nameFormat: {
						type: PreferenceNamingFormat$options;
						args: never;
					};
				};
				fragments: [];
			};
			MemberEdge: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					cursor: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "Member">;
						args: never;
					};
				};
				fragments: [];
			};
			MessageCompositionPreferences: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					convertEmoticons: {
						type: boolean;
						args: never;
					};
					sendShortcut: {
						type: PreferenceKeyCombination$options;
						args: never;
					};
				};
				fragments: [];
			};
			ModelBilling: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					amountPerCredit: {
						type: string | null;
						args: never;
					};
					currency: {
						type: string | null;
						args: never;
					};
					description: {
						type: string | null;
						args: never;
					};
					entitlements: {
						type: (Record<CacheTypeDef, "BillingAllowance">)[];
						args: never;
					};
					fee: {
						type: Record<CacheTypeDef, "BillingFee"> | null;
						args: never;
					};
					kind: {
						type: ModelBillingKind$options;
						args: never;
					};
					plan: {
						type: string | null;
						args: never;
					};
					product: {
						type: string | null;
						args: never;
					};
					requestRate: {
						type: Record<CacheTypeDef, "CapabilityRate"> | null;
						args: never;
					};
				};
				fragments: [];
			};
			ModelConnection: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					configuration: {
						type: Record<CacheTypeDef, "ConnectionConfiguration">;
						args: never;
					};
					protocol: {
						type: UpstreamProtocol$options;
						args: never;
					};
				};
				fragments: [];
			};
			ModelOperation: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					operation: {
						type: TextModelProtocol$options;
						args: never;
					};
					path: {
						type: string;
						args: never;
					};
				};
				fragments: [];
			};
			ModelTemplate: {
				idFields: {
					id: any;
				};
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					audio: {
						type: Record<CacheTypeDef, "AudioCapabilities">;
						args: never;
					};
					billing: {
						type: Record<CacheTypeDef, "ModelBilling"> | null;
						args: never;
					};
					billingModel: {
						type: BillingModel$options;
						args: never;
					};
					bodyDefaults: {
						type: (Record<CacheTypeDef, "BodyDefaultEntry">)[];
						args: never;
					};
					cacheBilling: {
						type: CacheBilling$options;
						args: never;
					};
					cachedPrice: {
						type: string | null;
						args: never;
					};
					capabilities: {
						type: (ProviderModelCapability$options)[];
						args: never;
					};
					connections: {
						type: (Record<CacheTypeDef, "ModelConnection">)[];
						args: never;
					};
					customFields: {
						type: (string)[];
						args: never;
					};
					description: {
						type: string | null;
						args: never;
					};
					displayName: {
						type: string;
						args: never;
					};
					effortMapping: {
						type: (Record<CacheTypeDef, "ThinkingEffort">)[];
						args: never;
					};
					headers: {
						type: (Record<CacheTypeDef, "ConfigurationEntry">)[];
						args: never;
					};
					id: {
						type: string;
						args: never;
					};
					imageGeneration: {
						type: Record<CacheTypeDef, "ImageGenerationSupport">;
						args: never;
					};
					inputPrice: {
						type: string | null;
						args: never;
					};
					maxContextTokens: {
						type: number | null;
						args: never;
					};
					maxInputTokens: {
						type: number | null;
						args: never;
					};
					maxOutputTokens: {
						type: number | null;
						args: never;
					};
					name: {
						type: string;
						args: never;
					};
					operations: {
						type: (Record<CacheTypeDef, "ModelOperation">)[];
						args: never;
					};
					origin: {
						type: ModelTemplateOrigin$options;
						args: never;
					};
					outputPrice: {
						type: string | null;
						args: never;
					};
					pdf: {
						type: Record<CacheTypeDef, "PdfSupport">;
						args: never;
					};
					pricingMode: {
						type: PricingMode$options;
						args: never;
					};
					providerTemplateId: {
						type: string;
						args: never;
					};
					routeSelection: {
						type: Record<CacheTypeDef, "RouteSelection"> | null;
						args: never;
					};
					search: {
						type: Record<CacheTypeDef, "SearchSupport">;
						args: never;
					};
					serviceKind: {
						type: ModelServiceKind$options | null;
						args: never;
					};
					structuredOutput: {
						type: Record<CacheTypeDef, "StructuredOutputSupport">;
						args: never;
					};
					text: {
						type: Record<CacheTypeDef, "TextCapabilities">;
						args: never;
					};
					thinking: {
						type: Record<CacheTypeDef, "ThinkingSupport">;
						args: never;
					};
					tools: {
						type: Record<CacheTypeDef, "ToolCapabilities">;
						args: never;
					};
					upstreamModelId: {
						type: string;
						args: never;
					};
					video: {
						type: Record<CacheTypeDef, "VideoSupport">;
						args: never;
					};
					vision: {
						type: Record<CacheTypeDef, "VisionSupport">;
						args: never;
					};
				};
				fragments: [];
			};
			ModelTemplateConnection: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					edges: {
						type: (Record<CacheTypeDef, "ModelTemplateEdge">)[];
						args: never;
					};
					pageInfo: {
						type: Record<CacheTypeDef, "PageInfo">;
						args: never;
					};
				};
				fragments: [];
			};
			ModelTemplateEdge: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					cursor: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "ModelTemplate">;
						args: never;
					};
				};
				fragments: [];
			};
			ModelTemplatePayload: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "ModelTemplate">;
						args: never;
					};
				};
				fragments: [];
			};
			Mutation: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					analyticUserProfilePut: {
						type: Record<CacheTypeDef, "AnalyticUserProfilePayload">;
						args: {
							input: AnalyticUserProfilePutInput;
						};
					};
					emailSigninConfirm: {
						type: Record<CacheTypeDef, "SessionCredentialsPayload">;
						args: {
							input: EmailCodeInput;
						};
					};
					emailSigninRequest: {
						type: Record<CacheTypeDef, "EmailChallengePayload">;
						args: {
							input: EmailInput;
						};
					};
					emailSignupConfirm: {
						type: Record<CacheTypeDef, "SessionCredentialsPayload">;
						args: {
							input: EmailCodeInput;
						};
					};
					emailSignupRequest: {
						type: Record<CacheTypeDef, "EmailChallengePayload">;
						args: {
							input: EmailInput;
						};
					};
					generateOrgAvatarUrl: {
						type: Record<CacheTypeDef, "AvatarUploadPayload">;
						args: {
							input: ImageUploadInput;
						};
					};
					generateUrlAvatarUrl: {
						type: Record<CacheTypeDef, "AvatarUploadPayload">;
						args: {
							input: ImageUploadInput;
						};
					};
					groupCreate: {
						type: Record<CacheTypeDef, "GroupPayload">;
						args: {
							input: GroupCreateInput;
						};
					};
					groupDelete: {
						type: Record<CacheTypeDef, "BooleanPayload">;
						args: {
							id: string | number;
						};
					};
					groupUpdate: {
						type: Record<CacheTypeDef, "GroupPayload">;
						args: {
							id: string | number;
							input: GroupUpdateInput;
						};
					};
					invitationCreate: {
						type: Record<CacheTypeDef, "InvitationIssuedPayload">;
						args: {
							input: InvitationCreateInput;
						};
					};
					invitationCreateMany: {
						type: Record<CacheTypeDef, "InvitationIssuedConnection">;
						args: {
							input: InvitationBatchCreateInput;
						};
					};
					invitationRevoke: {
						type: Record<CacheTypeDef, "InvitationPayload">;
						args: {
							id: string | number;
						};
					};
					modelTemplateCreate: {
						type: Record<CacheTypeDef, "ModelTemplatePayload">;
						args: {
							input: ModelTemplateCreateInput;
							providerTemplateId: string | number;
						};
					};
					modelTemplateCustomize: {
						type: Record<CacheTypeDef, "ModelTemplatePayload">;
						args: {
							id: string | number;
							input: ModelTemplateCustomizeInput;
							providerTemplateId: string | number;
						};
					};
					modelTemplateDelete: {
						type: Record<CacheTypeDef, "BooleanPayload">;
						args: {
							id: string | number;
							providerTemplateId: string | number;
						};
					};
					modelTemplateReset: {
						type: Record<CacheTypeDef, "ModelTemplatePayload">;
						args: {
							id: string | number;
							providerTemplateId: string | number;
						};
					};
					onboardingAdvance: {
						type: Record<CacheTypeDef, "OnboardingContextPayload">;
						args: {
							back: boolean;
						};
					};
					onboardingComplete: {
						type: Record<CacheTypeDef, "OnboardingContextPayload">;
						args: never;
					};
					onboardingPresent: {
						type: Record<CacheTypeDef, "OnboardingContextPayload">;
						args: {
							manual: boolean;
						};
					};
					organizationOnboardingSettingsUpdate: {
						type: Record<CacheTypeDef, "OrganizationOnboardingSettingsPayload">;
						args: {
							input: OrganizationOnboardingSettingsInput;
						};
					};
					organizationProfileUpdate: {
						type: Record<CacheTypeDef, "OrganizationPayload">;
						args: {
							input: OrganizationProfileUpdateInput;
						};
					};
					passkeyAuthenticationFinish: {
						type: Record<CacheTypeDef, "SessionCredentialsPayload">;
						args: {
							input: PasskeyFinishInput;
						};
					};
					passkeyAuthenticationStart: {
						type: Record<CacheTypeDef, "PasskeyCeremonyPayload">;
						args: {
							purpose: AuthenticationPurpose$options;
						};
					};
					passkeyRegistrationFinish: {
						type: Record<CacheTypeDef, "PasskeySummaryPayload">;
						args: {
							input: PasskeyFinishInput;
							name: PasskeyRegistrationInput;
						};
					};
					passkeyRegistrationStart: {
						type: Record<CacheTypeDef, "PasskeyCeremonyPayload">;
						args: never;
					};
					passkeyRemove: {
						type: Record<CacheTypeDef, "IDPayload">;
						args: {
							id: string | number;
						};
					};
					passkeyUpdate: {
						type: Record<CacheTypeDef, "PasskeySummaryPayload">;
						args: {
							id: string | number;
							input: PasskeyRegistrationInput;
						};
					};
					providerCreate: {
						type: Record<CacheTypeDef, "ProviderPayload">;
						args: {
							input: ProviderCreateFromTemplateInput;
						};
					};
					providerLogoUpload: {
						type: Record<CacheTypeDef, "ProviderLogoUploadPayload">;
						args: {
							input: ImageUploadInput;
						};
					};
					roleAssign: {
						type: Record<CacheTypeDef, "RolePayload">;
						args: {
							id: string | number;
							memberId: string | number;
						};
					};
					roleCreate: {
						type: Record<CacheTypeDef, "RolePayload">;
						args: {
							input: RoleCreateInput;
						};
					};
					roleDelete: {
						type: Record<CacheTypeDef, "BooleanPayload">;
						args: {
							id: string | number;
						};
					};
					roleUpdate: {
						type: Record<CacheTypeDef, "RolePayload">;
						args: {
							id: string | number;
							input: RoleUpdateInput;
						};
					};
					sessionRefresh: {
						type: Record<CacheTypeDef, "SessionCredentialsPayload">;
						args: never;
					};
					sessionRevoke: {
						type: Record<CacheTypeDef, "BooleanPayload">;
						args: never;
					};
					sessionRevokeOther: {
						type: Record<CacheTypeDef, "BooleanPayload">;
						args: {
							id: string | number;
						};
					};
					sessionRevokeOthers: {
						type: Record<CacheTypeDef, "BooleanPayload">;
						args: never;
					};
					userPreferencesReset: {
						type: Record<CacheTypeDef, "UserPreferencesPayload">;
						args: {
							groups: (UserPreferenceGroup$options)[];
						};
					};
					userPreferencesUpdate: {
						type: Record<CacheTypeDef, "UserPreferencesPayload">;
						args: {
							input: UserPreferencesPatchInput;
						};
					};
					userProfileUpdate: {
						type: Record<CacheTypeDef, "UserPayload">;
						args: {
							input: UserProfileUpdateInput;
						};
					};
				};
				fragments: [];
			};
			NavigationPreferences: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					home: {
						type: PreferenceDestination$options;
						args: never;
					};
				};
				fragments: [];
			};
			OnboardingContext: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					flow: {
						type: OnboardingFlow$options;
						args: never;
					};
					hasBeenPresented: {
						type: boolean;
						args: never;
					};
					required: {
						type: boolean;
						args: never;
					};
					settings: {
						type: Record<CacheTypeDef, "OrganizationOnboardingSettings">;
						args: never;
					};
					shouldPresent: {
						type: boolean;
						args: never;
					};
					step: {
						type: OnboardingStep$options;
						args: never;
					};
				};
				fragments: [];
			};
			OnboardingContextPayload: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "OnboardingContext">;
						args: never;
					};
				};
				fragments: [];
			};
			OnboardingLink: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					href: {
						type: string;
						args: never;
					};
					label: {
						type: string;
						args: never;
					};
				};
				fragments: [];
			};
			Organization: {
				idFields: {
					id: any;
				};
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					avatarUrl: {
						type: string | null;
						args: never;
					};
					canUpdateProfile: {
						type: boolean;
						args: never;
					};
					id: {
						type: string;
						args: never;
					};
					name: {
						type: string;
						args: never;
					};
					ownerMemberIds: {
						type: (string)[];
						args: never;
					};
				};
				fragments: [];
			};
			OrganizationOnboardingSettings: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					artworkColor: {
						type: string | null;
						args: never;
					};
					destination: {
						type: OnboardingDestination$options;
						args: never;
					};
					links: {
						type: (Record<CacheTypeDef, "OnboardingLink">)[];
						args: never;
					};
					requireAvatar: {
						type: boolean;
						args: never;
					};
					requireFullName: {
						type: boolean;
						args: never;
					};
					welcome: {
						type: string;
						args: never;
					};
				};
				fragments: [];
			};
			OrganizationOnboardingSettingsPayload: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "OrganizationOnboardingSettings">;
						args: never;
					};
				};
				fragments: [];
			};
			OrganizationPayload: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "Organization">;
						args: never;
					};
				};
				fragments: [];
			};
			OrganizationPermission: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					action: {
						type: string;
						args: never;
					};
					allowed: {
						type: boolean;
						args: never;
					};
				};
				fragments: [];
			};
			PageInfo: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					endCursor: {
						type: string | null;
						args: never;
					};
					hasNextPage: {
						type: boolean;
						args: never;
					};
					hasPreviousPage: {
						type: boolean;
						args: never;
					};
					startCursor: {
						type: string | null;
						args: never;
					};
				};
				fragments: [];
			};
			PalettePreferences: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					dark: {
						type: PreferenceDarkPalette$options;
						args: never;
					};
					light: {
						type: PreferenceLightPalette$options;
						args: never;
					};
				};
				fragments: [];
			};
			PasskeyCeremony: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					expiresAt: {
						type: string;
						args: never;
					};
					options: {
						type: Record<CacheTypeDef, "PasskeyOptions">;
						args: never;
					};
					token: {
						type: string;
						args: never;
					};
				};
				fragments: [];
			};
			PasskeyCeremonyPayload: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "PasskeyCeremony">;
						args: never;
					};
				};
				fragments: [];
			};
			PasskeyDescriptor: {
				idFields: {
					id: any;
				};
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					id: {
						type: string;
						args: never;
					};
					transports: {
						type: (string)[];
						args: never;
					};
				};
				fragments: [];
			};
			PasskeyExtensions: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					credProps: {
						type: boolean | null;
						args: never;
					};
					credentialProtectionPolicy: {
						type: string | null;
						args: never;
					};
					enforceCredentialProtectionPolicy: {
						type: boolean | null;
						args: never;
					};
				};
				fragments: [];
			};
			PasskeyOptions: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					algorithms: {
						type: (number)[];
						args: never;
					};
					challenge: {
						type: string;
						args: never;
					};
					credentials: {
						type: (Record<CacheTypeDef, "PasskeyDescriptor">)[];
						args: never;
					};
					extensions: {
						type: Record<CacheTypeDef, "PasskeyExtensions">;
						args: never;
					};
					residentKey: {
						type: string | null;
						args: never;
					};
					rpId: {
						type: string;
						args: never;
					};
					rpName: {
						type: string | null;
						args: never;
					};
					timeout: {
						type: number;
						args: never;
					};
					userDisplayName: {
						type: string | null;
						args: never;
					};
					userId: {
						type: string | null;
						args: never;
					};
					userName: {
						type: string | null;
						args: never;
					};
					userVerification: {
						type: string;
						args: never;
					};
				};
				fragments: [];
			};
			PasskeySummary: {
				idFields: {
					id: any;
				};
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					credentialId: {
						type: string;
						args: never;
					};
					id: {
						type: string;
						args: never;
					};
					lastUsedAt: {
						type: string | null;
						args: never;
					};
					name: {
						type: string;
						args: never;
					};
					registeredAt: {
						type: string;
						args: never;
					};
				};
				fragments: [];
			};
			PasskeySummaryConnection: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					edges: {
						type: (Record<CacheTypeDef, "PasskeySummaryEdge">)[];
						args: never;
					};
					pageInfo: {
						type: Record<CacheTypeDef, "PageInfo">;
						args: never;
					};
				};
				fragments: [];
			};
			PasskeySummaryEdge: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					cursor: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "PasskeySummary">;
						args: never;
					};
				};
				fragments: [];
			};
			PasskeySummaryPayload: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "PasskeySummary">;
						args: never;
					};
				};
				fragments: [];
			};
			PdfConfiguration: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					maxFiles: {
						type: number | null;
						args: never;
					};
					maxPages: {
						type: number | null;
						args: never;
					};
				};
				fragments: [];
			};
			PdfSupport: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					configuration: {
						type: Record<CacheTypeDef, "PdfConfiguration"> | null;
						args: never;
					};
					pricing: {
						type: Record<CacheTypeDef, "CapabilityPricing"> | null;
						args: never;
					};
					status: {
						type: CapabilityStatus$options;
						args: never;
					};
				};
				fragments: [];
			};
			PolicyAction: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					description: {
						type: string;
						args: never;
					};
					displayName: {
						type: string;
						args: never;
					};
					entityType: {
						type: string;
						args: never;
					};
					name: {
						type: string;
						args: never;
					};
					resourceType: {
						type: string;
						args: never;
					};
				};
				fragments: [];
			};
			PolicyActionConnection: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					edges: {
						type: (Record<CacheTypeDef, "PolicyActionEdge">)[];
						args: never;
					};
					pageInfo: {
						type: Record<CacheTypeDef, "PageInfo">;
						args: never;
					};
				};
				fragments: [];
			};
			PolicyActionEdge: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					cursor: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "PolicyAction">;
						args: never;
					};
				};
				fragments: [];
			};
			Provider: {
				idFields: {
					id: any;
				};
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					connection: {
						type: Record<CacheTypeDef, "ConnectionConfiguration">;
						args: never;
					};
					description: {
						type: string | null;
						args: never;
					};
					id: {
						type: string;
						args: never;
					};
					logoUrl: {
						type: string | null;
						args: never;
					};
					models: {
						type: (Record<CacheTypeDef, "ProviderModel">)[];
						args: never;
					};
					name: {
						type: string;
						args: never;
					};
					organizationId: {
						type: string;
						args: never;
					};
				};
				fragments: [];
			};
			ProviderConnection: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					edges: {
						type: (Record<CacheTypeDef, "ProviderEdge">)[];
						args: never;
					};
					pageInfo: {
						type: Record<CacheTypeDef, "PageInfo">;
						args: never;
					};
				};
				fragments: [];
			};
			ProviderEdge: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					cursor: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "Provider">;
						args: never;
					};
				};
				fragments: [];
			};
			ProviderLogoUpload: {
				idFields: {
					id: any;
				};
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					id: {
						type: string;
						args: never;
					};
					upload: {
						type: Record<CacheTypeDef, "AvatarUpload">;
						args: never;
					};
				};
				fragments: [];
			};
			ProviderLogoUploadPayload: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "ProviderLogoUpload">;
						args: never;
					};
				};
				fragments: [];
			};
			ProviderModel: {
				idFields: {
					id: any;
				};
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					cachedPrice: {
						type: string | null;
						args: never;
					};
					capabilities: {
						type: (ProviderModelCapability$options)[];
						args: never;
					};
					connection: {
						type: Record<CacheTypeDef, "ConnectionConfiguration">;
						args: never;
					};
					contextTokens: {
						type: number | null;
						args: never;
					};
					enabled: {
						type: boolean;
						args: never;
					};
					id: {
						type: string;
						args: never;
					};
					inputPrice: {
						type: string | null;
						args: never;
					};
					name: {
						type: string;
						args: never;
					};
					organizationId: {
						type: string;
						args: never;
					};
					outputPrice: {
						type: string | null;
						args: never;
					};
					protocol: {
						type: UpstreamProtocol$options;
						args: never;
					};
					providerId: {
						type: string;
						args: never;
					};
					text: {
						type: Record<CacheTypeDef, "TextCapabilities">;
						args: never;
					};
				};
				fragments: [];
			};
			ProviderPayload: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "Provider">;
						args: never;
					};
				};
				fragments: [];
			};
			ProviderTemplate: {
				idFields: {
					id: any;
				};
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					authentication: {
						type: (Record<CacheTypeDef, "AuthenticationConfiguration">)[];
						args: never;
					};
					baseUrl: {
						type: string | null;
						args: never;
					};
					connection: {
						type: Record<CacheTypeDef, "ConnectionConfiguration">;
						args: never;
					};
					description: {
						type: string | null;
						args: never;
					};
					endpoint: {
						type: Record<CacheTypeDef, "EndpointRequirements">;
						args: never;
					};
					headers: {
						type: (Record<CacheTypeDef, "ConfigurationEntry">)[];
						args: never;
					};
					id: {
						type: string;
						args: never;
					};
					logoUrl: {
						type: string | null;
						args: never;
					};
					models: {
						type: Record<CacheTypeDef, "ModelTemplateConnection">;
						args: {
							filter?: ModelTemplateFilterInput | null | undefined;
						};
					};
					name: {
						type: string;
						args: never;
					};
				};
				fragments: [];
			};
			ProviderTemplateConnection: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					edges: {
						type: (Record<CacheTypeDef, "ProviderTemplateEdge">)[];
						args: never;
					};
					pageInfo: {
						type: Record<CacheTypeDef, "PageInfo">;
						args: never;
					};
				};
				fragments: [];
			};
			ProviderTemplateEdge: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					cursor: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "ProviderTemplate">;
						args: never;
					};
				};
				fragments: [];
			};
			ProviderTemplatePayload: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "ProviderTemplate">;
						args: never;
					};
				};
				fragments: [];
			};
			Role: {
				idFields: {
					id: any;
				};
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					canReadPolicy: {
						type: boolean;
						args: never;
					};
					cedarSource: {
						type: string | null;
						args: never;
					};
					color: {
						type: string;
						args: never;
					};
					description: {
						type: string;
						args: never;
					};
					id: {
						type: string;
						args: never;
					};
					name: {
						type: string;
						args: never;
					};
					owner: {
						type: boolean;
						args: never;
					};
				};
				fragments: [];
			};
			RoleConnection: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					edges: {
						type: (Record<CacheTypeDef, "RoleEdge">)[];
						args: never;
					};
					pageInfo: {
						type: Record<CacheTypeDef, "PageInfo">;
						args: never;
					};
				};
				fragments: [];
			};
			RoleEdge: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					cursor: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "Role">;
						args: never;
					};
				};
				fragments: [];
			};
			RolePayload: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "Role">;
						args: never;
					};
				};
				fragments: [];
			};
			RouteOption: {
				idFields: {
					id: any;
				};
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					billing: {
						type: Record<CacheTypeDef, "ModelBilling"> | null;
						args: never;
					};
					contextTokens: {
						type: number | null;
						args: never;
					};
					id: {
						type: string;
						args: never;
					};
					implicitCaching: {
						type: boolean | null;
						args: never;
					};
					provider: {
						type: string;
						args: never;
					};
					text: {
						type: Record<CacheTypeDef, "TextCapabilities">;
						args: never;
					};
					upstreamModelId: {
						type: string;
						args: never;
					};
				};
				fragments: [];
			};
			RouteSelection: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					allowFallbacks: {
						type: boolean;
						args: never;
					};
					exactProviderSelection: {
						type: boolean;
						args: never;
					};
					options: {
						type: (Record<CacheTypeDef, "RouteOption">)[];
						args: never;
					};
					selected: {
						type: string | null;
						args: never;
					};
				};
				fragments: [];
			};
			SearchConfiguration: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					maxResults: {
						type: number | null;
						args: never;
					};
				};
				fragments: [];
			};
			SearchSupport: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					configuration: {
						type: Record<CacheTypeDef, "SearchConfiguration"> | null;
						args: never;
					};
					pricing: {
						type: Record<CacheTypeDef, "CapabilityPricing"> | null;
						args: never;
					};
					status: {
						type: CapabilityStatus$options;
						args: never;
					};
				};
				fragments: [];
			};
			Session: {
				idFields: {
					id: any;
				};
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					avatarUrl: {
						type: string | null;
						args: never;
					};
					expiresAt: {
						type: string;
						args: never;
					};
					id: {
						type: string;
						args: never;
					};
					ipAddress: {
						type: string | null;
						args: never;
					};
					lastSeenAt: {
						type: string;
						args: never;
					};
					organizationId: {
						type: string;
						args: never;
					};
					organizationName: {
						type: string;
						args: never;
					};
					passkey: {
						type: Record<CacheTypeDef, "PasskeySummary"> | null;
						args: never;
					};
					signedInAt: {
						type: string;
						args: never;
					};
					userAgent: {
						type: string | null;
						args: never;
					};
					userId: {
						type: string;
						args: never;
					};
				};
				fragments: [];
			};
			SessionConnection: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					edges: {
						type: (Record<CacheTypeDef, "SessionEdge">)[];
						args: never;
					};
					pageInfo: {
						type: Record<CacheTypeDef, "PageInfo">;
						args: never;
					};
				};
				fragments: [];
			};
			SessionCredentials: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					accessLifetime: {
						type: Record<CacheTypeDef, "TokenLifetime"> | null;
						args: never;
					};
					accessToken: {
						type: string | null;
						args: never;
					};
					passkeyChangeLifetime: {
						type: Record<CacheTypeDef, "TokenLifetime"> | null;
						args: never;
					};
					passkeyChangeToken: {
						type: string | null;
						args: never;
					};
					purpose: {
						type: AuthenticationPurpose$options;
						args: never;
					};
					refreshLifetime: {
						type: Record<CacheTypeDef, "TokenLifetime"> | null;
						args: never;
					};
					refreshToken: {
						type: string | null;
						args: never;
					};
				};
				fragments: [];
			};
			SessionCredentialsPayload: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "SessionCredentials">;
						args: never;
					};
				};
				fragments: [];
			};
			SessionEdge: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					cursor: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "Session">;
						args: never;
					};
				};
				fragments: [];
			};
			SessionPayload: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "Session">;
						args: never;
					};
				};
				fragments: [];
			};
			SidebarEntry: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					item: {
						type: PreferenceDestination$options;
						args: never;
					};
					visibility: {
						type: PreferenceSidebarVisibility$options;
						args: never;
					};
				};
				fragments: [];
			};
			SidebarLayoutSection: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					category: {
						type: PreferenceSidebarCategory$options;
						args: never;
					};
					items: {
						type: (PreferenceDestination$options)[];
						args: never;
					};
				};
				fragments: [];
			};
			SidebarPreferences: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					badgeStyle: {
						type: PreferenceBadgeStyle$options;
						args: never;
					};
					entries: {
						type: (Record<CacheTypeDef, "SidebarEntry">)[];
						args: never;
					};
					layout: {
						type: (Record<CacheTypeDef, "SidebarLayoutSection">)[];
						args: never;
					};
				};
				fragments: [];
			};
			StructuredOutputConfiguration: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					jsonMode: {
						type: Record<CacheTypeDef, "FeatureSupport">;
						args: never;
					};
					jsonSchema: {
						type: Record<CacheTypeDef, "FeatureSupport">;
						args: never;
					};
				};
				fragments: [];
			};
			StructuredOutputSupport: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					configuration: {
						type: Record<CacheTypeDef, "StructuredOutputConfiguration"> | null;
						args: never;
					};
					pricing: {
						type: Record<CacheTypeDef, "CapabilityPricing"> | null;
						args: never;
					};
					status: {
						type: CapabilityStatus$options;
						args: never;
					};
				};
				fragments: [];
			};
			TextCapabilities: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					input: {
						type: Record<CacheTypeDef, "TextInputSupport">;
						args: never;
					};
					output: {
						type: Record<CacheTypeDef, "TextGenerationSupport">;
						args: never;
					};
				};
				fragments: [];
			};
			TextGenerationConfiguration: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					maxTokens: {
						type: number | null;
						args: never;
					};
					thinking: {
						type: Record<CacheTypeDef, "ThinkingSupport">;
						args: never;
					};
				};
				fragments: [];
			};
			TextGenerationSupport: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					configuration: {
						type: Record<CacheTypeDef, "TextGenerationConfiguration"> | null;
						args: never;
					};
					pricing: {
						type: Record<CacheTypeDef, "CapabilityPricing"> | null;
						args: never;
					};
					status: {
						type: CapabilityStatus$options;
						args: never;
					};
				};
				fragments: [];
			};
			TextInputConfiguration: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					maxTokens: {
						type: number | null;
						args: never;
					};
				};
				fragments: [];
			};
			TextInputSupport: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					configuration: {
						type: Record<CacheTypeDef, "TextInputConfiguration"> | null;
						args: never;
					};
					pricing: {
						type: Record<CacheTypeDef, "CapabilityPricing"> | null;
						args: never;
					};
					status: {
						type: CapabilityStatus$options;
						args: never;
					};
				};
				fragments: [];
			};
			ThemePreferences: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					mode: {
						type: PreferenceAppearanceMode$options;
						args: never;
					};
					palettes: {
						type: Record<CacheTypeDef, "PalettePreferences">;
						args: never;
					};
				};
				fragments: [];
			};
			ThinkingBudget: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					defaultTokens: {
						type: number | null;
						args: never;
					};
					maxTokens: {
						type: number;
						args: never;
					};
					minTokens: {
						type: number;
						args: never;
					};
				};
				fragments: [];
			};
			ThinkingConfiguration: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					budget: {
						type: Record<CacheTypeDef, "ThinkingBudget"> | null;
						args: never;
					};
					defaultEffort: {
						type: ThinkingEffortLevel$options | null;
						args: never;
					};
					effortMapping: {
						type: (Record<CacheTypeDef, "ThinkingEffort">)[];
						args: never;
					};
				};
				fragments: [];
			};
			ThinkingEffort: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					level: {
						type: ThinkingEffortLevel$options;
						args: never;
					};
					value: {
						type: string;
						args: never;
					};
				};
				fragments: [];
			};
			ThinkingSupport: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					configuration: {
						type: Record<CacheTypeDef, "ThinkingConfiguration"> | null;
						args: never;
					};
					pricing: {
						type: Record<CacheTypeDef, "CapabilityPricing"> | null;
						args: never;
					};
					status: {
						type: CapabilityStatus$options;
						args: never;
					};
				};
				fragments: [];
			};
			TokenLifetime: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					expiresAt: {
						type: string;
						args: never;
					};
					expiresInSeconds: {
						type: string;
						args: never;
					};
					issuedAt: {
						type: string;
						args: never;
					};
					throttleSeconds: {
						type: string;
						args: never;
					};
				};
				fragments: [];
			};
			ToolCallingConfiguration: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					maxTools: {
						type: number | null;
						args: never;
					};
				};
				fragments: [];
			};
			ToolCallingSupport: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					configuration: {
						type: Record<CacheTypeDef, "ToolCallingConfiguration"> | null;
						args: never;
					};
					pricing: {
						type: Record<CacheTypeDef, "CapabilityPricing"> | null;
						args: never;
					};
					status: {
						type: CapabilityStatus$options;
						args: never;
					};
				};
				fragments: [];
			};
			ToolCapabilities: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					calling: {
						type: Record<CacheTypeDef, "ToolCallingSupport">;
						args: never;
					};
					forcedChoice: {
						type: Record<CacheTypeDef, "FeatureSupport">;
						args: never;
					};
					parallelCalls: {
						type: Record<CacheTypeDef, "FeatureSupport">;
						args: never;
					};
				};
				fragments: [];
			};
			TypographyPreferences: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					family: {
						type: PreferenceFontFamily$options;
						args: never;
					};
					size: {
						type: PreferenceRelativeFontSize$options;
						args: never;
					};
				};
				fragments: [];
			};
			UploadFormField: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					name: {
						type: string;
						args: never;
					};
					value: {
						type: string;
						args: never;
					};
				};
				fragments: [];
			};
			User: {
				idFields: {
					id: any;
				};
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					avatarUrl: {
						type: string | null;
						args: never;
					};
					displayName: {
						type: string;
						args: never;
					};
					email: {
						type: string;
						args: never;
					};
					id: {
						type: string;
						args: never;
					};
					username: {
						type: string;
						args: never;
					};
				};
				fragments: [];
			};
			UserPayload: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "User">;
						args: never;
					};
				};
				fragments: [];
			};
			UserPreferences: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					accessibility: {
						type: Record<CacheTypeDef, "AccessibilityPreferences">;
						args: never;
					};
					calendar: {
						type: Record<CacheTypeDef, "CalendarPreferences">;
						args: never;
					};
					codeAppearance: {
						type: Record<CacheTypeDef, "CodeAppearancePreferences">;
						args: never;
					};
					memberDisplay: {
						type: Record<CacheTypeDef, "MemberDisplayPreferences">;
						args: never;
					};
					messageComposition: {
						type: Record<CacheTypeDef, "MessageCompositionPreferences">;
						args: never;
					};
					navigation: {
						type: Record<CacheTypeDef, "NavigationPreferences">;
						args: never;
					};
					sidebar: {
						type: Record<CacheTypeDef, "SidebarPreferences">;
						args: never;
					};
					theme: {
						type: Record<CacheTypeDef, "ThemePreferences">;
						args: never;
					};
					typography: {
						type: Record<CacheTypeDef, "TypographyPreferences">;
						args: never;
					};
				};
				fragments: [];
			};
			UserPreferencesPayload: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					node: {
						type: Record<CacheTypeDef, "UserPreferences">;
						args: never;
					};
				};
				fragments: [];
			};
			ValidationIssue: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					code: {
						type: string;
						args: never;
					};
					message: {
						type: string;
						args: never;
					};
					path: {
						type: (string)[];
						args: never;
					};
				};
				fragments: [];
			};
			VideoConfiguration: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					formats: {
						type: (string)[] | null;
						args: never;
					};
					maxDurationMs: {
						type: number | null;
						args: never;
					};
					maxFrames: {
						type: number | null;
						args: never;
					};
				};
				fragments: [];
			};
			VideoSupport: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					configuration: {
						type: Record<CacheTypeDef, "VideoConfiguration"> | null;
						args: never;
					};
					pricing: {
						type: Record<CacheTypeDef, "CapabilityPricing"> | null;
						args: never;
					};
					status: {
						type: CapabilityStatus$options;
						args: never;
					};
				};
				fragments: [];
			};
			VisionConfiguration: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					formats: {
						type: (string)[] | null;
						args: never;
					};
					maxImages: {
						type: number | null;
						args: never;
					};
				};
				fragments: [];
			};
			VisionSupport: {
				idFields: never;
				fields: {
					__typename: {
						type: string;
						args: never;
					};
					configuration: {
						type: Record<CacheTypeDef, "VisionConfiguration"> | null;
						args: never;
					};
					pricing: {
						type: Record<CacheTypeDef, "CapabilityPricing"> | null;
						args: never;
					};
					status: {
						type: CapabilityStatus$options;
						args: never;
					};
				};
				fragments: [];
			};
		};
		lists: {
		};
		queries: [[AuthInvitationControllerInvitationContext$artifact, AuthInvitationControllerInvitationContext$result, AuthInvitationControllerInvitationContext$input], [AuthSession$artifact, AuthSession$result, AuthSession$input], [AuthSession2$artifact, AuthSession2$result, AuthSession2$input], [AuthUserProfile$artifact, AuthUserProfile$result, AuthUserProfile$input], [OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile$artifact, OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile$result, OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile$input], [OrganizationProviders$artifact, OrganizationProviders$result, OrganizationProviders$input], [PasskeyPasskeys$artifact, PasskeyPasskeys$result, PasskeyPasskeys$input], [PasskeyPasskeys2$artifact, PasskeyPasskeys2$result, PasskeyPasskeys2$input], [PasskeyPasskeysAndSession$artifact, PasskeyPasskeysAndSession$result, PasskeyPasskeysAndSession$input], [PermissionMe$artifact, PermissionMe$result, PermissionMe$input], [PortalAuthority$artifact, PortalAuthority$result, PortalAuthority$input], [PreferenceUserUserPreferences$artifact, PreferenceUserUserPreferences$result, PreferenceUserUserPreferences$input], [ProviderCatalogEffectiveModelTemplate$artifact, ProviderCatalogEffectiveModelTemplate$result, ProviderCatalogEffectiveModelTemplate$input], [ProviderCatalogEffectiveProviderTemplate$artifact, ProviderCatalogEffectiveProviderTemplate$result, ProviderCatalogEffectiveProviderTemplate$input], [ProviderCatalogEffectiveProviderTemplates$artifact, ProviderCatalogEffectiveProviderTemplates$result, ProviderCatalogEffectiveProviderTemplates$input], [ProviderCatalogModelTemplate$artifact, ProviderCatalogModelTemplate$result, ProviderCatalogModelTemplate$input], [ProviderProvider$artifact, ProviderProvider$result, ProviderProvider$input], [SettingsGroupGroupAndRolesAndMembers$artifact, SettingsGroupGroupAndRolesAndMembers$result, SettingsGroupGroupAndRolesAndMembers$input], [SettingsGroupMembers$artifact, SettingsGroupMembers$result, SettingsGroupMembers$input], [SettingsMeAndOrganizationAndRolesAndGroups$artifact, SettingsMeAndOrganizationAndRolesAndGroups$result, SettingsMeAndOrganizationAndRolesAndGroups$input], [SettingsMembersInvitations$artifact, SettingsMembersInvitations$result, SettingsMembersInvitations$input], [SettingsMembersMeAndOrganizationAndMembers$artifact, SettingsMembersMeAndOrganizationAndMembers$result, SettingsMembersMeAndOrganizationAndMembers$input], [SettingsOnboardingOrganizationOnboardingSettings$artifact, SettingsOnboardingOrganizationOnboardingSettings$result, SettingsOnboardingOrganizationOnboardingSettings$input], [SettingsOrganizationOrganizationProfile$artifact, SettingsOrganizationOrganizationProfile$result, SettingsOrganizationOrganizationProfile$input], [SettingsProfileUserProfile$artifact, SettingsProfileUserProfile$result, SettingsProfileUserProfile$input], [SettingsRoleOrganizationPolicyActions$artifact, SettingsRoleOrganizationPolicyActions$result, SettingsRoleOrganizationPolicyActions$input], [SettingsRoleRole$artifact, SettingsRoleRole$result, SettingsRoleRole$input], [SettingsSessions$artifact, SettingsSessions$result, SettingsSessions$input]];
		scalars: number | boolean | string | number | string;
};

declare module 'houdini/runtime' {
	interface CacheTypeDef {
		scalars: number | boolean | string | number | string
	}
}
