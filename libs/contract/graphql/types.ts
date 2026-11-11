export default {
  scalars: [2, 3, 5, 13, 18, 24, 38, 39],
  types: {
    AuthenticationMutation: {
      sessionRefresh: [37],
      sessionRevoke: [2],
      signinEmailConfirm: [
        37,
        {
          input: [7, 'EmailCodeInput!'],
        },
      ],
      signinEmailRequest: [
        6,
        {
          input: [8, 'EmailInput!'],
        },
      ],
      signupEmailConfirm: [
        37,
        {
          input: [7, 'EmailCodeInput!'],
        },
      ],
      signupEmailRequest: [
        6,
        {
          input: [8, 'EmailInput!'],
        },
      ],
      __typename: [38],
    },
    AuthenticationQuery: {
      session: [36],
      __typename: [38],
    },
    Boolean: {},
    ContextTokens: {},
    CreatedGroup: {
      description: [38],
      id: [13],
      name: [38],
      roleIds: [13],
      __typename: [38],
    },
    CredentialDelivery: {},
    EmailChallenge: {
      lifetime: [40],
      token: [38],
      __typename: [38],
    },
    EmailCodeInput: {
      code: [38],
      delivery: [5],
      __typename: [38],
    },
    EmailInput: {
      delivery: [5],
      email: [38],
      __typename: [38],
    },
    Group: {
      description: [38],
      id: [13],
      name: [38],
      __typename: [38],
    },
    GroupCreateInput: {
      description: [38],
      name: [38],
      roleIds: [38],
      __typename: [38],
    },
    GroupMutation: {
      create: [
        4,
        {
          input: [10, 'GroupCreateInput!'],
        },
      ],
      __typename: [38],
    },
    GroupQuery: {
      get: [
        9,
        {
          id: [13, 'ID!'],
        },
      ],
      list: [9],
      __typename: [38],
    },
    ID: {},
    Mutation: {
      authentication: [0],
      organization: [16],
      __typename: [38],
    },
    Organization: {
      id: [13],
      name: [38],
      __typename: [38],
    },
    OrganizationMutation: {
      groups: [11],
      roles: [34],
      __typename: [38],
    },
    OrganizationQuery: {
      get: [15],
      groups: [12],
      roles: [35],
      __typename: [38],
    },
    ProviderApiProtocol: {},
    ProviderCatalogRef: {
      model: [38],
      provider: [38],
      __typename: [38],
    },
    ProviderCreateInput: {
      apiKey: [38],
      baseUrl: [38],
      headers: [22],
      models: [26],
      name: [38],
      __typename: [38],
    },
    ProviderEffortMapping: {
      high: [38],
      low: [38],
      max: [38],
      medium: [38],
      xhigh: [38],
      __typename: [38],
    },
    ProviderHeader: {
      key: [38],
      value: [38],
      __typename: [38],
    },
    ProviderHeaderUpdate: {
      key: [38],
      value: [38],
      __typename: [38],
    },
    ProviderModelCapability: {},
    ProviderModelConfiguration: {
      cachedPrice: [38],
      capabilities: [24],
      catalogRef: [19],
      effortMapping: [21],
      enabled: [2],
      inputPrice: [38],
      maxContextTokens: [3],
      name: [38],
      outputPrice: [38],
      path: [38],
      protocol: [39],
      __typename: [38],
    },
    ProviderModelWrite: {
      configuration: [25],
      id: [13],
      __typename: [38],
    },
    ProviderPatchInput: {
      apiKey: [38],
      baseUrl: [38],
      headers: [23],
      models: [26],
      name: [38],
      __typename: [38],
    },
    ProviderReplaceInput: {
      apiKey: [38],
      baseUrl: [38],
      headers: [23],
      models: [26],
      name: [38],
      __typename: [38],
    },
    ProviderTemplate: {
      baseUrl: [38],
      id: [13],
      name: [38],
      protocol: [18],
      __typename: [38],
    },
    ProvidersCreateInput: {
      providers: [20],
      __typename: [38],
    },
    Query: {
      authentication: [1],
      organization: [17],
      __typename: [38],
    },
    Role: {
      description: [38],
      id: [13],
      name: [38],
      __typename: [38],
    },
    RoleCreateInput: {
      cedarSource: [38],
      description: [38],
      name: [38],
      __typename: [38],
    },
    RoleMutation: {
      create: [
        32,
        {
          input: [33, 'RoleCreateInput!'],
        },
      ],
      __typename: [38],
    },
    RoleQuery: {
      get: [
        32,
        {
          id: [13, 'ID!'],
        },
      ],
      list: [32],
      __typename: [38],
    },
    Session: {
      id: [13],
      organizationId: [13],
      organizationName: [38],
      userId: [13],
      __typename: [38],
    },
    SessionCredentials: {
      accessLifetime: [40],
      accessToken: [38],
      refreshLifetime: [40],
      refreshToken: [38],
      __typename: [38],
    },
    String: {},
    TextModelProtocol: {},
    TokenLifetime: {
      expiresAt: [38],
      expiresInSeconds: [38],
      issuedAt: [38],
      throttleSeconds: [38],
      __typename: [38],
    },
  },
}
