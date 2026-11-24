const artifact = {
    "name": "ProviderCatalogEffectiveProviderCatalogs",
    "kind": "HoudiniQuery",
    "hash": "f7868c902eaa954e9b576e5b2b6224d06dcc3a93d59feae8e2eea1dbd64d5764",
    "raw": `query ProviderCatalogEffectiveProviderCatalogs($effectiveProviderCatalogsAfter: String, $effectiveProviderCatalogsSearch: String) {
    effectiveProviderCatalogs(first: 20, after: $effectiveProviderCatalogsAfter, search: $effectiveProviderCatalogsSearch) {
        edges {
            node {
                id
                name
                baseUrl
                logoUrl
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
            "effectiveProviderCatalogs": {
                "type": "ProviderCatalogConnection",
                "keyRaw": "effectiveProviderCatalogs(first: 20, after: $effectiveProviderCatalogsAfter, search: $effectiveProviderCatalogsSearch)",

                "selection": {
                    "fields": {
                        "edges": {
                            "type": "ProviderCatalogEdge",
                            "keyRaw": "edges",

                            "selection": {
                                "fields": {
                                    "node": {
                                        "type": "ProviderCatalog",
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
            "effectiveProviderCatalogsAfter": "String",
            "effectiveProviderCatalogsSearch": "String",
        },

        "types": {},

        "defaults": {},

        "runtimeScalars": {},
    },

    "policy": "CacheOrNetwork",
    "partial": false
} as const

export default artifact

export type ProviderCatalogEffectiveProviderCatalogs = {
	readonly "input": ProviderCatalogEffectiveProviderCatalogs$input;
	readonly "result": ProviderCatalogEffectiveProviderCatalogs$result | undefined;
};

export type ProviderCatalogEffectiveProviderCatalogs$result = {
	readonly effectiveProviderCatalogs: {
		readonly edges: ({
			readonly node: {
				readonly id: string;
				readonly name: string;
				readonly baseUrl: string | null;
				readonly logoUrl: string | null;
			};
		})[];
		readonly pageInfo: {
			readonly endCursor: string | null;
			readonly hasNextPage: boolean;
		};
	};
};

export type ProviderCatalogEffectiveProviderCatalogs$input = {
	effectiveProviderCatalogsAfter?: string | null;
	effectiveProviderCatalogsSearch?: string | null;
};

export type ProviderCatalogEffectiveProviderCatalogs$unmasked = {
	readonly effectiveProviderCatalogs: {
		readonly edges: ({
			readonly node: {
				readonly id: string;
				readonly name: string;
				readonly baseUrl: string | null;
				readonly logoUrl: string | null;
				readonly __typename: "ProviderCatalog";
			};
			readonly __typename: "ProviderCatalogEdge";
		})[];
		readonly pageInfo: {
			readonly endCursor: string | null;
			readonly hasNextPage: boolean;
			readonly __typename: "PageInfo";
		};
		readonly __typename: "ProviderCatalogConnection";
	};
};

export type ProviderCatalogEffectiveProviderCatalogs$artifact = typeof artifact

"HoudiniHash=f7868c902eaa954e9b576e5b2b6224d06dcc3a93d59feae8e2eea1dbd64d5764"
