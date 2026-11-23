import type { ProviderModelCapability$options, UpstreamProtocol$options } from "$houdini/graphql/enums";
const artifact = {
    "name": "ProviderProvider",
    "kind": "HoudiniQuery",
    "hash": "363dd539854087483c6112a03371b267bad3500069ffd450850a5a41d8af35fe",
    "raw": `query ProviderProvider($providerId: ID!) {
    provider(id: $providerId) {
        node {
            id
            name
            description
            logoUrl
            connection {
                baseUrl
                __typename
            }
            models {
                id
                name
                enabled
                capabilities
                inputPrice
                outputPrice
                cachedPrice
                protocol
                connection {
                    path
                    __typename
                }
                contextTokens
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

    "rootType": "Query",
    "stripVariables": [] as Array<string>,

    "selection": {
        "fields": {
            "provider": {
                "type": "ProviderPayload",
                "keyRaw": "provider(id: $providerId)",
                "nullable": true,

                "selection": {
                    "fields": {
                        "node": {
                            "type": "Provider",
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

                                    "description": {
                                        "type": "String",
                                        "keyRaw": "description",
                                        "nullable": true,
                                        "visible": true,
                                    },

                                    "logoUrl": {
                                        "type": "String",
                                        "keyRaw": "logoUrl",
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

                                                "__typename": {
                                                    "type": "String",
                                                    "keyRaw": "__typename",
                                                },
                                            },
                                        },

                                        "visible": true,
                                    },

                                    "models": {
                                        "type": "ProviderModel",
                                        "keyRaw": "models",

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

                                                "enabled": {
                                                    "type": "Boolean",
                                                    "keyRaw": "enabled",
                                                    "visible": true,
                                                },

                                                "capabilities": {
                                                    "type": "ProviderModelCapability",
                                                    "keyRaw": "capabilities",
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

                                                "protocol": {
                                                    "type": "UpstreamProtocol",
                                                    "keyRaw": "protocol",
                                                    "visible": true,
                                                },

                                                "connection": {
                                                    "type": "ConnectionConfiguration",
                                                    "keyRaw": "connection",

                                                    "selection": {
                                                        "fields": {
                                                            "path": {
                                                                "type": "String",
                                                                "keyRaw": "path",
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

                                                "contextTokens": {
                                                    "type": "ContextTokens",
                                                    "keyRaw": "contextTokens",
                                                    "nullable": true,
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
            "providerId": "ID",
        },

        "types": {},

        "defaults": {},

        "runtimeScalars": {},
    },

    "policy": "CacheOrNetwork",
    "partial": false
} as const

export default artifact

export type ProviderProvider = {
	readonly "input": ProviderProvider$input;
	readonly "result": ProviderProvider$result | undefined;
};

export type ProviderProvider$result = {
	readonly provider: {
		readonly node: {
			/**
			 * Stable provider identity, serialized as a string.
			 */
			readonly id: string;
			/**
			 * Provider display name within its organization.
			 */
			readonly name: string;
			readonly description: string | null;
			readonly logoUrl: string | null;
			readonly connection: {
				readonly baseUrl: string | null;
			};
			readonly models: ({
				/**
				 * Stable configured-model identity, serialized as a string.
				 */
				readonly id: string;
				/**
				 * Configured upstream model name.
				 */
				readonly name: string;
				/**
				 * Configured availability; this flag alone does not authorize invocation.
				 */
				readonly enabled: boolean;
				readonly capabilities: (ProviderModelCapability$options)[];
				readonly inputPrice: string | null;
				readonly outputPrice: string | null;
				readonly cachedPrice: string | null;
				readonly protocol: UpstreamProtocol$options;
				readonly connection: {
					readonly path: string | null;
				};
				readonly contextTokens: number | null;
				readonly text: {
					readonly input: {
						readonly configuration: {
							readonly maxTokens: number | null;
						} | null;
					};
					readonly output: {
						readonly configuration: {
							readonly maxTokens: number | null;
						} | null;
					};
				};
			})[];
		};
	} | null;
};

export type ProviderProvider$input = {
	providerId: string;
};

export type ProviderProvider$unmasked = {
	readonly provider: {
		readonly node: {
			/**
			 * Stable provider identity, serialized as a string.
			 */
			readonly id: string;
			/**
			 * Provider display name within its organization.
			 */
			readonly name: string;
			readonly description: string | null;
			readonly logoUrl: string | null;
			readonly connection: {
				readonly baseUrl: string | null;
				readonly __typename: "ConnectionConfiguration";
			};
			readonly models: ({
				/**
				 * Stable configured-model identity, serialized as a string.
				 */
				readonly id: string;
				/**
				 * Configured upstream model name.
				 */
				readonly name: string;
				/**
				 * Configured availability; this flag alone does not authorize invocation.
				 */
				readonly enabled: boolean;
				readonly capabilities: (ProviderModelCapability$options)[];
				readonly inputPrice: string | null;
				readonly outputPrice: string | null;
				readonly cachedPrice: string | null;
				readonly protocol: UpstreamProtocol$options;
				readonly connection: {
					readonly path: string | null;
					readonly __typename: "ConnectionConfiguration";
				};
				readonly contextTokens: number | null;
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
							readonly __typename: "TextGenerationConfiguration";
						} | null;
						readonly __typename: "TextGenerationSupport";
					};
					readonly __typename: "TextCapabilities";
				};
				readonly __typename: "ProviderModel";
			})[];
			readonly __typename: "Provider";
		};
		readonly __typename: "ProviderPayload";
	} | null;
};

export type ProviderProvider$artifact = typeof artifact

"HoudiniHash=363dd539854087483c6112a03371b267bad3500069ffd450850a5a41d8af35fe"
