export default {
  scalars: [0, 2, 5, 15, 20, 26, 43, 44],
  types: {
    Boolean: {},
    BooleanPayload: {
      node: [0],
      __typename: [43],
    },
    ContextTokens: {},
    CreatedGroup: {
      description: [43],
      id: [15],
      name: [43],
      roleIds: [15],
      __typename: [43],
    },
    CreatedGroupPayload: {
      node: [3],
      __typename: [43],
    },
    CredentialDelivery: {},
    EmailChallenge: {
      lifetime: [45],
      token: [43],
      __typename: [43],
    },
    EmailChallengePayload: {
      node: [6],
      __typename: [43],
    },
    EmailCodeInput: {
      code: [43],
      delivery: [5],
      __typename: [43],
    },
    EmailInput: {
      delivery: [5],
      email: [43],
      __typename: [43],
    },
    Group: {
      description: [43],
      id: [15],
      name: [43],
      __typename: [43],
    },
    GroupConnection: {
      edges: [13],
      pageInfo: [19],
      __typename: [43],
    },
    GroupCreateInput: {
      description: [43],
      name: [43],
      roleIds: [43],
      __typename: [43],
    },
    GroupEdge: {
      cursor: [43],
      node: [10],
      __typename: [43],
    },
    GroupPayload: {
      node: [10],
      __typename: [43],
    },
    ID: {},
    Mutation: {
      emailSigninConfirm: [
        41,
        {
          input: [8, 'EmailCodeInput!'],
        },
      ],
      emailSigninRequest: [
        7,
        {
          input: [9, 'EmailInput!'],
        },
      ],
      emailSignupConfirm: [
        41,
        {
          input: [8, 'EmailCodeInput!'],
        },
      ],
      emailSignupRequest: [
        7,
        {
          input: [9, 'EmailInput!'],
        },
      ],
      groupCreate: [
        4,
        {
          input: [12, 'GroupCreateInput!'],
        },
      ],
      roleCreate: [
        38,
        {
          input: [36, 'RoleCreateInput!'],
        },
      ],
      sessionRefresh: [41],
      sessionRevoke: [1],
      __typename: [43],
    },
    Organization: {
      id: [15],
      name: [43],
      __typename: [43],
    },
    OrganizationPayload: {
      node: [17],
      __typename: [43],
    },
    PageInfo: {
      endCursor: [43],
      hasNextPage: [0],
      hasPreviousPage: [0],
      startCursor: [43],
      __typename: [43],
    },
    ProviderApiProtocol: {},
    ProviderCatalogRef: {
      model: [43],
      provider: [43],
      __typename: [43],
    },
    ProviderCreateInput: {
      apiKey: [43],
      baseUrl: [43],
      headers: [24],
      models: [28],
      name: [43],
      __typename: [43],
    },
    ProviderEffortMapping: {
      high: [43],
      low: [43],
      max: [43],
      medium: [43],
      xhigh: [43],
      __typename: [43],
    },
    ProviderHeader: {
      key: [43],
      value: [43],
      __typename: [43],
    },
    ProviderHeaderUpdate: {
      key: [43],
      value: [43],
      __typename: [43],
    },
    ProviderModelCapability: {},
    ProviderModelConfiguration: {
      cachedPrice: [43],
      capabilities: [26],
      catalogRef: [21],
      effortMapping: [23],
      enabled: [0],
      inputPrice: [43],
      maxContextTokens: [2],
      name: [43],
      outputPrice: [43],
      path: [43],
      protocol: [44],
      __typename: [43],
    },
    ProviderModelWrite: {
      configuration: [27],
      id: [15],
      __typename: [43],
    },
    ProviderPatchInput: {
      apiKey: [43],
      baseUrl: [43],
      headers: [25],
      models: [28],
      name: [43],
      __typename: [43],
    },
    ProviderReplaceInput: {
      apiKey: [43],
      baseUrl: [43],
      headers: [25],
      models: [28],
      name: [43],
      __typename: [43],
    },
    ProviderTemplate: {
      baseUrl: [43],
      id: [15],
      name: [43],
      protocol: [20],
      __typename: [43],
    },
    ProvidersCreateInput: {
      providers: [22],
      __typename: [43],
    },
    Query: {
      group: [
        14,
        {
          id: [15, 'ID!'],
        },
      ],
      groups: [11],
      organization: [18],
      role: [
        38,
        {
          id: [15, 'ID!'],
        },
      ],
      roles: [35],
      session: [42],
      __typename: [43],
    },
    Role: {
      color: [43],
      description: [43],
      id: [15],
      name: [43],
      __typename: [43],
    },
    RoleConnection: {
      edges: [37],
      pageInfo: [19],
      __typename: [43],
    },
    RoleCreateInput: {
      cedarSource: [43],
      description: [43],
      name: [43],
      __typename: [43],
    },
    RoleEdge: {
      cursor: [43],
      node: [34],
      __typename: [43],
    },
    RolePayload: {
      node: [34],
      __typename: [43],
    },
    Session: {
      id: [15],
      organizationId: [15],
      organizationName: [43],
      userId: [15],
      __typename: [43],
    },
    SessionCredentials: {
      accessLifetime: [45],
      accessToken: [43],
      refreshLifetime: [45],
      refreshToken: [43],
      __typename: [43],
    },
    SessionCredentialsPayload: {
      node: [40],
      __typename: [43],
    },
    SessionPayload: {
      node: [39],
      __typename: [43],
    },
    String: {},
    TextModelProtocol: {},
    TokenLifetime: {
      expiresAt: [43],
      expiresInSeconds: [43],
      issuedAt: [43],
      throttleSeconds: [43],
      __typename: [43],
    },
  },
}
