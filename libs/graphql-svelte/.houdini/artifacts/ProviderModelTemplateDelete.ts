const artifact = {
    "name": "ProviderModelTemplateDelete",
    "kind": "HoudiniMutation",
    "hash": "ca0c724819808d39b255a72cf5ec5c078477af0cd43c998e897bbabc6d9b0ead",
    "raw": `mutation ProviderModelTemplateDelete($modelTemplateDeleteId: ID!, $modelTemplateDeleteProviderTemplateId: ID!) {
    modelTemplateDelete(providerTemplateId: $modelTemplateDeleteProviderTemplateId, id: $modelTemplateDeleteId) {
        node
        __typename
    }
}
`,

    "rootType": "Mutation",
    "stripVariables": [] as Array<string>,

    "selection": {
        "fields": {
            "modelTemplateDelete": {
                "type": "BooleanPayload",
                "keyRaw": "modelTemplateDelete(providerTemplateId: $modelTemplateDeleteProviderTemplateId, id: $modelTemplateDeleteId)",

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
            "modelTemplateDeleteProviderTemplateId": "ID",
            "modelTemplateDeleteId": "ID",
        },

        "types": {},

        "defaults": {},

        "runtimeScalars": {},
    },

} as const

export default artifact

export type ProviderModelTemplateDelete = {
	readonly "input": ProviderModelTemplateDelete$input;
	readonly "result": ProviderModelTemplateDelete$result;
};

export type ProviderModelTemplateDelete$result = {
	readonly modelTemplateDelete: {
		readonly node: boolean;
	};
};

export type ProviderModelTemplateDelete$input = {
	modelTemplateDeleteProviderTemplateId: string;
	modelTemplateDeleteId: string;
};

export type ProviderModelTemplateDelete$optimistic = {
	readonly modelTemplateDelete?: {
		readonly node?: boolean;
	};
};

export type ProviderModelTemplateDelete$unmasked = {
	readonly modelTemplateDelete: {
		readonly node: boolean;
		readonly __typename: "BooleanPayload";
	};
};

export type ProviderModelTemplateDelete$artifact = typeof artifact

"HoudiniHash=ca0c724819808d39b255a72cf5ec5c078477af0cd43c998e897bbabc6d9b0ead"
