const artifact = {
    "name": "ProviderCatalogEffectiveProviderTemplates",
    "kind": "HoudiniQuery",
    "hash": "d8cf8d5f23d8dfa281a6d16209bcc2aafda93687c0c6b53f559c2cc11e1c9ecf",
    "raw": `query ProviderCatalogEffectiveProviderTemplates($effectiveProviderTemplatesAfter: String, $effectiveProviderTemplatesSearch: String) {
    effectiveProviderTemplates(first: 20, after: $effectiveProviderTemplatesAfter, search: $effectiveProviderTemplatesSearch) {
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
            "effectiveProviderTemplates": {
                "type": "ProviderTemplateConnection",
                "keyRaw": "effectiveProviderTemplates(first: 20, after: $effectiveProviderTemplatesAfter, search: $effectiveProviderTemplatesSearch)",

                "selection": {
                    "fields": {
                        "edges": {
                            "type": "ProviderTemplateEdge",
                            "keyRaw": "edges",

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
            "effectiveProviderTemplatesAfter": "String",
            "effectiveProviderTemplatesSearch": "String",
        },

        "types": {},

        "defaults": {},

        "runtimeScalars": {},
    },

    "policy": "CacheOrNetwork",
    "partial": false
} as const

export default artifact

export type ProviderCatalogEffectiveProviderTemplates = {
	readonly "input": ProviderCatalogEffectiveProviderTemplates$input;
	readonly "result": ProviderCatalogEffectiveProviderTemplates$result | undefined;
};

export type ProviderCatalogEffectiveProviderTemplates$result = {
	readonly effectiveProviderTemplates: {
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

export type ProviderCatalogEffectiveProviderTemplates$input = {
	effectiveProviderTemplatesAfter?: string | null;
	effectiveProviderTemplatesSearch?: string | null;
};

export type ProviderCatalogEffectiveProviderTemplates$unmasked = {
	readonly effectiveProviderTemplates: {
		readonly edges: ({
			readonly node: {
				readonly id: string;
				readonly name: string;
				readonly baseUrl: string | null;
				readonly logoUrl: string | null;
				readonly __typename: "ProviderTemplate";
			};
			readonly __typename: "ProviderTemplateEdge";
		})[];
		readonly pageInfo: {
			readonly endCursor: string | null;
			readonly hasNextPage: boolean;
			readonly __typename: "PageInfo";
		};
		readonly __typename: "ProviderTemplateConnection";
	};
};

export type ProviderCatalogEffectiveProviderTemplates$artifact = typeof artifact

"HoudiniHash=d8cf8d5f23d8dfa281a6d16209bcc2aafda93687c0c6b53f559c2cc11e1c9ecf"
