export default {
  scalars: [0, 2, 5, 15, 23, 29, 47, 48],
  types: {
    Boolean: {},
    BooleanPayload: {
      node: [0],
      __typename: [47],
    },
    ContextTokens: {},
    CreatedGroup: {
      description: [47],
      id: [15],
      name: [47],
      roleIds: [15],
      __typename: [47],
    },
    CreatedGroupPayload: {
      node: [3],
      __typename: [47],
    },
    CredentialDelivery: {},
    EmailChallenge: {
      lifetime: [49],
      token: [47],
      __typename: [47],
    },
    EmailChallengePayload: {
      node: [6],
      __typename: [47],
    },
    EmailCodeInput: {
      code: [47],
      delivery: [5],
      __typename: [47],
    },
    EmailInput: {
      delivery: [5],
      email: [47],
      __typename: [47],
    },
    Group: {
      description: [47],
      id: [15],
      name: [47],
      __typename: [47],
    },
    GroupConnection: {
      edges: [13],
      pageInfo: [19],
      __typename: [47],
    },
    GroupCreateInput: {
      description: [47],
      name: [47],
      roleIds: [47],
      __typename: [47],
    },
    GroupEdge: {
      cursor: [47],
      node: [10],
      __typename: [47],
    },
    GroupPayload: {
      node: [10],
      __typename: [47],
    },
    ID: {},
    Mutation: {
      emailSigninConfirm: [
        45,
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
        45,
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
      roleDelete: [
        1,
        {
          id: [15, 'ID!'],
        },
      ],
      roleUpdate: [
        41,
        {
          id: [15, 'ID!'],
          input: [42, 'RoleUpdateInput!'],
        },
      ],
      sessionRefresh: [45],
      sessionRevoke: [1],
      __typename: [47],
    },
    Organization: {
      id: [15],
      name: [47],
      __typename: [47],
    },
    OrganizationPayload: {
      node: [17],
      __typename: [47],
    },
    PageInfo: {
      endCursor: [47],
      hasNextPage: [0],
      hasPreviousPage: [0],
      startCursor: [47],
      __typename: [47],
    },
    PolicyAction: {
      description: [47],
      displayName: [47],
      entityType: [47],
      name: [47],
      resourceType: [47],
      __typename: [47],
    },
    PolicyActionConnection: {
      edges: [22],
      pageInfo: [19],
      __typename: [47],
    },
    PolicyActionEdge: {
      cursor: [47],
      node: [20],
      __typename: [47],
    },
    ProviderApiProtocol: {},
    ProviderCatalogRef: {
      model: [47],
      provider: [47],
      __typename: [47],
    },
    ProviderCreateInput: {
      apiKey: [47],
      baseUrl: [47],
      headers: [27],
      models: [31],
      name: [47],
      __typename: [47],
    },
    ProviderEffortMapping: {
      high: [47],
      low: [47],
      max: [47],
      medium: [47],
      xhigh: [47],
      __typename: [47],
    },
    ProviderHeader: {
      key: [47],
      value: [47],
      __typename: [47],
    },
    ProviderHeaderUpdate: {
      key: [47],
      value: [47],
      __typename: [47],
    },
    ProviderModelCapability: {},
    ProviderModelConfiguration: {
      cachedPrice: [47],
      capabilities: [29],
      catalogRef: [24],
      effortMapping: [26],
      enabled: [0],
      inputPrice: [47],
      maxContextTokens: [2],
      name: [47],
      outputPrice: [47],
      path: [47],
      protocol: [48],
      __typename: [47],
    },
    ProviderModelWrite: {
      configuration: [30],
      id: [15],
      __typename: [47],
    },
    ProviderPatchInput: {
      apiKey: [47],
      baseUrl: [47],
      headers: [28],
      models: [31],
      name: [47],
      __typename: [47],
    },
    ProviderReplaceInput: {
      apiKey: [47],
      baseUrl: [47],
      headers: [28],
      models: [31],
      name: [47],
      __typename: [47],
    },
    ProviderTemplate: {
      baseUrl: [47],
      id: [15],
      name: [47],
      protocol: [23],
      __typename: [47],
    },
    ProvidersCreateInput: {
      providers: [25],
      __typename: [47],
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
      session: [46],
      __typename: [47],
    },
    Role: {
      cedarSource: [47],
      color: [47],
      description: [47],
      id: [15],
      name: [47],
      __typename: [47],
    },
    RoleConnection: {
      edges: [40],
      pageInfo: [19],
      __typename: [47],
    },
    RoleCreateInput: {
      cedarSource: [47],
      description: [47],
      name: [47],
      __typename: [47],
    },
    RoleEdge: {
      cursor: [47],
      node: [37],
      __typename: [47],
    },
    RolePayload: {
      node: [37],
      __typename: [47],
    },
    RoleUpdateInput: {
      cedarSource: [47],
      description: [47],
      name: [47],
      __typename: [47],
    },
    Session: {
      id: [15],
      organizationId: [15],
      organizationName: [47],
      userId: [15],
      __typename: [47],
    },
    SessionCredentials: {
      accessLifetime: [49],
      accessToken: [47],
      refreshLifetime: [49],
      refreshToken: [47],
      __typename: [47],
    },
    SessionCredentialsPayload: {
      node: [44],
      __typename: [47],
    },
    SessionPayload: {
      node: [43],
      __typename: [47],
    },
    String: {},
    TextModelProtocol: {},
    TokenLifetime: {
      expiresAt: [47],
      expiresInSeconds: [47],
      issuedAt: [47],
      throttleSeconds: [47],
      __typename: [47],
    },
  },
}
