export default {
  scalars: [4, 13, 17, 27, 28, 29, 31, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 64, 70, 96, 97, 106],
  types: {
    AccessibilityPatchInput: {
      pointerCursors: [4],
      reduceMotion: [4],
      underlineLinks: [4],
      __typename: [96],
    },
    AccessibilityPreferences: {
      pointerCursors: [4],
      reduceMotion: [4],
      underlineLinks: [4],
      __typename: [96],
    },
    AvatarUpload: {
      expiresAt: [96],
      fields: [103],
      url: [96],
      __typename: [96],
    },
    AvatarUploadPayload: {
      node: [2],
      __typename: [96],
    },
    Boolean: {},
    BooleanPayload: {
      node: [4],
      __typename: [96],
    },
    CalendarPatchInput: {
      firstDay: [62],
      __typename: [96],
    },
    CalendarPreferences: {
      firstDay: [62],
      __typename: [96],
    },
    ChatCreateInput: {
      message: [96],
      modelId: [96],
      __typename: [96],
    },
    CodeAppearancePatchInput: {
      font: [11],
      theme: [51],
      __typename: [96],
    },
    CodeAppearancePreferences: {
      font: [12],
      theme: [51],
      __typename: [96],
    },
    CodeFontPatchInput: {
      family: [54],
      size: [58],
      __typename: [96],
    },
    CodeFontPreferences: {
      family: [54],
      size: [58],
      __typename: [96],
    },
    ContextTokens: {},
    CreatedGroup: {
      description: [96],
      id: [28],
      name: [96],
      roleIds: [28],
      __typename: [96],
    },
    CreatedGroupPayload: {
      node: [14],
      __typename: [96],
    },
    CredentialCreateInput: {
      name: [96],
      __typename: [96],
    },
    CredentialDelivery: {},
    EmailChallenge: {
      lifetime: [100],
      token: [96],
      __typename: [96],
    },
    EmailChallengePayload: {
      node: [18],
      __typename: [96],
    },
    EmailCodeInput: {
      code: [96],
      delivery: [17],
      __typename: [96],
    },
    EmailInput: {
      delivery: [17],
      email: [96],
      __typename: [96],
    },
    Group: {
      description: [96],
      id: [28],
      name: [96],
      __typename: [96],
    },
    GroupConnection: {
      edges: [25],
      pageInfo: [43],
      __typename: [96],
    },
    GroupCreateInput: {
      description: [96],
      name: [96],
      roleIds: [96],
      __typename: [96],
    },
    GroupEdge: {
      cursor: [96],
      node: [22],
      __typename: [96],
    },
    GroupPayload: {
      node: [22],
      __typename: [96],
    },
    HomeDestination: {},
    ID: {},
    ImageMediaType: {},
    ImageUploadInput: {
      mediaType: [29],
      size: [31],
      __typename: [96],
    },
    Int: {},
    MemberDisplayPatchInput: {
      nameFormat: [57],
      __typename: [96],
    },
    MemberDisplayPreferences: {
      nameFormat: [57],
      __typename: [96],
    },
    MessageCompositionPatchInput: {
      convertEmoticons: [4],
      sendShortcut: [55],
      __typename: [96],
    },
    MessageCompositionPreferences: {
      convertEmoticons: [4],
      sendShortcut: [55],
      __typename: [96],
    },
    Mutation: {
      emailSigninConfirm: [
        87,
        {
          input: [20, 'EmailCodeInput!'],
        },
      ],
      emailSigninRequest: [
        19,
        {
          input: [21, 'EmailInput!'],
        },
      ],
      emailSignupConfirm: [
        87,
        {
          input: [20, 'EmailCodeInput!'],
        },
      ],
      emailSignupRequest: [
        19,
        {
          input: [21, 'EmailInput!'],
        },
      ],
      generateOrgAvatarUrl: [
        3,
        {
          input: [30, 'ImageUploadInput!'],
        },
      ],
      generateUrlAvatarUrl: [
        3,
        {
          input: [30, 'ImageUploadInput!'],
        },
      ],
      groupCreate: [
        15,
        {
          input: [24, 'GroupCreateInput!'],
        },
      ],
      organizationProfileUpdate: [
        40,
        {
          input: [41, 'OrganizationProfileUpdateInput!'],
        },
      ],
      roleCreate: [
        83,
        {
          input: [81, 'RoleCreateInput!'],
        },
      ],
      roleDelete: [
        5,
        {
          id: [28, 'ID!'],
        },
      ],
      roleUpdate: [
        83,
        {
          id: [28, 'ID!'],
          input: [84, 'RoleUpdateInput!'],
        },
      ],
      sessionRefresh: [87],
      sessionRevoke: [5],
      userPreferencesReset: [
        109,
        {
          groups: [106, '[UserPreferenceGroup!]!'],
        },
      ],
      userPreferencesUpdate: [
        109,
        {
          input: [108, 'UserPreferencesPatchInput!'],
        },
      ],
      userProfileUpdate: [
        105,
        {
          input: [110, 'UserProfileUpdateInput!'],
        },
      ],
      __typename: [96],
    },
    NavigationPatchInput: {
      home: [53],
      __typename: [96],
    },
    NavigationPreferences: {
      home: [53],
      __typename: [96],
    },
    Organization: {
      avatarUrl: [96],
      canUpdateProfile: [4],
      id: [28],
      name: [96],
      __typename: [96],
    },
    OrganizationPayload: {
      node: [39],
      __typename: [96],
    },
    OrganizationProfileUpdateInput: {
      avatarUploaded: [4],
      name: [96],
      __typename: [96],
    },
    OrganizationSettingsUpdateInput: {
      announcements: [96],
      fiscalMonth: [96],
      home: [27],
      name: [96],
      slug: [96],
      welcome: [96],
      __typename: [96],
    },
    PageInfo: {
      endCursor: [96],
      hasNextPage: [4],
      hasPreviousPage: [4],
      startCursor: [96],
      __typename: [96],
    },
    PalettePatchInput: {
      dark: [52],
      light: [56],
      __typename: [96],
    },
    PalettePreferences: {
      dark: [52],
      light: [56],
      __typename: [96],
    },
    PolicyAction: {
      description: [96],
      displayName: [96],
      entityType: [96],
      name: [96],
      resourceType: [96],
      __typename: [96],
    },
    PolicyActionConnection: {
      edges: [48],
      pageInfo: [43],
      __typename: [96],
    },
    PolicyActionEdge: {
      cursor: [96],
      node: [46],
      __typename: [96],
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
      name: [96],
      username: [96],
      __typename: [96],
    },
    ProviderApiProtocol: {},
    ProviderCatalogRef: {
      model: [96],
      provider: [96],
      __typename: [96],
    },
    ProviderCreateInput: {
      apiKey: [96],
      baseUrl: [96],
      headers: [68],
      models: [72],
      name: [96],
      __typename: [96],
    },
    ProviderEffortMapping: {
      high: [96],
      low: [96],
      max: [96],
      medium: [96],
      xhigh: [96],
      __typename: [96],
    },
    ProviderHeader: {
      key: [96],
      value: [96],
      __typename: [96],
    },
    ProviderHeaderUpdate: {
      key: [96],
      value: [96],
      __typename: [96],
    },
    ProviderModelCapability: {},
    ProviderModelConfiguration: {
      cachedPrice: [96],
      capabilities: [70],
      catalogRef: [65],
      effortMapping: [67],
      enabled: [4],
      inputPrice: [96],
      maxContextTokens: [13],
      name: [96],
      outputPrice: [96],
      path: [96],
      protocol: [97],
      __typename: [96],
    },
    ProviderModelWrite: {
      configuration: [71],
      id: [28],
      __typename: [96],
    },
    ProviderPatchInput: {
      apiKey: [96],
      baseUrl: [96],
      headers: [69],
      models: [72],
      name: [96],
      __typename: [96],
    },
    ProviderReplaceInput: {
      apiKey: [96],
      baseUrl: [96],
      headers: [69],
      models: [72],
      name: [96],
      __typename: [96],
    },
    ProviderTemplate: {
      baseUrl: [96],
      id: [28],
      name: [96],
      protocol: [64],
      __typename: [96],
    },
    ProvidersCreateInput: {
      providers: [66],
      __typename: [96],
    },
    Query: {
      group: [
        26,
        {
          id: [28, 'ID!'],
        },
      ],
      groups: [23],
      organization: [40],
      organizationPolicyActions: [47],
      organizationProfile: [
        40,
        {
          id: [28, 'ID!'],
        },
      ],
      role: [
        83,
        {
          id: [28, 'ID!'],
        },
      ],
      roles: [80],
      session: [88],
      userPreferences: [109],
      userProfile: [
        105,
        {
          id: [28, 'ID!'],
        },
      ],
      __typename: [96],
    },
    ReportingScheduleInput: {
      timezone: [96],
      __typename: [96],
    },
    Role: {
      cedarSource: [96],
      color: [96],
      description: [96],
      id: [28],
      name: [96],
      owner: [4],
      __typename: [96],
    },
    RoleConnection: {
      edges: [82],
      pageInfo: [43],
      __typename: [96],
    },
    RoleCreateInput: {
      cedarSource: [96],
      description: [96],
      name: [96],
      __typename: [96],
    },
    RoleEdge: {
      cursor: [96],
      node: [79],
      __typename: [96],
    },
    RolePayload: {
      node: [79],
      __typename: [96],
    },
    RoleUpdateInput: {
      cedarSource: [96],
      description: [96],
      name: [96],
      __typename: [96],
    },
    Session: {
      avatarUrl: [96],
      id: [28],
      organizationId: [28],
      organizationName: [96],
      userId: [28],
      __typename: [96],
    },
    SessionCredentials: {
      accessLifetime: [100],
      accessToken: [96],
      refreshLifetime: [100],
      refreshToken: [96],
      __typename: [96],
    },
    SessionCredentialsPayload: {
      node: [86],
      __typename: [96],
    },
    SessionPayload: {
      node: [85],
      __typename: [96],
    },
    SidebarEntry: {
      item: [53],
      visibility: [61],
      __typename: [96],
    },
    SidebarLayoutSection: {
      category: [60],
      items: [53],
      __typename: [96],
    },
    SidebarLayoutSectionInput: {
      category: [60],
      items: [53],
      __typename: [96],
    },
    SidebarPatchInput: {
      badgeStyle: [50],
      layout: [91],
      order: [53],
      visibility: [94],
      __typename: [96],
    },
    SidebarPreferences: {
      badgeStyle: [50],
      entries: [89],
      layout: [90],
      __typename: [96],
    },
    SidebarVisibilityPatchInput: {
      item: [53],
      visibility: [61],
      __typename: [96],
    },
    SpendingLimitInput: {
      amount: [96],
      __typename: [96],
    },
    String: {},
    TextModelProtocol: {},
    ThemePatchInput: {
      mode: [49],
      palettes: [44],
      __typename: [96],
    },
    ThemePreferences: {
      mode: [49],
      palettes: [45],
      __typename: [96],
    },
    TokenLifetime: {
      expiresAt: [96],
      expiresInSeconds: [96],
      issuedAt: [96],
      throttleSeconds: [96],
      __typename: [96],
    },
    TypographyPatchInput: {
      family: [54],
      size: [59],
      __typename: [96],
    },
    TypographyPreferences: {
      family: [54],
      size: [59],
      __typename: [96],
    },
    UploadFormField: {
      name: [96],
      value: [96],
      __typename: [96],
    },
    User: {
      avatarUrl: [96],
      displayName: [96],
      email: [96],
      id: [28],
      username: [96],
      __typename: [96],
    },
    UserPayload: {
      node: [104],
      __typename: [96],
    },
    UserPreferenceGroup: {},
    UserPreferences: {
      accessibility: [1],
      calendar: [7],
      codeAppearance: [10],
      memberDisplay: [33],
      messageComposition: [35],
      navigation: [38],
      sidebar: [93],
      theme: [99],
      typography: [102],
      __typename: [96],
    },
    UserPreferencesPatchInput: {
      accessibility: [0],
      calendar: [6],
      codeAppearance: [9],
      memberDisplay: [32],
      messageComposition: [34],
      navigation: [37],
      sidebar: [92],
      theme: [98],
      typography: [101],
      __typename: [96],
    },
    UserPreferencesPayload: {
      node: [107],
      __typename: [96],
    },
    UserProfileUpdateInput: {
      avatarUploaded: [4],
      displayName: [96],
      username: [96],
      __typename: [96],
    },
  },
}
