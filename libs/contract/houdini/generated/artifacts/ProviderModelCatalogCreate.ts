import type { AuthenticationMethod$options, CacheBilling$options, ConfigurationInputKind$options, CredentialLocation$options, EndpointMode$options, ModelCatalogOrigin$options, OAuthFlow$options, PricingMode$options, ProviderModelCapability$options, ThinkingEffortLevel$options, UpstreamProtocol$options } from "$houdini/graphql/enums";
import type { ModelCatalogCreateInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "ProviderModelCatalogCreate",
    "kind": "HoudiniMutation",
    "hash": "3d3587adbdba4952ebad6cf937433d0159247e421a029fe8a2d40ff7c5514f1d",
    "raw": `mutation ProviderModelCatalogCreate($modelCatalogCreateInput: ModelCatalogCreateInput!, $modelCatalogCreateProviderCatalogId: ID!) {
    modelCatalogCreate(providerCatalogId: $modelCatalogCreateProviderCatalogId, input: $modelCatalogCreateInput) {
        node {
            id
            providerCatalogId
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
                __typename
            }
            headers {
                key
                value
                __typename
            }
            bodyDefaults {
                key
                value
                __typename
            }
            text {
                input {
                    configuration {
                        maxTokens
                        __typename
                    }
                    __typename
                }
                output {
                    configuration {
                        maxTokens
                        thinking {
                            configuration {
                                effortMapping {
                                    level
                                    value
                                    __typename
                                }
                                __typename
                            }
                            __typename
                        }
                        __typename
                    }
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

    "rootType": "Mutation",
    "stripVariables": [] as Array<string>,

    "selection": {
        "fields": {
            "modelCatalogCreate": {
                "type": "ModelCatalogPayload",
                "keyRaw": "modelCatalogCreate(providerCatalogId: $modelCatalogCreateProviderCatalogId, input: $modelCatalogCreateInput)",

                "selection": {
                    "fields": {
                        "node": {
                            "type": "ModelCatalog",
                            "keyRaw": "node",

                            "selection": {
                                "fields": {
                                    "id": {
                                        "type": "ID",
                                        "keyRaw": "id",
                                        "visible": true,
                                    },

                                    "providerCatalogId": {
                                        "type": "ID",
                                        "keyRaw": "providerCatalogId",
                                        "visible": true,
                                    },

                                    "origin": {
                                        "type": "ModelCatalogOrigin",
                                        "keyRaw": "origin",
                                        "visible": true,
                                    },

                                    "customFields": {
                                        "type": "String",
                                        "keyRaw": "customFields",
                                        "visible": true,
                                    },

                                    "name": {
                                        "type": "String",
                                        "keyRaw": "name",
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

                                    "cacheBilling": {
                                        "type": "CacheBilling",
                                        "keyRaw": "cacheBilling",
                                        "visible": true,
                                    },

                                    "connections": {
                                        "type": "ModelConnection",
                                        "keyRaw": "connections",

                                        "selection": {
                                            "fields": {
                                                "protocol": {
                                                    "type": "UpstreamProtocol",
                                                    "keyRaw": "protocol",
                                                    "visible": true,
                                                },

                                                "configuration": {
                                                    "type": "ConnectionConfiguration",
                                                    "keyRaw": "configuration",

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

                                    "bodyDefaults": {
                                        "type": "BodyDefaultEntry",
                                        "keyRaw": "bodyDefaults",

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

                                    "text": {
                                        "type": "TextCapabilities",
                                        "keyRaw": "text",

                                        "selection": {
                                            "fields": {
                                                "input": {
                                                    "type": "TextInputSupport",
                                                    "keyRaw": "input",

                                                    "selection": {
                                                        "fields": {
                                                            "configuration": {
                                                                "type": "TextInputConfiguration",
                                                                "keyRaw": "configuration",
                                                                "nullable": true,

                                                                "selection": {
                                                                    "fields": {
                                                                        "maxTokens": {
                                                                            "type": "Float",
                                                                            "keyRaw": "maxTokens",
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

                                                "output": {
                                                    "type": "TextGenerationSupport",
                                                    "keyRaw": "output",

                                                    "selection": {
                                                        "fields": {
                                                            "configuration": {
                                                                "type": "TextGenerationConfiguration",
                                                                "keyRaw": "configuration",
                                                                "nullable": true,

                                                                "selection": {
                                                                    "fields": {
                                                                        "maxTokens": {
                                                                            "type": "Float",
                                                                            "keyRaw": "maxTokens",
                                                                            "nullable": true,
                                                                            "visible": true,
                                                                        },

                                                                        "thinking": {
                                                                            "type": "ThinkingSupport",
                                                                            "keyRaw": "thinking",

                                                                            "selection": {
                                                                                "fields": {
                                                                                    "configuration": {
                                                                                        "type": "ThinkingConfiguration",
                                                                                        "keyRaw": "configuration",
                                                                                        "nullable": true,

                                                                                        "selection": {
                                                                                            "fields": {
                                                                                                "effortMapping": {
                                                                                                    "type": "ThinkingEffort",
                                                                                                    "keyRaw": "effortMapping",

                                                                                                    "selection": {
                                                                                                        "fields": {
                                                                                                            "level": {
                                                                                                                "type": "ThinkingEffortLevel",
                                                                                                                "keyRaw": "level",
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
            "modelCatalogCreateProviderCatalogId": "ID",
            "modelCatalogCreateInput": "ModelCatalogCreateInput",
        },

        "types": {
            "ModelCatalogCreateInput": {
                "configuration": "ModelCatalogCustomizeInput",
            },
            "ModelCatalogCustomizeInput": {
                "bodyDefaults": "ProviderBodyDefault",
                "cachedPrice": "ModelCatalogPriceInput",
                "capabilities": "ProviderModelCapability",
                "effortMapping": "ProviderEffortMapping",
                "headers": "ProviderHeader",
                "inputPrice": "ModelCatalogPriceInput",
                "limits": "ModelCatalogLimitsInput",
                "name": "String",
                "outputPrice": "ModelCatalogPriceInput",
                "path": "String",
                "protocol": "TextModelProtocol",
            },
            "ModelCatalogLimitsInput": {
                "context": "ContextTokens",
                "input": "ContextTokens",
                "output": "ContextTokens",
            },
            "ModelCatalogPriceInput": {
                "enabled": "Boolean",
                "value": "String",
            },
            "ProviderBodyDefault": {
                "key": "String",
                "value": "String",
            },
            "ProviderEffortMapping": {
                "high": "String",
                "low": "String",
                "max": "String",
                "medium": "String",
                "xhigh": "String",
            },
            "ProviderHeader": {
                "key": "String",
                "value": "String",
            },
        },

        "defaults": {},

        "runtimeScalars": {},
    },

} as const

export default artifact

export type ProviderModelCatalogCreate = {
	readonly "input": ProviderModelCatalogCreate$input;
	readonly "result": ProviderModelCatalogCreate$result;
};

export type ProviderModelCatalogCreate$result = {
	readonly modelCatalogCreate: {
		readonly node: {
			readonly id: string;
			readonly providerCatalogId: string;
			readonly origin: ModelCatalogOrigin$options;
			readonly customFields: (string)[];
			readonly name: string;
			readonly capabilities: (ProviderModelCapability$options)[];
			readonly maxContextTokens: number | null;
			readonly inputPrice: string | null;
			readonly outputPrice: string | null;
			readonly cachedPrice: string | null;
			readonly pricingMode: PricingMode$options;
			readonly cacheBilling: CacheBilling$options;
			readonly connections: ({
				readonly protocol: UpstreamProtocol$options;
				readonly configuration: {
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
			})[];
			readonly headers: ({
				readonly key: string;
				readonly value: string;
			})[];
			readonly bodyDefaults: ({
				readonly key: string;
				readonly value: string;
			})[];
			readonly text: {
				readonly input: {
					readonly configuration: {
						readonly maxTokens: number | null;
					} | null;
				};
				readonly output: {
					readonly configuration: {
						readonly maxTokens: number | null;
						readonly thinking: {
							readonly configuration: {
								readonly effortMapping: ({
									readonly level: ThinkingEffortLevel$options;
									readonly value: string;
								})[];
							} | null;
						};
					} | null;
				};
			};
		};
	};
};

export type ProviderModelCatalogCreate$input = {
	modelCatalogCreateProviderCatalogId: string;
	modelCatalogCreateInput: ModelCatalogCreateInput;
};

export type ProviderModelCatalogCreate$optimistic = {
	readonly modelCatalogCreate?: {
		readonly node?: {
			readonly id?: string;
			readonly providerCatalogId?: string;
			readonly origin?: ModelCatalogOrigin$options;
			readonly customFields?: (string)[];
			readonly name?: string;
			readonly capabilities?: (ProviderModelCapability$options)[];
			readonly maxContextTokens?: number | null;
			readonly inputPrice?: string | null;
			readonly outputPrice?: string | null;
			readonly cachedPrice?: string | null;
			readonly pricingMode?: PricingMode$options;
			readonly cacheBilling?: CacheBilling$options;
			readonly connections?: ({
				readonly protocol?: UpstreamProtocol$options;
				readonly configuration?: {
					readonly baseUrl?: string | null;
					readonly path?: string | null;
					readonly timeoutMs?: number | null;
					readonly query?: ({
						readonly key?: string;
						readonly value?: string;
					})[];
					readonly authentication?: {
						readonly method?: AuthenticationMethod$options;
						readonly configuration?: {
							readonly placement?: {
								readonly location?: CredentialLocation$options;
								readonly name?: string;
								readonly prefix?: string | null;
							} | null;
							readonly flow?: OAuthFlow$options | null;
							readonly authorizationUrl?: string | null;
							readonly tokenUrl?: string | null;
							readonly deviceAuthorizationUrl?: string | null;
							readonly scopes?: (string)[];
							readonly requiredInputs?: ({
								readonly name?: string;
								readonly kind?: ConfigurationInputKind$options;
								readonly required?: boolean;
								readonly allowedValues?: (string)[] | null;
							})[];
							readonly adapter?: string | null;
							readonly parameters?: ({
								readonly key?: string;
								readonly value?: string;
							})[];
						};
					} | null;
					readonly endpoint?: {
						readonly mode?: EndpointMode$options;
						readonly configuration?: {
							readonly urlTemplate?: string | null;
							readonly requiredInputs?: ({
								readonly name?: string;
								readonly kind?: ConfigurationInputKind$options;
								readonly required?: boolean;
								readonly allowedValues?: (string)[] | null;
							})[];
						};
					};
				};
			})[];
			readonly headers?: ({
				readonly key?: string;
				readonly value?: string;
			})[];
			readonly bodyDefaults?: ({
				readonly key?: string;
				readonly value?: string;
			})[];
			readonly text?: {
				readonly input?: {
					readonly configuration?: {
						readonly maxTokens?: number | null;
					} | null;
				};
				readonly output?: {
					readonly configuration?: {
						readonly maxTokens?: number | null;
						readonly thinking?: {
							readonly configuration?: {
								readonly effortMapping?: ({
									readonly level?: ThinkingEffortLevel$options;
									readonly value?: string;
								})[];
							} | null;
						};
					} | null;
				};
			};
		};
	};
};

export type ProviderModelCatalogCreate$unmasked = {
	readonly modelCatalogCreate: {
		readonly node: {
			readonly id: string;
			readonly providerCatalogId: string;
			readonly origin: ModelCatalogOrigin$options;
			readonly customFields: (string)[];
			readonly name: string;
			readonly capabilities: (ProviderModelCapability$options)[];
			readonly maxContextTokens: number | null;
			readonly inputPrice: string | null;
			readonly outputPrice: string | null;
			readonly cachedPrice: string | null;
			readonly pricingMode: PricingMode$options;
			readonly cacheBilling: CacheBilling$options;
			readonly connections: ({
				readonly protocol: UpstreamProtocol$options;
				readonly configuration: {
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
				readonly __typename: "ModelConnection";
			})[];
			readonly headers: ({
				readonly key: string;
				readonly value: string;
				readonly __typename: "ConfigurationEntry";
			})[];
			readonly bodyDefaults: ({
				readonly key: string;
				readonly value: string;
				readonly __typename: "BodyDefaultEntry";
			})[];
			readonly text: {
				readonly input: {
					readonly configuration: {
						readonly maxTokens: number | null;
						readonly __typename: "TextInputConfiguration";
					} | null;
					readonly __typename: "TextInputSupport";
				};
				readonly output: {
					readonly configuration: {
						readonly maxTokens: number | null;
						readonly thinking: {
							readonly configuration: {
								readonly effortMapping: ({
									readonly level: ThinkingEffortLevel$options;
									readonly value: string;
									readonly __typename: "ThinkingEffort";
								})[];
								readonly __typename: "ThinkingConfiguration";
							} | null;
							readonly __typename: "ThinkingSupport";
						};
						readonly __typename: "TextGenerationConfiguration";
					} | null;
					readonly __typename: "TextGenerationSupport";
				};
				readonly __typename: "TextCapabilities";
			};
			readonly __typename: "ModelCatalog";
		};
		readonly __typename: "ModelCatalogPayload";
	};
};

export type ProviderModelCatalogCreate$artifact = typeof artifact

"HoudiniHash=3d3587adbdba4952ebad6cf937433d0159247e421a029fe8a2d40ff7c5514f1d"
