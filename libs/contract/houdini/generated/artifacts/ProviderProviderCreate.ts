import type { ProviderCreateFromCatalogInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "ProviderProviderCreate",
    "kind": "HoudiniMutation",
    "hash": "a2c7c34e761bd023648cf700b5538c22e373bd90030b973add43d480b99578cc",
    "raw": `mutation ProviderProviderCreate($providerCreateInput: ProviderCreateFromCatalogInput!) {
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
            "providerCreateInput": "ProviderCreateFromCatalogInput",
        },

        "types": {
            "ProviderCatalogModelInput": {
                "id": "String",
                "protocol": "TextModelProtocol",
            },
            "ProviderCreateFromCatalogInput": {
                "apiKey": "String",
                "baseUrl": "String",
                "headers": "ProviderHeader",
                "logoUploadId": "String",
                "logoUrl": "String",
                "models": "ProviderCatalogModelInput",
                "name": "String",
                "providerCatalogId": "String",
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
	providerCreateInput: ProviderCreateFromCatalogInput;
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

"HoudiniHash=a2c7c34e761bd023648cf700b5538c22e373bd90030b973add43d480b99578cc"
