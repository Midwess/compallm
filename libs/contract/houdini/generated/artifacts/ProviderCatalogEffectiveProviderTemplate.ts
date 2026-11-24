import type { AuthenticationMethod$options, BillingModel$options, CacheBilling$options, ConfigurationInputKind$options, CredentialLocation$options, EndpointMode$options, ModelServiceKind$options, OAuthFlow$options, PricingMode$options, ProviderModelCapability$options, TextModelProtocol$options } from "$houdini/graphql/enums";
const artifact = {
    "name": "ProviderCatalogEffectiveProviderTemplate",
    "kind": "HoudiniQuery",
    "hash": "297d7379ffaa5b62f13c6b27e22541996ca43b260fb58cb13f24837d365ffdc0",
    "raw": `query ProviderCatalogEffectiveProviderTemplate($effectiveProviderTemplateId: ID!) {
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
                    __typename
                }
                authentication {
                    method
                    configuration {
                        placement {
                            location
                            name
                            prefix
                            __typename
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
                            __typename
                        }
                        adapter
                        parameters {
                            key
                            value
                            __typename
                        }
                        __typename
                    }
                    __typename
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
                            __typename
                        }
                        __typename
                    }
                    __typename
                }
                __typename
            }
            headers {
                key
                value
                __typename
            }
            authentication {
                method
                configuration {
                    placement {
                        location
                        name
                        prefix
                        __typename
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
                        __typename
                    }
                    adapter
                    parameters {
                        key
                        value
                        __typename
                    }
                    __typename
                }
                __typename
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
                            __typename
                        }
                        __typename
                    }
                    __typename
                }
                pageInfo {
                    hasNextPage
                    __typename
                }
                __typename
            }
            __typename
        }
        __typename
    }
}
`,

    "rootType": "Query",
    "stripVariables": [] as Array<string>,

    "selection": {
        "fields": {
            "effectiveProviderTemplate": {
                "type": "ProviderTemplatePayload",
                "keyRaw": "effectiveProviderTemplate(id: $effectiveProviderTemplateId)",
                "nullable": true,

                "selection": {
                    "fields": {
                        "node": {
                            "type": "ProviderTemplate",
                            "keyRaw": "node",

                            "selection": {
                                "fields": {
                                    "id": {
                                        "type": "ID",
                                        "keyRaw": "id",
                                        "visible": true,
                                    },

                                    "name": {
                                        "type": "String",
                                        "keyRaw": "name",
                                        "visible": true,
                                    },

                                    "baseUrl": {
                                        "type": "String",
                                        "keyRaw": "baseUrl",
                                        "nullable": true,
                                        "visible": true,
                                    },

                                    "logoUrl": {
                                        "type": "String",
                                        "keyRaw": "logoUrl",
                                        "nullable": true,
                                        "visible": true,
                                    },

                                    "description": {
                                        "type": "String",
                                        "keyRaw": "description",
                                        "nullable": true,
                                        "visible": true,
                                    },

                                    "connection": {
                                        "type": "ConnectionConfiguration",
                                        "keyRaw": "connection",

                                        "selection": {
                                            "fields": {
                                                "baseUrl": {
                                                    "type": "String",
                                                    "keyRaw": "baseUrl",
                                                    "nullable": true,
                                                    "visible": true,
                                                },

                                                "path": {
                                                    "type": "String",
                                                    "keyRaw": "path",
                                                    "nullable": true,
                                                    "visible": true,
                                                },

                                                "timeoutMs": {
                                                    "type": "Float",
                                                    "keyRaw": "timeoutMs",
                                                    "nullable": true,
                                                    "visible": true,
                                                },

                                                "query": {
                                                    "type": "ConfigurationEntry",
                                                    "keyRaw": "query",

                                                    "selection": {
                                                        "fields": {
                                                            "key": {
                                                                "type": "String",
                                                                "keyRaw": "key",
                                                                "visible": true,
                                                            },

                                                            "value": {
                                                                "type": "String",
                                                                "keyRaw": "value",
                                                                "visible": true,
                                                            },

                                                            "__typename": {
                                                                "type": "String",
                                                                "keyRaw": "__typename",
                                                            },
                                                        },
                                                    },

                                                    "visible": true,
                                                },

                                                "authentication": {
                                                    "type": "AuthenticationConfiguration",
                                                    "keyRaw": "authentication",
                                                    "nullable": true,

                                                    "selection": {
                                                        "fields": {
                                                            "method": {
                                                                "type": "AuthenticationMethod",
                                                                "keyRaw": "method",
                                                                "visible": true,
                                                            },

                                                            "configuration": {
                                                                "type": "AuthenticationParameters",
                                                                "keyRaw": "configuration",

                                                                "selection": {
                                                                    "fields": {
                                                                        "placement": {
                                                                            "type": "CredentialPlacement",
                                                                            "keyRaw": "placement",
                                                                            "nullable": true,

                                                                            "selection": {
                                                                                "fields": {
                                                                                    "location": {
                                                                                        "type": "CredentialLocation",
                                                                                        "keyRaw": "location",
                                                                                        "visible": true,
                                                                                    },

                                                                                    "name": {
                                                                                        "type": "String",
                                                                                        "keyRaw": "name",
                                                                                        "visible": true,
                                                                                    },

                                                                                    "prefix": {
                                                                                        "type": "String",
                                                                                        "keyRaw": "prefix",
                                                                                        "nullable": true,
                                                                                        "visible": true,
                                                                                    },

                                                                                    "__typename": {
                                                                                        "type": "String",
                                                                                        "keyRaw": "__typename",
                                                                                    },
                                                                                },
                                                                            },

                                                                            "visible": true,
                                                                        },

                                                                        "flow": {
                                                                            "type": "OAuthFlow",
                                                                            "keyRaw": "flow",
                                                                            "nullable": true,
                                                                            "visible": true,
                                                                        },

                                                                        "authorizationUrl": {
                                                                            "type": "String",
                                                                            "keyRaw": "authorizationUrl",
                                                                            "nullable": true,
                                                                            "visible": true,
                                                                        },

                                                                        "tokenUrl": {
                                                                            "type": "String",
                                                                            "keyRaw": "tokenUrl",
                                                                            "nullable": true,
                                                                            "visible": true,
                                                                        },

                                                                        "deviceAuthorizationUrl": {
                                                                            "type": "String",
                                                                            "keyRaw": "deviceAuthorizationUrl",
                                                                            "nullable": true,
                                                                            "visible": true,
                                                                        },

                                                                        "scopes": {
                                                                            "type": "String",
                                                                            "keyRaw": "scopes",
                                                                            "visible": true,
                                                                        },

                                                                        "requiredInputs": {
                                                                            "type": "ConfigurationInput",
                                                                            "keyRaw": "requiredInputs",

                                                                            "selection": {
                                                                                "fields": {
                                                                                    "name": {
                                                                                        "type": "String",
                                                                                        "keyRaw": "name",
                                                                                        "visible": true,
                                                                                    },

                                                                                    "kind": {
                                                                                        "type": "ConfigurationInputKind",
                                                                                        "keyRaw": "kind",
                                                                                        "visible": true,
                                                                                    },

                                                                                    "required": {
                                                                                        "type": "Boolean",
                                                                                        "keyRaw": "required",
                                                                                        "visible": true,
                                                                                    },

                                                                                    "allowedValues": {
                                                                                        "type": "String",
                                                                                        "keyRaw": "allowedValues",
                                                                                        "nullable": true,
                                                                                        "visible": true,
                                                                                    },

                                                                                    "__typename": {
                                                                                        "type": "String",
                                                                                        "keyRaw": "__typename",
                                                                                    },
                                                                                },
                                                                            },

                                                                            "visible": true,
                                                                        },

                                                                        "adapter": {
                                                                            "type": "String",
                                                                            "keyRaw": "adapter",
                                                                            "nullable": true,
                                                                            "visible": true,
                                                                        },

                                                                        "parameters": {
                                                                            "type": "ConfigurationEntry",
                                                                            "keyRaw": "parameters",

                                                                            "selection": {
                                                                                "fields": {
                                                                                    "key": {
                                                                                        "type": "String",
                                                                                        "keyRaw": "key",
                                                                                        "visible": true,
                                                                                    },

                                                                                    "value": {
                                                                                        "type": "String",
                                                                                        "keyRaw": "value",
                                                                                        "visible": true,
                                                                                    },

                                                                                    "__typename": {
                                                                                        "type": "String",
                                                                                        "keyRaw": "__typename",
                                                                                    },
                                                                                },
                                                                            },

                                                                            "visible": true,
                                                                        },

                                                                        "__typename": {
                                                                            "type": "String",
                                                                            "keyRaw": "__typename",
                                                                        },
                                                                    },
                                                                },

                                                                "visible": true,
                                                            },

                                                            "__typename": {
                                                                "type": "String",
                                                                "keyRaw": "__typename",
                                                            },
                                                        },
                                                    },

                                                    "visible": true,
                                                },

                                                "endpoint": {
                                                    "type": "EndpointRequirements",
                                                    "keyRaw": "endpoint",

                                                    "selection": {
                                                        "fields": {
                                                            "mode": {
                                                                "type": "EndpointMode",
                                                                "keyRaw": "mode",
                                                                "visible": true,
                                                            },

                                                            "configuration": {
                                                                "type": "EndpointConfiguration",
                                                                "keyRaw": "configuration",

                                                                "selection": {
                                                                    "fields": {
                                                                        "urlTemplate": {
                                                                            "type": "String",
                                                                            "keyRaw": "urlTemplate",
                                                                            "nullable": true,
                                                                            "visible": true,
                                                                        },

                                                                        "requiredInputs": {
                                                                            "type": "ConfigurationInput",
                                                                            "keyRaw": "requiredInputs",

                                                                            "selection": {
                                                                                "fields": {
                                                                                    "name": {
                                                                                        "type": "String",
                                                                                        "keyRaw": "name",
                                                                                        "visible": true,
                                                                                    },

                                                                                    "kind": {
                                                                                        "type": "ConfigurationInputKind",
                                                                                        "keyRaw": "kind",
                                                                                        "visible": true,
                                                                                    },

                                                                                    "required": {
                                                                                        "type": "Boolean",
                                                                                        "keyRaw": "required",
                                                                                        "visible": true,
                                                                                    },

                                                                                    "allowedValues": {
                                                                                        "type": "String",
                                                                                        "keyRaw": "allowedValues",
                                                                                        "nullable": true,
                                                                                        "visible": true,
                                                                                    },

                                                                                    "__typename": {
                                                                                        "type": "String",
                                                                                        "keyRaw": "__typename",
                                                                                    },
                                                                                },
                                                                            },

                                                                            "visible": true,
                                                                        },

                                                                        "__typename": {
                                                                            "type": "String",
                                                                            "keyRaw": "__typename",
                                                                        },
                                                                    },
                                                                },

                                                                "visible": true,
                                                            },

                                                            "__typename": {
                                                                "type": "String",
                                                                "keyRaw": "__typename",
                                                            },
                                                        },
                                                    },

                                                    "visible": true,
                                                },

                                                "__typename": {
                                                    "type": "String",
                                                    "keyRaw": "__typename",
                                                },
                                            },
                                        },

                                        "visible": true,
                                    },

                                    "headers": {
                                        "type": "ConfigurationEntry",
                                        "keyRaw": "headers",

                                        "selection": {
                                            "fields": {
                                                "key": {
                                                    "type": "String",
                                                    "keyRaw": "key",
                                                    "visible": true,
                                                },

                                                "value": {
                                                    "type": "String",
                                                    "keyRaw": "value",
                                                    "visible": true,
                                                },

                                                "__typename": {
                                                    "type": "String",
                                                    "keyRaw": "__typename",
                                                },
                                            },
                                        },

                                        "visible": true,
                                    },

                                    "authentication": {
                                        "type": "AuthenticationConfiguration",
                                        "keyRaw": "authentication",

                                        "selection": {
                                            "fields": {
                                                "method": {
                                                    "type": "AuthenticationMethod",
                                                    "keyRaw": "method",
                                                    "visible": true,
                                                },

                                                "configuration": {
                                                    "type": "AuthenticationParameters",
                                                    "keyRaw": "configuration",

                                                    "selection": {
                                                        "fields": {
                                                            "placement": {
                                                                "type": "CredentialPlacement",
                                                                "keyRaw": "placement",
                                                                "nullable": true,

                                                                "selection": {
                                                                    "fields": {
                                                                        "location": {
                                                                            "type": "CredentialLocation",
                                                                            "keyRaw": "location",
                                                                            "visible": true,
                                                                        },

                                                                        "name": {
                                                                            "type": "String",
                                                                            "keyRaw": "name",
                                                                            "visible": true,
                                                                        },

                                                                        "prefix": {
                                                                            "type": "String",
                                                                            "keyRaw": "prefix",
                                                                            "nullable": true,
                                                                            "visible": true,
                                                                        },

                                                                        "__typename": {
                                                                            "type": "String",
                                                                            "keyRaw": "__typename",
                                                                        },
                                                                    },
                                                                },

                                                                "visible": true,
                                                            },

                                                            "flow": {
                                                                "type": "OAuthFlow",
                                                                "keyRaw": "flow",
                                                                "nullable": true,
                                                                "visible": true,
                                                            },

                                                            "authorizationUrl": {
                                                                "type": "String",
                                                                "keyRaw": "authorizationUrl",
                                                                "nullable": true,
                                                                "visible": true,
                                                            },

                                                            "tokenUrl": {
                                                                "type": "String",
                                                                "keyRaw": "tokenUrl",
                                                                "nullable": true,
                                                                "visible": true,
                                                            },

                                                            "deviceAuthorizationUrl": {
                                                                "type": "String",
                                                                "keyRaw": "deviceAuthorizationUrl",
                                                                "nullable": true,
                                                                "visible": true,
                                                            },

                                                            "scopes": {
                                                                "type": "String",
                                                                "keyRaw": "scopes",
                                                                "visible": true,
                                                            },

                                                            "requiredInputs": {
                                                                "type": "ConfigurationInput",
                                                                "keyRaw": "requiredInputs",

                                                                "selection": {
                                                                    "fields": {
                                                                        "name": {
                                                                            "type": "String",
                                                                            "keyRaw": "name",
                                                                            "visible": true,
                                                                        },

                                                                        "kind": {
                                                                            "type": "ConfigurationInputKind",
                                                                            "keyRaw": "kind",
                                                                            "visible": true,
                                                                        },

                                                                        "required": {
                                                                            "type": "Boolean",
                                                                            "keyRaw": "required",
                                                                            "visible": true,
                                                                        },

                                                                        "allowedValues": {
                                                                            "type": "String",
                                                                            "keyRaw": "allowedValues",
                                                                            "nullable": true,
                                                                            "visible": true,
                                                                        },

                                                                        "__typename": {
                                                                            "type": "String",
                                                                            "keyRaw": "__typename",
                                                                        },
                                                                    },
                                                                },

                                                                "visible": true,
                                                            },

                                                            "adapter": {
                                                                "type": "String",
                                                                "keyRaw": "adapter",
                                                                "nullable": true,
                                                                "visible": true,
                                                            },

                                                            "parameters": {
                                                                "type": "ConfigurationEntry",
                                                                "keyRaw": "parameters",

                                                                "selection": {
                                                                    "fields": {
                                                                        "key": {
                                                                            "type": "String",
                                                                            "keyRaw": "key",
                                                                            "visible": true,
                                                                        },

                                                                        "value": {
                                                                            "type": "String",
                                                                            "keyRaw": "value",
                                                                            "visible": true,
                                                                        },

                                                                        "__typename": {
                                                                            "type": "String",
                                                                            "keyRaw": "__typename",
                                                                        },
                                                                    },
                                                                },

                                                                "visible": true,
                                                            },

                                                            "__typename": {
                                                                "type": "String",
                                                                "keyRaw": "__typename",
                                                            },
                                                        },
                                                    },

                                                    "visible": true,
                                                },

                                                "__typename": {
                                                    "type": "String",
                                                    "keyRaw": "__typename",
                                                },
                                            },
                                        },

                                        "visible": true,
                                    },

                                    "models": {
                                        "type": "ModelTemplateConnection",
                                        "keyRaw": "models",

                                        "selection": {
                                            "fields": {
                                                "edges": {
                                                    "type": "ModelTemplateEdge",
                                                    "keyRaw": "edges",

                                                    "selection": {
                                                        "fields": {
                                                            "node": {
                                                                "type": "ModelTemplate",
                                                                "keyRaw": "node",

                                                                "selection": {
                                                                    "fields": {
                                                                        "id": {
                                                                            "type": "ID",
                                                                            "keyRaw": "id",
                                                                            "visible": true,
                                                                        },

                                                                        "name": {
                                                                            "type": "String",
                                                                            "keyRaw": "name",
                                                                            "visible": true,
                                                                        },

                                                                        "displayName": {
                                                                            "type": "String",
                                                                            "keyRaw": "displayName",
                                                                            "visible": true,
                                                                        },

                                                                        "description": {
                                                                            "type": "String",
                                                                            "keyRaw": "description",
                                                                            "nullable": true,
                                                                            "visible": true,
                                                                        },

                                                                        "serviceKind": {
                                                                            "type": "ModelServiceKind",
                                                                            "keyRaw": "serviceKind",
                                                                            "nullable": true,
                                                                            "visible": true,
                                                                        },

                                                                        "capabilities": {
                                                                            "type": "ProviderModelCapability",
                                                                            "keyRaw": "capabilities",
                                                                            "visible": true,
                                                                        },

                                                                        "maxContextTokens": {
                                                                            "type": "ContextTokens",
                                                                            "keyRaw": "maxContextTokens",
                                                                            "nullable": true,
                                                                            "visible": true,
                                                                        },

                                                                        "maxInputTokens": {
                                                                            "type": "ContextTokens",
                                                                            "keyRaw": "maxInputTokens",
                                                                            "nullable": true,
                                                                            "visible": true,
                                                                        },

                                                                        "maxOutputTokens": {
                                                                            "type": "ContextTokens",
                                                                            "keyRaw": "maxOutputTokens",
                                                                            "nullable": true,
                                                                            "visible": true,
                                                                        },

                                                                        "inputPrice": {
                                                                            "type": "String",
                                                                            "keyRaw": "inputPrice",
                                                                            "nullable": true,
                                                                            "visible": true,
                                                                        },

                                                                        "outputPrice": {
                                                                            "type": "String",
                                                                            "keyRaw": "outputPrice",
                                                                            "nullable": true,
                                                                            "visible": true,
                                                                        },

                                                                        "cachedPrice": {
                                                                            "type": "String",
                                                                            "keyRaw": "cachedPrice",
                                                                            "nullable": true,
                                                                            "visible": true,
                                                                        },

                                                                        "pricingMode": {
                                                                            "type": "PricingMode",
                                                                            "keyRaw": "pricingMode",
                                                                            "visible": true,
                                                                        },

                                                                        "billingModel": {
                                                                            "type": "BillingModel",
                                                                            "keyRaw": "billingModel",
                                                                            "visible": true,
                                                                        },

                                                                        "cacheBilling": {
                                                                            "type": "CacheBilling",
                                                                            "keyRaw": "cacheBilling",
                                                                            "visible": true,
                                                                        },

                                                                        "operations": {
                                                                            "type": "ModelOperation",
                                                                            "keyRaw": "operations",

                                                                            "selection": {
                                                                                "fields": {
                                                                                    "operation": {
                                                                                        "type": "TextModelProtocol",
                                                                                        "keyRaw": "operation",
                                                                                        "visible": true,
                                                                                    },

                                                                                    "path": {
                                                                                        "type": "String",
                                                                                        "keyRaw": "path",
                                                                                        "visible": true,
                                                                                    },

                                                                                    "__typename": {
                                                                                        "type": "String",
                                                                                        "keyRaw": "__typename",
                                                                                    },
                                                                                },
                                                                            },

                                                                            "visible": true,
                                                                        },

                                                                        "__typename": {
                                                                            "type": "String",
                                                                            "keyRaw": "__typename",
                                                                        },
                                                                    },
                                                                },

                                                                "visible": true,
                                                            },

                                                            "__typename": {
                                                                "type": "String",
                                                                "keyRaw": "__typename",
                                                            },
                                                        },
                                                    },

                                                    "visible": true,
                                                },

                                                "pageInfo": {
                                                    "type": "PageInfo",
                                                    "keyRaw": "pageInfo",

                                                    "selection": {
                                                        "fields": {
                                                            "hasNextPage": {
                                                                "type": "Boolean",
                                                                "keyRaw": "hasNextPage",
                                                                "visible": true,
                                                            },

                                                            "__typename": {
                                                                "type": "String",
                                                                "keyRaw": "__typename",
                                                            },
                                                        },
                                                    },

                                                    "visible": true,
                                                },

                                                "__typename": {
                                                    "type": "String",
                                                    "keyRaw": "__typename",
                                                },
                                            },
                                        },

                                        "visible": true,
                                    },

                                    "__typename": {
                                        "type": "String",
                                        "keyRaw": "__typename",
                                    },
                                },
                            },

                            "visible": true,
                        },

                        "__typename": {
                            "type": "String",
                            "keyRaw": "__typename",
                        },
                    },
                },

                "visible": true,
            },
        },
    },

    "pluginData": {},

    "input": {
        "fields": {
            "effectiveProviderTemplateId": "ID",
        },

        "types": {},

        "defaults": {},

        "runtimeScalars": {},
    },

    "policy": "CacheOrNetwork",
    "partial": false
} as const

