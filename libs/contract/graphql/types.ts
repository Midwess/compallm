export default {
  scalars: [
    7, 16, 18, 24, 25, 32, 33, 36, 38, 68, 69, 72, 73, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109,
    110, 112, 118, 144, 146, 155,
  ],
  types: {
    AccessibilityPatchInput: {
      pointerCursors: [7],
      reduceMotion: [7],
      underlineLinks: [7],
      __typename: [144],
    },
    AccessibilityPreferences: {
      pointerCursors: [7],
      reduceMotion: [7],
      underlineLinks: [7],
      __typename: [144],
    },
    AnalyticUserProfile: {
      key: [144],
      value: [144],
      __typename: [144],
    },
    AnalyticUserProfilePayload: {
      node: [2],
      __typename: [144],
    },
    AnalyticUserProfilePutInput: {
      key: [144],
      value: [144],
      __typename: [144],
    },
    AvatarUpload: {
      expiresAt: [144],
      fields: [152],
      url: [144],
      __typename: [144],
    },
    AvatarUploadPayload: {
      node: [5],
      __typename: [144],
    },
    Boolean: {},
    BooleanPayload: {
      node: [7],
      __typename: [144],
    },
    CalendarPatchInput: {
      firstDay: [110],
      __typename: [144],
    },
    CalendarPreferences: {
      firstDay: [110],
      __typename: [144],
    },
    ChatCreateInput: {
      message: [144],
      modelId: [144],
      __typename: [144],
    },
    CodeAppearancePatchInput: {
      font: [14],
      theme: [99],
      __typename: [144],
    },
    CodeAppearancePreferences: {
      font: [15],
      theme: [99],
      __typename: [144],
    },
    CodeFontPatchInput: {
      family: [102],
      size: [106],
      __typename: [144],
    },
    CodeFontPreferences: {
      family: [102],
      size: [106],
      __typename: [144],
    },
    ContextTokens: {},
    CredentialCreateInput: {
      name: [144],
      __typename: [144],
    },
    CredentialDelivery: {},
    EmailChallenge: {
      lifetime: [149],
      token: [144],
      __typename: [144],
    },
    EmailChallengePayload: {
      node: [19],
      __typename: [144],
    },
    EmailCodeInput: {
      code: [144],
      delivery: [18],
      __typename: [144],
    },
    EmailInput: {
      delivery: [18],
      email: [144],
      invitation: [7],
      __typename: [144],
    },
    EmailReauthenticationInput: {
      code: [144],
      token: [144],
      __typename: [144],
    },
    FilterIs: {},
    Float: {},
    Group: {
      description: [144],
      id: [33],
      memberIds: [33],
      name: [144],
      roles: [128],
      __typename: [144],
    },
    GroupConnection: {
      edges: [29],
      pageInfo: [81],
      __typename: [144],
    },
    GroupCreateInput: {
      description: [144],
      name: [144],
      roleIds: [144],
      __typename: [144],
    },
    GroupEdge: {
      cursor: [144],
      node: [26],
      __typename: [144],
    },
    GroupPayload: {
      node: [26],
      __typename: [144],
    },
    GroupUpdateInput: {
      addMemberIds: [144],
      addRoleIds: [144],
      description: [144],
      name: [144],
      removeMemberIds: [144],
      removeRoleIds: [144],
      __typename: [144],
    },
    HomeDestination: {},
    ID: {},
    IDFilter: {
      eq: [33],
      __typename: [144],
    },
    IDPayload: {
      node: [33],
      __typename: [144],
    },
    ImageMediaType: {},
    ImageUploadInput: {
      mediaType: [36],
      size: [38],
      __typename: [144],
    },
    Int: {},
    Invitation: {
      email: [144],
      expiresAt: [25],
      id: [33],
      inviterId: [33],
      issuedAt: [25],
      organizationId: [33],
      status: [144],
      __typename: [144],
    },
    InvitationBatchCreateInput: {
      invitations: [44],
      __typename: [144],
    },
    InvitationConnection: {
      edges: [45],
      pageInfo: [81],
      __typename: [144],
    },
    InvitationContext: {
      email: [144],
      expiresAt: [25],
      organizationId: [33],
      organizationName: [144],
      signup: [7],
      __typename: [144],
    },
    InvitationContextPayload: {
      node: [42],
      __typename: [144],
    },
    InvitationCreateInput: {
      email: [144],
      __typename: [144],
    },
    InvitationEdge: {
      cursor: [144],
      node: [39],
      __typename: [144],
    },
    InvitationIssued: {
      emailAdmitted: [7],
      invitation: [39],
      url: [144],
      __typename: [144],
    },
    InvitationIssuedConnection: {
      edges: [48],
      pageInfo: [81],
      __typename: [144],
    },
    InvitationIssuedEdge: {
      cursor: [144],
      node: [46],
      __typename: [144],
    },
    InvitationIssuedPayload: {
      node: [46],
      __typename: [144],
    },
    InvitationPayload: {
      node: [39],
      __typename: [144],
    },
    Me: {
      cedarEntities: [144],
      cedarSource: [144],
      id: [33],
      organizationId: [33],
      organizationOwner: [7],
      sessionId: [33],
      __typename: [144],
    },
    MePayload: {
      node: [51],
      __typename: [144],
    },
    Member: {
      available: [7],
      displayName: [144],
      effectiveRoleIds: [33],
      email: [144],
      groupIds: [33],
      groups: [26],
      id: [33],
      joinedAt: [25],
      role: [127],
      __typename: [144],
    },
    MemberConnection: {
      edges: [57],
      pageInfo: [81],
      __typename: [144],
    },
    MemberDisplayPatchInput: {
      nameFormat: [105],
      __typename: [144],
    },
    MemberDisplayPreferences: {
      nameFormat: [105],
      __typename: [144],
    },
    MemberEdge: {
      cursor: [144],
      node: [53],
      __typename: [144],
    },
    MemberFilterInput: {
      displayName: [145],
      email: [145],
      or: [59],
      roleId: [34],
      __typename: [144],
    },
    MemberMatchInput: {
      displayName: [145],
      email: [145],
      __typename: [144],
    },
    MemberOrderInput: {
      displayName: [73],
      email: [73],
      id: [73],
      __typename: [144],
    },
    MessageCompositionPatchInput: {
      convertEmoticons: [7],
      sendShortcut: [103],
      __typename: [144],
    },
    MessageCompositionPreferences: {
      convertEmoticons: [7],
      sendShortcut: [103],
      __typename: [144],
    },
    Mutation: {
      analyticUserProfilePut: [
        3,
        {
          input: [4, 'AnalyticUserProfilePutInput!'],
        },
      ],
      emailReauthenticationConfirm: [
        8,
        {
          input: [23, 'EmailReauthenticationInput!'],
        },
      ],
      emailReauthenticationRequest: [85],
      emailSigninConfirm: [
        135,
        {
          input: [21, 'EmailCodeInput!'],
        },
      ],
      emailSigninRequest: [
        20,
        {
          input: [22, 'EmailInput!'],
        },
      ],
      emailSignupConfirm: [
        135,
        {
          input: [21, 'EmailCodeInput!'],
        },
      ],
      emailSignupRequest: [
        20,
        {
          input: [22, 'EmailInput!'],
        },
      ],
      generateOrgAvatarUrl: [
        6,
        {
          input: [37, 'ImageUploadInput!'],
        },
      ],
      generateUrlAvatarUrl: [
        6,
        {
          input: [37, 'ImageUploadInput!'],
        },
      ],
      groupCreate: [
        30,
        {
          input: [28, 'GroupCreateInput!'],
        },
      ],
      groupDelete: [
        8,
        {
          id: [33, 'ID!'],
        },
      ],
      groupUpdate: [
        30,
        {
          id: [33, 'ID!'],
          input: [31, 'GroupUpdateInput!'],
        },
      ],
      invitationCreate: [
        49,
        {
          input: [44, 'InvitationCreateInput!'],
        },
      ],
      invitationCreateMany: [
        47,
        {
          input: [40, 'InvitationBatchCreateInput!'],
        },
      ],
      invitationRevoke: [
        50,
        {
          id: [33, 'ID!'],
        },
      ],
      onboardingAdvance: [
        67,
        {
          back: [7, 'Boolean!'],
        },
      ],
      onboardingComplete: [67],
      onboardingPresent: [
        67,
        {
          manual: [7, 'Boolean!'],
        },
      ],
      organizationOnboardingSettingsUpdate: [
        77,
        {
          input: [76, 'OrganizationOnboardingSettingsInput!'],
        },
      ],
      organizationProfileUpdate: [
        78,
        {
          input: [79, 'OrganizationProfileUpdateInput!'],
        },
      ],
      passkeyAuthenticationFinish: [
        135,
        {
          input: [87, 'PasskeyFinishInput!'],
        },
      ],
      passkeyAuthenticationStart: [
        85,
        {
          email: [144, 'String!'],
        },
      ],
      passkeyReauthenticationFinish: [
        8,
        {
          input: [87, 'PasskeyFinishInput!'],
        },
      ],
      passkeyReauthenticationStart: [85],
      passkeyRegistrationFinish: [
        93,
        {
          input: [87, 'PasskeyFinishInput!'],
        },
      ],
      passkeyRegistrationStart: [
        85,
        {
          input: [89, 'PasskeyRegistrationInput!'],
        },
      ],
      passkeyRemove: [
        35,
        {
          id: [33, 'ID!'],
        },
      ],
      roleAssign: [
        131,
        {
          id: [33, 'ID!'],
          memberId: [33, 'ID!'],
        },
      ],
      roleCreate: [
        131,
        {
          input: [129, 'RoleCreateInput!'],
        },
      ],
      roleDelete: [
        8,
        {
          id: [33, 'ID!'],
        },
      ],
      roleUpdate: [
        131,
        {
          id: [33, 'ID!'],
          input: [132, 'RoleUpdateInput!'],
        },
      ],
      sessionRefresh: [135],
      sessionRevoke: [8],
      userPreferencesReset: [
        158,
        {
          groups: [155, '[UserPreferenceGroup!]!'],
        },
      ],
      userPreferencesUpdate: [
        158,
        {
          input: [157, 'UserPreferencesPatchInput!'],
        },
      ],
      userProfileUpdate: [
        154,
        {
          input: [159, 'UserProfileUpdateInput!'],
        },
      ],
      __typename: [144],
    },
    NavigationPatchInput: {
      home: [101],
      __typename: [144],
    },
    NavigationPreferences: {
      home: [101],
      __typename: [144],
    },
    OnboardingContext: {
      flow: [69],
      hasBeenPresented: [7],
      required: [7],
      settings: [75],
      shouldPresent: [7],
      step: [72],
      __typename: [144],
    },
    OnboardingContextPayload: {
      node: [66],
      __typename: [144],
    },
    OnboardingDestination: {},
    OnboardingFlow: {},
    OnboardingLink: {
      href: [144],
      label: [144],
      __typename: [144],
    },
    OnboardingLinkInput: {
      href: [144],
      label: [144],
      __typename: [144],
    },
    OnboardingStep: {},
    OrderByDirection: {},
    Organization: {
      avatarUrl: [144],
      canUpdateProfile: [7],
      id: [33],
      name: [144],
      ownerMemberIds: [33],
      __typename: [144],
    },
    OrganizationOnboardingSettings: {
      artworkColor: [144],
      destination: [68],
      links: [70],
      requireAvatar: [7],
      requireFullName: [7],
      welcome: [144],
      __typename: [144],
    },
    OrganizationOnboardingSettingsInput: {
      artworkColor: [144],
      destination: [68],
      links: [71],
      requireAvatar: [7],
      requireFullName: [7],
      welcome: [144],
      __typename: [144],
    },
    OrganizationOnboardingSettingsPayload: {
      node: [75],
      __typename: [144],
    },
    OrganizationPayload: {
      node: [74],
      __typename: [144],
    },
    OrganizationProfileUpdateInput: {
      avatarUploaded: [7],
      name: [144],
      __typename: [144],
    },
    OrganizationSettingsUpdateInput: {
      announcements: [144],
      fiscalMonth: [144],
      home: [32],
      name: [144],
      slug: [144],
      welcome: [144],
      __typename: [144],
    },
    PageInfo: {
      endCursor: [144],
      hasNextPage: [7],
      hasPreviousPage: [7],
      startCursor: [144],
      __typename: [144],
    },
    PalettePatchInput: {
      dark: [100],
      light: [104],
      __typename: [144],
    },
    PalettePreferences: {
      dark: [100],
      light: [104],
      __typename: [144],
    },
    PasskeyCeremony: {
      expiresAt: [144],
      options: [88],
      token: [144],
      __typename: [144],
    },
    PasskeyCeremonyPayload: {
      node: [84],
      __typename: [144],
    },
    PasskeyDescriptor: {
      id: [144],
      transports: [144],
      __typename: [144],
    },
    PasskeyFinishInput: {
      attestationObject: [144],
      authenticatorData: [144],
      clientDataJson: [144],
      delivery: [18],
      id: [144],
      rawId: [144],
      signature: [144],
      token: [144],
      transports: [144],
      userHandle: [144],
      __typename: [144],
    },
    PasskeyOptions: {
      algorithms: [38],
      challenge: [144],
      credentials: [86],
      rpId: [144],
      rpName: [144],
      timeout: [38],
      userDisplayName: [144],
      userId: [144],
      userName: [144],
      userVerification: [144],
      __typename: [144],
    },
    PasskeyRegistrationInput: {
      name: [144],
      __typename: [144],
    },
    PasskeySummary: {
      credentialId: [144],
      id: [33],
      lastUsedAt: [144],
      name: [144],
      registeredAt: [144],
      __typename: [144],
    },
    PasskeySummaryConnection: {
      edges: [92],
      pageInfo: [81],
      __typename: [144],
    },
    PasskeySummaryEdge: {
      cursor: [144],
      node: [90],
      __typename: [144],
    },
    PasskeySummaryPayload: {
      node: [90],
      __typename: [144],
    },
    PolicyAction: {
      description: [144],
      displayName: [144],
      entityType: [144],
      name: [144],
      resourceType: [144],
      __typename: [144],
    },
    PolicyActionConnection: {
      edges: [96],
      pageInfo: [81],
      __typename: [144],
    },
    PolicyActionEdge: {
      cursor: [144],
      node: [94],
      __typename: [144],
    },
    PreferenceAppearanceMode: {},
    PreferenceBadgeStyle: {},
    PreferenceCodeTheme: {},
    PreferenceDarkPalette: {},
    PreferenceDestination: {},
    PreferenceFontFamily: {},
    PreferenceKeyCombination: {},
    PreferenceLightPalette: {},
    PreferenceNamingFormat: {},
    PreferencePixelFontSize: {},
    PreferenceRelativeFontSize: {},
    PreferenceSidebarCategory: {},
    PreferenceSidebarVisibility: {},
    PreferenceWeekday: {},
    ProfileUpdateInput: {
      name: [144],
      username: [144],
      __typename: [144],
    },
    ProviderApiProtocol: {},
    ProviderCatalogRef: {
      model: [144],
      provider: [144],
      __typename: [144],
    },
    ProviderCreateInput: {
      apiKey: [144],
      baseUrl: [144],
      headers: [116],
      models: [120],
      name: [144],
      __typename: [144],
    },
    ProviderEffortMapping: {
      high: [144],
      low: [144],
      max: [144],
      medium: [144],
      xhigh: [144],
      __typename: [144],
    },
    ProviderHeader: {
      key: [144],
      value: [144],
      __typename: [144],
    },
    ProviderHeaderUpdate: {
      key: [144],
      value: [144],
      __typename: [144],
    },
    ProviderModelCapability: {},
    ProviderModelConfiguration: {
      cachedPrice: [144],
      capabilities: [118],
      catalogRef: [113],
      effortMapping: [115],
      enabled: [7],
      inputPrice: [144],
      maxContextTokens: [16],
      name: [144],
      outputPrice: [144],
      path: [144],
      protocol: [146],
      __typename: [144],
    },
    ProviderModelWrite: {
      configuration: [119],
      id: [33],
      __typename: [144],
    },
    ProviderPatchInput: {
      apiKey: [144],
      baseUrl: [144],
      headers: [117],
      models: [120],
      name: [144],
      __typename: [144],
    },
    ProviderReplaceInput: {
      apiKey: [144],
      baseUrl: [144],
      headers: [117],
      models: [120],
      name: [144],
      __typename: [144],
    },
    ProviderTemplate: {
      baseUrl: [144],
      id: [33],
      name: [144],
      protocol: [112],
      __typename: [144],
    },
    ProvidersCreateInput: {
      providers: [114],
      __typename: [144],
    },
    Query: {
      analyticUserProfile: [
        3,
        {
          key: [144, 'String!'],
        },
      ],
      emailAuthenticationMethods: [
        8,
        {
          email: [144, 'String!'],
        },
      ],
      group: [
        30,
        {
          id: [33, 'ID!'],
        },
      ],
      groups: [27],
      invitationContext: [43],
      invitations: [
        41,
        {
          after: [144],
          first: [38],
        },
      ],
      me: [52],
      members: [
        54,
        {
          after: [144],
          before: [144],
          currentUserFirst: [7],
          filter: [58],
          first: [38],
          last: [38],
          offset: [38],
          orderBy: [60, '[MemberOrderInput!]'],
        },
      ],
      onboardingContext: [67],
      organization: [78],
      organizationOnboardingSettings: [77],
      organizationPolicyActions: [95],
      organizationProfile: [
        78,
        {
          id: [33, 'ID!'],
        },
      ],
      passkeys: [91],
      role: [
        131,
        {
          id: [33, 'ID!'],
          includeUnseen: [7, 'Boolean!'],
        },
      ],
      roles: [
        128,
        {
          includeUnseen: [7, 'Boolean!'],
        },
      ],
      session: [136],
      userPreferences: [158],
      userProfile: [
        154,
        {
          id: [33, 'ID!'],
        },
      ],
      __typename: [144],
    },
    ReportingScheduleInput: {
      timezone: [144],
      __typename: [144],
    },
    Role: {
      cedarSource: [144],
      color: [144],
      description: [144],
      id: [33],
      name: [144],
      owner: [7],
      __typename: [144],
    },
    RoleConnection: {
      edges: [130],
      pageInfo: [81],
      __typename: [144],
    },
    RoleCreateInput: {
      cedarSource: [144],
      description: [144],
      name: [144],
      __typename: [144],
    },
    RoleEdge: {
      cursor: [144],
      node: [127],
      __typename: [144],
    },
    RolePayload: {
      node: [127],
      __typename: [144],
    },
    RoleUpdateInput: {
      cedarSource: [144],
      description: [144],
      name: [144],
      __typename: [144],
    },
    Session: {
      avatarUrl: [144],
      id: [33],
      organizationId: [33],
      organizationName: [144],
      userId: [33],
      __typename: [144],
    },
    SessionCredentials: {
      accessLifetime: [149],
      accessToken: [144],
      refreshLifetime: [149],
      refreshToken: [144],
      __typename: [144],
    },
    SessionCredentialsPayload: {
      node: [134],
      __typename: [144],
    },
    SessionPayload: {
      node: [133],
      __typename: [144],
    },
    SidebarEntry: {
      item: [101],
      visibility: [109],
      __typename: [144],
    },
    SidebarLayoutSection: {
      category: [108],
      items: [101],
      __typename: [144],
    },
    SidebarLayoutSectionInput: {
      category: [108],
      items: [101],
      __typename: [144],
    },
    SidebarPatchInput: {
      badgeStyle: [98],
      layout: [139],
      order: [101],
      visibility: [142],
      __typename: [144],
    },
    SidebarPreferences: {
      badgeStyle: [98],
      entries: [137],
      layout: [138],
      __typename: [144],
    },
    SidebarVisibilityPatchInput: {
      item: [101],
      visibility: [109],
      __typename: [144],
    },
    SpendingLimitInput: {
      amount: [144],
      __typename: [144],
    },
    String: {},
    StringFilter: {
      eq: [144],
      gt: [144],
      gte: [144],
      ilike: [144],
      in: [144],
      iregex: [144],
      is: [24],
      like: [144],
      lt: [144],
      lte: [144],
      neq: [144],
      regex: [144],
      startsWith: [144],
      __typename: [144],
    },
    TextModelProtocol: {},
    ThemePatchInput: {
      mode: [97],
      palettes: [82],
      __typename: [144],
    },
    ThemePreferences: {
      mode: [97],
      palettes: [83],
      __typename: [144],
    },
    TokenLifetime: {
      expiresAt: [144],
      expiresInSeconds: [144],
      issuedAt: [144],
      throttleSeconds: [144],
      __typename: [144],
    },
    TypographyPatchInput: {
      family: [102],
      size: [107],
      __typename: [144],
    },
    TypographyPreferences: {
      family: [102],
      size: [107],
      __typename: [144],
    },
    UploadFormField: {
      name: [144],
      value: [144],
      __typename: [144],
    },
    User: {
      avatarUrl: [144],
      displayName: [144],
      email: [144],
      id: [33],
      username: [144],
      __typename: [144],
    },
    UserPayload: {
      node: [153],
      __typename: [144],
    },
    UserPreferenceGroup: {},
    UserPreferences: {
      accessibility: [1],
      calendar: [10],
      codeAppearance: [13],
      memberDisplay: [56],
      messageComposition: [62],
      navigation: [65],
      sidebar: [141],
      theme: [148],
      typography: [151],
      __typename: [144],
    },
    UserPreferencesPatchInput: {
      accessibility: [0],
      calendar: [9],
      codeAppearance: [12],
      memberDisplay: [55],
      messageComposition: [61],
      navigation: [64],
      sidebar: [140],
      theme: [147],
      typography: [150],
      __typename: [144],
    },
    UserPreferencesPayload: {
      node: [156],
      __typename: [144],
    },
    UserProfileUpdateInput: {
      avatarUploaded: [7],
      displayName: [144],
      username: [144],
      __typename: [144],
    },
  },
}
