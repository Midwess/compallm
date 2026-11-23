import type { ProviderCreateFromTemplateInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "ProviderProviderCreate",
    "kind": "HoudiniMutation",
    "hash": "be7481b115fd124ba4bbb59fad93967376c1a038c5087a39f32270e41704cbcd",
    "raw": `mutation ProviderProviderCreate($providerCreateInput: ProviderCreateFromTemplateInput!) {
    providerCreate(input: $providerCreateInput) {
        node {
            id
            name
            logoUrl
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
            "providerCreate": {
                "type": "ProviderPayload",
                "keyRaw": "providerCreate(input: $providerCreateInput)",

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

                                    "logoUrl": {
                                        "type": "String",
                                        "keyRaw": "logoUrl",
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
        },
    },

    "pluginData": {},

    "input": {
        "fields": {
            "providerCreateInput": "ProviderCreateFromTemplateInput",
        },

        "types": {
            "ProviderCreateFromTemplateInput": {
                "apiKey": "String",
                "baseUrl": "String",
                "headers": "ProviderHeader",
                "logoUploadId": "String",
                "logoUrl": "String",
                "models": "ProviderTemplateModelInput",
                "name": "String",
                "providerTemplateId": "String",
            },
            "ProviderHeader": {
                "key": "String",
                "value": "String",
            },
            "ProviderTemplateModelInput": {
                "id": "String",
                "protocol": "TextModelProtocol",
            },
        },

        "defaults": {},

        "runtimeScalars": {},
    },

} as const

export default artifact

export type ProviderProviderCreate = {
	readonly "input": ProviderProviderCreate$input;
	readonly "result": ProviderProviderCreate$result;
};

export type ProviderProviderCreate$result = {
	readonly providerCreate: {
		readonly node: {
			/**
			 * Stable provider identity, serialized as a string.
			 */
			readonly id: string;
			/**
			 * Provider display name within its organization.
			 */
			readonly name: string;
			readonly logoUrl: string | null;
		};
	};
};

export type ProviderProviderCreate$input = {
	providerCreateInput: ProviderCreateFromTemplateInput;
};

export type ProviderProviderCreate$optimistic = {
	readonly providerCreate?: {
		readonly node?: {
			/**
			 * Stable provider identity, serialized as a string.
			 */
			readonly id?: string;
			/**
			 * Provider display name within its organization.
			 */
			readonly name?: string;
			readonly logoUrl?: string | null;
		};
	};
};

export type ProviderProviderCreate$unmasked = {
	readonly providerCreate: {
		readonly node: {
			/**
			 * Stable provider identity, serialized as a string.
			 */
			readonly id: string;
			/**
			 * Provider display name within its organization.
			 */
			readonly name: string;
			readonly logoUrl: string | null;
			readonly __typename: "Provider";
		};
		readonly __typename: "ProviderPayload";
	};
};

export type ProviderProviderCreate$artifact = typeof artifact

"HoudiniHash=be7481b115fd124ba4bbb59fad93967376c1a038c5087a39f32270e41704cbcd"