export default artifact

export type ProviderCatalogEffectiveProviderTemplate = {
	readonly "input": ProviderCatalogEffectiveProviderTemplate$input;
	readonly "result": ProviderCatalogEffectiveProviderTemplate$result | undefined;
};

export type ProviderCatalogEffectiveProviderTemplate$result = {
	readonly effectiveProviderTemplate: {
		readonly node: {
			readonly id: string;
			readonly name: string;
			readonly baseUrl: string | null;
			readonly logoUrl: string | null;
			readonly description: string | null;
			readonly connection: {
				readonly baseUrl: string | null;
				readonly path: string | null;
				readonly timeoutMs: number | null;
				readonly query: ({
					readonly key: string;
					readonly value: string;
				})[];
				readonly authentication: {
					readonly method: AuthenticationMethod$options;
					readonly configuration: {
						readonly placement: {
							readonly location: CredentialLocation$options;
							readonly name: string;
							readonly prefix: string | null;
						} | null;
						readonly flow: OAuthFlow$options | null;
						readonly authorizationUrl: string | null;
						readonly tokenUrl: string | null;
						readonly deviceAuthorizationUrl: string | null;
						readonly scopes: (string)[];
						readonly requiredInputs: ({
							readonly name: string;
							readonly kind: ConfigurationInputKind$options;
							readonly required: boolean;
							readonly allowedValues: (string)[] | null;
						})[];
						readonly adapter: string | null;
						readonly parameters: ({
							readonly key: string;
							readonly value: string;
						})[];
					};
				} | null;
				readonly endpoint: {
					readonly mode: EndpointMode$options;
					readonly configuration: {
						readonly urlTemplate: string | null;
						readonly requiredInputs: ({
							readonly name: string;
							readonly kind: ConfigurationInputKind$options;
							readonly required: boolean;
							readonly allowedValues: (string)[] | null;
						})[];
					};
				};
			};
			readonly headers: ({
				readonly key: string;
				readonly value: string;
			})[];
			readonly authentication: ({
				readonly method: AuthenticationMethod$options;
				readonly configuration: {
					readonly placement: {
						readonly location: CredentialLocation$options;
						readonly name: string;
						readonly prefix: string | null;
					} | null;
					readonly flow: OAuthFlow$options | null;
					readonly authorizationUrl: string | null;
					readonly tokenUrl: string | null;
					readonly deviceAuthorizationUrl: string | null;
					readonly scopes: (string)[];
					readonly requiredInputs: ({
						readonly name: string;
						readonly kind: ConfigurationInputKind$options;
						readonly required: boolean;
						readonly allowedValues: (string)[] | null;
					})[];
					readonly adapter: string | null;
					readonly parameters: ({
						readonly key: string;
						readonly value: string;
					})[];
				};
			})[];
			readonly models: {
				readonly edges: ({
					readonly node: {
						readonly id: string;
						readonly name: string;
						readonly displayName: string;
						readonly description: string | null;
						readonly serviceKind: ModelServiceKind$options | null;
						readonly capabilities: (ProviderModelCapability$options)[];
						readonly maxContextTokens: number | null;
						readonly maxInputTokens: number | null;
						readonly maxOutputTokens: number | null;
						readonly inputPrice: string | null;
						readonly outputPrice: string | null;
						readonly cachedPrice: string | null;
						readonly pricingMode: PricingMode$options;
						readonly billingModel: BillingModel$options;
						readonly cacheBilling: CacheBilling$options;
						readonly operations: ({
							readonly operation: TextModelProtocol$options;
							readonly path: string;
						})[];
					};
				})[];
				readonly pageInfo: {
					readonly hasNextPage: boolean;
				};
			};
		};
	} | null;
};

