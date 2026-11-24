const artifact = {
    "name": "ProviderModelCatalogDelete",
    "kind": "HoudiniMutation",
    "hash": "4af038d46c02120eeb70e21890d8e2617d5ee3782a9e78003d40d5ed39f70644",
    "raw": `mutation ProviderModelCatalogDelete($modelCatalogDeleteId: ID!, $modelCatalogDeleteProviderCatalogId: ID!) {
    modelCatalogDelete(providerCatalogId: $modelCatalogDeleteProviderCatalogId, id: $modelCatalogDeleteId) {
        node
        __typename
    }
}
`,

    "rootType": "Mutation",
    "stripVariables": [] as Array<string>,

    "selection": {
        "fields": {
            "modelCatalogDelete": {
                "type": "BooleanPayload",
                "keyRaw": "modelCatalogDelete(providerCatalogId: $modelCatalogDeleteProviderCatalogId, id: $modelCatalogDeleteId)",

                "selection": {
                    "fields": {
                        "node": {
                            "type": "Boolean",
                            "keyRaw": "node",
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
            "modelCatalogDeleteProviderCatalogId": "ID",
            "modelCatalogDeleteId": "ID",
        },

        "types": {},

        "defaults": {},

        "runtimeScalars": {},
    },

} as const

export default artifact

export type ProviderModelCatalogDelete = {
	readonly "input": ProviderModelCatalogDelete$input;
	readonly "result": ProviderModelCatalogDelete$result;
};

export type ProviderModelCatalogDelete$result = {
	readonly modelCatalogDelete: {
		readonly node: boolean;
	};
};

export type ProviderModelCatalogDelete$input = {
	modelCatalogDeleteProviderCatalogId: string;
	modelCatalogDeleteId: string;
};

export type ProviderModelCatalogDelete$optimistic = {
	readonly modelCatalogDelete?: {
		readonly node?: boolean;
	};
};

export type ProviderModelCatalogDelete$unmasked = {
	readonly modelCatalogDelete: {
		readonly node: boolean;
		readonly __typename: "BooleanPayload";
	};
};

export type ProviderModelCatalogDelete$artifact = typeof artifact

"HoudiniHash=4af038d46c02120eeb70e21890d8e2617d5ee3782a9e78003d40d5ed39f70644"
