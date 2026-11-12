export default {
  scalars: [0, 2, 5, 15, 23, 29, 46, 47],
  types: {
    Boolean: {},
    BooleanPayload: {
      node: [0],
      __typename: [46],
    },
    ContextTokens: {},
    CreatedGroup: {
      description: [46],
      id: [15],
      name: [46],
      roleIds: [15],
      __typename: [46],
    },
    CreatedGroupPayload: {
      node: [3],
      __typename: [46],
    },
    CredentialDelivery: {},
    EmailChallenge: {
      lifetime: [48],
      token: [46],
      __typename: [46],
    },
    EmailChallengePayload: {
      node: [6],
      __typename: [46],
    },
    EmailCodeInput: {
      code: [46],
      delivery: [5],
      __typename: [46],
    },
    EmailInput: {
      delivery: [5],
      email: [46],
      __typename: [46],
    },
    Group: {
      description: [46],
      id: [15],
      name: [46],
      __typename: [46],
    },
    GroupConnection: {
      edges: [13],
      pageInfo: [19],
      __typename: [46],
    },
    GroupCreateInput: {
      description: [46],
      name: [46],
      roleIds: [46],
      __typename: [46],
    },
    GroupEdge: {
      cursor: [46],
      node: [10],
      __typename: [46],
    },
    GroupPayload: {
      node: [10],
      __typename: [46],
    },
    ID: {},
    Mutation: {
      emailSigninConfirm: [
        44,
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
        44,
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
        41,
        {
          input: [39, 'RoleCreateInput!'],
        },
      ],
      sessionRefresh: [44],
      sessionRevoke: [1],
      __typename: [46],
    },
    Organization: {
      id: [15],
      name: [46],
      __typename: [46],
    },
    OrganizationPayload: {
      node: [17],
      __typename: [46],
    },
    PageInfo: {
      endCursor: [46],
      hasNextPage: [0],
      hasPreviousPage: [0],
      startCursor: [46],
      __typename: [46],
    },
    PolicyAction: {
      entityType: [46],
      name: [46],
      resourceType: [46],
      __typename: [46],
    },
    PolicyActionConnection: {
      edges: [22],
      pageInfo: [19],
      __typename: [46],
    },
    PolicyActionEdge: {
      cursor: [46],
      node: [20],
      __typename: [46],
    },
    ProviderApiProtocol: {},
    ProviderCatalogRef: {
      model: [46],
      provider: [46],
      __typename: [46],
    },
    ProviderCreateInput: {
      apiKey: [46],
      baseUrl: [46],
      headers: [27],
      models: [31],
      name: [46],
      __typename: [46],
    },
    ProviderEffortMapping: {
      high: [46],
      low: [46],
      max: [46],
      medium: [46],
      xhigh: [46],
      __typename: [46],
    },
    ProviderHeader: {
      key: [46],
      value: [46],
      __typename: [46],
    },
    ProviderHeaderUpdate: {
      key: [46],
      value: [46],
      __typename: [46],
    },
    ProviderModelCapability: {},
    ProviderModelConfiguration: {
      cachedPrice: [46],
      capabilities: [29],
      catalogRef: [24],
      effortMapping: [26],
      enabled: [0],
      inputPrice: [46],
      maxContextTokens: [2],
      name: [46],
      outputPrice: [46],
      path: [46],
      protocol: [47],
      __typename: [46],
    },
    ProviderModelWrite: {
      configuration: [30],
      id: [15],
      __typename: [46],
    },
    ProviderPatchInput: {
      apiKey: [46],
      baseUrl: [46],
      headers: [28],
      models: [31],
      name: [46],
      __typename: [46],
    },
    ProviderReplaceInput: {
      apiKey: [46],
      baseUrl: [46],
      headers: [28],
      models: [31],
      name: [46],
      __typename: [46],
    },
    ProviderTemplate: {
      baseUrl: [46],
      id: [15],
      name: [46],
      protocol: [23],
      __typename: [46],
    },
    ProvidersCreateInput: {
      providers: [25],
      __typename: [46],
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
      organizationPolicyActions: [21],
      role: [
        41,
        {
          id: [15, 'ID!'],
        },
      ],
      roles: [38],
      session: [45],
      __typename: [46],
    },
    Role: {
      color: [46],
      description: [46],
      id: [15],
      name: [46],
      __typename: [46],
    },
    RoleConnection: {
      edges: [40],
      pageInfo: [19],
      __typename: [46],
    },
    RoleCreateInput: {
      cedarSource: [46],
      description: [46],
      name: [46],
      __typename: [46],
    },
    RoleEdge: {
      cursor: [46],
      node: [37],
      __typename: [46],
    },
    RolePayload: {
      node: [37],
      __typename: [46],
    },
    Session: {
      id: [15],
      organizationId: [15],
      organizationName: [46],
      userId: [15],
      __typename: [46],
    },
    SessionCredentials: {
      accessLifetime: [48],
      accessToken: [46],
      refreshLifetime: [48],
      refreshToken: [46],
      __typename: [46],
    },
    SessionCredentialsPayload: {
      node: [43],
      __typename: [46],
    },
    SessionPayload: {
      node: [42],
      __typename: [46],
    },
    String: {},
    TextModelProtocol: {},
    TokenLifetime: {
      expiresAt: [46],
      expiresInSeconds: [46],
      issuedAt: [46],
      throttleSeconds: [46],
      __typename: [46],
    },
  },
}