export type ProviderCatalogEffectiveProviderTemplate$input = {
	effectiveProviderTemplateId: string;
};

export type ProviderCatalogEffectiveProviderTemplate$unmasked = {
	readonly effectiveProviderTemplate: {
		readonly node: {
			readonly id: string;
			readonly name: string;
			readonly baseUrl: string | null;
			readonly logoUrl: string | null;
			readonly description: string | null;
			readonly connection: {
				readonly baseUrl: string | null;
				readonly path: string | null;
				readonly timeoutMs: number | null;
				readonly query: ({
					readonly key: string;
					readonly value: string;
					readonly __typename: "ConfigurationEntry";
				})[];
				readonly authentication: {
					readonly method: AuthenticationMethod$options;
					readonly configuration: {
						readonly placement: {
							readonly location: CredentialLocation$options;
							readonly name: string;
							readonly prefix: string | null;
							readonly __typename: "CredentialPlacement";
						} | null;
						readonly flow: OAuthFlow$options | null;
						readonly authorizationUrl: string | null;
						readonly tokenUrl: string | null;
						readonly deviceAuthorizationUrl: string | null;
						readonly scopes: (string)[];
						readonly requiredInputs: ({
							readonly name: string;
							readonly kind: ConfigurationInputKind$options;
							readonly required: boolean;
							readonly allowedValues: (string)[] | null;
							readonly __typename: "ConfigurationInput";
						})[];
						readonly adapter: string | null;
						readonly parameters: ({
							readonly key: string;
							readonly value: string;
							readonly __typename: "ConfigurationEntry";
						})[];
						readonly __typename: "AuthenticationParameters";
					};
					readonly __typename: "AuthenticationConfiguration";
				} | null;
				readonly endpoint: {
					readonly mode: EndpointMode$options;
					readonly configuration: {
						readonly urlTemplate: string | null;
						readonly requiredInputs: ({
							readonly name: string;
							readonly kind: ConfigurationInputKind$options;
							readonly required: boolean;
							readonly allowedValues: (string)[] | null;
							readonly __typename: "ConfigurationInput";
						})[];
						readonly __typename: "EndpointConfiguration";
					};
					readonly __typename: "EndpointRequirements";
				};
				readonly __typename: "ConnectionConfiguration";
			};
			readonly headers: ({
				readonly key: string;
				readonly value: string;
				readonly __typename: "ConfigurationEntry";
			})[];
			readonly authentication: ({
				readonly method: AuthenticationMethod$options;
				readonly configuration: {
					readonly placement: {
						readonly location: CredentialLocation$options;
						readonly name: string;
						readonly prefix: string | null;
						readonly __typename: "CredentialPlacement";
					} | null;
					readonly flow: OAuthFlow$options | null;
					readonly authorizationUrl: string | null;
					readonly tokenUrl: string | null;
					readonly deviceAuthorizationUrl: string | null;
					readonly scopes: (string)[];
					readonly requiredInputs: ({
						readonly name: string;
						readonly kind: ConfigurationInputKind$options;
						readonly required: boolean;
						readonly allowedValues: (string)[] | null;
						readonly __typename: "ConfigurationInput";
					})[];
					readonly adapter: string | null;
					readonly parameters: ({
						readonly key: string;
						readonly value: string;
						readonly __typename: "ConfigurationEntry";
					})[];
					readonly __typename: "AuthenticationParameters";
				};
				readonly __typename: "AuthenticationConfiguration";
			})[];
			readonly models: {
				readonly edges: ({
					readonly node: {
						readonly id: string;
						readonly name: string;
						readonly displayName: string;
						readonly description: string | null;
						readonly serviceKind: ModelServiceKind$options | null;
						readonly capabilities: (ProviderModelCapability$options)[];
						readonly maxContextTokens: number | null;
						readonly maxInputTokens: number | null;
						readonly maxOutputTokens: number | null;
						readonly inputPrice: string | null;
						readonly outputPrice: string | null;
						readonly cachedPrice: string | null;
						readonly pricingMode: PricingMode$options;
						readonly billingModel: BillingModel$options;
						readonly cacheBilling: CacheBilling$options;
						readonly operations: ({
							readonly operation: TextModelProtocol$options;
							readonly path: string;
							readonly __typename: "ModelOperation";
						})[];
						readonly __typename: "ModelTemplate";
					};
					readonly __typename: "ModelTemplateEdge";
				})[];
				readonly pageInfo: {
					readonly hasNextPage: boolean;
					readonly __typename: "PageInfo";
				};
				readonly __typename: "ModelTemplateConnection";
			};
			readonly __typename: "ProviderTemplate";
		};
		readonly __typename: "ProviderTemplatePayload";
	} | null;
};

export type ProviderCatalogEffectiveProviderTemplate$artifact = typeof artifact

"HoudiniHash=297d7379ffaa5b62f13c6b27e22541996ca43b260fb58cb13f24837d365ffdc0"
