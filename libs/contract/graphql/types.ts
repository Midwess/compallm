export default {
  scalars: [2, 4, 7, 19, 26, 32, 51, 52],
  types: {
    AuthenticationMutation: {
      sessionRefresh: [49],
      sessionRevoke: [3],
      signinEmailConfirm: [
        49,
        {
          input: [10, 'EmailCodeInput!'],
        },
      ],
      signinEmailRequest: [
        9,
        {
          input: [11, 'EmailInput!'],
        },
      ],
      signupEmailConfirm: [
        49,
        {
          input: [10, 'EmailCodeInput!'],
        },
      ],
      signupEmailRequest: [
        9,
        {
          input: [11, 'EmailInput!'],
        },
      ],
      __typename: [51],
    },
    AuthenticationQuery: {
      session: [50],
      __typename: [51],
    },
    Boolean: {},
    BooleanPayload: {
      node: [2],
      __typename: [51],
    },
    ContextTokens: {},
    CreatedGroup: {
      description: [51],
      id: [19],
      name: [51],
      roleIds: [19],
      __typename: [51],
    },
    CreatedGroupPayload: {
      node: [5],
      __typename: [51],
    },
    CredentialDelivery: {},
    EmailChallenge: {
      lifetime: [53],
      token: [51],
      __typename: [51],
    },
    EmailChallengePayload: {
      node: [8],
      __typename: [51],
    },
    EmailCodeInput: {
      code: [51],
      delivery: [7],
      __typename: [51],
    },
    EmailInput: {
      delivery: [7],
      email: [51],
      __typename: [51],
    },
    Group: {
      description: [51],
      id: [19],
      name: [51],
      __typename: [51],
    },
    GroupConnection: {
      edges: [15],
      pageInfo: [25],
      __typename: [51],
    },
    GroupCreateInput: {
      description: [51],
      name: [51],
      roleIds: [51],
      __typename: [51],
    },
    GroupEdge: {
      cursor: [51],
      node: [12],
      __typename: [51],
    },
    GroupMutation: {
      create: [
        6,
        {
          input: [14, 'GroupCreateInput!'],
        },
      ],
      __typename: [51],
    },
    GroupPayload: {
      node: [12],
      __typename: [51],
    },
    GroupQuery: {
      get: [
        17,
        {
          id: [19, 'ID!'],
        },
      ],
      list: [13],
      __typename: [51],
    },
    ID: {},
    Mutation: {
      authentication: [0],
      organization: [22],
      __typename: [51],
    },
    Organization: {
      id: [19],
      name: [51],
      __typename: [51],
    },
    OrganizationMutation: {
      groups: [16],
      roles: [44],
      __typename: [51],
    },
    OrganizationPayload: {
      node: [21],
      __typename: [51],
    },
    OrganizationQuery: {
      get: [23],
      groups: [18],
      roles: [46],
      __typename: [51],
    },
    PageInfo: {
      endCursor: [51],
      hasNextPage: [2],
      hasPreviousPage: [2],
      startCursor: [51],
      __typename: [51],
    },
    ProviderApiProtocol: {},
    ProviderCatalogRef: {
      model: [51],
      provider: [51],
      __typename: [51],
    },
    ProviderCreateInput: {
      apiKey: [51],
      baseUrl: [51],
      headers: [30],
      models: [34],
      name: [51],
      __typename: [51],
    },
    ProviderEffortMapping: {
      high: [51],
      low: [51],
      max: [51],
      medium: [51],
      xhigh: [51],
      __typename: [51],
    },
    ProviderHeader: {
      key: [51],
      value: [51],
      __typename: [51],
    },
    ProviderHeaderUpdate: {
      key: [51],
      value: [51],
      __typename: [51],
    },
    ProviderModelCapability: {},
    ProviderModelConfiguration: {
      cachedPrice: [51],
      capabilities: [32],
      catalogRef: [27],
      effortMapping: [29],
      enabled: [2],
      inputPrice: [51],
      maxContextTokens: [4],
      name: [51],
      outputPrice: [51],
      path: [51],
      protocol: [52],
      __typename: [51],
    },
    ProviderModelWrite: {
      configuration: [33],
      id: [19],
      __typename: [51],
    },
    ProviderPatchInput: {
      apiKey: [51],
      baseUrl: [51],
      headers: [31],
      models: [34],
      name: [51],
      __typename: [51],
    },
    ProviderReplaceInput: {
      apiKey: [51],
      baseUrl: [51],
      headers: [31],
      models: [34],
      name: [51],
      __typename: [51],
    },
    ProviderTemplate: {
      baseUrl: [51],
      id: [19],
      name: [51],
      protocol: [26],
      __typename: [51],
    },
    ProvidersCreateInput: {
      providers: [28],
      __typename: [51],
    },
    Query: {
      authentication: [1],
      organization: [24],
      __typename: [51],
    },
    Role: {
      description: [51],
      id: [19],
      name: [51],
      __typename: [51],
    },
    RoleConnection: {
      edges: [43],
      pageInfo: [25],
      __typename: [51],
    },
    RoleCreateInput: {
      cedarSource: [51],
      description: [51],
      name: [51],
      __typename: [51],
    },
    RoleEdge: {
      cursor: [51],
      node: [40],
      __typename: [51],
    },
    RoleMutation: {
      create: [
        45,
        {
          input: [42, 'RoleCreateInput!'],
        },
      ],
      __typename: [51],
    },
    RolePayload: {
      node: [40],
      __typename: [51],
    },
    RoleQuery: {
      get: [
        45,
        {
          id: [19, 'ID!'],
        },
      ],
      list: [41],
      __typename: [51],
    },
    Session: {
      id: [19],
      organizationId: [19],
      organizationName: [51],
      userId: [19],
      __typename: [51],
    },
    SessionCredentials: {
      accessLifetime: [53],
      accessToken: [51],
      refreshLifetime: [53],
      refreshToken: [51],
      __typename: [51],
    },
    SessionCredentialsPayload: {
      node: [48],
      __typename: [51],
    },
    SessionPayload: {
      node: [47],
      __typename: [51],
    },
    String: {},
    TextModelProtocol: {},
    TokenLifetime: {
      expiresAt: [51],
      expiresInSeconds: [51],
      issuedAt: [51],
      throttleSeconds: [51],
      __typename: [51],
    },
  },
}
