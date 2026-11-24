const artifact = {
    "name": "OrganizationProviders",
    "kind": "HoudiniQuery",
    "hash": "1aa04be4c5e360a18517da1045a0748f9a81866ab170ef58613a97cd44a13a01",
    "raw": `query OrganizationProviders($providersAfter: String) {
    providers(first: 100, after: $providersAfter) {
        edges {
            node {
                id
                name
                __typename
            }
            __typename
        }
        pageInfo {
            endCursor
            hasNextPage
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
            "providers": {
                "type": "ProviderConnection",
                "keyRaw": "providers(first: 100, after: $providersAfter)",

                "selection": {
                    "fields": {
                        "edges": {
                            "type": "ProviderEdge",
                            "keyRaw": "edges",

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
                                    "endCursor": {
                                        "type": "String",
                                        "keyRaw": "endCursor",
                                        "nullable": true,
                                        "visible": true,
                                    },

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
        },
    },

    "pluginData": {},

    "input": {
        "fields": {
            "providersAfter": "String",
        },

        "types": {},

        "defaults": {},

        "runtimeScalars": {},
    },

    "policy": "CacheOrNetwork",
    "partial": false
} as const

export default artifact

export type OrganizationProviders = {
	readonly "input": OrganizationProviders$input;
	readonly "result": OrganizationProviders$result | undefined;
};

export type OrganizationProviders$result = {
	readonly providers: {
		readonly edges: ({
			readonly node: {
				/**
				 * Stable provider identity, serialized as a string.
				 */
				readonly id: string;
				/**
				 * Provider display name within its organization.
				 */
				readonly name: string;
			};
		})[];
		readonly pageInfo: {
			readonly endCursor: string | null;
			readonly hasNextPage: boolean;
		};
	};
};

export type OrganizationProviders$input = {
	providersAfter?: string | null;
};

export type OrganizationProviders$unmasked = {
	readonly providers: {
		readonly edges: ({
			readonly node: {
				/**
				 * Stable provider identity, serialized as a string.
				 */
				readonly id: string;
				/**
				 * Provider display name within its organization.
				 */
				readonly name: string;
				readonly __typename: "Provider";
			};
			readonly __typename: "ProviderEdge";
		})[];
		readonly pageInfo: {
			readonly endCursor: string | null;
			readonly hasNextPage: boolean;
			readonly __typename: "PageInfo";
		};
		readonly __typename: "ProviderConnection";
	};
};

export type OrganizationProviders$artifact = typeof artifact

"HoudiniHash=1aa04be4c5e360a18517da1045a0748f9a81866ab170ef58613a97cd44a13a01"
